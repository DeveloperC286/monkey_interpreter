macro_rules! semicolon {
    ($self:expr) => {
        match $self.tokens.peek() {
            Some(token) => {
                if **token == Token::SemiColon {
                    trace!("Ignoring expression's semi colon.");
                    $self.tokens.next();
                }
            }
            None => {}
        }
    };
}

macro_rules! assert_token {
    ($self:expr, $expect_token:expr, $failure_returning:expr) => {
        match $self.tokens.next() {
            Some(token) => {
                if *token != $expect_token {
                    return $failure_returning;
                }
            }
            None => {
                return $failure_returning;
            }
        }
    };
}
