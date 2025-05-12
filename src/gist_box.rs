use serde_json::json;

#[tracing::instrument]
pub fn get_gist(gist_id: &str) -> serde_json::Value {
    let gist_url = format!("https://api.github.com/gists/{gist_id}");

    ureq::get(&gist_url)
        .header("Accept", "application/vnd.github.raw+json")
        .call()
        .expect("Failed to get gist")
        .body_mut()
        .read_json::<serde_json::Value>()
        .expect("Failed to parse gist data")
}

#[tracing::instrument(skip(github_token, gist_content))]
pub fn update_gist_if_changed(github_token: &str, gist_id: &str, gist_content: &str) {
    let gist_url = format!("https://api.github.com/gists/{gist_id}");

    let gist = get_gist(gist_id);
    let current_gist_content = gist["files"]["events.md"]["content"].as_str().unwrap_or("");
    if gist_content == current_gist_content {
        return;
    }

    ureq::patch(&gist_url)
        .header("Accept", "application/vnd.github.raw+json")
        .header("Authorization", format!("Bearer {github_token}"))
        .send_json(json!({
            "files": {
                "events.md": { "content": gist_content }
            }
        }))
        .expect("Unable to update gist");
}
