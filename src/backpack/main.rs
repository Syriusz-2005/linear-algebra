use std::{
    time::{SystemTime, UNIX_EPOCH},
    vec,
};

#[derive(Debug)]
struct Item {
    size: u64,
    value: u64,
}

fn solve_backpack_v1(items: &Vec<Item>, capacity: u64) -> Vec<&Item> {
    let iterations = 2u64.pow(items.len() as u32) - 1;
    let mut max_value: u64 = u64::MIN;
    let mut winning_configuration: u64 = 0;
    for i in 0..iterations {
        let mut size_sum: u64 = 0;
        let mut value_sum: u64 = 0;
        for bit_index in 0..items.len() {
            let mask = 2u64.pow(bit_index as u32);
            let bit_value = i & mask;
            if bit_value > 0 {
                size_sum += items[bit_index].size;
                value_sum += items[bit_index].value;
            }
        }
        if size_sum <= capacity && max_value < value_sum {
            max_value = value_sum;
            winning_configuration = i;
        }
    }
    let mut output: Vec<&Item> = Vec::new();
    for bit_index in 0..items.len() {
        let mask = (2 as u64).pow(bit_index as u32);
        let bit_value = winning_configuration & mask;
        if bit_value > 0 {
            output.push(&items[bit_index]);
        }
    }
    return output;
}

fn solve_backpack_v2(items: &Vec<Item>, capacity: u64) -> Vec<&Item> {
    let iterations = 2u64.pow(items.len() as u32) - 1;
    let masks: Vec<u64> = items
        .iter()
        .enumerate()
        .map(|(index, _)| 2u64.pow(index as u32))
        .collect();
    let mut max_value: u64 = u64::MIN;
    let mut winning_configuration: u64 = 0;
    for i in 0..iterations {
        let mut size_sum: u64 = 0;
        let mut value_sum: u64 = 0;
        for bit_index in 0..items.len() {
            let bit_value = i & masks[bit_index];
            if bit_value > 0 {
                size_sum += items[bit_index].size;
                value_sum += items[bit_index].value;
            }
        }
        if size_sum <= capacity && max_value < value_sum {
            max_value = value_sum;
            winning_configuration = i;
        }
    }
    let mut output: Vec<&Item> = Vec::new();
    for bit_index in 0..items.len() {
        let mask = (2 as u64).pow(bit_index as u32);
        let bit_value = winning_configuration & mask;
        if bit_value > 0 {
            output.push(&items[bit_index]);
        }
    }
    return output;
}

fn solve_backpack_v3(items: &Vec<Item>, capacity: u64) -> Vec<&Item> {
    let iterations = 2u64.pow(items.len() as u32) - 1;
    let mut max_value: u64 = u64::MIN;
    let mut winning_configuration: u64 = 0;
    for i in 0..iterations {
        let mut size_sum: u64 = 0;
        let mut value_sum: u64 = 0;
        // (0..items.len())
        //     .map(|bit_index| (bit_index, (i >> bit_index) & 1u64))
        //     .filter(|(_, bit_value)| bit_value != 0)
        for bit_index in 0..items.len() {
            let bit_value = (i >> bit_index) & 1u64;
            if bit_value > 0 {
                size_sum += items[bit_index].size;
                value_sum += items[bit_index].value;
            }
        }
        if size_sum <= capacity && max_value < value_sum {
            max_value = value_sum;
            winning_configuration = i;
        }
    }
    let mut output: Vec<&Item> = Vec::new();
    for bit_index in 0..items.len() {
        let bit_value = (winning_configuration >> bit_index) & 1u64;
        if bit_value > 0 {
            output.push(&items[bit_index]);
        }
    }
    return output;
}

fn main() {
    println!("Solving backpack...");
    let items = vec![
        Item { size: 2, value: 3 },
        Item { size: 1, value: 2 },
        Item { size: 3, value: 4 },
        Item { size: 1, value: 3 },
        Item { size: 4, value: 5 },
        Item { size: 5, value: 1 },
        Item { size: 1, value: 4 },
        Item { size: 3, value: 5 },
        Item { size: 6, value: 2 },
        Item { size: 3, value: 8 },
        Item { size: 1, value: 2 },
        Item { size: 1, value: 3 },
        Item { size: 1, value: 2 },
    ];

    let iterations = 1_000;
    let capacity = 11;

    let mut solutions: Vec<Vec<&Item>> = Vec::with_capacity(iterations);
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    for _ in 0..iterations {
        solutions.push(solve_backpack_v1(&items, capacity));
    }
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Time v1: {} ms", (end - start).as_millis());

    let mut solutions_2: Vec<Vec<&Item>> = Vec::with_capacity(iterations);
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    for _ in 0..iterations {
        solutions_2.push(solve_backpack_v2(&items, capacity));
    }
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Time v2: {} ms", (end - start).as_millis());

    let mut solutions_3: Vec<Vec<&Item>> = Vec::with_capacity(iterations);
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    for _ in 0..iterations {
        solutions_3.push(solve_backpack_v3(&items, capacity));
    }
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Time v3: {} ms", (end - start).as_millis());

    println!("1: {:?}", solutions[0]);
    println!("2: {:?}", solutions_2[0]);
    println!("3: {:?}", solutions_3[0]);
}
