use crate::template::lib::utils::list_all_versions;

pub fn list_all(gh_repo: &str) -> Vec<String> {
    list_all_versions(gh_repo)
}
