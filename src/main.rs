mod template {
    pub mod lib {
        pub mod utils;
    }
}

use template::lib::utils::{
    download_release, get_arch, get_os, install_version, list_all_versions,
};

fn main() {
    static GH_REPO: &str = "https://github.com/marcosnils/bin";
    static TOOL_NAME: &str = "bin";

    println!("{GH_REPO} : {TOOL_NAME} : {} : {}", get_os(), get_arch());
    let version_list = list_all_versions(GH_REPO);
    for version in version_list.iter() {
        print!("{version} ")
    }
    println!();
    download_release(GH_REPO, TOOL_NAME, &version_list[0], "bin");
    install_version(TOOL_NAME, "version", &version_list[0], "./path");
}
