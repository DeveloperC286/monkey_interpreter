# Monkey Interpreter
[![pipeline status](https://img.shields.io/badge/Version-0.15.2-blue)](https://gitlab.com/DeveloperC/monkey_interpreter/commits/master) [![pipeline status](https://gitlab.com/DeveloperC/monkey_interpreter/badges/master/pipeline.svg)](https://gitlab.com/DeveloperC/monkey_interpreter/commits/master) [![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org) [![License: AGPL v3](https://img.shields.io/badge/License-AGPLv3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)


![The Monkey Programming Language Logo](https://cloud.githubusercontent.com/assets/1013641/22617482/9c60c27c-eb09-11e6-9dfa-b04c7fe498ea.png)


Implementation of an interpreter for the Monkey language written in Rust, currently under active development.


## What is Monkey?
The [Monkey](https://monkeylang.org/) programming language was devised by Thorsten Ball for use in his follow along books [Writing An Interpreter In Go](https://interpreterbook.com) and [Writing A Compiler In Go](https://compilerbook.com/).

Monkey has a C-like syntax, supports **variable bindings**, **prefix** and **infix operators**, has **first-class** and **higher-order functions**, can handle **closures** with ease and has **integers**, **booleans**, **arrays** and **hashes** built-in.


## Content
 * [Usage](#usage)
   + [Usage - Logging](#usage-logging)
 * [Compiling](#compiling)
 * [Unit Testing](#unit-testing)
 * [Issues/Feature Requests](#issuesfeature-requests)


## Usage
```
target/release/monkey_interpreter
```

## Usage - Logging
The crates `pretty_env_logger` and `log` are used to provide logging.
The environment variable `RUST_LOG` can be used to set the logging level.
See [https://crates.io/crates/pretty_env_logger](https://crates.io/crates/pretty_env_logger) for more detailed documentation.


## Compiling
Checkout the code repository locally, change into the repository's directory and then build via Cargo.
Using the `--release` flag produces an optimised binary but takes longer to compile.

```
git clone git@gitlab.com:DeveloperC/monkey_interpreter.git
cd monkey_interpreter/
cargo build --release
```

The compiled binary is present in `target/release/monkey_interpreter`.


## Unit Testing
The unit test suite has tests for every component of the interpreter for every feature, ensuring correctness.
Cargo is used to set up and run all the unit tests.

```
cargo test
```


## Issues/Feature Requests
To report a bug/issue or request a new feature use [https://gitlab.com/DeveloperC/monkey_interpreter/-/issues](https://gitlab.com/DeveloperC/monkey_interpreter/-/issues).
