use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// https://docs.github.com/en/webhooks-and-events/events/github-event-types
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum GithubEvents {
    /// Activity related to an issue or pull request comment.
    ///
    /// https://docs.github.com/en/rest/using-the-rest-api/github-event-types?apiVersion=2022-11-28#issuesevent
    IssueCommentEvent(Value),

    /// Activity related to an issue.
    ///
    /// https://docs.github.com/en/rest/using-the-rest-api/github-event-types?apiVersion=2022-11-28#issuesevent
    IssuesEvent(Value),

    /// When a private repository is made public.
    ///
    /// https://docs.github.com/en/rest/using-the-rest-api/github-event-types?apiVersion=2022-11-28#publicevent
    PublicEvent(Value),

    /// Activity related to pull requests.
    ///
    /// https://docs.github.com/en/rest/using-the-rest-api/github-event-types?apiVersion=2022-11-28#pullrequestevent
    PullRequestEvent(Value),

    /// Activity related to a release
    ///
    /// https://docs.github.com/en/rest/using-the-rest-api/github-event-types?apiVersion=2022-11-28#releaseevent
    ReleaseEvent(Value),

    /// Activity related to a sponsorship listing.
    ///
    /// https://docs.github.com/en/rest/using-the-rest-api/github-event-types?apiVersion=2022-11-28#sponsorshipevent
    SponsorshipEvent(Value),

    #[serde(other)]
    Others,
}

impl GithubEvents {
    pub fn is_public_event(&self) -> bool {
        match self {
            GithubEvents::IssueCommentEvent(v) => v["public"].as_bool().unwrap_or(false),
            GithubEvents::IssuesEvent(v) => v["public"].as_bool().unwrap_or(false),
            GithubEvents::PublicEvent(v) => v["public"].as_bool().unwrap_or(false),
            GithubEvents::PullRequestEvent(v) => v["public"].as_bool().unwrap_or(false),
            GithubEvents::ReleaseEvent(v) => v["public"].as_bool().unwrap_or(false),
            GithubEvents::SponsorshipEvent(v) => v["public"].as_bool().unwrap_or(false),
            GithubEvents::Others => false,
        }
    }
}

// Bulk of formatting here lmao
impl Display for GithubEvents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GithubEvents::IssueCommentEvent(v)
                if v["payload"]["action"].as_str().unwrap() == "created" =>
            {
                write!(
                    f,
                    "🗣 Commented on #{} in ${}",
                    v["payload"]["issue"]["number"].as_str().unwrap(),
                    v["repo"]["name"].as_str().unwrap(),
                )
            }

            GithubEvents::IssuesEvent(v)
                if v["payload"]["action"].as_str().unwrap() == "opened" =>
            {
                write!(
                    f,
                    "❗️ Opened issue #{} in ${}",
                    v["payload"]["issue"]["number"].as_str().unwrap(),
                    v["repo"]["name"].as_str().unwrap(),
                )
            }
            GithubEvents::IssuesEvent(v)
                if v["payload"]["action"].as_str().unwrap() == "closed" =>
            {
                write!(
                    f,
                    "❗️ Closed issue #{} in ${}",
                    v["payload"]["issue"]["number"].as_str().unwrap(),
                    v["repo"]["name"].as_str().unwrap(),
                )
            }
            GithubEvents::IssuesEvent(v)
                if v["payload"]["action"].as_str().unwrap() == "reopened" =>
            {
                write!(
                    f,
                    "❗️ Reopened issue #{} in ${}",
                    v["payload"]["issue"]["number"].as_str().unwrap(),
                    v["repo"]["name"].as_str().unwrap(),
                )
            }

            GithubEvents::PublicEvent(v) => {
                write!(f, "🔓 Made {} public!", v["repo"]["name"].as_str().unwrap())
            }

            GithubEvents::PullRequestEvent(v)
                if v["payload"]["action"].as_str().unwrap() == "opened"
                    || v["payload"]["action"].as_str().unwrap() == "reopened" =>
            {
                write!(
                    f,
                    "📝 Opened PR #{} in {}",
                    v["payload"]["number"].as_number().unwrap(),
                    v["repo"]["name"].as_str().unwrap()
                )
            }
            GithubEvents::PullRequestEvent(v)
                if v["payload"]["action"].as_str().unwrap() == "closed" =>
            {
                write!(
                    f,
                    "🛑 Closed PR #{} in {}",
                    v["payload"]["number"].as_number().unwrap(),
                    v["repo"]["name"].as_str().unwrap()
                )
            }

            GithubEvents::ReleaseEvent(v) => write!(
                f,
                "🚀 Shipped a new version of {}",
                v["repo"]["name"].as_str().unwrap()
            ),
            GithubEvents::SponsorshipEvent(_) => write!(f, "🤝 Started sponsoring a developer!"),

            _ => Ok(()),
        }
    }
}
