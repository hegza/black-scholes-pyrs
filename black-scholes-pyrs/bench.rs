use std::collections::HashMap;
use std::*;

use black_scholes_dp::*;
use black_scholes_ndp::*;
use timeit::timeit;
fn bench_euro_vanilla_call() {
    euro_vanilla_call(50, 100, 1, 0.05, 0.25);
}
fn bench_euro_vanilla_put() {
    euro_vanilla_put(50, 100, 1, 0.05, 0.25);
}
fn bench_euro_vanilla() {
    euro_vanilla(50, 100, 1, 0.05, 0.25, "put");
}
fn bench_black_scholes_call_div() {
    black_scholes_call_div(50, 100, 1, 0.05, 0.06, 0.25);
}
fn bench_black_scholes_put_div() {
    black_scholes_put_div(50, 100, 1, 0.05, 0.06, 0.25);
}
fn bench_euro_vanilla_dividend() {
    euro_vanilla_dividend(50, 100, 1, 0.05, 0.06, 0.25, "put");
}
println!(
    "{:?} {:?} ",
    "euro_vanilla_call",
    String::from(timeit(bench_euro_vanilla_call, 1000))
);
println!(
    "{:?} {:?} ",
    "euro_vanilla_put",
    String::from(timeit(bench_euro_vanilla_put, 1000))
);
println!(
    "{:?} ",
    "euro_vanilla: " + String::from(timeit(bench_euro_vanilla, 1000))
);
println!(
    "{:?} {:?} ",
    "black_scholes_call_div",
    String::from(timeit(bench_black_scholes_call_div, 1000))
);
println!(
    "{:?} {:?} ",
    "black_scholes_put_div",
    String::from(timeit(bench_black_scholes_put_div, 1000))
);
println!(
    "{:?} ",
    "euro_vanilla_dividend: " + String::from(timeit(bench_euro_vanilla_dividend, 1000))
);
