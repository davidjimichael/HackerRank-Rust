#[derive(Debug)]
struct Arr {
    arr: Vec::<Vec::<i32>>
}

impl Arr {
    fn new() -> Arr {
        Arr {
            arr: vec![
                vec![0,0,0,0,0,0],
                vec![0,0,0,0,0,0],
                vec![0,0,0,0,0,0],
                vec![0,0,0,0,0,0],
                vec![0,0,0,0,0,0],
                vec![0,0,0,0,0,0]
            ]
        }
    }

    fn from<'a>(lines: Vec::<&'a str>) -> Arr {
        let arr = lines.iter().map(|line| {
            line.split(' ')
                .filter(|x| x.trim().len() > 0)
                .map(|x| x.parse::<i32>().expect("Improper i32"))
                .collect::<Vec::<i32>>()
        }).collect::<Vec::<Vec::<i32>>>();

        Arr { arr }
    }

    /// Returns hourglass sum of Arr
    fn hourglass_sum(&self) -> i32 {
        let mut max: i32 = 0;

        for i in 1..self.arr.len()-1 {
            for j in 1..self.arr.len()-1 {
                let mut s: i32 = 0;
                
                s += self.arr[i-1][j-1];
                s += self.arr[i-1][j];
                s += self.arr[i-1][j+1];

                s +=  self.arr[i][j];
                
                s +=  self.arr[i+1][j-1];
                s +=  self.arr[i+1][j];
                s +=  self.arr[i+1][j+1];
                
                if s > max { max = s; }
            }
        }
        max
    }

}


fn main() {
    println!("{:?}", rstdin());
}

fn rstdin() -> Arr {
    Arr::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_hourglass() {
        assert_eq!(Arr::new().hourglass_sum(), 0);
    }

    #[test]
    fn case0() {
        let v = vec![
            "1 1 1 0 0 0",
            "0 1 0 0 0 0",
            "1 1 1 0 0 0",
            "0 0 0 0 0 0",
            "0 0 0 0 0 0",
            "0 0 0 0 0 0"
        ];

        assert_eq!(Arr::from(v).hourglass_sum(), 7);
    }

    #[test]
    fn case1() {
        let v = vec![
          "-9 -9 -9  1 1 1 ",
          " 0 -9  0  4 3 2 ",
          "-9 -9 -9  1 2 3 ",
          " 0  0  8  6 6 0 ",
          " 0  0  0 -2 0 0 ",
          " 0  0  1  2 4 0 "
        ];

        assert_eq!(Arr::from(v).hourglass_sum(), 28);
    }

    #[test]
    fn case2() {
        let v = vec![
           "1 1 1 0 0 0",
           "0 1 0 0 0 0",
           "1 1 1 0 0 0",
           "0 0 2 4 4 0",
           "0 0 0 2 0 0",
           "0 0 1 2 4 0",
        ];

        assert_eq!(Arr::from(v).hourglass_sum(), 19);
    }
}
