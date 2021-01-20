# Monkey Interpreter
[![pipeline status](https://img.shields.io/badge/Version-0.13.0-blue)](https://gitlab.com/DeveloperC/monkey_interpreter/commits/master) [![pipeline status](https://gitlab.com/DeveloperC/monkey_interpreter/badges/master/pipeline.svg)](https://gitlab.com/DeveloperC/monkey_interpreter/commits/master) [![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org) [![License: AGPL v3](https://img.shields.io/badge/License-AGPLv3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)


![The Monkey Programming Language Logo](https://cloud.githubusercontent.com/assets/1013641/22617482/9c60c27c-eb09-11e6-9dfa-b04c7fe498ea.png)


Implementation of an interpreter for the Monkey language written in Rust, currently under active development.


## What is Monkey?
The Monkey language was devised by Thorsten Ball for use in his book [Writing An Interpreter In Go](https://interpreterbook.com/#the-monkey-programming-language).

Monkey has a C-like syntax, supports **variable bindings**, **prefix** and **infix operators**, has **first-class** and **higher-order functions**, can handle **closures** with ease and has **integers**, **booleans**, **arrays** and **hashes** built-in.


## Why?
I am developing this implementation of an interpreter to have a large complex project to pratice real world Rust and also learn more about compilers/interpreter.

I choose to follow along with the book and use Rust instead of Go so it would be a challange rather than copy and pasting or slightly refactoring the code. Along with the added benefits of Rust not have a garabage collector and no run time cost for abstraction and better performance metrics; which is important in an interpreter.


## Building
```
cargo build --release
```


## Testing
```
cargo test
```


## Running the Interactive Interpreter
```
target/release/monkey_interpreter
```


## Running with Logging
You can increase or decrease the level of the logs by altering the enviroment variable 'RUST_LOG'.

```
RUST_LOG=trace target/release/monkey_interpreter
```
