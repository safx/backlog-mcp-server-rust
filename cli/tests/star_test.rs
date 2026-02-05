use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    command: StarCommands,
}

#[derive(Subcommand, Debug)]
enum StarCommands {
    Add {
        #[clap(subcommand)]
        target: StarTarget,
    },
}

#[derive(Subcommand, Debug, PartialEq)]
enum StarTarget {
    Issue { issue_id: u32 },
    Comment { issue_id: u32, comment_id: u32 },
    Wiki { wiki_id: u32 },
    Pr { pr_id: u32 },
    PrComment { pr_comment_id: u32 },
}

#[test]
fn test_star_add_issue() {
    let args = Cli::try_parse_from(["prog", "add", "issue", "123"]).unwrap();
    match args.command {
        StarCommands::Add { target } => {
            assert_eq!(target, StarTarget::Issue { issue_id: 123 });
        }
    }
}

#[test]
fn test_star_add_comment() {
    let args = Cli::try_parse_from(["prog", "add", "comment", "100", "200"]).unwrap();
    match args.command {
        StarCommands::Add { target } => {
            assert_eq!(
                target,
                StarTarget::Comment {
                    issue_id: 100,
                    comment_id: 200
                }
            );
        }
    }
}

#[test]
fn test_star_add_wiki() {
    let args = Cli::try_parse_from(["prog", "add", "wiki", "456"]).unwrap();
    match args.command {
        StarCommands::Add { target } => {
            assert_eq!(target, StarTarget::Wiki { wiki_id: 456 });
        }
    }
}

#[test]
fn test_star_add_pr() {
    let args = Cli::try_parse_from(["prog", "add", "pr", "789"]).unwrap();
    match args.command {
        StarCommands::Add { target } => {
            assert_eq!(target, StarTarget::Pr { pr_id: 789 });
        }
    }
}

#[test]
fn test_star_add_pr_comment() {
    let args = Cli::try_parse_from(["prog", "add", "pr-comment", "321"]).unwrap();
    match args.command {
        StarCommands::Add { target } => {
            assert_eq!(target, StarTarget::PrComment { pr_comment_id: 321 });
        }
    }
}
