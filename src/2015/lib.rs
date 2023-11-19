// https://adventofcode.com/2015/


pub fn day_20() {
    let mut ans = 0;
    for i in (1..=1000080).rev() {
        if num_presents(i) >= 3_090_909 {
            ans = i;
        }
    }
    println!("{ans}")
}

fn num_presents(house: u32) -> u32 {
    let mut presents = 0;
    let mut i = 1;
    while i * i <= house {
        if house % i == 0 {
            if house / i <= 50 {
                presents += i;
            }
            if i * i != house {
                if i <= 50 {
                    presents += house / i;
                }
            }
        }
        i += 1;
    }
    presents
}
