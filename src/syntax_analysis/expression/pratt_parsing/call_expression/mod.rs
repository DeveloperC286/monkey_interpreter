use log::debug;

use crate::syntax_analysis::SyntaxAnalysis;
use crate::syntax_analysis::model::syntax_tree_node::Expression;

impl SyntaxAnalysis<'_> {
    pub(crate) fn parse_call_expression(
        &mut self,
        function: Expression,
    ) -> anyhow::Result<Expression> {
        debug!("Parsing a call expression.");

        // check call expression was correctly called by an identifier on inlined function.
        match &function {
            Expression::Identifier { identifier: _ } => {}
            Expression::Function {
                parameters: _,
                block: _,
            } => {}
            _ => {
                anyhow::bail!(
                    "A call expression is not calling either an identifier or an inlined function."
                );
            }
        }

        // parse call expression
        let arguments = self.parse_comma_separated_list("call expression's arguments", Ok)?;

        Ok(Expression::Call {
            function: Box::new(function),
            arguments,
        })
    }
}
