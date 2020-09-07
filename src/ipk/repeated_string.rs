#[derive(Debug)]
struct Pattern(String, usize);

fn main () {
    println!("{}", repeating_string(read_stdin()));
}

fn repeating_string(p: Pattern) -> usize {
    let mut c = 0;
    let mut overflow_c = 0;
    let s = p.0.as_bytes();
    let overflow = p.1 % s.len();

    for i in 0..s.len() {
        if s[i] == 'a' as u8 {
            if i < overflow {
                overflow_c += 1;
            }
            c += 1;
        }
    }

    c * (p.1 / s.len()) as usize + overflow_c
}

fn read_stdin() -> Pattern {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n = n.parse::<usize>().unwrap();

    Pattern(s.trim().to_string(), n)
}

#[cfg(test)]
mod tests {
    use super::{Pattern, repeating_string};

    #[test]
    fn case0() {
        assert_eq!(7, repeating_string(Pattern("aba".to_string(), 10)));
    }

    #[test]
    fn case1() {
        assert_eq!(1_000_000_000, repeating_string(Pattern("a".to_string(), 1_000_000_000)));
    }

    #[test]
    fn case2() {
        assert_eq!(0, repeating_string(Pattern("bcdefg".to_string(), 6)));
    }

    #[test]
    fn case3() {
        assert_eq!(0, repeating_string(Pattern("baaaaaaaaaaaaaaaaaaaaaaa".to_string(), 1)));
    }
}

