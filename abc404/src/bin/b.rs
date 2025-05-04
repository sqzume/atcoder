use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
        t: [Chars; n],
    };

    let mut count_vec: Vec<i32> = vec![];
    let mut s_rotate = vec![vec![vec![' '; n]; n]; 4];

    for i in 0..n {
        for j in 0..n {
            s_rotate[0][i][j] = s[i][j];
        }
    }

    for k in 1..4 {
        for i in 0..n {
            for j in 0..n {
                s_rotate[k][j][n - 1 - i] = s_rotate[k - 1][i][j];
            }
        }
    }

    for k in 0..4 {
        let mut count = 0;
        for i in 0..n {
            for j in 0..n {
                if t[i][j] == s_rotate[k][i][j] {
                    count += 1;
                }
            }
        }
        count_vec.push(count);
    }

    let mut max = (0, 0);
    for i in 0..count_vec.len() {
        if max.0 < count_vec[i] {
            max.0 = count_vec[i];
            max.1 = i;
        }
    }

    println!("{}", max.1 + (n * n - max.0 as usize));
}
