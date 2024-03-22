fn loop_finding(s: &str) -> (i32, &str) {
    if s.len() >= 2 {
        let tab: Vec<&str> = s.split("").filter(|x| !x.is_empty()).collect();
        let first = tab.first().unwrap();
        let last = tab.last().unwrap();

        if last == &"(" {
            (0, &s[0..s.len() - 1])
        } else if first == &")" {
            (0, &s[1..s.len()])
        } else {
            (1, &s[1..s.len() - 1])
        }
    } else {
        (0, "")
    }
}

fn test_loop() {
    let mut nb = 0;
    let mut a = loop_finding("())(()))");

    nb += a.0;
    println!("nb: {} str: {}", nb, a.1);
    a = loop_finding(a.1);
    nb += a.0;
    println!("nb: {} str: {}", nb, a.1);
    a = loop_finding(a.1);
    nb += a.0;
    println!("nb: {} str: {}", nb, a.1);
    a = loop_finding(a.1);
    nb += a.0;
    println!("nb: {} str: {}", nb, a.1);
    a = loop_finding(a.1);
    nb += a.0;
    println!("nb: {} str: {}", nb, a.1);
}

fn find_longest(s: &str) -> (i32, i32) {
    // u32
    // end of sub
    let mut layers: i32 = 0;
    let mut loops: i32 = 0;
    let mut last_letter: &str = "";

    for x in s.split("") {
        if x == "(" {
            layers += 1;
        } else if x == ")" && layers > 0 {
            if last_letter == ")" {
                layers = 0;
            }

            layers -= 1;
            loops += 1;
        }
        last_letter = x;
    }

    (layers, loops * 2)
}

fn test() {
    let res_1 = find_longest("()");
    let res_2 = find_longest(")()");
    let res_3 = find_longest("()()");
    let res_4 = find_longest("()()(");
    let res_5 = find_longest("(()())");
    let res_6 = find_longest("(()(())");
    let res_7 = find_longest("())(()))");
    println!("layers: {}, loops: {} for ()", res_1.0, res_1.1);
    println!("layers: {}, loops: {} for )()", res_2.0, res_2.1);
    println!("layers: {}, loops: {} for ()()", res_3.0, res_3.1);
    println!("layers: {}, loops: {} for ()()(", res_4.0, res_4.1);
    println!("layers: {}, loops: {} for (()())", res_5.0, res_5.1);
    println!("layers: {}, loops: {} for (()(())", res_6.0, res_6.1);
    println!("layers: {}, loops: {} for ())(()))", res_7.0, res_7.1);
}
