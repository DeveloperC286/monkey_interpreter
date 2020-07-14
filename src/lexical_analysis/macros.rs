macro_rules! parse_characters {
    ($iterator:expr, $current_character:expr, $valid_character:ident) => {
        let mut characters = vec![$current_character];

        loop {
            match $iterator.peek() {
                Some(character) => {
                    if $valid_character(*character) {
                        characters.push($iterator.next().unwrap());
                    } else {
                        break;
                    }
                }
                None => {
                    break;
                }
            }
        }

        let string = String::from_iter(characters.iter());
        return ($iterator, string);
    };
}

macro_rules! check_next_character {
    ($iterator:expr, $expected_next_character:expr, $literal:expr, $token_type:expr) => {
        match $iterator.peek() {
            Some(next_character) => match next_character {
                $expected_next_character => {
                    $iterator.next();
                    return_token!($iterator, $literal, $token_type);
                }
                _ => {}
            },
            None => {}
        }
    };
}

macro_rules! return_token {
    ($iterator:expr, $literal:expr, $token_type:expr) => {
        return (
            $iterator,
            Token {
                token_type: $token_type,
                literal: $literal.to_string(),
            },
        );
    };
}
