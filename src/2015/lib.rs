// https://adventofcode.com/2015/

use std::collections::HashMap;

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