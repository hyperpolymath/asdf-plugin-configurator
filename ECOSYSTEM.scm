;; SPDX-License-Identifier: AGPL-3.0-or-later
;; ECOSYSTEM.scm - asdf-plugin-configurator ecosystem position

(ecosystem
  (version . "1.0.0")
  (name . "asdf-plugin-configurator")
  (type . "cli-tool")
  (purpose . "Declarative configuration management for asdf plugins")

  (position-in-ecosystem . "tooling")

  (what-this-is
    "Rust CLI for declarative asdf plugin management"
    "YAML/TOML configuration file format"
    "Plugin installation, validation, and synchronization"
    "Team collaboration features for shared configurations"
    "CI/CD integration for automated plugin management")

  (what-this-is-not
    "Not a replacement for asdf - extends it"
    "Not a plugin registry - uses asdf-metaiconic-plugin for that"
    "Not a visual interface - asdf-ui-plugin provides that")

  (related-projects
    ;; Direct ecosystem relationships
    (("hyperpolymath/asdf-control-tower" . "coordination-hub")
     ("hyperpolymath/asdf-metaiconic-plugin" . "metadata-source")
     ("hyperpolymath/asdf-ui-plugin" . "visual-frontend")
     ("hyperpolymath/asdf-ghjk" . "sibling-tool"))

    ;; Dependencies
    (("asdf-vm/asdf" . "parent-ecosystem")
     ("hyperpolymath/mustfile" . "build-tool")))

  (integration-points
    (provides
      "CLI commands: init, validate, list, install, sync, export"
      "Configuration file parsing and validation"
      "Plugin version resolution"
      "Team sync protocol")

    (consumes
      "Plugin metadata from asdf-metaiconic-plugin registry"
      "Version lists from asdf plugin list-all"
      "Installation via asdf install")))
