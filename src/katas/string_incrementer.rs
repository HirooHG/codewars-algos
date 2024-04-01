pub fn increment_string(s: &str) -> String {
    if s.is_empty() {
        return "1".to_string();
    }

    let mut plus_one = 0;
    let mut word: Vec<char> = Vec::new();
    let mut has_added = false;
    let mut index = s.len();
    let a = s.chars().rev();
    for (i, x) in a.clone().enumerate() {
        if let Ok(parsed) = x.to_string().parse::<i8>() {
            if has_added && plus_one == 0 {
                word.push(parsed.to_string().parse::<char>().unwrap());
                continue;
            }
            has_added = true;
            let mut added = parsed + 1;
            if added == 10 {
                plus_one = 1;
                word.push('0');
            } else {
                plus_one = 0;
                added += plus_one;
                word.push(added.to_string().parse::<char>().unwrap());
            }
        } else {
            index = i;
            break;
        }
    }

    if plus_one == 1 {
        word.push('1');
    }
    let b: Vec<char> = a.skip(index).collect();
    word = [word, b].concat();
    word.reverse();

    if !has_added {
        word.push('1');
    }

    let map: Vec<String> = word.iter().map(|x| x.to_string()).collect();

    map.join("")
}

// solution
mod solution {
    fn increment_string(s: &str) -> String {
        if let Some(last) = s.chars().last() {
            match last.to_digit(10) {
                Some(9) => format!("{}0", &increment_string(&s[..s.len() - 1])),
                Some(num) => format!("{}{}", &s[..s.len() - 1], num + 1),
                None => format!("{s}1"),
            }
        } else {
            format!("1")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::increment_string;

    fn dotest(s: &str, expected: &str) {
        let actual = increment_string(s);
        assert!(
            actual == expected,
            "Test failed with s = \"{s}\"\nExpected \"{expected}\"\nBut got \"{actual}\""
        )
    }

    #[test]
    fn sample_tests() {
        dotest("foo", "foo1");
        dotest("foobar001", "foobar002");
        dotest("foobar1", "foobar2");
        dotest("foobar00", "foobar01");
        dotest("foobar99", "foobar100");
        dotest("foobar099", "foobar100");
        dotest("", "1");
    }
}
