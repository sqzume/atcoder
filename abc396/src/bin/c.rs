use proconio::input;
fn main() {
    input!(
        n: usize,
        m: usize,
        b: [i32; n],
        w: [i32; m],
    );
    let mut b = Vec::from(b);
    let mut w = Vec::from(w);
    b.sort_by(|a, b| b.cmp(a));
    w.sort_by(|a, b| b.cmp(a));

    // 配列の長さを揃える（0..を使用）
    if b.len() < w.len() {
        for _ in 0..(w.len() - b.len()) {
            b.push(0);
        }
    } else {
        for _ in 0..(b.len() - w.len()) {
            w.push(0);
        }
    }

    let mut max_b = Vec::new();
    let mut max_w = Vec::new();
    max_b.push(0);
    max_w.push(0);

    let mut num = 0;
    for i in 0..b.len() {
        num += b[i];
        max_b.push(num);
    }

    num = 0;
    for i in 0..w.len() {
        if w[i] > 0 {
            // より明確な条件
            num += w[i];
        }
        max_w.push(num);
    }

    let mut max = 0;
    for i in 0..=b.len() {
        // 0からb.lenまで（inclusive）
        if i < max_w.len() && max_b[i] + max_w[i] > max {
            // 合計の比較と範囲チェック
            max = max_b[i] + max_w[i];
        }
    }

    println!("{}", max);
}
