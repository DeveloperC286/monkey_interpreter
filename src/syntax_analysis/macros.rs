macro_rules! expect_token {
    ($self:expr, $expect_token:expr, $failure_returning:expr) => {
        if $self.current_token.token_type != $expect_token {
            $self
                .syntax_parsing_errors
                .push(format!("Syntax error : Expected a {:?}.", $expect_token));
            return $failure_returning;
        }

        $self.increment_token_index();
    };
}

macro_rules! syntax_error {
    ($self:expr, $syntax_error:expr) => {
        $self.syntax_parsing_errors.push($syntax_error);
        error!("{}", $syntax_error);
    };
}
