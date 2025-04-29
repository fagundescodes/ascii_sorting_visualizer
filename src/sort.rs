pub fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            crate::ui::visualize(arr, Some(j), Some(j + 1));
            if arr[j + 1] < arr[j] {
                arr.swap(j, j + 1);
            }
        }
    }
    crate::ui::visualize(arr, None, None);
}

pub fn insertion_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 1..len {
        let key = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            crate::ui::visualize(arr, Some(j - 1), Some(j));
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
        crate::ui::visualize(arr, Some(j), None);
    }
    crate::ui::visualize(arr, None, None);
}

pub fn selection_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        let mut min_idx = i;
        for j in i + 1..len {
            crate::ui::visualize(arr, Some(min_idx), Some(j));
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        arr.swap(i, min_idx);
        crate::ui::visualize(arr, Some(i), Some(min_idx));
    }
    crate::ui::visualize(arr, None, None);
}
