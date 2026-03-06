use crate::{collect, data::Collected, shell};
use std::path::Path;

pub fn get_current_repo_name() -> String {
    let repo_out = shell::capture_cmd_lossy("git rev-parse --show-toplevel");
    let root_path = Path::new(repo_out.trim());
    let repo_name_os = root_path.file_stem().expect("Could not find root repo.");
    let repo_name = repo_name_os.to_str().expect("Could not parse repo root.");
    repo_name.to_string()
}

pub fn collect_git_log_data() -> Collected {
    // Get all the commits
    let cmd = "git log --format=\"%at %an <%ae>\"";
    let out = shell::capture_cmd_lossy(cmd);

    // Collect data as timestamps per author
    let collected = collect::collect_by_author(&out);

    collected
}
