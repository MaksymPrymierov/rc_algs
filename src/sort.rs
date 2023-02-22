/*!
# Sorting module

This module provides implementations for several sorting algorithms: Bubble sort, Shaker sort,
Insertion sort, and Gnome sort. Each sorting function takes a mutable vector of i32 values and an
Ordering enum to specify whether the vector should be sorted in ascending or descending order. The
code also includes a helper function, [find_number], which is used by the insertion sort function
to determine the correct index for inserting each element.
*/

use std::cmp::Ordering;

/**
Bubble sort

This function implements bubble sort to sort a mutable vector of i32 values in either ascending or
descending order.
**/
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

/**
Shaker sort

This function implements shaker sort to sort a mutable vector of i32 values in either ascending or
descending order. It uses a bidirectional approach to reduce the number of iterations.
 **/
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

/**
Find number

This function searches for the index of the first number in a slice of i32 values that is either
greater or less than a given value depending on the specified order. It searches only within the
first "boundary" elements of the slice. If no such number is found within the given boundary, it
returns the value of the boundary.
 **/
fn find_number(values: &[i32], boundary: usize, value: i32, order: Ordering) -> usize {
    for (i, n) in values.iter().take(boundary).enumerate() {
        if value.cmp(n) == order.reverse() {
            return i;
        }
    }
    boundary
}

/**
Insertion Sort

This function implements insertion sort to sort a mutable vector of i32 values in either ascending
or descending order. It iterates over the vector and inserts each element into the correct position
within the already-sorted portion of the vector using the [find_number] function to determine the
correct index. It removes the original element at the end of each iteration to maintain the vector
length.
**/
pub fn insertion_sort(values: &mut Vec<i32>, order: Ordering) {
    for i in 1..values.len() {
        values.insert(find_number(values, i, values[i], order), values[i]);
        values.remove(i + 1);
    }
}

/**
Gnome sort

This function implements gnome sort to sort a mutable vector of i32 values in either ascending or
descending order. It iterates over the vector and compares each adjacent pair of elements, swapping
them if they are in the wrong order. If a swap is made, it moves the index back one position,
unless it is already at the beginning of the vector. If no swap is made, it moves the index
forward one position. It continues this process until the entire vector is sorted.
**/
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

pub fn merge_sort(values: &mut [i32], order: Ordering) {
    let mid = values.len() / 2;

    match values.len() {
        n if n > 3 => {
            merge_sort(&mut values[..mid], order);
            merge_sort(&mut values[mid..], order);

            let mut index = 0;
            while index < mid {
                let min = find_number(values, mid + index, values[mid + index], order);
                if min < mid + index {
                    values.swap(min, mid + index);
                } else {
                    index += 1;
                }
            }
        }
        3 => {
            if values[0].cmp(&values[1]) == order {
                values.swap(0, 1);
            }
            if values[1].cmp(&values[2]) == order {
                values.swap(1, 2);
            }
            if values[0].cmp(&values[1]) == order {
                values.swap(0, 1);
            }
        }
        2 => {
            if values[0].cmp(&values[1]) == order {
                values.swap(0, 1);
            }
        }
        _ => {}
    }
}
