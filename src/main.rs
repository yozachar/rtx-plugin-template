mod template {
    pub mod lib {
        pub mod utils;
    }
    pub mod bin {
        pub mod download;
        pub mod install;
        pub mod latest_stable;
        pub mod list_all;
    }
}

use template::lib::utils::get_arch;

use template::bin::{
    download::download, install::install, latest_stable::latest_stable, list_all::list_all,
};

fn main() {
    static GH_REPO: &str = "https://github.com/marcosnils/bin";
    static TOOL_NAME: &str = "bin";

    println!("{}", get_arch());

    println!("{}", latest_stable(GH_REPO));
    for version in list_all(GH_REPO).iter() {
        print!("{version} ")
    }
    println!();
    download(GH_REPO, TOOL_NAME);
    install(TOOL_NAME);
}
