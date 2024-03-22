fn main() {
    println!("{}", to_camel_case("the-stealth-warrior"));
}

fn to_camel_case(text: &str) -> String {
    let mut is_first_passed = false;

    let tab: Vec<String> = text
        .split_terminator(&['-', '_'][..])
        .map(|x| {
            if !is_first_passed {
                is_first_passed = true;
                return x.to_string();
            }
            let mut c = x.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect();
    tab.join("")
}

#[cfg(test)]
mod tests {
    use super::to_camel_case;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(s: &str, expected: &str) {
        assert_eq!(to_camel_case(s), expected, "{ERR_MSG} with text = \"{s}\"")
    }

    #[test]
    fn fixed_tests() {
        dotest("", "");
        dotest("the_stealth_warrior", "theStealthWarrior");
        dotest("The-Stealth-Warrior", "TheStealthWarrior");
        dotest("A-B-C", "ABC");
    }
}
