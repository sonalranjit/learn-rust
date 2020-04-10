fn main() {
    let mut numbers = vec![1, 2, 3, 4,];
    let sum: u8 = numbers.iter().sum();
    println!("The total sum is: {}", sum);

    let avg: f32 = (sum as f32) / (numbers.len() as f32);
    println!("The average is: {}", avg);

    let mut sorted_numbers = &mut numbers;
    sorted_numbers.sort();
    let median_index = (sorted_numbers.len() - 1 ) / 2;
    println!("Median index is {}", median_index);
    let median = sorted_numbers[median_index];
    println!("Median is {}", median);

}
