use std::collections::{HashMap, HashSet};

pub fn day8() -> usize {
    let input = include_str!("input/raw.txt");
    let mut ans = 0;
    for line in input.lines() {
        let mut delta = 2;
        let mut chars = line.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.peek() {
                    Some(&'x') => {
                        delta += 3;
                        chars.next();
                        chars.next();
                        chars.next();
                    }
                    _ => {
                        delta += 1;
                        chars.next();
                    }
                }
            }
        }
        ans += delta
    }
    ans
}

pub fn day7() -> u16 {
    let input = include_str!("input/raw.txt");
    let mut wires: HashMap<&str, &str> = HashMap::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(" -> ").collect();
        wires.insert(parts[1], parts[0]);
    }
    let mut cache: HashMap<&str, u16> = HashMap::new();
    let a_value = eval("a", &wires, &mut cache);
    cache.clear();
    cache.insert("b", a_value);
    eval("a", &wires, &mut cache)
}

// eval resolves a single variable by looking at its wires
fn eval<'a>(
    variable: &'a str,
    wires: &HashMap<&str, &'a str>,
    cache: &mut HashMap<&'a str, u16>,
) -> u16 {
    // look up in cache
    if let Some(&n) = cache.get(variable) {
        return n;
    }

    // base case - plain number
    if let Ok(n) = variable.parse::<u16>() {
        return n;
    }

    let mut result: u16 = 0;
    let expr = wires.get(variable).unwrap();
    let parts: Vec<&str> = expr.split(" ").collect();
    let first = parts.get(0).unwrap();
    match parts.len() {
        1 => {
            // plain number
            result = eval(first, wires, cache)
        }
        2 => {
            // NOT
            assert_eq!(*first, "NOT");
            let second = parts.get(1).unwrap();
            result = !eval(second, wires, cache);
        }
        3 => {
            // a binary operation
            let op = *parts.get(1).unwrap();
            let first_result = eval(first, wires, cache);
            let second_result = eval(parts.get(2).unwrap(), wires, cache);
            match op {
                "AND" => result = first_result & second_result,
                "OR" => result = first_result | second_result,
                "LSHIFT" => result = first_result << second_result,
                "RSHIFT" => result = first_result >> second_result,
                _ => eprintln!("unexpected op code: {}", op),
            }
        }
        _ => eprintln!("unexpected parts length: {}", parts.len()),
    }

    cache.insert(variable, result);
    result
}


pub fn day6() -> u32 {
    // initialize grid
    let mut grid = [[0u32; 1000]; 1000];
    let input = include_str!("input/raw.txt");
    let (mut r1, mut c1, mut r2, mut c2);
    let mut num_idx;
    for line in input.lines() {
        let v: Vec<&str> = line.split_terminator(&[' ', ','][..]).collect();
        let f = match v[1] {
            "on" => {
                num_idx = 2;
                |x| x + 1
            }
            "off" => {
                num_idx = 2;
                |x| {
                    if x == 0 {
                        return 0
                    }
                    x - 1
                }
            }
            _ => { // toggle
                num_idx = 1;
                |x| x + 2
            }
        };
        (r1, c1, r2, c2) = (
            v[num_idx].parse().unwrap(),
            v[num_idx + 1].parse().unwrap(),
            v[num_idx + 3].parse().unwrap(),
            v[num_idx + 4].parse().unwrap(),
        );
        alter_grid(&mut grid, f, r1, c1, r2, c2);
    }
    // count 1s
    let mut count = 0;
    for r in 0..1000 {
        for c in 0..1000 {
            count += grid[r][c];
        }
    }
    count
}

pub fn alter_grid<F: Fn(u32) -> u32>(
    grid: &mut [[u32; 1000]; 1000],
    f: F,
    r1: usize,
    c1: usize,
    r2: usize,
    c2: usize,
) {
    for r in r1..=r2 {
        for c in c1..=c2 {
            grid[r][c] = f(grid[r][c])
        }
    }
}


pub fn day5() -> u32 {
    println!("{}", is_valid2("ieodomkazucvgmuy"));
    let input = include_str!("input/raw.txt");
    let mut total = 0;
    for line in input.lines() {
        if is_valid2(line) {
            total += 1;
        }
    }
    total
}

fn is_valid2(s: &str) -> bool {
    let mut map: HashMap<&str, usize> = HashMap::new();
    let mut pair = false;

    for end in 2..=s.len() {
        // pair
        let part = &s[end-2..end];
        if let Some(idx) = map.get(part) {
            if end - idx >= 2 {
                pair = true;
                break
            }
        }
        map.insert(part, end);
    }

    if !pair {
        return false
    }

    let mut chars = s.chars();
    let mut two_prev = chars.next().unwrap();
    let mut one_prev = chars.next().unwrap();
    for char in chars {
        if two_prev == char {
            return true
        }
        two_prev = one_prev;
        one_prev = char;
    }

    false
}


fn is_valid(s: &str) -> bool {
    let mut chars = s.chars();
    let mut num_vowels = 0;
    let first = chars.next().unwrap();
    if is_vowel(first) {
        num_vowels = 1;
    }
    let mut prev = first;
    let mut double = false;
    for char in chars {
        if is_vowel(char) {
            num_vowels += 1;
        }
        if prev == char {
            double = true;
        }
        if (prev == 'a' && char == 'b')
            || (prev == 'c' && char == 'd')
            || (prev == 'p' && char == 'q')
            || (prev == 'x' && char == 'y')
        {
            return false
        }
        prev = char;
    }
    num_vowels >= 3 && double
}

fn is_vowel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}

pub fn day4() -> u32 {
    let mut data = String::from("ckczppom");
    let prefix_len = data.len();
    let mut i = 1;
    loop {
        data.push_str(&*i.to_string());
        let digest = md5::compute(data.as_bytes());
        if digest[0] == 0 && digest[1] == 0 && digest[2] == 0 {
            println!("MD5 hash: {:x}", digest);
            return i;
        }
        data.truncate(prefix_len);
        i += 1;
    }
}

pub fn day3() -> usize {
    let mut set: HashSet<(i32, i32)> = HashSet::new();

    let mut posA: (i32, i32) = (0, 0);
    let mut posB: (i32, i32) = (0, 0);

    set.insert(posA);
    let mut pos: (i32, i32) = (0, 0);
    let input = include_str!("input/raw.txt");
    for (i, char) in input.chars().enumerate() {
        if i % 2 == 0 {
            posB = pos;
            pos = posA;
        } else {
            posA = pos;
            pos = posB;
        }
        match char {
            'v' => pos.0 += 1,
            '^' => pos.0 -= 1,
            '>' => pos.1 += 1,
            '<' => pos.1 -= 1,
            _ => println!("unrecognized char: {}", char),
        }
        set.insert(pos);
    }

    return set.len();
}

pub fn day2() -> u32 {
    // read input line by line from file
    let input = include_str!("input/raw.txt");
    let mut total = 0;
    for line in input.lines() {
        let mut dimensions: Vec<u32> = line.split('x').map(|x| x.parse().unwrap()).collect();
        dimensions.sort();
        let [a, b, c] = <[u32; 3]>::try_from(dimensions).ok().unwrap();
        total += 2 * a + 2 * b + a * b * c;
    }
    total
}

pub fn day1() -> i32 {
    let input = "()((((";
    println!("{}", input.len());
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        if c == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }
        if floor == -1 {
            return (i + 1) as i32;
        }
    }
    return -1;
}
