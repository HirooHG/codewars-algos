pub fn find_odd(arr: &[i32]) -> i32 {
    let mut a: Vec<(&i32, usize)> = arr
        .iter()
        .map(|x| {
            let mut b = arr.to_vec();
            b.retain(|y| x == y);
            (x, b.len())
        })
        .collect();
    a.retain(|x| x.1 % 2 != 0);

    *a.first().unwrap().0
}

mod correction {
    fn find_odd(arr: &[i32]) -> i32 {
        arr.iter().fold(0_i32, |a, v| a ^ v)
    }
}

#[cfg(test)]
mod tests {
    use super::find_odd;

    #[test]
    fn basic_tests() {
        assert_eq!(
            find_odd(&vec![
                20, 1, -1, 2, -2, 3, 3, 5, 5, 1, 2, 4, 20, 4, -1, -2, 5
            ]),
            5
        );
        assert_eq!(find_odd(&vec![1, 1, 2, -2, 5, 2, 4, 4, -1, -2, 5]), -1);
        assert_eq!(find_odd(&vec![20, 1, 1, 2, 2, 3, 3, 5, 5, 4, 20, 4, 5]), 5);
        assert_eq!(find_odd(&vec![10]), 10);
        assert_eq!(find_odd(&vec![1, 1, 1, 1, 1, 1, 10, 1, 1, 1, 1]), 10);
        assert_eq!(find_odd(&vec![5, 4, 3, 2, 1, 5, 4, 3, 2, 10, 10]), 1);
    }
}
