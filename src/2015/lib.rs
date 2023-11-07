// https://adventofcode.com/2015/day/1

extern crate core;

pub fn day2() -> u32 {
    // read input line by line from file
    let input = include_str!("input/day 2/raw.txt");
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
