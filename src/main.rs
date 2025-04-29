use rand::seq::SliceRandom;
mod sort;
mod ui;

fn main() {
    let mut rng = rand::rng();
    let mut numbers: Vec<i32> = (1..=20).collect();
    numbers.shuffle(&mut rng);
    let mut my_array: Vec<i32> = numbers[0..10].to_vec();

    sort::bubble_sort(&mut my_array);
    // sort::insertion_sort(&mut my_array);
    // sort::selection_sort(&mut my_array);
}
