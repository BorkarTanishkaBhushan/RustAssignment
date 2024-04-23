//Implement a function that finds the longest common prefix of a given set of strings.

fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new(); 
    }

    let mut longest_prefix = strings[0].clone(); 

    for string in strings.iter().skip(1) {
        
        while !string.starts_with(&longest_prefix) {
            longest_prefix.pop();
            if longest_prefix.is_empty() {
                return String::new(); 
            }
        }
    }

    longest_prefix
}

fn main() {
    let strings = vec![
        String::from("laptop"),
        String::from("lamp"),
        String::from("lagoon"),
    ];
    let lcp = longest_common_prefix(&strings);
    println!("Longest Common Prefix: {}", lcp);
}
