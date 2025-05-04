use proconio::input;

fn main() {
    input!(
        n: usize,
        _m: usize,
        q: usize,
    );

    let mut permitted_pages = vec![vec![]; n + 1];
    let mut permitted_pages_all = vec![false; n + 1];

    for _ in 0..q {
        input! {
            query: usize,
        }

        match query {
            1 => {
                input! {
                    user: usize,
                    page: usize,
                }
                if !permitted_pages[user].contains(&page) {
                    permitted_pages[user].push(page);
                }
            }
            2 => {
                input! {
                    user: usize,
                }
                permitted_pages_all[user] = true;
            }
            3 => {
                input! {
                    user: usize,
                    page: usize,
                }

                if permitted_pages_all[user] || permitted_pages[user].contains(&page) {
                    println!("Yes");
                } else {
                    println!("No");
                }
            }
            _ => (),
        }
    }
}
