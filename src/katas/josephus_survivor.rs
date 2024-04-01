mod solution {
    fn josephus_survivor(n: i32, k: i32) -> i32 {
        (1..=n).fold(1, |i, j| (i + k) % j) + 1
    }
}

fn josephus_survivor(n: i32, k: i32) -> i32 {
    let mut index = k.clone();
    let mut vec: Vec<i32> = (1..n + 1).collect();
    while vec.len() > 1 {
        let len = vec.len() as i32;
        println!("{:?}", vec);
        if len < index {
            let a = index % len;
            println!("index {index} len {len}");
            index = if index % len == 0 {
                len.clone()
            } else {
                index % len
            };
            println!("bruh {index} {a}");
        }
        vec.remove((index - 1) as usize);
        index += k - 1;
    }

    *vec.first().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(josephus_survivor(7, 3), 4);
        assert_eq!(josephus_survivor(11, 19), 10);
        assert_eq!(josephus_survivor(40, 3), 28);
        assert_eq!(josephus_survivor(14, 2), 13);
        assert_eq!(josephus_survivor(100, 1), 100);
        assert_eq!(josephus_survivor(1, 300), 1);
        assert_eq!(josephus_survivor(2, 300), 1);
        assert_eq!(josephus_survivor(5, 300), 1);
        assert_eq!(josephus_survivor(7, 300), 7);
        assert_eq!(josephus_survivor(300, 300), 265);
    }
}
