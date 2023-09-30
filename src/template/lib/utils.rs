use fs_extra::dir::{copy as copy_dir, CopyOptions};
use regex::Regex;
use std::{
    env, fs,
    io::{BufRead, BufReader},
    path::Path,
    process::{Command, Stdio},
    str,
};

pub fn get_os() -> &'static str {
    let os = env::consts::OS;
    match os {
        "darwin" => "Darwin",
        "linux" => "Linux",
        "windows" => "Windows",
        _ => panic!("{os} is unsupported"),
    }
}

pub fn get_arch() -> &'static str {
    let arch = env::consts::ARCH;
    match arch {
        "x86_64" => "x86_64",
        "arm64" => "arm64",
        "i386" => "i386",
        _ => panic!("Architecture {arch} is unsupported"),
    }
}

// pub fn sort_versions(gh_repo: &str) -> Vec<String> {
//     return list_github_tags(gh_repo);
// }

fn list_github_tags(gh_repo: &str) -> Vec<String> {
    let git_process = Command::new("git")
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
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute 'git'")
        .wait_with_output()
        .expect("Failed to wait on git");
    let re = Regex::new(r"v(\d+\.\d+\.\d+)").unwrap();
    let mut versions: Vec<String> = Vec::new();
    let git_output =
        str::from_utf8(&git_process.stdout).expect("Invalid UTF-8 in response from git");
    for cap in re.captures_iter(git_output) {
        versions.push(cap[1].to_string());
    }
    versions
}

pub fn list_all_versions(gh_repo: &str) -> Vec<String> {
    list_github_tags(gh_repo)
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
        if !gh_api_token.is_empty() {
            auth = format!("-H Authorization: token {gh_api_token}");
            curl_opts.insert(1, &auth);
        }
    }
    println!("* Downloading {tool_name} release {version} ...");
    // println!("* With {auth} from {url}");
    let curl_process = Command::new("curl")
        .args(curl_opts)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute curl")
        .stdout
        .expect("Failed to get curl output");
    _ = auth;
    let curl_op_reader = BufReader::new(curl_process);
    curl_op_reader
        .lines()
        .map_while(Result::ok)
        .for_each(|line| println!("{}", line));
}

fn test_windows_exec(tool_path: &str, tool_check: &[&str]) -> bool {
    /*
        icacls "file.exe" /grant "BUILTIN\Users":RX
    */
    _ = tool_path;
    _ = tool_check;
    unimplemented!();
}

fn test_nix_exec(tool_path: &str, tool_check: &[&str]) -> bool {
    if !Command::new("chmod")
        .args(["u+x", tool_path])
        .output()
        .expect("Failed to execute chmod")
        .status
        .success()
    {
        return false;
    }
    if !Command::new("test")
        .args(["-x", tool_path])
        .output()
        .expect("Failed to execute test")
        .status
        .success()
    {
        return false;
    }

    if !Command::new(tool_check[0])
        .args(&tool_check[1..])
        .output()
        .unwrap_or_else(|_| panic!("Failed to execute {}", tool_check[0]))
        .status
        .success()
    {
        return false;
    }
    true
}

pub fn install_version(tool_name: &str, install_type: &str, version: &str, install_path: &str) {
    if install_type != "version" {
        panic!("rtx-{tool_name} supports release installs only")
    }

    // install
    let rtx_download_path_str =
        env::var("RTX_DOWNLOAD_PATH").expect("RTX_DOWNLOAD_PATH is not set");
    // rtx_download_path_str = format!("{rtx_download_path_str}/*");
    let install_src = Path::new(&rtx_download_path_str);
    let install_dest_str = if !(install_path.ends_with("/bin") || install_path.ends_with("/bin/")) {
        format!("{install_path}/bin")
    } else {
        install_path.to_string()
    };
    let install_dest = Path::new(&install_dest_str);

    fs::create_dir_all(install_dest)
        .unwrap_or_else(|_| panic!("Unable to create directory at {install_dest_str}"));
    let cpy_options = CopyOptions::new().overwrite(true).content_only(true);
    copy_dir(install_src, install_dest, &cpy_options).expect("Failed to copy directory");

    // test
    let tool_check = vec![tool_name, "--version"];
    let test_exec = if get_os() == "Windows" {
        test_windows_exec(&format!("{install_dest_str}/{tool_name}"), &tool_check)
    } else {
        test_nix_exec(&format!("{install_dest_str}/{tool_name}"), &tool_check)
    };

    if !test_exec {
        fs::remove_dir_all(install_dest).expect("Failed to remove directory");
        panic!(
            "Expected {install_dest_str}/{} to be executable",
            tool_check[0]
        );
    }

    println!("Installed {tool_name} {version} successfully!")
}
