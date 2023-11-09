// https://adventofcode.com/2015/


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

