use thiserror::Error;

use crate::lexical_analysis::model::token::Token;

#[derive(Error, Debug)]
pub(crate) enum SyntaxError {
    // Return statement.
    #[error("A return statement must start with Return token.")]
    MissingReturn,

    // Let statement.
    #[error("A let statement must start with Let token.")]
    MissingLet,
    #[error("A let statement must have a variable identifier after the Let token.")]
    MissingLetIdentifier,
    #[error("A let statement must have an assignment operator after the Identifier token.")]
    MissingLetAssignment,

    // Right hand prefix.
    #[error("A prefix expression must have a right hand expression.")]
    MissingRightHandToPrefixExpression,

    // If expression.
    #[error("A if expression must start with If token.")]
    MissingIf,
    #[error("A if expression must have a OpeningRoundBracket token after the If token.")]
    MissingIfOpeningRoundBracket,
    #[error("A if expression must have a ClosingRoundBracket token after the conditional.")]
    MissingIfClosingRoundBracket,

    // Grouped expression.
    #[error("A grouped expression must start with a OpeningRoundBracket token.")]
    MissingGroupedOpeningRoundBracket,
    #[error("A grouped expression must end with a ClosingRoundBracket token.")]
    MissingGroupedClosingRoundBracket,

    // Function expression.
    #[error("A function expression must start with Function token.")]
    MissingFunction,
    #[error(
        "A function expression must have a OpeningRoundBracket token after the Function token."
    )]
    MissingFunctionOpeningRoundBracket,
    #[error("A function expression must have a ClosingRoundBracket token after the parameters.")]
    MissingFunctionClosingRoundBracket,
    #[error("Only allowed identifiers in function expression's parameters.")]
    FunctionParameterNotIdentifier,
    #[error("Parameters must be comma seperated identifiers.")]
    FunctionParameterNotCommaSeperated,
    #[error("FunctionParametersEndedAbruptly.")]
    FunctionParametersEndedAbruptly,

    // Block expression.
    #[error("A block must start with a OpeningCurlyBracket token.")]
    MissingBlockOpeningCurlyBracket,
    #[error("A block must end with a ClosingCurlyBracket token.")]
    MissingBlockClosingCurlyBracket,

    // Call expression.
    #[error("A call expression is not calling either an identifier or an inlined function.")]
    CallExpressionNotIdentifierOrFunction,
    #[error("A call expression must have a OpeningRoundBracket token after the Function token.")]
    MissingCallExpressionOpeningRoundBracket,
    #[error("A call expression must have a ClosingRoundBracket token after the parameters.")]
    MissingCallExpressionClosingRoundBracket,
    #[error("Parameters must be comma seperated identifiers.")]
    CallExpressionParameterNotCommaSeperated,
    #[error("CallExpressionParametersEndedAbruptly.")]
    CallExpressionParametersEndedAbruptly,

    // Expression unparsable.
    #[error("Do not know how to parse {0:?} as an expression.")]
    UnparsableAsExpression(Token),

    // Unparsable.
    #[error("No token to parse.")]
    NoTokenToParse,
}
