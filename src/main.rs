use rand::Rng;
use std::time::Instant;

mod sort;

fn main() {
    let mut values: Vec<i32> = Vec::new();
    fill_vector_with_rand_values(&mut values, 10000);

    let mut values_for_sort = values.clone();
    sort_array(&mut values_for_sort, &sort::bubble_sort, "Bubble Sort");

    let mut values_for_sort = values.clone();
    sort_array(&mut values_for_sort, &sort::shaker_sort, "Shaker Sort");

    let mut values_for_sort = values.clone();
    sort_array(
        &mut values_for_sort,
        &sort::insertion_sort,
        "Insertion Sort",
    );
}

fn get_rand_value() -> i32 {
    rand::thread_rng().gen_range(0..100)
}

fn fill_vector_with_rand_values(vector: &mut Vec<i32>, count: usize) {
    for _ in 0..count {
        vector.push(get_rand_value());
    }
}

fn sort_array(vector: &mut Vec<i32>, sort_method: &dyn Fn(&mut Vec<i32>, bool), name: &str) {
    println!("-----------------------------------------------------------------------------------");
    //    println!("Array Before Sorting\n{:?}", &vector);
    println!("\n{} start...", name);
    let time = Instant::now();

    sort_method(vector, true);

    println!("{} finish with time {:?}", name, Instant::now() - time);
    //    println!("\nArray After Sorting\n{:?}", &vector);
    println!("-----------------------------------------------------------------------------------");
}
