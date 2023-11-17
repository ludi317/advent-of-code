use std::collections::{HashMap, HashSet};


use std::mem;

pub fn day_18() {
    let mut grid = [[0u8; 100]; 100];

    let input = include_str!("input/raw.txt");
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char == '#' {
                grid[i][j] = 1;
            }
        }
    }

    let mut out = [[0u8; 100]; 100];
    for _ in 0..100 {
        one_step(grid, &mut out);
        mem::swap(&mut grid, &mut out);
    }

    let mut sum: u16 = 0;
    for row in grid.iter() {
        for &value in row.iter() {
            sum += value as u16;
        }
    }
    println!("{}", sum);
}

fn one_step(grid: [[u8; 100]; 100], out: &mut [[u8; 100]; 100]) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let mut num_neighs = 0;
            let row_min = 0.max(i as i8 - 1) as usize;
            for row in row_min..=(grid.len() - 1).min(i + 1) {
                let col_min = 0.max(j as i8 - 1) as usize;
                for col in col_min..=(grid[0].len() - 1).min(j + 1) {
                    if i == row && col == j {
                        continue;
                    }
                    if grid[row][col] == 1 {
                        num_neighs += 1;
                    }
                }
            }
            out[i][j] = grid[i][j];
            if grid[i][j] == 1 && !(num_neighs == 2 || num_neighs == 3) {
                out[i][j] = 0;
            } else if grid[i][j] == 0 && num_neighs == 3 {
                out[i][j] = 1;
            }
        }
    }
    out[0][0] = 1;
    out[0][99] = 1;
    out[99][0] = 1;
    out[99][99] = 1;

}

pub fn day_17() {
    let input = include_str!("input/raw.txt");
    let mut containers: Vec<i32> = vec![];
    for line in input.lines() {
        containers.push(line.parse().unwrap());
    }
    let liters = 150;
    let mut ans = &mut 0;
    fill(liters, &containers, 0, 0, &mut 0, &mut ans);
    println!("{}", *ans);
}

fn fill(
    total: i32,
    containers: &Vec<i32>,
    idx: usize,
    num_containers: u32,
    min_num_containers: &mut u32,
    ans: &mut u32,
) {
    if total < 0 {
        return;
    }
    if total == 0 {
        if *min_num_containers == 0 || num_containers < *min_num_containers {
            *min_num_containers = num_containers;
            *ans = 1;
        } else if *min_num_containers == num_containers {
            *ans += 1;
        }
        return;
    }
    for i in idx..containers.len() {
        fill(
            total - containers[i],
            containers,
            i + 1,
            num_containers + 1,
            min_num_containers,
            ans,
        );
    }
}


pub fn day_16() {
    let mut clues: HashMap<&str, u8> = HashMap::new();

    clues.insert("children", 3);
    clues.insert("cats", 7);
    clues.insert("samoyeds", 2);
    clues.insert("pomeranians", 3);
    clues.insert("akitas", 0);
    clues.insert("vizslas", 0);
    clues.insert("goldfish", 5);
    clues.insert("trees", 3);
    clues.insert("cars", 2);
    clues.insert("perfumes", 1);

    let input = include_str!("input/raw.txt");
    let mut i = 0;
    'outerLoop: for line in input.lines() {
        i += 1;
        let parts: Vec<&str> = line.split(" ").collect();

        let key1 = &parts[2][..parts[2].len() - 1];
        let value1: u8 = (&parts[3][..parts[3].len() - 1]).parse().unwrap();

        let key2 = &parts[4][..parts[4].len() - 1];
        let value2: u8 = (&parts[5][..parts[5].len() - 1]).parse().unwrap();

        let key3 = &parts[6][..parts[6].len() - 1];
        let value3: u8 = parts[7].parse().unwrap();

        let keys = vec![key1, key2, key3];
        let values = vec![value1, value2, value3];

        for i in 0..3 {
            let clue_value = *clues.get(keys[i]).unwrap();
            match keys[i] {
                "cats" | "trees" => {
                    if values[i] <= clue_value {
                        continue 'outerLoop;
                    }
                }
                "pomeranians" | "goldfish" => {
                    if values[i] >= clue_value {
                        continue 'outerLoop;
                    }
                }
                _ => {
                    if values[i] != clue_value {
                        continue 'outerLoop;
                    }
                }
            }
        }

        println!("{}", i);
        return;
    }
}


