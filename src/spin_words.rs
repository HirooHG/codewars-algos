fn spin_words(input: &str) -> String {
    let mut new_tab: Vec<String> = Vec::new();

    let binding = input.to_string();
    let tab: Vec<&str> = binding.split(' ').collect();

    for (_, letter) in tab.iter().enumerate() {
        if letter.len() >= 5 {
            let spinned = spin_word(letter);
            new_tab.push(spinned.to_string());
        } else {
            new_tab.push(letter.to_string());
        }
    }

    new_tab.join(" ")
}

fn spin_word(word: &str) -> String {
    let mut tab: Vec<&str> = word.split("").collect();
    tab.reverse();
    tab.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(spin_words("Welcome"), "emocleW");
        assert_eq!(spin_words("Hey fellow warriors"), "Hey wollef sroirraw");
        assert_eq!(spin_words("This is a test"), "This is a test");
        assert_eq!(spin_words("This is another test"), "This is rehtona test");
        assert_eq!(
            spin_words("You are almost to the last test"),
            "You are tsomla to the last test"
        );
        assert_eq!(
            spin_words("Just kidding there is still one more"),
            "Just gniddik ereht is llits one more"
        );
        assert_eq!(
            spin_words("Seriously this is the last one"),
            "ylsuoireS this is the last one"
        );
    }
}
