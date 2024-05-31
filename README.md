# Monkey Interpreter
![GitHub Release](https://img.shields.io/github/v/release/DeveloperC286/monkey_interpreter)
[![Continuous Integration (CI)](https://github.com/DeveloperC286/monkey_interpreter/actions/workflows/continuous-integration.yml/badge.svg)](https://github.com/DeveloperC286/monkey_interpreter/actions/workflows/continuous-integration.yml)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org)
[![License](https://img.shields.io/badge/License-AGPLv3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)


![The Monkey Programming Language Logo](https://cloud.githubusercontent.com/assets/1013641/22617482/9c60c27c-eb09-11e6-9dfa-b04c7fe498ea.png)


Implementation of an interpreter for the Monkey language written in Rust, currently under active development.


## What is Monkey?
The [Monkey](https://monkeylang.org/) programming language was devised by Thorsten Ball for use in his follow along books [Writing An Interpreter In Go](https://interpreterbook.com) and [Writing A Compiler In Go](https://compilerbook.com/).

Monkey has a C-like syntax, supports **variable bindings**, **prefix** and **infix operators**, has **first-class** and **higher-order functions**, can handle **closures** with ease and has **integers**, **booleans**, **arrays** and **hashes** built-in.


## Content
 * [Usage](#usage)
   + [Usage - Logging](#usage-logging)
 * [Downloading Binary](#downloading-binary)
 * [Compiling via Local Repository](#compiling-via-local-repository)
 * [Unit Testing](#unit-testing)
 * [Issues/Feature Requests](#issuesfeature-requests)


## Usage


### Usage - Logging
The crates `pretty_env_logger` and `log` are used to provide logging.
The environment variable `RUST_LOG` can be used to set the logging level.
See [https://crates.io/crates/pretty_env_logger](https://crates.io/crates/pretty_env_logger) for more detailed documentation.


## Downloading Binary
Statically linked compiled binaries are available for download.
Visit the releases page at [https://gitlab.com/DeveloperC/monkey_interpreter/-/releases](https://gitlab.com/DeveloperC/monkey_interpreter/-/releases) to see all the releases, the release notes contains links to binary downloads for various architectures.

If you do not trust the provided binaries another option is to compile your own and then make it available for remote download, so your CICD etc can then download it.


## Compiling via Local Repository
Checkout the code repository locally, change into the repository's directory and then build via Cargo.
Using the `--release` flag produces an optimised binary but takes longer to compile.

```sh
git clone git@gitlab.com:DeveloperC/monkey_interpreter.git
cd monkey_interpreter/
cargo build --release
```

The compiled binary is present in `target/release/monkey_interpreter`.


## Unit Testing
The unit test suite has several parameterised tests, Cargo is used to set up and run all the unit tests.

```sh
cargo test
```


## Issues/Feature Requests
To report a bug/issue or request a new feature use [https://github.com/DeveloperC286/monkey_interpreter/issues](https://github.com/DeveloperC286/monkey_interpreter/issues).
