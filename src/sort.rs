pub fn bubble_sort(values: &mut Vec<i32>, is_increasing: bool) {
    let mut is_swapped = true;
    while is_swapped {
        is_swapped = false;
        for i in 0..(values.len() - 1) {
            if is_increasing && values[i] > values[i + 1]
                || !is_increasing && values[i] < values[i + 1]
            {
                values.swap(i, i + 1);
                is_swapped = true;
            }
        }
    }
}

pub fn shaker_sort(values: &mut Vec<i32>, is_increasing: bool) {
    let mut left: usize = 0;
    let mut right: usize = values.len() - 1;
    let mut is_swapped = true;

    while left < right && is_swapped {
        is_swapped = false;
        for i in left..right {
            if is_increasing && values[i] > values[i + 1]
                || !is_increasing && values[i] < values[i + 1]
            {
                values.swap(i, i + 1);
                is_swapped = true;
            }
        }
        right -= 1;

        for i in ((left)..right).rev() {
            if is_increasing && values[i] > values[i + 1]
                || !is_increasing && values[i] < values[i + 1]
            {
                values.swap(i, i + 1);
                is_swapped = true;
            }
        }
        left += 1;
    }
}

fn find_number(values: &[i32], boundary: usize, value: i32, is_increasing: bool) -> usize {
    for (i, n) in values.iter().take(boundary).enumerate() {
        if (is_increasing && value < *n) || (!is_increasing && value > *n) {
            return i;
        }
    }
    boundary
}

pub fn insertion_sort(values: &mut Vec<i32>, is_increasing: bool) {
    for i in 1..values.len() {
        values.insert(find_number(values, i, values[i], is_increasing), values[i]);
        values.remove(i + 1);
    }
}
