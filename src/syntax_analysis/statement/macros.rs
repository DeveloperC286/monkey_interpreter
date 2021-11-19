macro_rules! consume_expression {
    ($self:expr) => {
        match $self.get_expression(ExpressionPrecedence::Lowest) {
            Some(expression) => expression,
            None => {
                return None;
            }
        }
    };
}
