use log::debug;

use crate::lexical_analysis::Token;
use crate::syntax_analysis::Expression;
use crate::syntax_analysis::SyntaxAnalysis;

impl SyntaxAnalysis<'_> {
    pub(super) fn parse_function_expression(&mut self) -> anyhow::Result<Expression> {
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
