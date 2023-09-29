use fs_extra::dir::{copy as copy_dir, CopyOptions};
use regex::Regex;
use std::fs::remove_dir_all;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};
use std::{env, fs};

pub fn get_os() -> &'static str {
    let os = env::consts::OS;
    return match os {
        "darwin" => "Darwin",
        "linux" => "Linux",
        "windows" => "Windows",
        _ => panic!("{os} is unsupported"),
    };
}

pub fn get_arch() -> &'static str {
    let arch = env::consts::ARCH;
    return match arch {
        "x86_64" => "x86_64",
        "arm64" => "arm64",
        "i386" => "i386",
        _ => panic!("Architecture {arch} is unsupported"),
    };
}

// pub fn sort_versions(gh_repo: &str) -> Vec<String> {
//     return list_github_tags(gh_repo);
// }

fn list_github_tags(gh_repo: &str) -> Vec<String> {
    let git_ls_remote = Command::new("git")
        .args([
            "ls-remote",
            "--sort",
            // - sign to sort reverse order,
            // ie. here, newest to oldest
            "-version:refname",
            "--tags",
            "--refs",
            gh_repo,
        ])
        .output();
    let re = Regex::new(r"v(\d+\.\d+\.\d+)").unwrap();
    let mut versions = Vec::new();
    let output = String::from_utf8(git_ls_remote.unwrap().stdout).unwrap();
    for cap in re.captures_iter(&output) {
        versions.push(cap[1].to_string());
    }
    return versions;
}

pub fn list_all_versions(gh_repo: &str) -> Vec<String> {
    return list_github_tags(gh_repo);
}

pub fn download_release(gh_repo: &str, tool_name: &str, version: &str, filename: &str) {
    let url = format!(
        "{gh_repo}/releases/download/v{version}/bin_{version}_{}_{}",
        get_os(),
        get_arch()
    );
    let mut auth = String::from("");
    let mut curl_opts = vec!["-fsSL", "-o", filename, "-C", "-", &url];
    if let Ok(gh_api_token) = env::var("GITHUB_API_TOKEN") {
        if gh_api_token != "" {
            auth = format!("-H Authorization: token {gh_api_token}");
            curl_opts.insert(1, &auth);
        }
    }
    println!("* Downloading {tool_name} release {version}...");
    // println!("* With {auth} from {url}");
    let stdout = Command::new("curl")
        .args(curl_opts)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .stdout
        .unwrap();

    let reader = BufReader::new(stdout);
    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("{}", line));
    _ = auth;
}

pub fn install_version(tool_name: &str, install_type: &str, version: &str, install_path: &str) {
    if install_type != "version" {
        panic!("rtx-{tool_name} supports release installs only")
    }

    // install
    let mut rtx_download_path_str = env::var("RTX_DOWNLOAD_PATH").unwrap();
    rtx_download_path_str = format!("{rtx_download_path_str}/*");
    let install_src = Path::new(&rtx_download_path_str);
    let install_dest_str = if !install_path.ends_with("/bin") {
        format!("{install_path}/bin")
    } else {
        install_path.to_string()
    };
    let install_dest = Path::new(&install_dest_str);

    fs::create_dir(install_dest).unwrap();
    let cpy_options = CopyOptions::new();
    copy_dir(install_src, install_dest, &cpy_options).unwrap();
    // chmod

    // test
    let tool_check = vec![tool_name, "--version"];
    if !Command::new(tool_check[0])
        .args([tool_check[1]])
        .output()
        .unwrap()
        .status
        .success()
    {
        remove_dir_all(install_dest).unwrap();
        println!(
            "Expected {install_dest_str}/{} to be executable.",
            tool_check[0]
        );
    }

    println!("Installed {tool_name} {version} successfully!")
}
