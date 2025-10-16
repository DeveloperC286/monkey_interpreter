# Changelog

## 0.17.0 (2025-10-01)

## What's Changed
* refactor: adding lexical analysis logging by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/24
* fix: not returns type mismatch error on non boolean by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/26
* fix: minus returns type mismatch error on non int by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/27
* refactor: removing end of file token by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/28
* refactor: inlining lexical analysis utilities by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/29
* build: adding fix-rust-formatting target by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/30
* build: correcting WORKDIR name by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/31
* ci: adding GitHub Actions workflow formatting check by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/32
* docs: updating repository link to GitHub by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/33
* docs: changing URLs to GitHub by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/34
* docs: removing release binary references by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/35
* ci: removing component name from tag by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/36
* ci: replacing deprecated release-please GitHub Action by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/39
* chore: Configure Renovate by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/40
* chore(deps): update actions/checkout action to v4 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/44
* ci: converting to earthly/actions-setup@v1 by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/47
* docs: removing Continuous Integration (CI) badge by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/48
* fix(deps): update rust crate log to v0.4.22 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/41
* chore(deps): update rust crate rstest to 0.23.0 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/46
* fix(deps): update rust crate thiserror to v1.0.64 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/42
* chore(deps): update rust crate insta to v1.40.0 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/45
* feat: converting error handing to anyhow by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/49
* fix(deps): update rust crate thiserror to v2 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/55
* chore(deps): update rust crate insta to v1.41.1 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/54
* fix(deps): update rust crate anyhow to v1.0.93 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/52
* fix(deps): update rust crate thiserror to v2.0.3 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/56
* ci: Alpine migration for pinning/updating all dependencies by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/57
* ci: adding Renovate GitHub Workflow Earthly updating by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/63
* chore(deps): update earthly/earthly docker tag to v0.8.15 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/64
* ci: removing name referencing Earthly version by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/69
* build: converting to Rust's Alpine image by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/70
* build: correcting WORKDIR name by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/72
* chore(deps): update dependency developerc286/conventional_commits_linter to v0.14.3 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/65
* build: adding cargo --locked and removing --workspace by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/73
* ci: removing Alpine Renovate version updating by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/74
* chore(deps): update rust crate insta to v1.42.0 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/76
* chore(deps): update dependency google/yamlfmt to v0.14.0 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/59
* chore(deps): update dependency mvdan/sh to v3.10.0 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/60
* chore(deps): update dependency rhysd/actionlint to v1.7.6 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/61
* chore(deps): update rust crate rstest to 0.24.0 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/75
* chore(deps): update golang docker tag to v1.23.4 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/62
* fix(deps): update rust crate thiserror to v2.0.9 - autoclosed by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/66
* fix(deps): update rust crate anyhow to v1.0.95 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/67
* ci: pinning actions/checkout at v4.2.2 by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/82
* ci: pinning googleapis/release-please-action at v4.1.3 by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/83
* ci: pinning earthly/actions-setup at v1.0.13 by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/84
* ci: using Earthly GitHub releases by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/85
* chore(deps): pin rust docker tag to 214477e by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/77
* chore(deps): update rust docker tag to v1.83.0 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/71
* ci: Renovate automerge PRs by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/95
* chore(deps): update golang:1.23.4 docker digest to 9820aca by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/91
* chore(deps): update rust:1.83.0-alpine3.20 docker digest to d3f1be1 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/86
* chore(deps): update golang docker tag to v1.23.5 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/92
* chore(deps): update dependency rhysd/actionlint to v1.7.7 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/93
* chore(deps): update rust crate insta to v1.42.1 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/94
* fix(deps): update rust crate log to v0.4.25 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/89
* fix(deps): update rust crate thiserror to v2.0.11 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/87
* chore(deps): update dependency google/yamlfmt to v0.15.0 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/90
* chore(deps): update rust docker tag to v1.84.1 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/88
* chore(deps): update golang:1.23.5 docker digest to e213430 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/96
* chore(deps): update golang docker tag to v1.23.6 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/97
* chore(deps): update golang:1.23.6 docker digest to b2a6f50 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/98
* chore(deps): update golang:1.23.6 docker digest to 958bd2e by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/99
* chore(deps): update golang:1.23.6 docker digest to 9271129 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/100
* chore(deps): pin dependencies by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/101
* chore(deps): update rust crate insta to v1.42.1 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/102
* chore(deps): update dependency google/yamlfmt to v0.16.0 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/103
* chore(deps): update golang docker tag to v1.24.0 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/104
* chore(deps): update golang:1.24.0 docker digest to 2b1cbf2 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/105
* chore(deps): update dependency alpine_3_20/musl-dev to v1.2.5-r1 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/106
* chore(deps): update rust:1.84.1-alpine3.20 docker digest to a1d9745 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/107
* chore(deps): update rust:1.84.1-alpine3.20 docker digest to ac5caa3 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/108
* fix(deps): update rust crate anyhow to v1.0.96 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/109
* fix(deps): update rust crate log to v0.4.26 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/111
* chore(deps): update rust docker tag to v1.85.0 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/110
* chore(deps): update golang:1.24.0 docker digest to 5255fad by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/112
* chore(deps): update golang:1.24.0 docker digest to a14c5a6 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/113
* chore(deps): update golang:1.24.0 docker digest to 58cf31c by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/114
* chore(deps): update golang:1.24.0 docker digest to cd0c949 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/115
* chore(deps): update golang:1.24.0 docker digest to 44b186e by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/117
* chore(deps): update googleapis/release-please-action action to v4.1.4 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/118
* chore(deps): update dependency developerc286/clean_git_history to v1 by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/119
* chore(deps): update golang:1.24.0 docker digest to 3f74443 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/120
* chore(deps): update rust crate insta to v1.42.2 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/121
* chore(deps): update rust crate rstest to v0.25.0 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/122
* fix(deps): update rust crate anyhow to v1.0.97 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/123
* fix(deps): update rust crate thiserror to v2.0.12 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/124
* chore(deps): update rust:1.85.0-alpine3.20 docker digest to f0cef6c by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/125
* chore(deps): update golang docker tag to v1.24.1 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/126
* chore(deps): update googleapis/release-please-action action to v4.1.5 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/127
* chore(deps): update dependency mvdan/sh to v3.11.0 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/128
* chore(deps): update rust:1.85.0-alpine3.20 docker digest to c2f212d by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/129
* chore(deps): update googleapis/release-please-action action to v4.2.0 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/130
* chore(deps): update golang:1.24.1 docker digest to 8678013 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/131
* chore(deps): update golang:1.24.1 docker digest to fa145a3 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/132
* chore(deps): update golang:1.24.1 docker digest to 762bb9c by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/133
* chore(deps): update golang:1.24.1 docker digest to af0bb30 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/134
* chore(deps): update rust docker tag to v1.85.1 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/135
* chore(deps): update rust:1.85.1-alpine3.20 docker digest to 4ec3fed by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/136
* chore(deps): update golang:1.24.1 docker digest to 52ff1b3 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/137
* fix(deps): update rust crate log to v0.4.27 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/138
* chore(deps): update golang docker tag to v1.24.2 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/139
* chore(deps): update rust docker tag to v1.86.0 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/140
* chore(deps): update golang:1.24.2 docker digest to b51b7be by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/141
* chore(deps): update golang:1.24.2 docker digest to fb224f9 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/142
* chore(deps): update golang:1.24.2 docker digest to c0b66cf by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/143
* chore(deps): update golang:1.24.2 docker digest to 1ecc479 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/144
* chore(deps): update golang:1.24.2 docker digest to 18a1f2d by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/145
* fix(deps): update rust crate anyhow to v1.0.98 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/147
* ci: Renovate update Rust's Alpine version regex by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/148
* ci: update inlined Alpine package versions by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/150
* chore(deps): update golang:1.24.2 docker digest to 1ecc479 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/146
* ci: fixing check-clean-git-history.sh argument passing by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/151
* chore(deps): update golang:1.24.2 docker digest to d9db321 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/152
* chore(deps): update rust docker tag to v1.86.0-alpine3.21 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/149
* chore(deps): update rust crate insta to v1.43.0 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/153
* chore(deps): update dependency developerc286/clean_git_history to v1.0.1 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/154
* chore(deps): update golang:1.24.2 docker digest to 8131d99 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/155
* chore(deps): update golang:1.24.2 docker digest to f52b85c by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/156
* chore(deps): update golang:1.24.2 docker digest to 3a060d6 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/157
* chore(deps): update rust crate insta to v1.43.1 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/158
* chore(deps): update golang:1.24.2 docker digest to 30baaea by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/159
* chore(deps): update rust:1.86.0-alpine3.21 docker digest to 8c042ca by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/160
* chore(deps): update rust:1.86.0-alpine3.21 docker digest to 661d708 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/161
* chore(deps): update golang docker tag to v1.24.3 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/162
* chore(deps): update golang:1.24.3 docker digest to 86b4cff by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/164
* chore(deps): update rust docker tag to v1.87.0 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/165
* chore(deps): update dependency developerc286/clean_git_history to v1.0.2 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/166
* chore(deps): update golang:1.24.3 docker digest to 1bcf884 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/167
* chore(deps): update golang:1.24.3 docker digest to 02a2275 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/168
* chore(deps): update golang:1.24.3 docker digest by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/169
* chore(deps): update golang:1.24.3 docker digest to 4c0a181 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/170
* chore(deps): update dependency google/yamlfmt to v0.17.0 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/171
* chore(deps): update golang:1.24.3 docker digest to 81bf592 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/173
* chore(deps): update golang docker tag to v1.24.4 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/174
* chore(config): migrate renovate config by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/163
* chore(deps): update dependency developerc286/clean_git_history to v1.0.3 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/175
* chore(deps): update golang:1.24.4 docker digest to 01f861b by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/177
* chore(deps): update golang:1.24.4 docker digest to db5d0af by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/178
* build: replacing Earthly with Makefile & Docker build/run by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/176
* ci: removing continuous delivery workflow by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/179
* ci: Renovate updating Makefile's Docker Images by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/180
* ci: simplifying mirroring workflow by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/181
* chore(deps): update ghcr.io/google/yamlfmt docker tag to v0.17.1 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/182
* chore(deps): update ghcr.io/google/yamlfmt docker tag to v0.17.2 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/183
* chore(deps): update dependency developerc286/clean_git_history to v1.0.4 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/184
* ci: migrating to clean_git_history Docker image by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/185
* chore(deps): update rust docker tag to v1.88.0 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/186
* ci: conventional commits linter Docker image by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/188
* refactor: GitHub Actions setting permissions by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/189
* build: simplifying Makefile by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/190
* chore(deps): update mvdan/shfmt docker tag to v3.12.0 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/191
* feat: adding verbose CLI option by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/192
* docs: condensed/simplified README.md by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/193
* chore(deps): update rust:1.88.0-alpine3.21 docker digest to 54e937b by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/194
* chore(deps): update rust crate rstest to v0.26.1 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/195
* fix(deps): update rust crate clap to v4.5.43 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/196
* chore(deps): update rust docker tag to v1.89.0 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/197
* chore(deps): update actions/checkout action to v4.3.0 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/198
* chore(deps): update actions/checkout action to v5 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/199
* fix(deps): update rust crate clap to v4.5.44 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/200
* fix(deps): update rust crate thiserror to v2.0.13 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/201
* fix(deps): update rust crate anyhow to v1.0.99 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/202
* fix(deps): update rust crate thiserror to v2.0.14 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/203
* fix(deps): update rust crate clap to v4.5.45 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/204
* fix(deps): update rust crate thiserror to v2.0.15 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/205
* fix(deps): update rust crate thiserror to v2.0.16 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/206
* chore(deps): update googleapis/release-please-action action to v4.3.0 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/207
* fix(deps): update rust crate clap to v4.5.46 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/208
* chore(deps): update rust crate insta to v1.43.2 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/209
* fix(deps): update rust crate clap to v4.5.47 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/210
* fix(deps): update rust crate log to v0.4.28 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/211
* chore(deps): update ghcr.io/developerc286/clean_git_history docker tag to v1.1.0 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/212
* chore(deps): update rust docker tag to v1.90.0 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/214
* fix(deps): update rust crate anyhow to v1.0.100 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/215
* fix(deps): update rust crate clap to v4.5.48 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/216
* ci: adding Claude Code workflows by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/217
* chore(deps): update ghcr.io/developerc286/conventional_commits_linter docker tag to v0.16.0 by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/213
* ci: correcting Convention Commit Linter arguments by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/218
* chore(deps): update anthropics/claude-code-action digest to 426380f by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/219
* chore(deps): update anthropics/claude-code-action digest to 90d189f by @renovate[bot] in https://github.com/DeveloperC286/monkey_interpreter/pull/220

