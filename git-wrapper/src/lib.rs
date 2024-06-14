use std::process::{Command, Output};

pub fn git_pull() -> anyhow::Result<Output> {
    let child = Command::new("git").arg("pull").arg("--rebase").output()?;
    Ok(child)
}

pub fn git_push() -> anyhow::Result<Output> {
    let child = Command::new("git").arg("push").output()?;
    Ok(child)
}

pub fn git_rebase() -> anyhow::Result<Output> {
    let merge_base = Command::new("git")
        .arg("show-branch")
        .arg("--merge-base")
        .output()?
        .stdout;
    let merge_base = std::str::from_utf8(&merge_base)?
        .split('\n')
        .next()
        .unwrap_or("HEAD");
    let child = Command::new("git")
        .arg("rebase")
        .arg(merge_base)
        .arg("--empty")
        .arg("drop")
        .output()?;
    Ok(child)
}

pub fn git_merge(branch_name: &str) -> anyhow::Result<Output> {
    let child = Command::new("git")
        .arg("merge")
        .arg(branch_name)
        .arg("--ff-only")
        .output()?;
    Ok(child)
}

pub fn git_branch(branch_name: &str) -> anyhow::Result<Output> {
    let child = Command::new("git")
        .arg("switch")
        .arg("--create")
        .arg(branch_name)
        .output()?;
    Ok(child)
}

pub fn git_switch(branch_name: &str) -> anyhow::Result<Output> {
    let child = Command::new("git")
        .arg("switch")
        .arg(branch_name)
        .output()?;
    Ok(child)
}
