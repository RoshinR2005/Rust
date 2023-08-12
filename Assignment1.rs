fn main() {
    let limit = 100; // Change this to set the limit
    println!("Multiples of 9 up to {}:", limit);

    for i in 1..=limit / 9 {
        let multiple = i * 9;
        println!("{}", multiple);
    }
}
