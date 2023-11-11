// https://adventofcode.com/2015/

pub fn day_10() {
    let mut v = String::from("1113222113");
    let iter = 50;
    for _ in 0..iter {
        // println!("{}", v);
        v = look_and_say(v);
    }
    println!("{}", v.len());
}

fn look_and_say(s: String) -> String {
    assert!(s.len() > 0);
    let mut chars = s.chars();
    let mut prev_char = chars.next().unwrap();
    let mut prev_count = 1;
    let mut out_str: String = Default::default();

    while let Some(c) = chars.next() {
        if c == prev_char {
            prev_count += 1;
        } else {
            out_str.push_str(&*prev_count.to_string());
            out_str.push(prev_char);
            prev_count = 1;
            prev_char = c;
        }
    }

    out_str.push_str(&*prev_count.to_string());
    out_str.push(prev_char);
    out_str
}
