use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let strings = find_pair(input);

    println!("{}\n{}", strings.0, strings.1);
}

fn find_pair(input: String) -> (String, String) {
    let lines: Vec<&str> = input.split("\n").collect();
    for (i, fst) in lines.iter().enumerate() {
        for snd in lines.iter().skip(i + 1) {
            if diff_by_one(fst, snd) {
                return (fst.to_string(), snd.to_string());
            }
        }
    }
    panic!("lel");
}

fn diff_by_one(fst: &str, snd: &str) -> bool {
    fst.chars().zip(snd.chars()).scan(0, |cnt, (f, s)| {
        if f != s {
            *cnt += 1;
        }
        Some(*cnt)
    }).all(|x| x <= 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diff_by_one_en_una_ok(){
        assert!(diff_by_one("abcde", "jbcde"));
        assert!(diff_by_one("abcde", "abrde"));
        assert!(diff_by_one("abcde", "abcdf"));
    }

    #[test]
    fn diff_by_one_en_una_no_ok(){
        assert!(!diff_by_one("abcde", "abcfg"));
    }

    #[test]
    fn working_test() {
        let example = String::from("abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz");
        assert_eq!(find_pair(example), ("fghij".to_string(), "fguij".to_string()));
    }
}