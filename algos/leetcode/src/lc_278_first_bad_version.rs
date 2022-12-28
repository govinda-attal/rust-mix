struct Solution {
    bad_version: i32,
}

impl Solution {
    pub fn new(bv: i32) -> Self {
        Self {
            bad_version: bv,
        }
    }

    fn is_bad_version(&self, v: i32) -> bool {
        self.bad_version <= v
    }

    pub fn get_first_bad_version(&self, n: i32) -> i32 {
        let mut good = 1;
        let mut bad = n;
        loop {
            let mid = (bad + good)/ 2;
            if self.is_bad_version(mid) {
                bad = mid
            } else {
                good = mid
            }
            if good + 1 == bad {
                return bad
            }
            if good == bad {
                return bad
            }
        }
    } 
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn test_bad_version() {
        let sol = Solution::new(4);
        let bv = sol.get_first_bad_version(5);
        assert_eq!(bv, 4)
    }

    #[test]
    fn test_first() {
        let sol = Solution::new(1);
        let bv = sol.get_first_bad_version(1);
        assert_eq!(bv, 1)
    }
}