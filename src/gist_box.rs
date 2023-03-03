pub const MAX_LENGTH: i32 = 63;
pub const MAX_LINES: i32 = 5;

pub const MAX_IMAGE_WIDTH: i32 = 325;
pub const MAX_IMAGE_HEIGHT: i32 = 100;

pub const GITHUB_API_GIST_URL: &str = "https://api.github.com/gists/{gist_id}";

#[tracing::instrument]
pub fn get_gist(gist_id: &str) -> serde_json::Value {
    let gist_url = GITHUB_API_GIST_URL.replace("{gist_id}", gist_id);

    let gist = ureq::get(&gist_url)
        .set("accept", "application/vnd.github+json")
        .call()
        .expect("Failed to get gist");

    gist.into_json::<serde_json::Value>()
        .expect("Failed to parse gist data")
}

#[tracing::instrument]
pub fn update_gist(
    gist_id: &str,
    gist_content: &str,
    github_token: &str,
) -> Result<ureq::Response, ureq::Error> {
    let gist_url = GITHUB_API_GIST_URL.replace("{gist_id}", gist_id);

    ureq::patch(&gist_url)
        .set("accept", "application/vnd.github+json")
        .set("authorization", &format!("Bearer {github_token}"))
        .send_json(ureq::json!({
            "gist_id": gist_id,
            "files": {
                "gist": gist_content
            }
        }))
}
