// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.


fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let char_list:Vec<char> = input.chars().collect();
    if char_list.len() == 0 {
        return String::from("");
    }
    let mut front_index = 0;
    let mut end_index = char_list.len() - 1;
    let mut empty = true;
    for index in 0..char_list.len() {
        if empty {
            if char_list[index] != ' ' {
                front_index = index;
                empty = false;
            } else {
                continue;
            }
        } else {
            if char_list[index] != ' ' {
                end_index = index + 1;
            }
        }
    }
    if empty {
        return String::from("");
    }
    return String::from_iter(&char_list[front_index..end_index]);
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    let mut new_str = String::from(input);
    new_str.push_str(" world!");
    new_str
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    if let Some(index) = input.find("cars") {
        let front_str = &input[..index];
        let end_index = index + "cars".len();
        let end_str = &input[end_index..];
        format!("{front_str}balloons{end_str}")
    } else {
        String::from(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
