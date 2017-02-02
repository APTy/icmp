extern crate libicmp;

use std::{time, thread};

use libicmp::Icmp;

fn main() {
    let c = 5;
    let started = time::Instant::now();
    let timeout = time::Duration::from_secs(5);
    let time_between_pings = time::Duration::from_secs(1);

    let ip = "8.8.8.8";
    let mut icmp = Icmp::new(ip);
    println!("PING {}", ip);

    let mut i = 1;
    let mut last_ping_sent = started;
    icmp.echo_request();
    loop {
        // check for timeout
        if time::Instant::now() > started + timeout {
            return;
        }

        // poll for new messages
        match icmp.poll() {
            Some(ready) => if !ready { continue; },
            _ => continue,
        }

        // check for replies
        if let Some((reply, dur)) = icmp.get_echo_reply() {
            let time = (dur.as_secs() * 1000) as f64 + dur.subsec_nanos() as f64 / 1e6;
            println!("{} bytes from {}: icmp_seq={} time={:.1} ms", reply.len(), ip, reply.seq(), time);
        }

        // exit after we've made enough
        if i > c {
            break;
        }

        // sleep until it's time to send a new ping
        thread::sleep(((last_ping_sent + time_between_pings) as time::Instant).duration_since(time::Instant::now()));

        // send a new echo request
        icmp.echo_request();
        last_ping_sent = time::Instant::now();
        i += 1;
    }

}
