macro_rules! semicolon {
    ($self:expr) => {
        match $self.tokens.peek() {
            Some(positioned_token) if positioned_token.token == Token::SemiColon => {
                trace!("Ignoring expression's semi colon.");
                $self.tokens.next();
            }
            _ => {}
        }
    };
}

macro_rules! assert_token {
    ($self:expr, $expect_token:expr, $failure_msg:expr) => {
        match $self.tokens.next() {
            Some(positioned_token) => {
                if positioned_token.token != $expect_token {
                    anyhow::bail!(
                        "{} Found {:?} at {} instead.",
                        $failure_msg,
                        positioned_token.token,
                        positioned_token.position
                    );
                }
            }
            None => {
                anyhow::bail!("{} Reached the end of the code instead.", $failure_msg);
            }
        }
    };
}
