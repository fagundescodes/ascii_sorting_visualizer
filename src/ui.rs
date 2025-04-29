use colored::Colorize;
use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};
use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

pub fn visualize(arr: &[i32], visual_1: Option<usize>, visual_2: Option<usize>) {
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

pub fn clear_log() {
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
}
