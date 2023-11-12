use std::collections::{HashMap, HashSet};

pub fn day_12() {
    let input = include_str!("input/raw.txt");
    let total = sum_nums(input);
    let red_total = day_12_reds(input);
    println!("{}", total - red_total);
}

pub fn sum_nums(s: &str) -> i32 {
    let mut ans = 0;
    let mut val: i32 = 0;
    let mut neg = 1;
    for c in s.chars() {
        match c {
            '0'..='9' => {
                let d: i32 = (u8::try_from(c).unwrap() - u8::try_from('0').unwrap()) as i32;
                val = 10 * val + d
            }
            '-' => neg = -1,
            _ => {
                ans += val * neg;
                val = 0;
                neg = 1;
            }
        };
    }
    ans += val * neg;
    ans
}

#[derive(Debug)]
struct JsObjInterval {
    start: usize,
    end: usize,
}

fn day_12_reds(s: &str) -> i32 {
    let mut j = 0;
    let mut indices: Vec<JsObjInterval> = vec![];
    while let Some(mut red_start) = s[j..].find("red") {
        red_start += j;
        let mut i = red_start;
        j = red_start + 3;
        let mut curly_count = 0;
        let mut square_count = 0;
        while curly_count != 1 && square_count != 1 {
            i -= 1;
            match s.chars().nth(i).unwrap() {
                '{' => curly_count += 1,
                '}' => curly_count -= 1,
                '[' => square_count += 1,
                ']' => square_count -= 1,
                _ => {}
            }
        }
        if curly_count == 1 {
            curly_count = 0;
            while curly_count != -1 {
                match s.chars().nth(j).unwrap() {
                    '{' => curly_count += 1,
                    '}' => curly_count -= 1,
                    '[' => square_count += 1,
                    ']' => square_count -= 1,
                    _ => {}
                }
                j += 1;
            }
            // eliminate overlapping js objects, only want disjoint intervals
            while indices.len() > 0 && i < indices[indices.len() - 1].end {
                indices.pop();
            }
            indices.push(JsObjInterval { start: i, end: j })
        }
    }
    let mut total = 0;
    for ind in indices {
        total += sum_nums(&s[ind.start..ind.end]);
    }
    total
}

pub fn day_11() {
    // let input = String::from("ghijklmn");
    let input = String::from("vzbxxyzz");
    assert_eq!(input.len(), 8);
    let mut b = [0u8; 8];
    for (i, char) in input.chars().enumerate() {
        b[i] = u8::try_from(char).unwrap() - u8::try_from('a').unwrap();
    }

    incr(&mut b);

    while !streak(&mut b) || !two_pair(&mut b) {
        incr(&mut b)
    }

    for i in 0..b.len() {
        b[i] += u8::try_from('a').unwrap();
    }

    println!("{}", std::str::from_utf8(&b).unwrap())
}

fn streak(b: &mut [u8; 8]) -> bool {
    let mut two_prev = b[0] as i8;
    let mut one_prev = b[1] as i8;
    for i in 2..b.len() {
        let cur = b[i] as i8;
        if two_prev == one_prev - 1 && one_prev == cur - 1 {
            return true;
        }
        two_prev = one_prev;
        one_prev = cur;
    }
    false
}

fn two_pair(b: &mut [u8; 8]) -> bool {
    let mut forward = b.len();
    for i in 0..b.len() - 1 {
        if b[i] == b[i + 1] {
            forward = i;
            break;
        }
    }
    let mut backward = b.len();
    for i in (1..b.len()).rev() {
        if b[i] == b[i - 1] {
            backward = i - 1;
            break;
        }
    }
    backward - forward >= 2
}
// increments and enforces rule 2 (no i, o, or l)
fn incr(b: &mut [u8; 8]) {
    let mut end = b.len() - 1;
    b[end] += 1;

    while b[end] == 26 {
        b[end] = 0;
        end -= 1;
        b[end] += 1
    }
    // assert_ne!(end, -1, "overflow");

    let the_i: u8 = u8::try_from('i').unwrap() - u8::try_from('a').unwrap();
    let the_o: u8 = u8::try_from('o').unwrap() - u8::try_from('a').unwrap();
    let the_l: u8 = u8::try_from('l').unwrap() - u8::try_from('a').unwrap();

    for (i, v) in b.iter().enumerate() {
        if v == &the_i || v == &the_o || v == &the_l {
            b[i] += 1;
            for j in i + 1..b.len() {
                b[j] = 0;
            }
            break;
        }
    }
}

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


pub fn day25() -> u64 {
    let (row, col) = (3010, 3019);
    // go down the rows
    let mut v = row * (row - 1) / 2 + 1;
    // go across the columns
    v += col * (col - 1) / 2 + (col - 1) * row;
    let mut ans = 20151125;
    for _ in 2..=v {
        ans = (ans * 252533) % 33554393
    }
    ans
}

struct Edge {
    node: usize,
    dist: u32,
}


pub fn day9() -> u32 {
    let input = include_str!("input/raw.txt");

    // build edges
    let n = 8;
    let mut edges: Vec<Vec<Edge>> = Vec::with_capacity(n);
    for _ in 0..n {
        edges.push(vec![]);
    }
    let mut name_to_num: HashMap<&str, usize> = HashMap::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(" = ").collect();
        let dist = parts[1].parse().unwrap();
        let locs: Vec<&str> = parts[0].split(" to ").collect();
        let mut len = name_to_num.len();
        let loc0 = *name_to_num.entry(locs[0]).or_insert(len);
        len = name_to_num.len();
        let loc1 = *name_to_num.entry(locs[1]).or_insert(len);
        edges[loc0].push(Edge { node: loc1, dist });
        edges[loc1].push(Edge { node: loc0, dist });
    }
    assert_eq!(name_to_num.len(), n);

    // build cache: 2-D [1<<n][n] : [visited][last]
    let mut cache: Vec<Vec<u32>> = Vec::with_capacity(1 << n);
    for _ in 0..1 << n {
        cache.push(vec![0; n])
    }

    let mut min_dist = u32::MAX;
    for node in 0..n {
        min_dist = min_dist.min(day9_h(node, &edges, 1 << node, &mut cache, n - 1))
    }

    min_dist
}

fn day9_h(
    last_node: usize,
    edges: &Vec<Vec<Edge>>,
    visited: usize,
    cache: &mut Vec<Vec<u32>>,
    rem: usize,
) -> u32 {
    if rem == 0 {
        return 0;
    }
    if cache[visited][last_node] != 0 {
        return cache[visited][last_node];
    }
    let mut result = u32::MAX;

    let neighs: &Vec<Edge> = &edges[last_node];
    for neigh in neighs {
        if visited & 1 << neigh.node == 0 {
            let path_toll = day9_h(neigh.node, edges, visited | 1 << neigh.node, cache, rem - 1);
            result = result.min(path_toll + neigh.dist);
        }
    }

    cache[visited][last_node] = result;
    result
}


pub fn day8_p2() -> usize {
    let input = include_str!("input/raw.txt");
    let mut ans = 0;
    for line in input.lines() {
        let mut delta = 4;
        for char in line[1..line.len()-1].chars() {
            if char == '\\' || char == '\"' {
                delta += 1
            }
        }
        ans += delta
    }
    ans
}


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
