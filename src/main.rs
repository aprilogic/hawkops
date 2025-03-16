mod api;
mod auth;
mod cmd;
mod config;
mod error;

use clap::Parser;
use cmd::{Cli, Commands};
use error::{HawkOpsError, HawkOpsResult};
use tracing::{error, info};
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() -> HawkOpsResult<()> {
    // Parse command line arguments
    let cli = Cli::parse();

    // Setup logging
    setup_logging(&cli.log_level)?;

    // Load configuration
    let mut config = config::HawkOpsConfig::load()
        .map_err(|e| HawkOpsError::ConfigError(format!("Failed to load config: {}", e)))?;

    // Create auth manager
    let auth_manager = auth::AuthManager::new(config.clone());

    // Create API client
    let api_client = api::ApiClient::new(auth_manager);

    // Execute command
    match &cli.command {
        Commands::Config { api_key } => {
            if let Some(key) = api_key {
                config.api.api_key = Some(key.clone());
                config.save()?;
                info!("API key updated successfully");
            } else {
                info!("Current configuration:");
                info!("API Key: {}", config.api.api_key.as_deref().unwrap_or("Not set"));
                info!("Base URL: {}", config.api.base_url);
            }
        }

        Commands::Apps { org_id } => {
            let endpoint = if let Some(id) = org_id {
                format!("{}/api/v1/orgs/{}/applications", config.api.base_url, id)
            } else {
                format!("{}/api/v1/applications", config.api.base_url)
            };

            let apps: Vec<api::Application> = api_client.get(&endpoint).await?;
            for app in apps {
                info!("App: {} ({})", app.name, app.id);
            }
        }

        Commands::Scan { command } => handle_scan_command(&api_client, command, &config).await?,
        Commands::Team { command } => handle_team_command(&api_client, command, &config).await?,
        Commands::User { command } => handle_user_command(&api_client, command, &config).await?,
    }

    Ok(())
}

fn setup_logging(log_level: &str) -> HawkOpsResult<()> {
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(log_level))
        .map_err(|e| HawkOpsError::ConfigError(format!("Invalid log level: {}", e)))?;

    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(false)
        .with_line_number(false)
        .with_level(true)
        .init();

    Ok(())
}

async fn handle_scan_command(
    client: &api::ApiClient,
    command: &cmd::ScanCommands,
    config: &config::HawkOpsConfig,
) -> HawkOpsResult<()> {
    match command {
        cmd::ScanCommands::List { app_id, limit } => {
            let endpoint = format!(
                "{}/api/v1/applications/{}/scans?limit={}",
                config.api.base_url, app_id, limit
            );
            let scans: Vec<api::Scan> = client.get(&endpoint).await?;
            for scan in scans {
                info!(
                    "Scan: {} (Status: {}, Findings: {})",
                    scan.id,
                    scan.status,
                    scan.findings_count.unwrap_or(0)
                );
            }
        }
        cmd::ScanCommands::Get { scan_id } => {
            let endpoint = format!("{}/api/v1/scans/{}", config.api.base_url, scan_id);
            let scan: api::Scan = client.get(&endpoint).await?;
            info!("Scan details:");
            info!("  ID: {}", scan.id);
            info!("  Status: {}", scan.status);
            info!("  Created: {}", scan.created_at);
            if let Some(completed) = scan.completed_at {
                info!("  Completed: {}", completed);
            }
            if let Some(findings) = scan.findings_count {
                info!("  Findings: {}", findings);
            }
        }
        cmd::ScanCommands::Delete { scan_id } => {
            let endpoint = format!("{}/api/v1/scans/{}", config.api.base_url, scan_id);
            client.delete(&endpoint).await?;
            info!("Scan {} deleted successfully", scan_id);
        }
    }
    Ok(())
}

async fn handle_team_command(
    client: &api::ApiClient,
    command: &cmd::TeamCommands,
    config: &config::HawkOpsConfig,
) -> HawkOpsResult<()> {
    match command {
        cmd::TeamCommands::List { org_id } => {
            let endpoint = if let Some(id) = org_id {
                format!("{}/api/v1/orgs/{}/teams", config.api.base_url, id)
            } else {
                format!("{}/api/v1/teams", config.api.base_url)
            };
            let teams: Vec<api::Team> = client.get(&endpoint).await?;
            for team in teams {
                info!("Team: {} ({})", team.name, team.id);
            }
        }
        cmd::TeamCommands::Get { team_id } => {
            let endpoint = format!("{}/api/v1/teams/{}", config.api.base_url, team_id);
            let team: api::Team = client.get(&endpoint).await?;
            info!("Team details:");
            info!("  ID: {}", team.id);
            info!("  Name: {}", team.name);
            info!("  Organization: {}", team.organization_id);
        }
        cmd::TeamCommands::Create { org_id, name } => {
            let endpoint = format!("{}/api/v1/orgs/{}/teams", config.api.base_url, org_id);
            let team: api::Team = client
                .post(
                    &endpoint,
                    &serde_json::json!({
                        "name": name,
                        "organization_id": org_id,
                    }),
                )
                .await?;
            info!("Team created successfully:");
            info!("  ID: {}", team.id);
            info!("  Name: {}", team.name);
        }
    }
    Ok(())
}

async fn handle_user_command(
    client: &api::ApiClient,
    command: &cmd::UserCommands,
    config: &config::HawkOpsConfig,
) -> HawkOpsResult<()> {
    match command {
        cmd::UserCommands::List { org_id } => {
            let endpoint = if let Some(id) = org_id {
                format!("{}/api/v1/orgs/{}/users", config.api.base_url, id)
            } else {
                format!("{}/api/v1/users", config.api.base_url)
            };
            let users: Vec<api::User> = client.get(&endpoint).await?;
            for user in users {
                info!(
                    "User: {} ({}) {}",
                    user.email,
                    user.id,
                    user.name.unwrap_or_default()
                );
            }
        }
        cmd::UserCommands::Get { user_id } => {
            let endpoint = format!("{}/api/v1/users/{}", config.api.base_url, user_id);
            let user: api::User = client.get(&endpoint).await?;
            info!("User details:");
            info!("  ID: {}", user.id);
            info!("  Email: {}", user.email);
            if let Some(name) = user.name {
                info!("  Name: {}", name);
            }
        }
    }
    Ok(())
}


