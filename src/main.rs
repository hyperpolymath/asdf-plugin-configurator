// SPDX-License-Identifier: AGPL-3.0-or-later
//! asdf-config: Declarative configuration management for asdf plugins

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::Colorize;
use std::path::PathBuf;

mod config;
mod commands;

use commands::{init, validate, list, install, sync, export};

/// Declarative configuration management for asdf plugins
#[derive(Parser)]
#[command(name = "asdf-config")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Configuration file path (default: .asdf-config.yaml)
    #[arg(short, long, global = true)]
    config: Option<PathBuf>,

    /// Verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new configuration file
    Init {
        /// Output format (yaml or toml)
        #[arg(short, long, default_value = "yaml")]
        format: String,
    },

    /// Validate configuration file
    Validate,

    /// List configured plugins
    List {
        /// Show all available plugins (from metaiconic registry)
        #[arg(short, long)]
        all: bool,

        /// Filter by category
        #[arg(short = 'c', long)]
        category: Option<String>,
    },

    /// Install plugins from configuration
    Install {
        /// Only install specified plugin
        #[arg(short, long)]
        plugin: Option<String>,

        /// Dry run (show what would be installed)
        #[arg(short, long)]
        dry_run: bool,
    },

    /// Sync plugin versions across team
    Sync {
        /// Pull latest versions
        #[arg(short, long)]
        pull: bool,

        /// Push local versions
        #[arg(short = 'P', long)]
        push: bool,
    },

    /// Export current asdf setup to configuration
    Export {
        /// Output format (yaml or toml)
        #[arg(short, long, default_value = "yaml")]
        format: String,

        /// Output file (default: stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Search available plugins (from metaiconic registry)
    Search {
        /// Search query
        query: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config_path = cli.config.unwrap_or_else(|| {
        PathBuf::from(".asdf-config.yaml")
    });

    match cli.command {
        Commands::Init { format } => {
            init::run(&format, cli.verbose)?;
        }
        Commands::Validate => {
            validate::run(&config_path, cli.verbose)?;
        }
        Commands::List { all, category } => {
            list::run(&config_path, all, category.as_deref(), cli.verbose)?;
        }
        Commands::Install { plugin, dry_run } => {
            install::run(&config_path, plugin.as_deref(), dry_run, cli.verbose)?;
        }
        Commands::Sync { pull, push } => {
            sync::run(&config_path, pull, push, cli.verbose)?;
        }
        Commands::Export { format, output } => {
            export::run(&format, output.as_deref(), cli.verbose)?;
        }
        Commands::Search { query } => {
            search(&query, cli.verbose)?;
        }
    }

    Ok(())
}

fn search(query: &str, verbose: bool) -> Result<()> {
    println!("{} Searching for '{}'...", "→".blue(), query);

    // TODO: Fetch from metaiconic registry
    println!("{}", "Note: Search requires asdf-metaiconic-plugin registry".yellow());
    println!("Registry URL: https://github.com/hyperpolymath/asdf-metaiconic-plugin");

    if verbose {
        println!("{} This feature will query the metaiconic plugin registry", "ℹ".cyan());
    }

    Ok(())
}
