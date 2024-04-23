//Implement a function that checks whether a given string is a palindrome or not.

fn check_palindrome(s: &str) -> bool{
    let s = s.to_lowercase(); 
    let s = s.chars().filter(|c| c.is_alphanumeric()).collect::<String>(); 
    let reversed = s.chars().rev().collect::<String>(); 

    return s == reversed
}

fn main() {
    let input1 = "ma'am";
    let input2 = "Hello ma'am";
    println!("Is '{}' a palindrome? {}", input1, check_palindrome(input1));
    println!("Is '{}' a palindrome? {}", input2, check_palindrome(input2));
}