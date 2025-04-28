use rand::seq::SliceRandom;

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
    let mut numbers: Vec<i32> = (1..=20).collect();
    numbers.shuffle(&mut rng);
    let mut my_array: Vec<i32> = numbers[0..10].to_vec();

    // bubble_sort(&mut my_array);
    // selection_sort(&mut my_array);
    insertion_sort(&mut my_array);
}

fn insertion_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 1..len {
        let key = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            visualize(arr, Some(j - 1), Some(j));
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
        visualize(arr, Some(j), None);
    }
    visualize(arr, None, None);
}

fn selection_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        let mut min_idx = i;
        for j in i + 1..len {
            visualize(arr, Some(min_idx), Some(j));
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        arr.swap(i, min_idx);
        visualize(arr, Some(i), Some(min_idx));
    }
    visualize(arr, None, None);
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
