use errors::*;
use run;
use std::fs;
use std::path::Path;

pub fn shallow_clone_or_pull(url: &str, dir: &Path) -> Result<()> {
    let url = frob_url(url);

    if !dir.exists() {
        info!("cloning {} into {}", url, dir.display());
        let r = run::run(
            "git",
            &["clone", "--depth", "1", &url, &dir.to_string_lossy()],
            &[],
        ).chain_err(|| format!("unable to clone {}", url));

        if r.is_err() && dir.exists() {
            fs::remove_dir_all(dir)?;
        }

        r
    } else {
        info!("pulling existing url {} into {}", url, dir.display());
        run::cd_run(dir, "git", &["pull"], &[]).chain_err(|| format!("unable to pull {}", url))
    }
}

pub fn reset_to_sha(dir: &Path, sha: &str) -> Result<()> {
    run::cd_run(dir, "git", &["reset", "--hard", sha], &[])
        .chain_err(|| format!("unable to reset {} to {}", dir.display(), sha))
}

fn frob_url(url: &str) -> String {
    // With https git will interactively ask for a password for private repos.
    // Switch to the unauthenticated git protocol to just generate an error instead.
    url.replace("https://", "git://")
}
