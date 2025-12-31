// SPDX-License-Identifier: AGPL-3.0-or-later
//! Install plugins from configuration

use anyhow::{Context, Result};
use colored::Colorize;
use std::path::Path;
use std::process::Command;

use crate::config::Config;

pub fn run(config_path: &Path, plugin: Option<&str>, dry_run: bool, verbose: bool) -> Result<()> {
    if !config_path.exists() {
        println!("{} Configuration file not found: {}", "✗".red(), config_path.display());
        return Ok(());
    }

    let config = Config::load(config_path)?;

    let plugins_to_install: Vec<_> = if let Some(name) = plugin {
        config.plugins.iter()
            .filter(|(n, _)| *n == name)
            .collect()
    } else {
        config.plugins.iter().collect()
    };

    if plugins_to_install.is_empty() {
        println!("{} No plugins to install", "!".yellow());
        return Ok(());
    }

    println!("{} Installing {} plugin(s){}",
        "→".blue(),
        plugins_to_install.len(),
        if dry_run { " (dry run)" } else { "" }
    );
    println!();

    for (name, plugin_config) in &plugins_to_install {
        let source = get_plugin_source(name, &plugin_config.source);

        if dry_run {
            println!("  {} Would install {} @ {} from {}",
                "→".cyan(),
                name.bold(),
                plugin_config.version.green(),
                source.dimmed()
            );
            continue;
        }

        // Add plugin
        println!("  {} Adding plugin {}...", "→".cyan(), name);

        if verbose {
            println!("    Source: {}", source);
        }

        let add_result = Command::new("asdf")
            .args(["plugin", "add", name, &source])
            .output()
            .context("Failed to run asdf")?;

        if !add_result.status.success() {
            let stderr = String::from_utf8_lossy(&add_result.stderr);
            if !stderr.contains("already added") {
                println!("  {} Failed to add {}: {}", "✗".red(), name, stderr.trim());
                continue;
            }
        }

        // Install version
        println!("  {} Installing {} @ {}...", "→".cyan(), name, plugin_config.version);

        let version = resolve_version(&plugin_config.version);
        let install_result = Command::new("asdf")
            .args(["install", name, &version])
            .output()
            .context("Failed to run asdf install")?;

        if !install_result.status.success() {
            let stderr = String::from_utf8_lossy(&install_result.stderr);
            println!("  {} Failed to install {}: {}", "✗".red(), name, stderr.trim());
            continue;
        }

        // Set global version
        let _ = Command::new("asdf")
            .args(["global", name, &version])
            .output();

        // Run post-install commands
        for cmd in &plugin_config.post_install {
            if verbose {
                println!("    Running: {}", cmd);
            }
            let _ = Command::new("sh")
                .args(["-c", cmd])
                .output();
        }

        println!("  {} Installed {} @ {}", "✓".green(), name.bold(), version.green());
    }

    println!();
    println!("{} Installation complete", "✓".green());

    Ok(())
}

fn get_plugin_source(name: &str, source: &str) -> String {
    match source {
        "official" => format!("https://github.com/asdf-vm/asdf-{}.git", name),
        "hyperpolymath" => format!("https://github.com/hyperpolymath/asdf-{}-plugin.git", name),
        url if url.starts_with("http") => url.to_string(),
        _ => format!("https://github.com/hyperpolymath/asdf-{}-plugin.git", name),
    }
}

fn resolve_version(constraint: &str) -> String {
    // For now, simple resolution
    match constraint {
        "latest" | "stable" => "latest".to_string(),
        v if v.starts_with('^') || v.starts_with('~') || v.starts_with('>') || v.starts_with('<') => {
            // Would need to query available versions and resolve
            // For now, try latest
            "latest".to_string()
        }
        v => v.to_string(),
    }
}
