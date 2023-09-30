use crate::template::lib::utils::download_release;
use std::{env, fs::create_dir_all, path::Path};

pub fn download(gh_repo: &str, tool_name: &str) {
    let rtx_download_path = env::var("RTX_DOWNLOAD_PATH").expect("RTX_DOWNLOAD_PATH is not set");
    create_dir_all(Path::new(&rtx_download_path)).expect(&format!(
        "Unable to create directory at {rtx_download_path}"
    ));
    let rtx_install_version =
        env::var("RTX_INSTALL_VERSION").expect("RTX_INSTALL_VERSION is not set");
    let release_file = format!("{rtx_download_path}/{tool_name}");
    download_release(&gh_repo, &tool_name, &rtx_install_version, &release_file)
}
