# So new files are owned by the user.
UID := $(shell id -u)
GID := $(shell id -g)

.PHONY: check-clean-git-history check-conventional-commits-linting check-rust-formatting check-python-formatting check-yaml-formatting fix-rust-formatting fix-python-formatting fix-yaml-formatting check-rust-linting check-github-actions-workflows-linting compile unit-test static-binary-test end-to-end-test publish-binary publish-crate

check-clean-git-history:
	docker build -t check-clean-git-history -f ci/check-clean-git-history.Dockerfile .
	docker run --rm -v $(PWD):/workspace -u $(UID):$(GID) check-clean-git-history $(FROM)

check-conventional-commits-linting:
	docker build -t check-conventional-commits-linting -f ci/check-conventional-commits-linting.Dockerfile .
	docker run --rm -v $(PWD):/workspace -u $(UID):$(GID) check-conventional-commits-linting $(FROM)

check-rust-formatting:
	docker build -t check-rust-formatting -f ci/check-rust-formatting.Dockerfile .
	docker run --rm -v $(PWD):/workspace -u $(UID):$(GID) check-rust-formatting

check-shell-formatting:
	docker pull mvdan/shfmt:v3.11.0-alpine
	docker run --rm -v $(PWD):/workspace -w /workspace -u $(UID):$(GID) mvdan/shfmt:v3.11.0-alpine --simplify --diff ci/* 

check-yaml-formatting:
	docker pull ghcr.io/google/yamlfmt:0.17.0
	docker run --rm -v $(PWD):/workspace -u $(UID):$(GID) ghcr.io/google/yamlfmt:0.17.0 -verbose -lint -dstar .github/workflows/*

fix-rust-formatting:
	docker build -t fix-rust-formatting -f ci/fix-rust-formatting.Dockerfile .
	docker run --rm -v $(PWD):/workspace -u $(UID):$(GID) fix-rust-formatting

fix-shell-formatting:
	docker pull mvdan/shfmt:v3.11.0-alpine
	docker run --rm -v $(PWD):/workspace -w /workspace -u $(UID):$(GID) mvdan/shfmt:v3.11.0-alpine --simplify --write ci/* 

fix-yaml-formatting:
	docker pull ghcr.io/google/yamlfmt:0.17.0
	docker run --rm -v $(PWD):/workspace -u $(UID):$(GID) ghcr.io/google/yamlfmt:0.17.0 -verbose -dstar .github/workflows/*

check-rust-linting:
	docker build -t check-rust-linting -f ci/check-rust-linting.Dockerfile .
	docker run --rm -v $(PWD):/workspace -u $(UID):$(GID) check-rust-linting

check-github-actions-workflows-linting:
	docker pull rhysd/actionlint:1.7.7
	docker run --rm -v $(PWD):/workspace -w /workspace -u $(UID):$(GID) rhysd/actionlint:1.7.7 -verbose -color

compile:
	docker build -t compile -f ci/compile.Dockerfile .
	docker run --rm -v $(PWD):/workspace -u $(UID):$(GID) compile

unit-test:
	docker build -t unit-test -f ci/unit-test.Dockerfile .
	docker run --rm -v $(PWD):/workspace -u $(UID):$(GID) unit-test