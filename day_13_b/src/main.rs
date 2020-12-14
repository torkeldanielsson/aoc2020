fn main() {
    let conditions = vec![
        (48, 613),
        (17, 367),
        (7, 41),
        (11, 37),
        (46, 29),
        (40, 23),
        (36, 19),
        (0, 17),
        (61, 13),
    ];

    //let conditions = vec![(4, 59), (6, 31), (7, 19), (1, 13), (0, 7)];

    let mut a: i64 = 0;

    let mut c = 0;

    loop {
        loop {
            a += 1;
            if (a * conditions[0].1 - conditions[0].0 + conditions[1].0) % conditions[1].1 == 0 {
                break;
            }
        }
        let v = a * conditions[0].1 - conditions[0].0;

        let mut ok = true;
        for condition in &conditions {
            if (v + condition.0) % condition.1 != 0 {
                ok = false;
                break;
            }
        }
        if ok {
            println!("res: {}", v);
            break;
        }

        c += 1;
        if c % 1000000 == 0 {
            println!("v: {}, a: {}", v / 1000000000000, a);
        }
    }
}
