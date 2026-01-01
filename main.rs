
//create a function average of given numbers and returns its value
fn average(numbers: &[f64]) -> f64 {   
    let sum: f64 = numbers.iter().sum();
    let count = numbers.len() as f64;
    if count == 0.0 {
        0.0
    } else {
        sum / count
    }
}
fn main() {
    println!("Hello, world!");
}