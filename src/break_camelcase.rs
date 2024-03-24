fn solution(s: &str) -> String {
    s.chars()
        .map(|x| match x {
            i if i.is_ascii_uppercase() => format!(" {}", x),
            _ => x.to_string(),
        })
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution("camelCasing"), "camel Casing");
        assert_eq!(solution("camelCasingTest"), "camel Casing Test");
    }
}
