pub fn likes(names: &[&str]) -> String {
    match names.len() {
        1 => format!("{} likes this", names.first().unwrap()),
        2 => format!(
            "{} and {} like this",
            names.first().unwrap(),
            names.get(1).unwrap()
        ),
        3 => format!(
            "{}, {} and {} like this",
            names.first().unwrap(),
            names.get(1).unwrap(),
            names.get(2).unwrap()
        ),
        i if i > 3 => format!(
            "{}, {} and {} others like this",
            names[0],
            names[1],
            names.len() - 2
        ),
        _ => "no one likes this".to_string(),
    }
}

mod correction {
    fn likes(names: &[&str]) -> String {
        match names {
            [] => format!("no one likes this"),
            [a] => format!("{} likes this", a),
            [a, b] => format!("{} and {} like this", a, b),
            [a, b, c] => format!("{}, {} and {} like this", a, b, c),
            [a, b, rest @ ..] => format!("{}, {} and {} others like this", a, b, rest.len()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::likes;

    #[test]
    fn example_tests() {
        assert_eq!(likes(&[]), "no one likes this");
        assert_eq!(likes(&["Peter"]), "Peter likes this");
        assert_eq!(likes(&["Jacob", "Alex"]), "Jacob and Alex like this");
        assert_eq!(
            likes(&["Max", "John", "Mark"]),
            "Max, John and Mark like this"
        );
        assert_eq!(
            likes(&["Alex", "Jacob", "Mark", "Max"]),
            "Alex, Jacob and 2 others like this"
        );
    }
}
