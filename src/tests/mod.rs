use rstest::rstest;
use rstest_reuse::{self, *};

#[macro_use]
mod macros;

mod boolean_expression;
mod boolean_infix_expression;
mod boolean_prefix_expression;
mod call_expression;
mod edge_cases;
mod fn_expression;
mod if_expression;
mod integer_expression;
mod integer_infix_boolean_expression;
mod integer_infix_expression;
mod integer_prefix_expression;
mod invalid_escaping_in_string_expression;
mod invalid_string_expression;
mod let_statement;
mod return_statement;
mod string_expression;
mod string_infix_boolean_expression;
mod string_infix_expression;
