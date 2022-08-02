use std::collections::HashMap;

fn main() {
    let numbers = vec![1, 5, 7, 8, 1, 2, 8, 9, 2, 2];

    println!("Median: {}", get_median(&numbers));
    println!("Mode: {}", get_mode(&numbers));
}

// Yep, for now, this returns only one mode (it should be fixed later)
fn get_mode(numbers: &Vec<i32>) -> i32 {
    let mut numbers_count: HashMap<i32, u32> = HashMap::new();

    for number in numbers {
        numbers_count
            .entry(*number)
            .and_modify(|count| { *count += 1 })
            .or_insert(1);
    };

    let mut higher = (0, 0);

    for (number, count) in numbers_count {
        if count > higher.1 {
            higher = (number, count);
        }
    };

    higher.0
}

fn get_median(numbers: &Vec<i32>) -> f32 {
    let sorted = sort(numbers);

    let halfway = (sorted.len() / 2) - 1;

    if sorted.len() % 2 != 0 {
        return sorted[halfway] as f32
    }

    (sorted[halfway] as f32 + sorted[halfway + 1] as f32) / 2 as f32
}

fn sort(numbers: &Vec<i32>) -> Vec<i32> {
    let mut numbers = numbers.clone();
    let mut sorted: Vec<i32> = Vec::new();

    for _number in 0..numbers.len() {
        let lower = get_lower(&numbers);

        sorted.push(numbers.remove(lower));
    }

    sorted
}

fn get_lower(numbers: &Vec<i32>) -> usize {
    let mut lower = (0, numbers[0]);

    for (index, number) in numbers.iter().enumerate() {
        if *number < lower.1 {
            lower = (index, *number);
        }
    };

    lower.0
}
