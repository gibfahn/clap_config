SHELL := /bin/bash -euo pipefail

# Default target
.DEFAULT_GOAL := help

# To add a target to the help, add a double comment (##) on the target line.
.PHONY: help
help: ## Print help for targets.
	@printf "For more targets and info see the comments in the Makefile.\n\n"
	@grep -E '^[a-zA-Z0-9._-]+:.*?## .*$$' Makefile | sort | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

.PHONY: release
release: ## Ship a new release of clap_config (specify version: `make release bump=minor`).
	meta/release $(bump)
