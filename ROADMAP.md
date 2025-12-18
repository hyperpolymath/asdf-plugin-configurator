# ROADMAP - asdf-plugin-configurator

> Unified configuration management for asdf plugins across development environments

## Project Overview

**asdf-plugin-configurator** is a tool designed to simplify and standardize the configuration of asdf version manager plugins across projects and teams. It provides declarative configuration, validation, and automation for plugin management.

---

## Current Status: Phase 0 - Infrastructure

### Completed
- [x] Repository initialization
- [x] Hub-and-spoke SCM mirroring (GitHub → GitLab, Codeberg, Bitbucket)
- [x] Security-hardened CI/CD workflow with:
  - SSH known_hosts pre-configuration (MITM protection)
  - `--force-with-lease` instead of `--force` (history protection)
  - Retry logic with exponential backoff (network resilience)
  - Structured logging with GitHub Actions groups
  - Least-privilege permissions (`read-all`)

---

## Phase 1 - Core Foundation

### 1.1 Project Structure
- [ ] Create project scaffolding (bin/, lib/, test/, docs/)
- [ ] Initialize shell script linting (shellcheck)
- [ ] Add POSIX-compliant base scripts
- [ ] Create LICENSE file (AGPL-3.0-or-later)
- [ ] Create CONTRIBUTING.md guidelines
- [ ] Create SECURITY.md policy

### 1.2 Configuration Schema
- [ ] Design YAML/TOML configuration schema for plugin definitions
- [ ] Support for:
  - Plugin name and source URL
  - Version constraints
  - Platform-specific configurations
  - Post-install hooks
  - Environment variables
- [ ] JSON Schema for validation

### 1.3 Core CLI
- [ ] `asdf-config init` - Initialize configuration in a project
- [ ] `asdf-config validate` - Validate configuration file
- [ ] `asdf-config list` - List configured plugins
- [ ] `asdf-config status` - Show plugin installation status

---

## Phase 2 - Plugin Management

### 2.1 Plugin Operations
- [ ] `asdf-config install` - Install all configured plugins
- [ ] `asdf-config update` - Update plugins to match configuration
- [ ] `asdf-config sync` - Sync plugin versions across team
- [ ] `asdf-config export` - Export current setup to configuration

### 2.2 Version Resolution
- [ ] Support semantic version ranges (^1.0.0, ~1.2.0, >=1.0 <2.0)
- [ ] Platform-aware version selection (linux, darwin, arm64, x86_64)
- [ ] Fallback version chains
- [ ] Version pinning with lock file support

### 2.3 Plugin Sources
- [ ] Official asdf plugin registry
- [ ] Custom Git repositories
- [ ] Local plugin paths
- [ ] Plugin aliasing

---

## Phase 3 - Team Collaboration

### 3.1 Shared Configurations
- [ ] Team configuration inheritance
- [ ] Organization-wide defaults
- [ ] Project-specific overrides
- [ ] Configuration merging strategies

### 3.2 Environment Profiles
- [ ] Development/staging/production profiles
- [ ] CI-specific configurations
- [ ] Docker/container optimizations
- [ ] Minimal installation mode

### 3.3 Integration
- [ ] Pre-commit hooks for configuration validation
- [ ] GitHub Actions for plugin verification
- [ ] GitLab CI templates
- [ ] Bitbucket Pipelines integration

---

## Phase 4 - Advanced Features

### 4.1 Security Enhancements
- [ ] Plugin checksum verification
- [ ] GPG signature validation
- [ ] Vulnerability scanning integration
- [ ] Audit logging

### 4.2 Performance
- [ ] Parallel plugin installation
- [ ] Plugin caching
- [ ] Incremental updates
- [ ] Lazy loading for large configurations

### 4.3 Ecosystem Integration
- [ ] VS Code extension
- [ ] Neovim plugin
- [ ] direnv integration
- [ ] mise/rtx compatibility layer

---

## Phase 5 - Enterprise Features

### 5.1 Compliance
- [ ] Policy enforcement for approved plugins
- [ ] License compliance checking
- [ ] Usage analytics (opt-in)
- [ ] Centralized plugin registry (self-hosted)

### 5.2 Advanced Workflows
- [ ] Plugin dependency resolution
- [ ] Conditional plugin installation
- [ ] Plugin groups and bundles
- [ ] Migration tooling from other version managers

---

## Security Considerations

### Implemented
- SSH key management via GitHub organization secrets
- Pinned GitHub Action versions (commit SHA)
- Least-privilege workflow permissions
- SSH known_hosts validation
- `--force-with-lease` for safe mirroring

### Planned
- Plugin source verification
- Checksum validation for downloads
- Security advisory integration
- Automated dependency updates (Dependabot/Renovate)

---

## Contributing

Contributions are welcome! Please see CONTRIBUTING.md (coming soon) for guidelines.

### Development Setup
```bash
# Clone the repository
git clone https://github.com/hyperpolymath/asdf-plugin-configurator.git
cd asdf-plugin-configurator

# Run tests (coming soon)
./test/run.sh

# Lint shell scripts (coming soon)
shellcheck bin/* lib/**/*.sh
```

---

## License

AGPL-3.0-or-later - See LICENSE file for details.

---

## Changelog

### Unreleased
- Security-hardened mirror workflow
- Added SSH known_hosts pre-configuration
- Replaced `--force` with `--force-with-lease`
- Added retry logic with exponential backoff
- Added structured logging for CI/CD

### 2025-12-17
- Initial repository setup
- Hub-and-spoke mirror workflow created
