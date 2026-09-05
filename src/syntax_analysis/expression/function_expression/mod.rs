use log::debug;

use crate::lexical_analysis::model::token::Token;
use crate::syntax_analysis::SyntaxAnalysis;
use crate::syntax_analysis::model::syntax_tree_node::Expression;

impl SyntaxAnalysis<'_> {
    pub(crate) fn parse_function_expression(&mut self) -> anyhow::Result<Expression> {
        debug!("Parsing a function expression.");

        // parse function expression
        assert_token!(
            self,
            Token::Function,
            "A function expression must start with Function token."
        );
        let parameters =
            self.parse_comma_separated_list("function expression's parameters", |expression| {
                match expression {
                    Expression::Identifier { identifier } => Ok(identifier),
                    _ => {
                        anyhow::bail!(
                            "Only allowed identifiers in function expression's parameters."
                        )
                    }
                }
            })?;

        // check function expression was parsed correctly
        let block = self.parse_block()?;

        Ok(Expression::Function {
            parameters,
            block: Box::new(block),
        })
    }
}
