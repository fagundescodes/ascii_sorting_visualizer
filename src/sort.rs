use std::io;

use ratatui::Terminal;

use crate::app::AppState;

pub fn bubble_sort(
    state: &mut AppState,
    terminal: &mut Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
) -> io::Result<()> {
    let len = state.array.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            state.step_count += 1;
            crate::ui::render(state, terminal)?;
            if state.array[j + 1] < state.array[j] {
                state.array.swap(j, j + 1);
            }
        }
    }
    state.step_count += 1;
    crate::ui::render(state, terminal)?;
    Ok(())
}

// pub fn insertion_sort(arr: &mut [i32]) {
//     let len = arr.len();
//     for i in 1..len {
//         let key = arr[i];
//         let mut j = i;
//         while j > 0 && arr[j - 1] > key {
//             crate::ui::render(arr, Some(j - 1), Some(j));
//             arr[j] = arr[j - 1];
//             j -= 1;
//         }
//         arr[j] = key;
//         crate::ui::render(arr, Some(j), None);
//     }
//     crate::ui::render(arr, None, None);
// }
//
// pub fn selection_sort(arr: &mut [i32]) {
//     let len = arr.len();
//     for i in 0..len {
//         let mut min_idx = i;
//         for j in i + 1..len {
//             crate::ui::render(arr, Some(min_idx), Some(j));
//             if arr[j] < arr[min_idx] {
//                 min_idx = j;
//             }
//         }
//         arr.swap(i, min_idx);
//         crate::ui::render(arr, Some(i), Some(min_idx));
//     }
//     crate::ui::render(arr, None, None);
// }
