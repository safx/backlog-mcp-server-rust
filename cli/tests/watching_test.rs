use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    command: WatchingSubcommand,
}

#[derive(Subcommand, Debug)]
enum WatchingSubcommand {
    Get(GetWatchingArgs),
    Add(AddWatchingArgs),
}

#[derive(Args, Debug, Clone)]
struct GetWatchingArgs {
    watching_id: u32,
}

#[derive(Args, Debug, Clone)]
struct AddWatchingArgs {
    issue: String,
    #[arg(short, long)]
    note: Option<String>,
}

#[test]
fn test_watching_get_command() {
    let args = Cli::try_parse_from(["prog", "get", "12345"]).unwrap();
    match args.command {
        WatchingSubcommand::Get(args) => {
            assert_eq!(args.watching_id, 12345);
        }
        _ => panic!("Expected Get command"),
    }
}

#[test]
fn test_watching_add_with_note() {
    let args =
        Cli::try_parse_from(["prog", "add", "PROJECT-123", "--note", "Important issue"]).unwrap();
    match args.command {
        WatchingSubcommand::Add(args) => {
            assert_eq!(args.issue, "PROJECT-123");
            assert_eq!(args.note, Some("Important issue".to_string()));
        }
        _ => panic!("Expected Add command"),
    }
}
