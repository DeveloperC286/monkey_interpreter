# Monkey Interpreter
[![pipeline status](https://gitlab.com/DeveloperC/rust-monkey-interpreter/badges/master/pipeline.svg)](https://gitlab.com/DeveloperC/rust-monkey-interpreter/commits/master) [![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

Implementation of an interpreter for the Monkey language written in Rust, currently under active development.

This interpreter is being built with the inspiration and help from the book `Writing an interpreter in Go' by Thorsten Ball. I choose to follow along with the book and use Rust instead of Go so it would be a challange rather than copy and pasting or slightly refactoring the code. Along with the added benefits of Rust not have a garabage collector and no run time cost for abstraction and better performance metrics; which is important in an interpreter. I have currently implemented lexical analysis and am working upon syntax analysis.

The Monkey language was devised by Thorsten Ball for use in his book.
