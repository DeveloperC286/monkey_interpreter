VERSION 0.7


COPY_CI_DATA:
    COMMAND
    COPY --dir "ci/" ".github/" "./"


COPY_METADATA:
    COMMAND
    DO +COPY_CI_DATA
    COPY --dir ".git/" "./"


rust-base:
    FROM rust:1.87.0-alpine3.21@sha256:fa7c28576553c431224a85c897c38f3a6443bd831be37061ab3560d9e797dc82
    RUN apk add --no-cache \
        bash=5.2.37-r0 \
        musl-dev=1.2.5-r9
    RUN rustup component add rustfmt clippy
    WORKDIR "/monkey_interpreter"


check-clean-git-history:
    FROM +rust-base
    # renovate: datasource=github-releases depName=DeveloperC286/clean_git_history
    ENV CLEAN_GIT_HISTORY_VERSION="v1.0.2"
    RUN wget -O - "https://github.com/DeveloperC286/clean_git_history/releases/download/${CLEAN_GIT_HISTORY_VERSION}/x86_64-unknown-linux-musl.tar.gz" | tar xz --directory "/usr/bin/"
    DO +COPY_METADATA
    ARG from="origin/HEAD"
    RUN ./ci/check-clean-git-history.sh "${from}"


check-conventional-commits-linting:
    FROM +rust-base
    # renovate: datasource=github-releases depName=DeveloperC286/conventional_commits_linter
    ENV CONVENTIONAL_COMMITS_LINTER_VERSION="v0.14.3"
    RUN wget -O - "https://github.com/DeveloperC286/conventional_commits_linter/releases/download/${CONVENTIONAL_COMMITS_LINTER_VERSION}/x86_64-unknown-linux-musl.gz" | gzip -d > /usr/bin/conventional_commits_linter && chmod 755 /usr/bin/conventional_commits_linter
    DO +COPY_METADATA
    ARG from_reference="origin/HEAD"
    RUN ./ci/check-conventional-commits-linting.sh --from-reference "${from_reference}"


COPY_SOURCECODE:
    COMMAND
    DO +COPY_CI_DATA
    COPY --if-exists "Cargo.lock" "./"
    COPY --dir "Cargo.toml" "src/" "./"


sourcecode-base:
    FROM +rust-base
    DO +COPY_SOURCECODE


check-rust-formatting:
    FROM +sourcecode-base
    RUN ./ci/check-rust-formatting.sh


golang-base:
    FROM golang:1.24.4@sha256:db5d0afbfb4ab648af2393b92e87eaae9ad5e01132803d80caef91b5752d289c
    WORKDIR "/monkey_interpreter"


shell-formatting-base:
    FROM +golang-base
    # renovate: datasource=github-releases depName=mvdan/sh
    ENV SHFMT_VERSION="v3.11.0"
    RUN go install mvdan.cc/sh/v3/cmd/shfmt@$SHFMT_VERSION
    DO +COPY_CI_DATA


check-shell-formatting:
    FROM +shell-formatting-base
    RUN ./ci/check-shell-formatting.sh


yaml-formatting-base:
    FROM +golang-base
    # renovate: datasource=github-releases depName=google/yamlfmt
    ENV YAMLFMT_VERSION="v0.17.0"
    RUN go install github.com/google/yamlfmt/cmd/yamlfmt@$YAMLFMT_VERSION
    COPY ".yamlfmt" "./"
    DO +COPY_CI_DATA


check-yaml-formatting:
    FROM +yaml-formatting-base
    RUN ./ci/check-yaml-formatting.sh


check-formatting:
    BUILD +check-rust-formatting
    BUILD +check-shell-formatting
    BUILD +check-yaml-formatting


fix-rust-formatting:
    FROM +sourcecode-base
    RUN ./ci/fix-rust-formatting.sh
    SAVE ARTIFACT "src/" AS LOCAL "./"


fix-shell-formatting:
    FROM +shell-formatting-base
    RUN ./ci/fix-shell-formatting.sh
    SAVE ARTIFACT "ci/" AS LOCAL "./"


fix-yaml-formatting:
    FROM +yaml-formatting-base
    RUN ./ci/fix-yaml-formatting.sh
    SAVE ARTIFACT ".github/" AS LOCAL "./"


fix-formatting:
    BUILD +fix-rust-formatting
    BUILD +fix-shell-formatting
    BUILD +fix-yaml-formatting


check-rust-linting:
    FROM +sourcecode-base
    RUN ./ci/check-rust-linting.sh


check-shell-linting:
    FROM +rust-base
    RUN apk add --no-cache \
        shellcheck=0.10.0-r1
    DO +COPY_CI_DATA
    RUN ./ci/check-shell-linting.sh


check-github-actions-workflows-linting:
    FROM +golang-base
    # renovate: datasource=github-releases depName=rhysd/actionlint
    ENV ACTIONLINT_VERSION="v1.7.7"
    RUN go install github.com/rhysd/actionlint/cmd/actionlint@$ACTIONLINT_VERSION
    DO +COPY_CI_DATA
    RUN ./ci/check-github-actions-workflows-linting.sh


check-linting:
    BUILD +check-rust-linting
    BUILD +check-shell-linting
    BUILD +check-github-actions-workflows-linting


compile:
    FROM +sourcecode-base
    RUN ./ci/compile.sh
    SAVE ARTIFACT "target/" AS LOCAL "./"
    SAVE ARTIFACT "Cargo.lock" AS LOCAL "./"


unit-test:
    FROM +sourcecode-base
    RUN ./ci/unit-test.sh
