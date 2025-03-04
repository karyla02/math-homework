
use std::collections::HashMap;

fn main() {
    let mut hm = HashMap::new();
    hm.insert("1", 2);
    hm.insert("3", 4);
    println!("{:?}", hm);
}