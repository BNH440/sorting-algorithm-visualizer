use textplots::{Chart, Plot, Shape};
use rand::{thread_rng, Rng};
use std::{thread, time};

// rand_array vars
const MAX: i32 = 1000;
const LEN: usize = 30;

fn rand_array() -> [f32; LEN] {
    // Generates a random array of numbers to sort
    let mut rng = thread_rng();

    let mut number_list = [(0f32); LEN];

    for i in 0..number_list.len() {
        number_list[i] = rng.gen_range(0..MAX) as f32;
    }

    return number_list;
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

fn print_chart(number_list: &[f32]) {
    // Print the bar chart to the terminal

    // Create a vector of points to plot
    let mut new_number_list: Vec<(f32, f32)> = Vec::new();

    // Add a -1 to fix the chart
    new_number_list.push((-1f32, 0f32));

    // Add the points to the vector
    for i in 0..number_list.len() {
        new_number_list.push((i as f32, number_list[i]));
    }


    spacer();
    Chart::new_with_y_range(250, 64, 0.0, (number_list.len() - 1) as f32, 0.0, max(&new_number_list))
        .lineplot(&Shape::Bars(&new_number_list[..]))
        .nice();
}

enum SortType {
    Selection,
    Insertion,
    Bubble,
}

fn sort(number_list: &mut [f32], sort_type: SortType) {
    match sort_type {
        SortType::Selection => {
            for i in 0..number_list.len() {
                let mut small = i;
                for j in (i + 1)..number_list.len() {
                    if number_list[j] < number_list[small] {
                        small = j;
                    }
                }
                number_list.swap(small, i);

                print_chart(&number_list);

                delay(100);
            }
        }
        SortType::Insertion => {
            for i in 1..number_list.len() {
                let mut j = i;
                while j > 0 && number_list[j - 1] > number_list[j] {
                    number_list.swap(j, j - 1);
                    j -= 1;

                    print_chart(&number_list);

                    delay(25);
                }
            }
        }
        SortType::Bubble => {
            for i in 0..number_list.len() {
                for j in 0..(number_list.len() - i - 1) {
                    if number_list[j] > number_list[j + 1] {
                        number_list.swap(j, j + 1);

                        print_chart(&number_list);

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
    let mut number_list: Vec<f32>;

    // Handle user input
    if input.trim() == "random" {
        // Generate random number list
        number_list = Vec::from(rand_array());
    }
    else {
        // Parse user input and add to number list
        number_list = input
            .split_whitespace()
            .map(|s| s.parse::<f32>().expect("Error parsing input"))
            .collect();
    }

    // Ask user what sorting algorithm to use
    println!("Enter a sorting algorithm to use: ");
    println!("1. Selection Sort");
    println!("2. Insertion Sort");
    println!("3. Bubble Sort");

    // Get user input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // Parse user input and run the correct sorting algorithm
    match input.trim() {
        "1" => sort(&mut number_list, SortType::Selection),
        "2" => sort(&mut number_list, SortType::Insertion),
        "3" => sort(&mut number_list, SortType::Bubble),
        _ => println!("Invalid input"),
    }
}