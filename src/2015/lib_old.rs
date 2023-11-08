
use std::collections::{HashMap, HashSet};


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
