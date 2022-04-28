
use std::thread;
use std::time::Duration;

pub mod concurrency;


pub fn main() {
    println!("Hello, world!");
    concurrency::parallel::parallel();
    concurrency::global_mutable::global_mutable();
}
