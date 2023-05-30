fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return; // No need to sort if tehre's one element or it's empty
    }
//Last element chosen as pivot
    let pivot_index = partition(arr);
    quick_sort(&mut arr[..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..]);
}

// called to rearrange the elemets => smaller or equal to the pivot
// go to the left of pivot; grater than pivot to the right

fn partition(arr: &mut [i32]) -> usize {
    let pivot_index = arr.len() - 1;
    let mut i = 0;

    for j in 0..pivot_index {
        if arr[j] <= arr[pivot_index] {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, pivot_index);
    i
}

fn main() {
    let mut numbers = [5, 3, 8, 2, 1, 24, -2];

    println!("Before sorting: {:?}", numbers);

    quick_sort(&mut numbers);

    println!("After sorting: {:?}", numbers);
}
