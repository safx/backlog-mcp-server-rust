use clap::Parser;

#[cfg(feature = "project")]
#[derive(Parser)]
pub struct ActivityArgs {
    #[clap(subcommand)]
    pub command: ActivityCommands,
}

#[cfg(feature = "project")]
#[derive(Parser)]
pub enum ActivityCommands {
    /// Get recent activities in a project
    Project {
        /// Project ID or key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id: String,

        /// Filter by activity type IDs (comma-separated)
        #[clap(long)]
        type_ids: Option<String>,

        /// Maximum number of results (default: 20, max: 100)
        #[clap(long)]
        count: Option<u32>,

        /// Sort order (asc or desc)
        #[clap(long)]
        order: Option<String>,
    },
    #[cfg(feature = "space")]
    /// Get recent activities in the space
    Space {
        /// Filter by activity type IDs (comma-separated)
        #[clap(long)]
        type_ids: Option<String>,

        /// Maximum number of results (default: 20, max: 100)
        #[clap(long)]
        count: Option<u32>,

        /// Sort order (asc or desc)
        #[clap(long)]
        order: Option<String>,
    },
}
