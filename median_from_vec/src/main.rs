// Excercise from rustbook 8.3
// Given a list of integers, use a vector and return the median
// (when sorted, the value in the middle position)
// and mode (the value that occurs most often;
// a hash map will be helpful here) of the list.

fn main() {
    let mut list: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    list.sort();
    let mid = list.len() / 2;
    let median = if list.len() % 2 == 0 {
        (list[mid] + list[mid - 1]) as f64 / 2.0
    } else {
        list[mid / 2] as f64
    };
    println!("{median}")
}
