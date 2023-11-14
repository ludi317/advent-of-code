// https://adventofcode.com/2015/

use std::collections::HashMap;

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
