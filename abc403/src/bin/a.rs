use proconio::input;

fn main() {
    input!(
        n: usize,
        a: [i32; n],
    );

    let mut num = 0;

    for i in 0..a.len() {
        if (i + 1) % 2 == 1 {
            num += a[i];
        }
    }

    println!("{}", num);
}
