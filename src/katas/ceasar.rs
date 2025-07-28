// ASCII letters
const A_LETTER: u8 = 65;
const Z_LETTER: u8 = 90;
const LETTERS: u8 = 26;

struct CaesarCipher {
    shift: u32,
}

impl CaesarCipher {
    fn new(shift: u32) -> CaesarCipher {
        CaesarCipher { shift }
    }

    fn code(message: &str, f: impl Fn(&u8) -> u8) -> String {
        String::from_utf8(
            message
                .to_uppercase()
                .as_bytes()
                .iter()
                .map(|x| {
                    if *x >= A_LETTER && *x <= Z_LETTER {
                        return f(x);
                    }
                    return *x;
                })
                .collect::<Vec<u8>>(),
        )
        .unwrap_or_default()
    }

    fn encode(&self, message: &str) -> String {
        CaesarCipher::code(message, |x| {
            let z = *x + self.shift as u8;
            if z > Z_LETTER {
                // to begin before the A, A being excluded otherwise
                return (A_LETTER - 1) + ((z - Z_LETTER) % LETTERS);
            }
            return z;
        })
    }

    fn decode(&self, message: &str) -> String {
        CaesarCipher::code(message, |x| {
            let z = *x - self.shift as u8;
            if z < A_LETTER {
                // to begin after the Z, Z being excluded otherwise
                return (Z_LETTER + 1) - ((A_LETTER - z) % LETTERS);
            }
            return z;
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(shift: u32, message: &str, expected_encoded: &str, expected_decoded: &str) {
        let cipher = CaesarCipher::new(shift);
        assert_eq!(
            cipher.encode(&message),
            expected_encoded,
            "Encoded message does not match expected message for input: \"{message}\""
        );
        assert_eq!(
            cipher.decode(&expected_encoded),
            expected_decoded,
            "Decoded message does not match expected message for input: \"{expected_encoded}\""
        );
    }

    #[test]
    fn shift_of_5() {
        dotest(5, "Codewars", "HTIJBFWX", "CODEWARS");
        dotest(5, "WAFFLES", "BFKKQJX", "WAFFLES");
        dotest(
            5,
            "IT'S A SHIFT CIPHER!!",
            "NY'X F XMNKY HNUMJW!!",
            "IT'S A SHIFT CIPHER!!",
        );
        dotest(
            5,
            "IT\'S A SHIFT CIPHER!!",
            "NY\'X F XMNKY HNUMJW!!",
            "IT\'S A SHIFT CIPHER!!",
        );
    }

    #[test]
    fn shift_of_13() {
        dotest(13, "CNAPNXRF", "PANCAKES", "CNAPNXRF");
        dotest(13, "JAVASCRIPT", "WNINFPEVCG", "JAVASCRIPT");
    }

    #[test]
    fn simple_test() {
        dotest(0, "Codewars", "CODEWARS", "CODEWARS");
        dotest(1, "", "", "");
        dotest(2, " ", " ", " ");
    }
}
