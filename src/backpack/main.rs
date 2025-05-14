use std::rc::Rc;
use std::thread::{self, JoinHandle};
use std::{
    time::{SystemTime, UNIX_EPOCH},
    vec,
};

#[derive(Debug, Clone)]
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
            if bit_value != 0 {
                size_sum += items[bit_index].size;
                value_sum += items[bit_index].value;
            }
        }
        if size_sum <= capacity && max_value < value_sum {
            max_value = value_sum;
            winning_configuration = i;
        }
    }
    let mut output: Vec<&Item> = Vec::with_capacity(items.len());
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
    let mut output: Vec<&Item> = Vec::with_capacity(items.len());
    for bit_index in 0..items.len() {
        let bit_value = (winning_configuration >> bit_index) & 1u64;
        if bit_value > 0 {
            output.push(&items[bit_index]);
        }
    }
    return output;
}

#[derive(Clone, Copy)]
struct ComputationResult {
    winning_configuration: u64,
    value_sum: u64,
}

fn solve_backpack_multithreaded(items: &Vec<Item>, capacity: u64) -> Vec<&Item> {
    let iterations = 2u64.pow(items.len() as u32) - 1;
    let threads_count = 12;
    let batch_size = iterations / threads_count;
    let thread_handles = (0..threads_count)
        .map(|thread_index| {
            let items = items.clone();
            let start_i = thread_index * batch_size;
            let end_i = start_i + batch_size;
            let thread_handle = thread::spawn(move || -> ComputationResult {
                let mut max_value: u64 = u64::MIN;
                let mut winning_configuration: u64 = 0;
                for i in start_i..end_i {
                    let mut size_sum: u64 = 0;
                    let mut value_sum: u64 = 0;
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
                return ComputationResult {
                    winning_configuration,
                    value_sum: max_value,
                };
            });
            println!("Thread created");
            return thread_handle;
        })
        .collect::<Vec<JoinHandle<ComputationResult>>>();

    let winning_configuration = thread_handles
        .into_iter()
        .map(|thread_handle| thread_handle.join().unwrap())
        .inspect(|thread| println!("Thread completed, best: {}", thread.value_sum))
        .min_by(|ra, rb| rb.value_sum.cmp(&ra.value_sum))
        .take()
        .unwrap()
        .winning_configuration;

    let mut output: Vec<&Item> = Vec::with_capacity(items.len());
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
        Item {
            size: 12,
            value: 24,
        },
        Item { size: 7, value: 13 },
        Item {
            size: 11,
            value: 23,
        },
        Item { size: 8, value: 15 },
        Item { size: 9, value: 16 },
        Item { size: 5, value: 9 },
        Item {
            size: 10,
            value: 19,
        },
        Item { size: 6, value: 10 },
        Item {
            size: 13,
            value: 24,
        },
        Item { size: 9, value: 18 },
        Item {
            size: 16,
            value: 28,
        },
        Item { size: 5, value: 11 },
        Item { size: 8, value: 15 },
        Item {
            size: 13,
            value: 21,
        },
        Item {
            size: 15,
            value: 26,
        },
        Item {
            size: 11,
            value: 19,
        },
        Item { size: 6, value: 11 },
        Item {
            size: 12,
            value: 22,
        },
        Item {
            size: 10,
            value: 18,
        },
        Item {
            size: 14,
            value: 23,
        },
        Item { size: 7, value: 14 },
        Item {
            size: 11,
            value: 21,
        },
        Item {
            size: 17,
            value: 25,
        },
        Item { size: 9, value: 14 },
        Item {
            size: 19,
            value: 29,
        },
        Item { size: 5, value: 9 },
        Item {
            size: 14,
            value: 21,
        },
        Item { size: 8, value: 16 },
        Item {
            size: 11,
            value: 20,
        },
        Item { size: 7, value: 13 },
        Item { size: 6, value: 12 },
    ];

    let capacity = 165;
    solve_backpack_v2(&vec![Item { size: 2, value: 3 }], capacity);

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let result = solve_backpack_multithreaded(&items, capacity);
    // let result = solve_backpack_v3(&items, capacity);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Time: {} ms", (end - start).as_millis());

    println!(
        "Solution: {:?}",
        result.iter().fold(0, |acc, curr| acc + curr.value)
    );
}
