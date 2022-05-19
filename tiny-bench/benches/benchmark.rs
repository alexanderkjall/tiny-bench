use std::time::Duration;
use tiny_bench::black_box;

fn main() {
    bench_test_one();
    bench_test_two();
}

fn bench_test_one() {
    tiny_bench::bench_labeled("test one", || {
        let mut v: Vec<i32> = Vec::with_capacity(1_000_000);
        for i in 0..black_box(1_000_000) {
            v.push(black_box(i));
        }
        let mut sum = 0;
        for i in black_box(v) {
            sum += black_box(i);
        }
        assert!(sum >= black_box(1));
    })
}

fn bench_test_two() {
    tiny_bench::bench_with_setup_labeled(
        "test two",
        || {
            std::thread::sleep(Duration::from_micros(1));
            let mut v: Vec<i32> = Vec::with_capacity(1_000_000);
            for i in 0..1_000_000 {
                v.push(black_box(i));
            }
            v
        },
        |mut v| {
            let mut sum = 0;
            for i in black_box(v) {
                sum += black_box(i);
            }
            assert!(sum >= black_box(1));
        },
    )
}