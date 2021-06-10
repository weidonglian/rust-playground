use std::time::Instant;
use std::future::Future;

struct Delay {
    when: Instant,
}

impl Delay for Future {

}
#[test]
fn delay_future() {
    let start = Instant::now();

}