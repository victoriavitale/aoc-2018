use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let mut pairs = 0;
    let mut triplets = 0;

    for val in input.split("\n") {
        if has_repeated(&val, 2) { pairs += 1 }
        if has_repeated(&val, 3) { triplets += 1 }
    }

    println! ("{}", checksum = pairs * triplets)
}

pub fn has_repeated(box_id: &str, k: u32) -> bool {
 
    let mut counts:HashMap<char, u32> = HashMap::new();
        
    for c in box_id.chars() {
        let n = match counts.get(&c) {
            Some(&v) => v + 1,
            None => 1
        };
        counts.insert(c, n);
    }

    for &val in counts.values(){
        if val == k {
            return true;
        }
    }

    return false;
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn has_none() {
        assert_eq!(has_repeated("abcdef", 2), false);
        assert_eq!(has_repeated("abcdef", 3), false);
    }
    #[test]
    fn has_pair() {
        assert_eq!(has_repeated("baxabc", 2), true);
        assert_eq!(has_repeated("baxabc", 3), false);
    }
    #[test]
    fn has_triplet() {
        assert_eq!(has_repeated("babxbc", 3), true);
        assert_eq!(has_repeated("babxbc", 2), false);
    }
    #[test]
    fn has_both() {
        assert_eq!(has_repeated("bababc", 2), true);
        assert_eq!(has_repeated("bababc", 3), true);
    }
}