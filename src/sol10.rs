//Check if a number is prime in Rust

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }

    for i in 2..=(n as f64).sqrt() as u64 {
        if n % i == 0 {
            return false; 
        }
    }
    return true 
}

fn main() {
    let num = 55; 
    if is_prime(num) {
        println!("{} is prime.", num);
    } else {
        println!("{} is not prime.", num);
    }
}