pub fn day_15() {
    let tsp = 100;
    // scores is n x p
    let mut scores: Vec<Vec<i32>> = vec![];
    let input = include_str!("input/raw.txt");
    let mut n = 0;
    for line in input.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        scores.push(vec![
            (&parts[2][..parts[2].len() - 1]).parse().unwrap(),
            (&parts[4][..parts[4].len() - 1]).parse().unwrap(),
            (&parts[6][..parts[6].len() - 1]).parse().unwrap(),
            (&parts[8][..parts[8].len() - 1]).parse().unwrap(),
            parts[10].parse().unwrap(),
        ]);
        n += 1;
    }
    for score in &scores {
        println!("{:?}", score);
    }

    let mut counts: Vec<u32> = vec![0u32; n];

    println!("{}", rec(n, &mut counts, tsp, &scores));
}

fn rec(n: usize, counts: &mut Vec<u32>, rem: u32, scores: &Vec<Vec<i32>>) -> u32 {
    if n == 1 {
        let num = counts.len();
        counts[num - 1] = rem;
        let p = scores[0].len();
        let mut sums = vec![0i32; p];
        for i in 0..num {
            for j in 0..p {
                sums[j] += scores[i][j] * counts[i] as i32
            }
        }
        let mut prod: u32 = 1;
        for i in 0..p-1 {
            if sums[i] <= 0 {
                return 0;
            }
            prod *= sums[i] as u32;
        }
        if sums[p-1] != 500 {
            return 0
        }
        return prod;
    }
    let mut result = 0;
    for v in 0..rem {
        let num = counts.len();
        counts[num - n] = v;
        result = result.max(rec(n - 1, counts, rem - v, scores));
    }
    result
}


struct Reindeer {
    time_block: u32,
    dist_block: u32,

    run_pace: u32,
    run_time: u32,
}

pub fn day_14() {
    let input = include_str!("input/raw.txt");
    let mut reindeers: Vec<Reindeer> = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        reindeers.push(Reindeer{
            time_block: parts[6].parse::<u32>().unwrap() + parts[13].parse::<u32>().unwrap(),
            dist_block: parts[3].parse::<u32>().unwrap() * parts[6].parse::<u32>().unwrap(),

            run_pace: parts[3].parse().unwrap(),
            run_time: parts[6].parse().unwrap(),
        })
    }

    let dur: u32 = 2503;
    let mut scores: Vec<u32> = vec![0; reindeers.len()];
    for d in 1..=dur {
        let mut winner_indices: Vec<usize> = Vec::new();
        let mut winner_dist = 0;
        for (i, reindeer) in reindeers.iter().enumerate() {
            let num_blocks = d / reindeer.time_block;
            let mut dist = num_blocks * reindeer.dist_block;
            let rem_secs = d % reindeer.time_block;
            dist += rem_secs.min(reindeer.run_time) * reindeer.run_pace;
            if dist > winner_dist {
                winner_dist = dist;
                winner_indices.clear();
                winner_indices.push(i);
            } else if dist == winner_dist {
                winner_indices.push(i);
            }
        }
        for winner in winner_indices {
            scores[winner] += 1;
        }
    }

    println!("{:?}", scores);

    let result = scores.iter().max().unwrap();

    println!("{}", result);
}

pub fn day_13() {
    let input = include_str!("input/raw.txt");
    let mut name_to_num: HashMap<&str, usize> = HashMap::new();

    let n = 8 + 1;
    let mut edges: Vec<Vec<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        edges.push(vec![0; n]);
    }

    for line in input.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        let mut len = name_to_num.len();
        let from = *name_to_num.entry(parts[0]).or_insert(len);
        len = name_to_num.len();
        let to = *name_to_num
            .entry(&parts[10][..(parts[10].len() - 1)])
            .or_insert(len);
        let mut points: i32 = parts[3].parse().unwrap();
        if parts[2] == "lose" {
            points = -points;
        }
        edges[from][to] = points;
    }


    for edge in &edges {
        println!("{:?}", edge);
    }

    let mut cache: Vec<Vec<Option<i32>>> = Vec::with_capacity(1 << n);
    for _ in 0..1 << n {
        cache.push(vec![None; n]);
    }

    println!("{}", day_13_h(0, &edges, 1 << 0, &mut cache, n - 1))
}

pub fn day_13_h(
    last_node: usize,
    edges: &Vec<Vec<i32>>,
    visited: usize,
    cache: &mut Vec<Vec<Option<i32>>>,
    rem: usize,
) -> i32 {
    if rem == 0 {
        // connect back to 'A'
        return edges[last_node][0] + edges[0][last_node];
    }
    if let Some(points) = cache[visited][last_node] {
        return points;
    }

    let mut result: i32 = i32::MIN;
    for neigh in 0..edges.len() {
        if visited & 1 << neigh == 0 {
            let path = day_13_h(neigh, edges, visited | 1 << neigh, cache, rem - 1);
            // two-way connection
            result = result.max(path + edges[neigh][last_node] + edges[last_node][neigh]);
        }
    }

    cache[visited][last_node] = Some(result);
    result
}


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
