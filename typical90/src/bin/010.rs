use proconio::input;

fn main() {
    input! {
        n: usize,
        score: [[usize; 2]; n],
        q: usize,
        question: [[usize; 2]; q],
    };

    // 最初に足しておく
    let mut class1_sum: Vec<usize> = vec![0; n];
    let mut class2_sum: Vec<usize> = vec![0; n];
    if score[0][0] == 1 {
        class1_sum[0] += score[0][1];
    } else {
        class2_sum[0] += score[0][1];
    }
    for i in 1..n {
        if score[i][0] == 1 {
            class1_sum[i] += class1_sum[i - 1] + score[i][1];
            class2_sum[i] += class2_sum[i - 1];
        } else {
            class1_sum[i] += class1_sum[i - 1];
            class2_sum[i] += class2_sum[i - 1] + score[i][1];
        }
    }
    class2_sum.insert(0, 0);
    class1_sum.insert(0, 0);

    // question の処理
    for i in 0..q {
        let class1 = class1_sum[question[i][1]] - class1_sum[question[i][0] - 1];
        let class2 = class2_sum[question[i][1]] - class2_sum[question[i][0] - 1];
        println!("{} {}", class1, class2);
    }
}
