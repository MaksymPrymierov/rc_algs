use std::cmp::Ordering;

pub fn bubble_sort(values: &mut Vec<i32>, order: Ordering) {
    let mut is_swapped = true;
    while is_swapped {
        is_swapped = false;
        for i in 0..(values.len() - 1) {
            if values[i].cmp(&values[i + 1]) == order {
                values.swap(i, i + 1);
                is_swapped = true;
            }
        }
    }
}

pub fn shaker_sort(values: &mut Vec<i32>, order: Ordering) {
    let mut left: usize = 0;
    let mut right: usize = values.len() - 1;
    let mut is_swapped = true;

    while left < right && is_swapped {
        is_swapped = false;
        for i in left..right {
            if values[i].cmp(&values[i + 1]) == order {
                values.swap(i, i + 1);
                is_swapped = true;
            }
        }
        right -= 1;

        for i in ((left)..right).rev() {
            if values[i].cmp(&values[i + 1]) == order {
                values.swap(i, i + 1);
                is_swapped = true;
            }
        }
        left += 1;
    }
}

fn find_number(values: &[i32], boundary: usize, value: i32, order: Ordering) -> usize {
    for (i, n) in values.iter().take(boundary).enumerate() {
        if value.cmp(n) == order.reverse() {
            return i;
        }
    }
    boundary
}

pub fn insertion_sort(values: &mut Vec<i32>, order: Ordering) {
    for i in 1..values.len() {
        values.insert(find_number(values, i, values[i], order), values[i]);
        values.remove(i + 1);
    }
}

pub fn gnome_sort(values: &mut Vec<i32>, order: Ordering) {
    let mut index = 0;

    while index < values.len() - 1 {
        if values[index].cmp(&values[index + 1]) == order {
            values.swap(index, index + 1);
            if index != 0 {
                index -= 1;
            }
        } else {
            index += 1;
        }
    }
}
