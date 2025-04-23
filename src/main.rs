use std::{
    io::{Write, stdout},
    thread::sleep,
    time::Duration,
};

use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};

fn main() {
    let mut my_array = [13, 2, 8, 21, 19, 3, 4, 1];
    bubble_sort(&mut my_array);

    println!("The array sorted is {:?}", my_array);
}

fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            visualize(arr, j, j + 1);
            // println!("{:?}, {:?}", j, j+1);
            if arr[j + 1] < arr[j] {
                arr.swap(j, j + 1);
            }
        }
    }
    visualize(arr, 0, 0);
}

fn visualize(arr: &[i32], visual_1: usize, visual_2: usize) {
    clear_log();

    for (i, &val) in arr.iter().enumerate() {
        print!("{:2} |", val);
        for _ in 0..val {
            if i == visual_1 || i == visual_2 {
                print!("#");
            } else {
                print!("-");
            }
        }
        println!();
    }
    stdout().flush().unwrap();
    sleep(Duration::from_millis(700));
    println!();
}

fn clear_log() {
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
}
