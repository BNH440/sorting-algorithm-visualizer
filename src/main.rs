use textplots::{Chart, Plot, Shape};
use rand::{thread_rng, Rng};
use std::{thread, time};

// rand_array vars
const MAX: i32 = 1000;
const LEN: usize = 30;

fn rand_array() -> [f32; LEN] {
    // Generates a random array of (x, y) coordinates for the chart
    let mut rng = thread_rng();

    let mut arr = [(0f32); LEN];

    for i in 0..arr.len() {
        arr[i] = rng.gen_range(0..MAX) as f32;
    }

    return arr;
}

fn delay(ms: u64) {
    // Delays the thread
    thread::sleep(time::Duration::from_millis(ms));
}

fn max(vec: &[(f32, f32)]) -> f32 {
    // Gets the max value of a vector
    let mut max = 0f32;
    for i in 0..vec.len() {
        if vec[i].1 > max {
            max = vec[i].1;
        }
    }

    return max;
}

fn spacer(){
    // Prints a space in the terminal
    print!("\x1B[2J\x1B[1;1H");
}

fn print_chart(vector: &[f32]) {
    let mut new_vector: Vec<(f32, f32)> = Vec::new();

    new_vector.push((-1f32, 0f32));

    for i in 0..vector.len() {
        new_vector.push((i as f32, vector[i]));
    }


    spacer();
    Chart::new_with_y_range(250, 64, 0.0, (vector.len() - 1) as f32, 0.0, max(&new_vector))
        .lineplot(&Shape::Bars(&new_vector[..]))
        .nice();
}

enum SortType {
    Selection,
    Insertion,
    Bubble,
}

fn sort(vector: &mut [f32], sort_type: SortType) {
    match sort_type {
        SortType::Selection => {
            for i in 0..vector.len() {
                let mut small = i;
                for j in (i + 1)..vector.len() { // TODO: refactor this
                    if vector[j] < vector[small] {
                        small = j;
                    }
                }
                vector.swap(small, i);

                print_chart(&vector);

                delay(100);
            }
        }
        SortType::Insertion => {
            for i in 1..vector.len() {
                let mut j = i;
                while j > 0 && vector[j - 1] > vector[j] {
                    vector.swap(j, j - 1);
                    j -= 1;

                    print_chart(&vector);

                    delay(25);
                }
            }
        }
        SortType::Bubble => {
            for i in 0..vector.len() {
                for j in 0..(vector.len() - i - 1) {
                    if vector[j] > vector[j + 1] {
                        vector.swap(j, j + 1);

                        print_chart(&vector);

                        delay(25);
                    }
                }
            }
        }
    }
}

fn main() {
    println!("Enter a list of numbers to sort seperated by spaces, enter 'random' to generate a random 25 digit list: ");

    // Get user input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // Create new vector to hold numbers to be sorted
    let mut vector: Vec<f32>;

    // Parse user input
    if input.trim() == "random" {
        vector = Vec::from(rand_array());
    }
    else {
        vector = input
            .split_whitespace()
            .map(|s| s.parse::<f32>().expect("Error parsing input"))
            .collect();
    }

    println!("Enter a sorting algorithm to use: ");
    println!("1. Selection Sort");
    println!("2. Insertion Sort");
    println!("3. Bubble Sort");

    // Get user input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "1" => sort(&mut vector, SortType::Selection),
        "2" => sort(&mut vector, SortType::Insertion),
        "3" => sort(&mut vector, SortType::Bubble),
        _ => println!("Invalid input"),
    }
}