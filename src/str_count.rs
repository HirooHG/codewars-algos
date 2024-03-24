use std::collections::HashMap;

fn count(input: &str) -> HashMap<char, i32> {
    let mut map: HashMap<char, i32> = HashMap::new();
    input.chars().for_each(|x| {
        if let Some(y) = map.get_mut(&x) {
            *y += 1;
        } else {
            map.insert(x, 1);
        }
    });
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    #[test]
    fn test_empty_string() {
        let test_input = "";
        let expected: HashMap<char, i32> = HashMap::new();

        assert_eq!(
            count(test_input),
            expected,
            "{ERR_MSG} with input: \"{test_input}\""
        );
    }

    #[test]
    fn test_string_with_two_equal_letters() {
        let test_input = "aa";
        let mut expected: HashMap<char, i32> = HashMap::new();
        expected.insert('a', 2);

        assert_eq!(
            count(test_input),
            expected,
            "{ERR_MSG} with input: \"{test_input}\""
        );
    }

    #[test]
    fn test_string_with_different_letters() {
        let test_input = "aabb";
        let mut expected: HashMap<char, i32> = HashMap::new();
        expected.insert('a', 2);
        expected.insert('b', 2);

        assert_eq!(
            count(test_input),
            expected,
            "{ERR_MSG} with input: \"{test_input}\""
        );
    }
}
