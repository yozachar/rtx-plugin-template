use crate::template::lib::utils::list_all_versions;
use std::{
    env,
    process::{Command, Stdio},
    str,
};

pub fn latest_stable(gh_repo: &str) -> String {
    let mut auth = String::from("");
    let mut curl_opts = vec!["-sI"];
    if let Ok(gh_api_token) = env::var("GITHUB_API_TOKEN") {
        if !gh_api_token.is_empty() {
            auth = format!("-H Authorization: token {gh_api_token}");
            curl_opts.push(&auth);
        }
    }
    // curl of REPO/releases/latest is expected to be a 302 to another URL
    // when no releases redirect_url="REPO/releases"
    // when there are releases redirect_url="REPO/releases/tag/v<VERSION>"
    let mut redirect_url = format!("{gh_repo}/releases/latest");
    curl_opts.push(&redirect_url);
    let curl_process = Command::new("curl")
        .args(curl_opts)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute curl")
        .wait_with_output()
        .expect("Failed to wait on curl");
    let curl_response =
        str::from_utf8(&curl_process.stdout).expect("Invalid UTF-8 in response from curl");
    _ = auth;
    redirect_url = curl_response
        .lines()
        .find(|line| line.starts_with("location: "))
        .map(|line| line.trim_start_matches("location: ").to_string())
        .expect("Failed to find redirection URL.");

    println!("redirect url: {redirect_url}");
    return if format!("{gh_repo}/releases/latest") == redirect_url {
        list_all_versions(gh_repo)[0].clone()
    } else {
        redirect_url
            .trim_start_matches(&format!("{gh_repo}/releases/tag/v"))
            .to_string()
    };
}
