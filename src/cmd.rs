use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "hawkops")]
#[command(about = "A CLI tool for interacting with the StackHawk platform API", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Set the log level (error, warn, info, debug, trace)
    #[arg(short, long, global = true, default_value = "info")]
    pub log_level: String,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Configure hawkops settings
    Config {
        /// Set the API key
        #[arg(long)]
        api_key: Option<String>,
    },

    /// List applications
    Apps {
        /// Filter by organization ID
        #[arg(long)]
        org_id: Option<String>,
    },

    /// Manage scans
    Scan {
        #[command(subcommand)]
        command: ScanCommands,
    },

    /// Manage teams
    Team {
        #[command(subcommand)]
        command: TeamCommands,
    },

    /// Manage users
    User {
        #[command(subcommand)]
        command: UserCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum ScanCommands {
    /// List scans for an application
    List {
        /// Application ID
        #[arg(long)]
        app_id: String,
        /// Maximum number of scans to return
        #[arg(long, default_value = "10")]
        limit: u32,
    },
    /// Get details for a specific scan
    Get {
        /// Scan ID
        #[arg(long)]
        scan_id: String,
    },
    /// Delete a scan
    Delete {
        /// Scan ID
        #[arg(long)]
        scan_id: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum TeamCommands {
    /// List teams
    List {
        /// Organization ID
        #[arg(long)]
        org_id: Option<String>,
    },
    /// Get team details
    Get {
        /// Team ID
        #[arg(long)]
        team_id: String,
    },
    /// Create a new team
    Create {
        /// Organization ID
        #[arg(long)]
        org_id: String,
        /// Team name
        #[arg(long)]
        name: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum UserCommands {
    /// List users
    List {
        /// Organization ID
        #[arg(long)]
        org_id: Option<String>,
    },
    /// Get user details
    Get {
        /// User ID
        #[arg(long)]
        user_id: String,
    },
} 