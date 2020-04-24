macro_rules! expect_token {
    ($self:expr, $expect_token:expr) => {
        if $self.current_token.token_type != $expect_token {
            $self
                .syntax_parsing_errors
                .push(format!("Syntax error : Expected a {:?}.", $expect_token));
            return None;
        }

        $self.increment_token_index();
    };
}
