pub mod gist_box;
pub mod utils;

use std::env;
use std::fmt::Write;
use tracing::{debug, info};
use tracing_subscriber::prelude::*;

struct GithubInterestingActivities {
    event_name: &'static str,
    event_display: &'static str,
}

const GITHUB_PUBLIC_EVENTS_URL: &str =
    "https://api.github.com/users/{username}/events/public?per_page=100";

// https://docs.github.com/en/webhooks-and-events/events/github-event-types
const INTERESTING_EVENTS: [GithubInterestingActivities; 5] = [
    GithubInterestingActivities {
        event_name: "IssueCommentEvent",
        event_display: "🗣 Commented on #{IssueNumber} in {RepositoryName}",
    },
    GithubInterestingActivities {
        event_name: "IssuesEvent",
        event_display: "❗️ {Action} issue #{IssueNumber} in {RepositoryName}",
    },
    GithubInterestingActivities {
        event_name: "PullRequestevent",
        event_display: "{Emoji} {Action} PR #{PRNumber} in {RepositoryName}",
    },
    GithubInterestingActivities {
        event_name: "PublicEvent",
        event_display: "🔓 Made {RepositoryName} public",
    },
    GithubInterestingActivities {
        event_name: "ReleaseEvent",
        event_display: "🚀 Released version {Version} of {RepositoryName}",
    },
];

fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    debug!("Starting...");

    debug!("Parsing environment variables");
    dotenvy::dotenv()
        .and_then(|path| {
            debug!("Loaded .env file");
            Ok(path)
        })
        .ok();

    // Extract GIST_ID, GH_USERNAME and GH_PAT from environment variables
    let gist_id = env::var("GIST_ID").expect("Env var `GIST_ID` must be set");
    let gh_username = env::var("GH_USERNAME").expect("Env var `GH_USERNAME` must be set");
    let gh_pat = env::var("GH_PAT").expect("Env var `GH_PAT` must be set");
    let max_activities = env::var("MAX_ACTIVITIES").unwrap_or("5".to_string());
    let max_activities_count = max_activities
        .parse::<usize>()
        .expect("Env var `MAX_ACTIVITIES` must be a number");

    debug!("Finished parsing environment variables");
    debug!("Getting activity for {}", &gh_username);

    let public_event_url = GITHUB_PUBLIC_EVENTS_URL.replace("{username}", &gh_username);

    let activities = ureq::get(&public_event_url)
        .set("accept", "application/vnd.github+json")
        .call()
        .expect("Failed to get public events")
        .into_json::<Vec<serde_json::Value>>()
        .expect("Failed to parse public events");

    info!("Found {} activities", &activities.len());

    // Filter out events that are not interesting
    let interesting_activities: Vec<serde_json::Value> = activities
        .into_iter()
        .filter(|activity| {
            INTERESTING_EVENTS
                .iter()
                .any(|interesting_event| activity["type"] == *interesting_event.event_name)
        })
        .collect();

    info!(
        "Found {} interesting activities",
        &interesting_activities.len()
    );

    let mut serialized_activities = String::new();

    if interesting_activities.is_empty() {
        writeln!(serialized_activities, "No activities recently...").unwrap();
    } else {
        for activity in interesting_activities.iter().take(max_activities_count) {
            let event = INTERESTING_EVENTS
                .iter()
                .find(|interesting_event| activity["type"] == interesting_event.event_name)
                .unwrap();
            let mut text_display = event.event_display.to_string();

            if text_display.contains("{IssueNumber}") {
                text_display = text_display.replace(
                    "{IssueNumber}",
                    activity["payload"]["issue"]["number"].as_str().unwrap(),
                );
            }

            if text_display.contains("{PRNumber}") {
                text_display = text_display.replace(
                    "{PRNumber}",
                    activity["payload"]["pull_request"]["number"]
                        .as_str()
                        .unwrap(),
                );
            }

            if text_display.contains("{Action}") {
                text_display = text_display
                    .replace("{Action}", activity["payload"]["action"].as_str().unwrap());
            }

            if text_display.contains("{Emoji}") {
                let emoji = match activity["payload"]["action"].as_str().unwrap() {
                    "opened" => "📝",
                    "closed" => "🗑",
                    "reopened" => "🔓",
                    _ => "❓",
                };
                text_display = text_display.replace("{Emoji}", emoji);
            }

            if text_display.contains("{Version}") {
                text_display = text_display.replace(
                    "{Version}",
                    activity["payload"]["release"]["tag_name"].as_str().unwrap(),
                );
            }

            if let true = text_display.contains("{RepositoryName}") {
                text_display = text_display.replace(
                    "{RepositoryName}",
                    activity["repo"]["name"].as_str().unwrap(),
                );
            }

            writeln!(
                &mut serialized_activities,
                "{}",
                utils::truncate_string(
                    text_display.as_str(),
                    usize::try_from(gist_box::MAX_LENGTH).unwrap()
                )
            )
            .unwrap();
        }
    }

    debug!("Finished serializing activities");

    gist_box::update_gist(
        gist_id.as_str(),
        serialized_activities.as_str(),
        gh_pat.as_str(),
    )
    .unwrap();

    info!("Finished updating gist");
}
