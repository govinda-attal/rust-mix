struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        // SOL-1
        // if n <= 3 {
        //     return n;
        // }
        // return Self::climb_stairs(n - 1) + Self::climb_stairs(n - 2);

        // SOL-2
        // let mut fib = vec![1,1];
        // for i in fib.len()..=n as usize {
        //     fib.push(fib[i-1] + fib[i-2]);
        // }
        // fib[n as usize]

        // SOL-3

        let mut prv = 0;
        let mut prior_prv;
        let mut curr = 1;
        for _ in 1..=n {
            prior_prv = prv;
            prv = curr;
            curr = prv + prior_prv;
        }
        return curr;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::climb_stairs(1), 1)
    }
    #[test]
    fn test_2() {
        assert_eq!(Solution::climb_stairs(2), 2)
    }
    #[test]
    fn test_3() {
        assert_eq!(Solution::climb_stairs(3), 3)
    }
    #[test]
    fn test_4() {
        assert_eq!(Solution::climb_stairs(4), 5)
    }
    #[test]
    fn test_5() {
        assert_eq!(Solution::climb_stairs(5), 8)
    }

    #[test]
    fn test_44() {
        assert_eq!(Solution::climb_stairs(44), 1134903170)
    }
}
