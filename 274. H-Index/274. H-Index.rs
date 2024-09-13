impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len();
        let mut count = vec![0; n + 1];

        // Count the frequency of each citation count
        for &c in citations.iter() {
            if c >= n as i32 {
                count[n] += 1;
            } else {
                count[c as usize] += 1;
            }
        }

        // Calculate the h-index
        let mut total = 0;
        for i in (0..=n).rev() {
            total += count[i];
            if total >= i {
                return i as i32;
            }
        }

        0
    }
}

// The following code is for testing purposes and can be removed if not needed
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h_index() {
        assert_eq!(Solution::h_index(vec![3,0,6,1,5]), 3);
        assert_eq!(Solution::h_index(vec![1,3,1]), 1);
        assert_eq!(Solution::h_index(vec![100]), 1);
        assert_eq!(Solution::h_index(vec![0]), 0);
    }
}

// If you need to run this as a standalone program, you can uncomment and modify this main function:
/*
fn main() {
    println!("{}", Solution::h_index(vec![3,0,6,1,5]));
    println!("{}", Solution::h_index(vec![1,3,1]));
}
*/