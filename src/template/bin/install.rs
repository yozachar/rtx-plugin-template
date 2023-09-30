use crate::template::lib::utils::install_version;
use std::env;

pub fn install(tool_name: &str) {
    let rtx_install_type = env::var("RTX_INSTALL_TYPE").expect("RTX_INSTALL_TYPE is not set");
    let rtx_install_path = env::var("RTX_INSTALL_PATH").expect("RTX_INSTALL_PATH is not set");
    let rtx_install_version =
        env::var("RTX_INSTALL_VERSION").expect("RTX_INSTALL_VERSION is not set");
    install_version(
        &tool_name,
        &rtx_install_type,
        &rtx_install_version,
        &rtx_install_path,
    )
}
