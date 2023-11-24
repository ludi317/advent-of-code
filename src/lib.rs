// https://adventofcode.com/2015/

struct Purchase  {
    cost: u32,
    damage: u32,
    armor: u32
}

pub fn day_21() {
    let mut weapons: Vec<Purchase> = Vec::new();
    weapons.push(Purchase{   cost: 8, damage:     4,armor:       0});
    weapons.push(Purchase{   cost: 10, damage:     5,armor:       0});
    weapons.push(Purchase{    cost: 25, damage:     6,armor:       0});
    weapons.push(Purchase{    cost: 40, damage:     7,armor:       0});
    weapons.push(Purchase{    cost: 74, damage:     8,armor:       0});

    let mut armor: Vec<Purchase> = Vec::new();
    armor.push(Purchase{cost:      13, damage:     0, armor:       1});
    armor.push(Purchase{cost:    31, damage:     0, armor:       2});
    armor.push(Purchase{cost:   53, damage:     0, armor:       3});
    armor.push(Purchase{cost:   75, damage:     0, armor:       4});
    armor.push(Purchase{cost:  102, damage:     0, armor:       5});
    // optional
    armor.push(Purchase{cost:  0, damage:     0, armor:       0});

    let mut rings: Vec<Purchase> = Vec::new();
    // 1 ring
    rings.push(Purchase{cost:    25, damage:     1, armor:       0});
    rings.push(Purchase{cost:    50, damage:     2, armor:       0});
    rings.push(Purchase{cost:   100, damage:     3, armor:       0});
    rings.push(Purchase{cost:   20, damage:     0, armor:       1});
    rings.push(Purchase{cost:   40, damage:     0, armor:       2});
    rings.push(Purchase{cost:  80, damage:     0, armor:       3});

    // 2 rings
    let rlen = rings.len();
    for i in 0..rlen {
        for j in i+1..rlen {
            rings.push(Purchase{cost:rings[i].cost + rings[j].cost, damage: rings[i].damage + rings[j].damage, armor: rings[i].armor + rings[j].armor })
        }
    }

    // 0 rings
    rings.push(Purchase{cost:  0, damage:     0, armor:       0});

    let mut the_cost = 0;
    for w in &weapons {
        for a in &armor {
            for r in &rings {
                let my_damage = w.damage + a.damage + r.damage;
                let my_armor = w.armor + a.armor + r.armor;
                if my_armor + my_damage < 10 {
                    let my_cost = w.cost + a.cost + r.cost;
                    the_cost = the_cost.max(my_cost);
                }
            }
        }
    }
    println!("{the_cost}");
}