mod length_of_longest_substring;

fn main() {
    let a: Vec<char> = ('a'..'z').collect();
    for c in a {
        let us = c as usize;
        println!("{}-->{}", c, us);
    }
}
