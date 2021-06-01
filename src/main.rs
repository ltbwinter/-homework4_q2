fn sum(vector: &[u32]) -> Option<u32> {
    let mut total: u32 = 0;
    for i in vector.iter() {
        match total.checked_add(*i) {
            Some(sum_result) => total = sum_result,
            None => return None,
        }
    }
    Some(total)
}
fn main() {
    let vector = vec![1, 2, 3, 4, 5];
    println!("VECTOR USED = [1, 2, 3, 4, 5]");
    // let vector = vec![1, 2, 3, 4, 5, 4294967295];
    // println!("VECTOR USED = [1, 2, 3, 4, 5, 4294967295]");

    let vector_sum = sum(&vector);
    let vector_sum_matching_value = match vector_sum {
        Some(value) => value,
        None => 0,
    };
    println!("VECTOR SUM MATCHING VALUE = {}", vector_sum_matching_value);
}
