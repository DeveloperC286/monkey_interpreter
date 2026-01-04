DOCKER_RUN_OPTS := --rm -v $(PWD):/workspace -w /workspace

UID := $(shell id -u)
GID := $(shell id -g)
DOCKER_RUN_WRITE_OPTS := $(DOCKER_RUN_OPTS) -u $(UID):$(GID)

.PHONY: default
default: compile

# renovate: depName=ghcr.io/developerc286/clean_git_history
CLEAN_GIT_HISTORY_VERSION=1.1.6@sha256:93fd9c692f6e629956921b8d068ccad33760882b6e0c6d4d32cd963380aec25f

.PHONY: check-clean-git-history
check-clean-git-history:
	docker run $(DOCKER_RUN_WRITE_OPTS) ghcr.io/developerc286/clean_git_history:$(CLEAN_GIT_HISTORY_VERSION) $(FROM)

# renovate: depName=ghcr.io/developerc286/conventional_commits_linter
CONVENTIONAL_COMMITS_LINTER_VERSION=0.17.1@sha256:f1b947937ee884ba7f886d04939cd4858f9aeafb50dcf94925a516c50e43021b

.PHONY: check-conventional-commits-linting
check-conventional-commits-linting:
	docker run $(DOCKER_RUN_WRITE_OPTS) ghcr.io/developerc286/conventional_commits_linter:$(CONVENTIONAL_COMMITS_LINTER_VERSION) --type angular $(FROM)

.PHONY: check-rust-formatting
check-rust-formatting:
	docker build -t check-rust-formatting -f ci/check-rust-formatting.Dockerfile .
	docker run $(DOCKER_RUN_OPTS) check-rust-formatting

# renovate: depName=mvdan/shfmt
SHFMT_VERSION=v3.12.0-alpine@sha256:204a4d2d876123342ad394bd9a28fb91e165abc03868790d4b39cfa73233f150

.PHONY: check-shell-formatting
check-shell-formatting:
	docker run $(DOCKER_RUN_OPTS) mvdan/shfmt:$(SHFMT_VERSION) --simplify --diff ci/*

# renovate: depName=ghcr.io/google/yamlfmt
YAMLFMT_VERSION=0.20.0@sha256:cd11483ba1119371593a7d55386d082da518e27dd932ee00db32e5fb6f3a58c0

.PHONY: check-yaml-formatting
check-yaml-formatting:
	docker run $(DOCKER_RUN_OPTS) ghcr.io/google/yamlfmt:$(YAMLFMT_VERSION) -verbose -lint -dstar .github/workflows/*

.PHONY: fix-rust-formatting
fix-rust-formatting:
	docker build -t fix-rust-formatting -f ci/fix-rust-formatting.Dockerfile .
	docker run $(DOCKER_RUN_WRITE_OPTS) fix-rust-formatting

.PHONY: fix-shell-formatting
fix-shell-formatting:
	docker run $(DOCKER_RUN_WRITE_OPTS) mvdan/shfmt:$(SHFMT_VERSION) --simplify --write ci/*

.PHONY: fix-yaml-formatting
fix-yaml-formatting:
	docker run $(DOCKER_RUN_WRITE_OPTS) ghcr.io/google/yamlfmt:$(YAMLFMT_VERSION) -verbose -dstar .github/workflows/*

.PHONY: check-rust-linting
check-rust-linting:
	docker build -t check-rust-linting -f ci/check-rust-linting.Dockerfile .
	docker run $(DOCKER_RUN_OPTS) check-rust-linting

# renovate: depName=rhysd/actionlint
ACTIONLINT_VERSION=1.7.10@sha256:ef8299f97635c4c30e2298f48f30763ab782a4ad2c95b744649439a039421e36

.PHONY: check-github-actions-workflows-linting
check-github-actions-workflows-linting:
	docker run $(DOCKER_RUN_WRITE_OPTS) rhysd/actionlint:$(ACTIONLINT_VERSION) -verbose -color

.PHONY: compile
compile:
	docker build -t compile -f ci/compile.Dockerfile .
	docker run $(DOCKER_RUN_WRITE_OPTS) compile

.PHONY: unit-test
unit-test:
	docker build -t unit-test -f ci/unit-test.Dockerfile .
	docker run $(DOCKER_RUN_WRITE_OPTS) unit-test
