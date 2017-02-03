extern crate libicmp;

use libicmp::PingBuilder;

fn main() {
    let p = PingBuilder::new()
        .host("8.8.8.8")
        .num_pings(5)
        .interval_secs(1)
        .timeout_secs(5)
        .debug(true)
        .build();
    p.ping();
}
