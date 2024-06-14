use std::io::{self, BufRead};

use git_wrapper::{git_branch, git_merge, git_rebase, git_switch};

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let branch_name = &line?;
        let result = git_branch(branch_name)?;
        println!("{:#?}", result);
        let result = git_rebase()?;
        println!("{:#?}", result);
        let result = git_switch("master")?;
        println!("{:#?}", result);
        let result = git_merge(branch_name)?;
        println!("{:#?}", result);
        println!();
        println!();
    }
    Ok(())
}
