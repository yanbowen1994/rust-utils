pub const COMMIT_ID: &str = include_str!(concat!(env!("OUT_DIR"), "/git-commit-id.txt"));

pub const COMMIT_DATE: &str = include_str!(concat!(env!("OUT_DIR"), "/git-commit-date.txt"));

pub const BUILD_DATE: &str = include_str!(concat!(env!("OUT_DIR"), "/build_date.txt"));

pub const BRANCH: &str = include_str!(concat!(env!("OUT_DIR"), "/branch.txt"));

pub fn get_version_info() -> String {
    let json = serde_json::json!({
        "Build date": BUILD_DATE,
        "Commit date": COMMIT_DATE,
        "Commit id": COMMIT_ID,
        "Branch": BRANCH,
    });
    json.to_string()
}

pub fn version_info_to_log() {
    log::info!("{}", get_version_info());
}

#[test]
fn print_version() {
    println!("{}", get_version_info());
}
