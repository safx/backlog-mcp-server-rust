use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    command: WebhookCommands,
}

#[derive(Subcommand, Debug)]
enum WebhookCommands {
    #[clap(alias = "ls")]
    List {
        #[arg(short, long)]
        project: String,
        #[arg(short, long, value_enum, default_value = "table")]
        format: OutputFormat,
    },
    Get {
        #[arg(short, long)]
        project: String,
        #[arg(short, long)]
        webhook_id: u32,
        #[arg(short, long, value_enum, default_value = "table")]
        format: OutputFormat,
    },
    Add {
        #[arg(short, long)]
        project: String,
        #[arg(short, long)]
        name: String,
        #[arg(short = 'u', long)]
        hook_url: String,
        #[arg(short, long)]
        description: Option<String>,
        #[arg(long)]
        all_event: Option<bool>,
        #[arg(long, value_delimiter = ',')]
        activity_type_ids: Option<Vec<u32>>,
    },
    Update {
        #[arg(short, long)]
        project: String,
        #[arg(short = 'w', long)]
        webhook_id: u32,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        hook_url: Option<String>,
        #[arg(long)]
        all_event: Option<bool>,
        #[arg(long, value_delimiter = ',')]
        activity_type_ids: Option<Vec<u32>>,
    },
    #[clap(alias = "rm")]
    Delete {
        #[arg(short, long)]
        project: String,
        #[arg(short = 'w', long)]
        webhook_id: u32,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq)]
enum OutputFormat {
    Table,
    Json,
    Csv,
}

#[test]
fn test_webhook_list_command() {
    let args = Cli::try_parse_from(["prog", "list", "--project", "TEST"]).unwrap();
    match args.command {
        WebhookCommands::List { project, format } => {
            assert_eq!(project, "TEST");
            assert_eq!(format, OutputFormat::Table);
        }
        _ => panic!("Expected List command"),
    }
}

#[test]
fn test_webhook_get_command() {
    let args =
        Cli::try_parse_from(["prog", "get", "--project", "TEST", "--webhook-id", "123"]).unwrap();
    match args.command {
        WebhookCommands::Get {
            project,
            webhook_id,
            format,
        } => {
            assert_eq!(project, "TEST");
            assert_eq!(webhook_id, 123);
            assert_eq!(format, OutputFormat::Table);
        }
        _ => panic!("Expected Get command"),
    }
}

#[test]
fn test_webhook_add_command() {
    let args = Cli::try_parse_from([
        "prog",
        "add",
        "--project",
        "TEST",
        "--name",
        "My Webhook",
        "--hook-url",
        "https://example.com/hook",
    ])
    .unwrap();
    match args.command {
        WebhookCommands::Add {
            project,
            name,
            hook_url,
            description,
            all_event,
            activity_type_ids,
        } => {
            assert_eq!(project, "TEST");
            assert_eq!(name, "My Webhook");
            assert_eq!(hook_url, "https://example.com/hook");
            assert_eq!(description, None);
            assert_eq!(all_event, None);
            assert_eq!(activity_type_ids, None);
        }
        _ => panic!("Expected Add command"),
    }
}

#[test]
fn test_webhook_update_minimal() {
    let args = Cli::try_parse_from([
        "prog",
        "update",
        "--project",
        "TEST",
        "--webhook-id",
        "456",
        "--name",
        "Updated Name",
    ])
    .unwrap();
    match args.command {
        WebhookCommands::Update {
            project,
            webhook_id,
            name,
            description,
            hook_url,
            all_event,
            activity_type_ids,
        } => {
            assert_eq!(project, "TEST");
            assert_eq!(webhook_id, 456);
            assert_eq!(name, Some("Updated Name".to_string()));
            assert_eq!(description, None);
            assert_eq!(hook_url, None);
            assert_eq!(all_event, None);
            assert_eq!(activity_type_ids, None);
        }
        _ => panic!("Expected Update command"),
    }
}

#[test]
fn test_webhook_update_full() {
    let args = Cli::try_parse_from([
        "prog",
        "update",
        "--project",
        "TEST",
        "--webhook-id",
        "789",
        "--name",
        "Full Update",
        "--description",
        "A description",
        "--hook-url",
        "https://new.example.com/hook",
        "--all-event",
        "true",
        "--activity-type-ids",
        "1,2,3",
    ])
    .unwrap();
    match args.command {
        WebhookCommands::Update {
            project,
            webhook_id,
            name,
            description,
            hook_url,
            all_event,
            activity_type_ids,
        } => {
            assert_eq!(project, "TEST");
            assert_eq!(webhook_id, 789);
            assert_eq!(name, Some("Full Update".to_string()));
            assert_eq!(description, Some("A description".to_string()));
            assert_eq!(hook_url, Some("https://new.example.com/hook".to_string()));
            assert_eq!(all_event, Some(true));
            assert_eq!(activity_type_ids, Some(vec![1, 2, 3]));
        }
        _ => panic!("Expected Update command"),
    }
}

#[test]
fn test_webhook_delete_command() {
    let args = Cli::try_parse_from(["prog", "delete", "--project", "TEST", "--webhook-id", "999"])
        .unwrap();
    match args.command {
        WebhookCommands::Delete {
            project,
            webhook_id,
        } => {
            assert_eq!(project, "TEST");
            assert_eq!(webhook_id, 999);
        }
        _ => panic!("Expected Delete command"),
    }
}
