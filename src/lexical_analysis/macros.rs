macro_rules! parse_characters {
    ($self:expr, $valid_character:ident) => {
        let mut chars: Vec<char> = vec![];

        loop {
            chars.push($self.current_character.unwrap());

            match $self.next_character {
                Some(character) => {
                    if $valid_character(character) {
                        $self.increment_character_index();
                    } else {
                        break;
                    }
                }
                None => {
                    break;
                }
            }
        }

        let string = String::from_iter(chars.iter());
        return string;
    };
}

macro_rules! check_next_character {
    ($self:expr, $expected_next_character:expr, $literal:expr, $token_type:expr) => {
        match $self.next_character {
            Some(next_character) => match next_character {
                $expected_next_character => {
                    $self.increment_character_index();
                    return_token!($literal, $token_type);
                }
                _ => {}
            },
            None => {}
        }
    };
}

macro_rules! return_token {
    ($literal:expr, $token_type:expr) => {
        return Token {
            token_type: $token_type,
            literal: $literal.to_string(),
        };
    };
}
