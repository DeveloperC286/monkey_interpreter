use rstest::rstest;
use rstest_reuse::{self, *};

#[macro_use]
mod macros;

use crate::tests::macros::INIT;

mod boolean_expression;
mod boolean_infix_expression;
mod boolean_prefix_expression;
mod call_expression;
mod code_samples;
mod edge_cases;
mod environment;
mod evaluator_error;
mod fn_expression;
mod identifier_expression;
mod if_expression;
mod integer_expression;
mod integer_infix_boolean_expression;
mod integer_infix_expression;
mod integer_prefix_expression;
mod let_statement;
mod lexical_analysis_error;
mod return_statement;
mod string_expression;
mod string_infix_boolean_expression;
mod string_infix_expression;
mod syntax_analysis_error;
