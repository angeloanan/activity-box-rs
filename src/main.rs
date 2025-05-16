pub mod gist_box;
pub mod structs;
pub mod utils;

use std::env;
use structs::GithubEvents;
use tracing::{debug, info};
use utils::truncate_string;

/// Up to 300 events made in the last 30 days will be included
/// https://docs.github.com/en/rest/activity/events?apiVersion=2022-11-28#about-github-events
const MAX_PAGE: u16 = 3;

fn main() {
    tracing_subscriber::fmt::init();

    debug!("Starting...");

    debug!("Parsing environment variables");
    dotenvy::dotenv()
        .inspect(|path| debug!("Loaded .env file from {path:?}"))
        .ok();

    // Extract GIST_ID, GH_USERNAME and GH_PAT from environment variables
    let gist_id = env::var("GIST_ID").expect("Env var `GIST_ID` must be set");
    let gh_username = env::var("GH_USERNAME").expect("Env var `GH_USERNAME` must be set");
    let gh_pat = env::var("GH_PAT").expect("Env var `GH_PAT` must be set");

    debug!("Finished parsing environment variables");
    debug!("Getting activity for {}", &gh_username);

    let mut activities = Vec::new();
    for page in 1..MAX_PAGE {
        let paged_activity = ureq::get(format!(
            "https://api.github.com/users/{gh_username}/events/public?per_page=100&page={page}",
        ))
        .header("accept", "application/vnd.github+json")
        .call()
        .expect("Failed to get public events")
        .body_mut()
        .read_json::<Vec<GithubEvents>>()
        .expect("Failed to parse public events");
        activities.extend_from_slice(&paged_activity)
    }

    info!("Found {} activities", activities.len());

    let interesting_activities = activities
        .iter()
        .filter(|a| a.is_public_event())
        // Trial and error length
        .map(|s| truncate_string(s, 63))
        .filter(|s| !s.is_empty())
        .take(5)
        .collect::<Vec<String>>();

    info!(
        "Found {} interesting activities",
        &interesting_activities.len()
    );

    for a in &interesting_activities {
        println!("{a}");
    }

    let seralized_acts = if interesting_activities.is_empty() {
        "☕ No activities recently...".into()
    } else {
        interesting_activities.join("\n")
    };

    gist_box::update_gist_if_changed(gh_pat.as_str(), gist_id.as_str(), &seralized_acts);

    info!("Finished updating gist!");
}
