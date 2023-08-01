struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let len: usize = matrix.len();
        let wid: usize = matrix[0].len();
        let mut a: usize = 0;
        let mut c: usize = wid * len - 1;
        while c >= a {
            let b: usize = (a + c) / 2;
            println!("{:?}", matrix);
            println!("a: {}, b: {}, c: {}", a, b, c);
            if matrix[b / wid][b % wid] > target {
                if b == c {
                    return false;
                }
                c = b;
            } else if matrix[b / wid][b % wid] < target {
                a = b + 1;
            } else {
                return true;
            }
        }
        return false;
    }
}

fn main() {
    // let matrix: Vec<Vec<i32>> = vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]];
    // println!("{:?}", matrix);
    // let target: i32 = 3;
    // let matrix: Vec<Vec<i32>> = vec![vec![1]];
    // println!("{:?}", matrix);
    // let target: i32 = 0;
    let matrix: Vec<Vec<i32>> = vec![vec![1, 1]];
    println!("{:?}", matrix);
    let target: i32 = 0;
    let result = Solution::search_matrix(matrix, target);
    println!("{:?}", result);
}
