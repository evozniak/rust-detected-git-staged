use git2::{Repository, StatusOptions};

fn print_staged() -> Result<(), git2::Error> {
    let git_repo = Repository::discover(".")?;
    let mut status_opts = StatusOptions::new();
    status_opts.include_untracked(false);
    let statuses = git_repo.statuses(Some(&mut status_opts))?;
    for file_status in statuses.iter().filter(|e| e.status().is_index_modified() || e.status().is_index_new()) {
        if let Some(path) = file_status.path() {
            println!("{}", git_repo.workdir().unwrap().join(path).display());
        }
    }
    Ok(())
}

fn main() {
    print_staged().ok();
}
