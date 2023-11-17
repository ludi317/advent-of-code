// https://adventofcode.com/2015/

use std::collections::{HashMap, HashSet};
use rand::seq::SliceRandom; // Import the necessary trait
use rand::thread_rng;
pub fn day_19() {
    let input = include_str!("input/raw.txt");
    let lines = input.lines();
    let mut replace_list: Vec<(&str, &str)> = Vec::new();
    let mut parse_molecule = false;
    let mut to_e: HashSet<String> = HashSet::new();
    let mut orig_target = "".to_string();
    for line in lines {
        if line == "" {
            parse_molecule = true;
            continue;
        }
        if parse_molecule {
            orig_target = line.to_string();
        } else {
            let parts: Vec<&str> = line.split(" => ").collect();
            if parts[0] == "e" {
                to_e.insert(parts[1].to_string());
                continue;
            }
            replace_list.push((parts[1], parts[0]));
        }
    }

    let mut count = 0;
    let mut partial_count = 0;
    let mut target:String = orig_target.clone();

    'outerLoop: while !to_e.contains(&target) {
        let prev_target = target.clone();
        (target, partial_count) = many_steps(&replace_list, target);
        if prev_target == target {
            // no progress, try again
            println!("failed to reduce target to nothing, reshuffling replacement order");
            target = orig_target.clone();
            count = 0;
            let mut rng = thread_rng();
            replace_list.shuffle(&mut rng);
            continue 'outerLoop;
        }
        count += partial_count;
        println!("{target} {count}");
    }

    println!("{}", count + 1);
}

fn many_steps(replace_list: &Vec<(&str, &str)>, mut mol: String) -> (String, usize) {
    let mut count = 0;
    for (k, v) in replace_list {
        count += mol.match_indices(k).count();
        mol = mol.replace(k, v);
    }
    (mol, count)
}

fn one_step(replace_map: &HashMap<&str, &str>, mol: &str) -> Vec<String> {
    let mut replaced: Vec<String> = vec![];
    for (k, v) in replace_map {
        let vec: Vec<_> = mol.match_indices(k).collect();
        for (ind, _) in vec {
            let new_mol = format!("{}{}{}", &mol[..ind], v, &mol[(ind + (k.len()))..]);
            replaced.push(new_mol);
        }
    }
    replaced
}
