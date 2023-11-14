// https://adventofcode.com/2015/

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
