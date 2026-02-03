use std::io::{self, Write};
use std::process::{self, Command};

struct Branch {
    name: String,
    relative_date: String,
}

/// Returns the default branch by checking what origin/HEAD points to.
/// Falls back to "main" then "master" if no remote is configured.
fn default_branch() -> Option<String> {
    let output = Command::new("git")
        .args(["symbolic-ref", "refs/remotes/origin/HEAD"])
        .output()
        .ok()?;

    if output.status.success() {
        let full = String::from_utf8_lossy(&output.stdout).trim().to_string();
        return full.strip_prefix("refs/remotes/origin/").map(String::from);
    }

    // No remote HEAD — check if main or master exists locally
    for name in ["main", "master"] {
        let check = Command::new("git")
            .args(["rev-parse", "--verify", &format!("refs/heads/{}", name)])
            .output()
            .ok()?;
        if check.status.success() {
            return Some(name.to_string());
        }
    }

    None
}

fn local_branches() -> Vec<Branch> {
    let output = Command::new("git")
        .args([
            "for-each-ref",
            "--sort=-committerdate",
            "--format=%(committerdate:relative)\t%(refname:short)",
            "refs/heads/",
        ])
        .output()
        .expect("failed to run git");

    if !output.status.success() {
        eprintln!(
            "git for-each-ref failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        process::exit(1);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .lines()
        .filter_map(|line| {
            let (date, name) = line.split_once('\t')?;
            Some(Branch {
                relative_date: date.to_string(),
                name: name.to_string(),
            })
        })
        .collect()
}

fn delete_branch(name: &str) -> Result<String, String> {
    let output = Command::new("git")
        .args(["branch", "-d", name])
        .output()
        .expect("failed to run git");

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

fn force_delete_branch(name: &str) -> Result<String, String> {
    let output = Command::new("git")
        .args(["branch", "-D", name])
        .output()
        .expect("failed to run git");

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_lowercase()
}

fn main() {
    let default = default_branch();
    let branches: Vec<Branch> = local_branches()
        .into_iter()
        .filter(|b| default.as_ref() != Some(&b.name))
        .collect();

    if branches.is_empty() {
        println!("No branches to delete.");
        return;
    }

    let mut deleted = 0u32;
    let mut skipped = 0u32;

    let total = branches.len();

    for (i, branch) in branches.iter().enumerate() {
        let answer = loop {
            let a = prompt(&format!(
                "\x1b[1m({}/{}) Delete {} (last touched {}) [y,n,q,?]?\x1b[0m ",
                i + 1,
                total,
                branch.name,
                branch.relative_date
            ));
            if a == "?" {
                println!(
                    "y - delete this branch\n\
                     n - skip this branch\n\
                     q - quit\n\
                     ? - print help"
                );
                continue;
            }
            break a;
        };

        match answer.as_str() {
            "y" => match delete_branch(&branch.name) {
                Ok(out) => {
                    println!("    {}", out);
                    deleted += 1;
                }
                Err(e) => {
                    let msg: String = e
                        .lines()
                        .filter(|l| !l.starts_with("hint:"))
                        .collect::<Vec<_>>()
                        .join("\n");
                    println!("    Failed: {}", msg);
                    let retry = prompt("    Force delete? [y/n]: ");
                    if retry == "y" {
                        match force_delete_branch(&branch.name) {
                            Ok(out) => {
                                println!("    {}", out);
                                deleted += 1;
                            }
                            Err(e2) => println!("    Force-delete failed: {}", e2),
                        }
                    }
                }
            },
            "q" => {
                println!("Quit.");
                break;
            }
            _ => {
                skipped += 1;
            }
        }
    }

    println!(
        "\nDone. Deleted {} branch{}, skipped {}.",
        deleted,
        if deleted == 1 { "" } else { "es" },
        skipped,
    );
}
