use rand::Rng;

use std::{
    io::{Write, stdout},
    thread::sleep,
    time::Duration,
};

use colored::Colorize;
use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};

fn main() {
    let mut rng = rand::rng();
    let random_array: Vec<i32> = (0..10).map(|_| rng.random_range(1..=20)).collect();
    let mut my_array: [i32; 10] = random_array.try_into().unwrap();

    bubble_sort(&mut my_array);

    println!("The array sorted is {:?}", my_array);
}

fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            visualize(arr, Some(j), Some(j + 1));
            if arr[j + 1] < arr[j] {
                arr.swap(j, j + 1);
            }
        }
    }
    visualize(arr, None, None);
}

fn visualize(arr: &[i32], visual_1: Option<usize>, visual_2: Option<usize>) {
    clear_log();

    for (i, &val) in arr.iter().enumerate() {
        print!("{:2} │", val);
        for _ in 0..val {
            if Some(i) == visual_1 || Some(i) == visual_2 {
                print!("{}", "▇".red());
            } else {
                print!("{}", "▇".blue());
            }
        }
        println!();
    }
    stdout().flush().unwrap();
    sleep(Duration::from_millis(300));
    println!();
}

fn clear_log() {
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
}
