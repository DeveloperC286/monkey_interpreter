macro_rules! check_next_character {
    ($iterator:expr, $expected_next_character:expr, $token_type:expr) => {
        match $iterator.peek() {
            Some($expected_next_character) => {
                $iterator.next();
                return $token_type;
            }
            _ => {}
        }
    };
}
