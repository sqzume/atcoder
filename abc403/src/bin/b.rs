use proconio::input;
use proconio::marker::Chars;

fn main() {
    input!(
        t: Chars,
        u: Chars,
    );

    let mut ans = "No";

    for i in 0..=(t.len() - u.len()) {
        let mut check = 0;
        for j in 0..u.len() {
            if u[j] != t[j + i] && t[j + i] != '?' {
                break;
            } else {
                check += 1;
            }
        }
        if check == u.len() {
            ans = "Yes";
            break;
        }
    }
    println!("{}", ans);
}
