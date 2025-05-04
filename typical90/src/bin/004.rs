use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[i32; w]; h],
    };

    let mut sum_length = vec![0; h];
    let mut sum_width = vec![0; w];

    for i in 0..h {
        for j in 0..w {
            sum_length[i] += a[i][j];
        }
    }
    for i in 0..w {
        for j in 0..h {
            sum_width[i] += a[j][i];
        }
    }
    for i in 0..h {
        for j in 0..w {
            if j != w - 1 {
                print!("{} ", sum_width[j] + sum_length[i] - a[i][j]);
            } else {
                print!("{}", sum_width[j] + sum_length[i] - a[i][j]);
            }
        }
        println!();
    }
    /*
    for i in 0..h {
        for j in 0..w {
            let mut sum = 0;
            for k in 0..w {
                sum += a[i][k];
            }
            for k in 0..h {
                sum += a[k][j];
            }
            if j != w - 1 {
                print!("{} ", sum - a[i][j]);
            } else {
                print!("{}", sum - a[i][j]);
            }
        }
        println!();
    }
    */
}
