// SPDX-License-Identifier: AGPL-3.0-or-later
//! List plugins

use anyhow::Result;
use colored::Colorize;
use std::path::Path;

use crate::config::Config;

pub fn run(config_path: &Path, all: bool, category: Option<&str>, verbose: bool) -> Result<()> {
    if all {
        list_all_available(category, verbose)?;
    } else {
        list_configured(config_path, verbose)?;
    }

    Ok(())
}

fn list_configured(config_path: &Path, verbose: bool) -> Result<()> {
    if !config_path.exists() {
        println!("{} No configuration file found", "!".yellow());
        println!("  Run 'asdf-config init' to create one");
        println!("  Or use 'asdf-config list --all' to see available plugins");
        return Ok(());
    }

    let config = Config::load(config_path)?;

    println!("{} Configured plugins:", "→".blue());
    println!();

    for (name, plugin) in &config.plugins {
        let optional = if plugin.optional { " (optional)".dimmed().to_string() } else { String::new() };
        println!("  {} {} @ {}{}", "•".cyan(), name.bold(), plugin.version.green(), optional);

        if verbose && !plugin.post_install.is_empty() {
            for cmd in &plugin.post_install {
                println!("    {} {}", "→".dimmed(), cmd.dimmed());
            }
        }
    }

    println!();
    println!("{} Total: {} plugins", "ℹ".blue(), config.plugins.len());

    Ok(())
}

fn list_all_available(category: Option<&str>, _verbose: bool) -> Result<()> {
    println!("{} Available plugins from hyperpolymath ecosystem:", "→".blue());
    println!();

    // Hardcoded list - will be fetched from metaiconic registry
    let plugins = vec![
        ("security", vec![
            ("trivy", "Security scanner"),
            ("grype", "Vulnerability scanner"),
            ("syft", "SBOM generator"),
            ("cosign", "Container signing"),
            ("gitleaks", "Secret scanning"),
            ("age", "File encryption"),
        ]),
        ("database", vec![
            ("arangodb", "Multi-model database"),
            ("mariadb", "MySQL fork"),
            ("neo4j", "Graph database"),
        ]),
        ("config", vec![
            ("nickel", "Configuration language"),
            ("dhall", "Programmable config"),
            ("cue", "Data validation"),
            ("yq", "YAML processor"),
            ("taplo", "TOML toolkit"),
        ]),
        ("network", vec![
            ("coredns", "DNS server"),
            ("envoy", "Proxy"),
            ("pomerium", "Access proxy"),
        ]),
        ("crypto", vec![
            ("step-ca", "Certificate authority"),
            ("cfssl", "PKI toolkit"),
            ("lego", "ACME client"),
        ]),
    ];

    for (cat, items) in &plugins {
        if category.is_some() && category != Some(*cat) {
            continue;
        }

        println!("  {} {}:", "▸".cyan(), cat.bold());
        for (name, desc) in items {
            println!("    {} {} - {}", "•".dimmed(), name.green(), desc.dimmed());
        }
        println!();
    }

    if category.is_none() {
        println!("{} Use --category <name> to filter", "ℹ".blue());
    }

    println!("{} Full registry at: {}", "ℹ".blue(), "github.com/hyperpolymath/asdf-metaiconic-plugin".cyan());

    Ok(())
}
