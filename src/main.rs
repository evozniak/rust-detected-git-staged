use git2::{self, Diff};

fn print_staged_files(diff: &Diff) {
    for delta in diff.deltas() {
        let path = delta.new_file().path().unwrap_or_else(|| delta.old_file().path().unwrap());
        println!("{}", path.display());
    }
}

fn main() {
    let git_repo = git2::Repository::open_from_env().unwrap();
    let read_staged_files = git_repo.diff_tree_to_index(None, None, None);
    let diff = match &read_staged_files {
        diff=> diff.as_ref().unwrap()
    };
    print_staged_files(diff);
}
