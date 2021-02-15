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
    ($iterator:expr, $expected_next_character:expr, $token_type:expr) => {
        match $iterator.peek() {
            Some($expected_next_character) => {
                $iterator.next();
                return ($iterator, $token_type);
            }
            _ => {}
        }
    };
}
