use proconio::input;

fn main() {
    input! {
        n: usize, // num of cutting coordinates
        l: u64, // total length of Yokan
        k: usize, // num of selected coordinates
        a: [u64; n], // cutting coordinates
    }

    let mut left = 0;
    let mut right = l + 1;
    while right - left > 1 {
        let mid = left + (right - left) / 2;
        if solve(n, l, k, &a, mid) {
            left = mid;
        } else {
            right = mid;
        }
    }
    println!("{:?}", left);
}

fn solve(n: usize, l: u64, k: usize, a: &Vec<u64>, mid: u64) -> bool {
    let mut count = 0;
    let mut prev = 0;
    for i in 0..n {
        if (a[i] - prev >= mid) && (l - a[i] >= mid) {
            count += 1;
            prev = a[i];
        }
    }
    count >= k
}
