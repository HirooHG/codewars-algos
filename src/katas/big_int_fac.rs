use std::collections::VecDeque;

struct MyBigInt {
    digits: VecDeque<u32>,
}

impl MyBigInt {
    fn from_vec(digits: Vec<u32>) -> Self {
        MyBigInt {
            digits: VecDeque::from(digits),
        }
    }

    fn from_num(x: u32) -> Self {
        let num: Vec<u32> = x
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        MyBigInt::from_vec(num)
    }

    fn mul(&mut self, b: &MyBigInt) {
        // reverse b digits to match self reversed digits
        let b_digits = b
            .digits
            .iter()
            .rev()
            .map(|x| x.to_owned())
            .collect::<VecDeque<u32>>();
        let n = self.digits.len();
        let m = b_digits.len();
        let u = n + m;

        let mut v = VecDeque::new();
        v.resize(u, 0);

        for i in (0..n).rev() {
            for j in (0..m).rev() {
                v[i + j] += self.digits[i] * b_digits[j];
            }
        }

        let mut t = 0; // carry
        for i in 0..u {
            let s = t + v[i]; // take carry and sums it with current
            v[i] = s % 10; // take modulo of 10, a digit no number
            t = s / 10; // take rest into carry
        }

        // remove extra zeros
        let mut i = u - 1;
        while v[i] == 0 {
            v.pop_back();
            i -= 1;
        }

        self.digits = v;
    }
}

impl ToString for MyBigInt {
    fn to_string(&self) -> String {
        self.digits
            .iter()
            .map(|x| char::from_digit(*x, 10).unwrap())
            .rev()
            .collect::<String>()
    }
}

fn factorial(x: u32) -> String {
    let mut y = MyBigInt::from_num(1);

    for i in 1..(x + 1) {
        let big_i = MyBigInt::from_num(i);
        y.mul(&big_i);
    }

    y.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(factorial(1), String::from("1"));
        assert_eq!(factorial(5), String::from("120"));
        assert_eq!(factorial(9), String::from("362880"));
        assert_eq!(factorial(15), String::from("1307674368000"));
        assert_eq!(
            factorial(100),
            "93326215443944152681699238856266700490715968264381621468592963895217599993229915608941463976156518286253697920827223758251185210916864000000000000000000000000"
        );
    }
}
