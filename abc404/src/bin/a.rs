use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    let s_vec: Vec<char> = s;
    for i in 'a'..='z' {
        if !s_vec.contains(&i) {
            println!("{}", i);
            break;
        }
    }
}
