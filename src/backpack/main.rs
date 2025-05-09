use std::{
    time::{SystemTime, UNIX_EPOCH},
    vec,
};

#[derive(Debug)]
struct Item {
    size: u64,
    value: u64,
}

fn solve_backpack(items: &Vec<Item>, capacity: u64) -> Vec<&Item> {
    let iterations = (2 as u64).pow(items.len() as u32) - 1;
    let mut max_value: u64 = u64::MIN;
    let mut winning_configuration: u64 = 0;
    for i in 0..iterations {
        let mut size_sum: u64 = 0;
        let mut value_sum: u64 = 0;
        for bit_index in 0..items.len() {
            let mask = (2 as u64).pow(bit_index as u32);
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

fn solve_backpack_improved(items: &Vec<Item>, capacity: u64) -> Vec<&Item> {
    let iterations = (2 as u64).pow(items.len() as u32) - 1;
    let masks: Vec<u64> = items
        .iter()
        .enumerate()
        .map(|(index, _)| (2 as u64).pow(index as u32))
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

    let mut solutions: Vec<Vec<&Item>> = Vec::with_capacity(10_000);
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    for _ in 0..1_000 {
        solutions.push(solve_backpack(&items, 11));
    }
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Time: {} ms", (end - start).as_millis());

    let mut solutions_2: Vec<Vec<&Item>> = Vec::with_capacity(10_000);
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    for _ in 0..1_000 {
        solutions_2.push(solve_backpack_improved(&items, 11));
    }
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Time improved: {} ms", (end - start).as_millis());

    println!("1: {:?}", solutions[0]);
    println!("2: {:?}", solutions_2[0]);
}