## New Contributors
* @renovate[bot] made their first contribution in https://github.com/DeveloperC286/monkey_interpreter/pull/40

**Full Changelog**: https://github.com/DeveloperC286/monkey_interpreter/compare/v0.16.1...v0.17.0

## 0.16.1 (2024-05-31)

## What's Changed
* ci: removing GitLab CI/CD configuration by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/1
* ci: adding GitHub Actions Workflows linting by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/2
* docs: updating issues URL to GitHub by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/7
* ci: adding GitLab mirroring GitHub Action by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/8
* ci: adding Clean Git History workflow by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/9
* ci: adding Conventional Commits linting workflow by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/10
* ci: adding formatting Continuous Integration by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/11
* build: upgrading to Earthly v0.8.12 by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/13
* ci: using matrix strategy for repeated language jobs by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/12
* ci: adding Rust & Shell linting by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/14
* ci: adding Rust compiling by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/15
* ci: adding Rust unit testing by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/16
* docs: replacing GitLab CI with GitHub Actions badge by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/17
* ci: adding release-please by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/18
* docs: adding latest release badge by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/19
* ci: release-please using project's latest version by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/21
* revert: "ci: release-please using project's latest version (#21)" by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/22
* build: updating dependencies by @DeveloperC286 in https://github.com/DeveloperC286/monkey_interpreter/pull/23

## New Contributors
* @DeveloperC286 made their first contribution in https://github.com/DeveloperC286/monkey_interpreter/pull/1

**Full Changelog**: https://github.com/DeveloperC286/monkey_interpreter/compare/0.10.0...monkey_interpreter-v0.16.1
