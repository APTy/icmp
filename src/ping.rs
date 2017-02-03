use std::{time, thread};

pub use icmp::Icmp;

const DEFAULT_NUM_PINGS: u64 = 5;
const DEFAULT_INTERVAL_SECS: u64 = 1;
const DEFAULT_TIMEOUT_SECS: u64 = 10;

/// Used for configuring a Ping operation.
pub struct PingBuilder<'a> {
    host: &'a str,
    num_pings: u64,
    interval_secs: u64,
    timeout_secs: u64,
    debug: bool,
}

impl<'a> PingBuilder<'a> {

    /// Returns a new PingBuilder for configuring a Ping operation.
    pub fn new() -> PingBuilder<'a> {
        PingBuilder {
            host: "127.0.0.1",
            num_pings: DEFAULT_NUM_PINGS,
            interval_secs: DEFAULT_INTERVAL_SECS,
            timeout_secs: DEFAULT_TIMEOUT_SECS,
            debug: false,
        }
    }

    /// Sets the host that will be pinged.
    pub fn host(self, host: &'a str) -> PingBuilder<'a> {
        PingBuilder { host: host, .. self }
    }

    /// Sets the number of echo requests to be issued.
    pub fn num_pings(self, num_pings: u64) -> PingBuilder<'a> {
        PingBuilder { num_pings: num_pings, .. self }
    }

    /// Sets the interval in seconds between echo requests.
    pub fn interval_secs(self, interval_secs: u64) -> PingBuilder<'a> {
        PingBuilder { interval_secs: interval_secs, .. self }
    }

    /// Sets the time in seconds before the ping operation times out.
    pub fn timeout_secs(self, timeout_secs: u64) -> PingBuilder<'a> {
        PingBuilder { timeout_secs: timeout_secs, .. self }
    }

    /// Prints debug information to standard out if set to `true`.
    pub fn debug(self, debug: bool) -> PingBuilder<'a> {
        PingBuilder { debug: debug, .. self }
    }

    /// Returns a new Ping object.
    pub fn build(self) -> Ping<'a> {
        Ping {
            host: self.host,
            num_pings: self.num_pings,
            interval_secs: self.interval_secs,
            timeout_secs: self.timeout_secs,
            debug: self.debug,
        }
    }
}

/// Used for pinging a host.
pub struct Ping<'a> {
    host: &'a str,
    num_pings: u64,
    interval_secs: u64,
    timeout_secs: u64,
    debug: bool,
}

impl<'a> Ping<'a> {
    /// Sends a series of pings to a host.
    pub fn ping(&self) {
        let started = time::Instant::now();
        let timeout = time::Duration::from_secs(self.timeout_secs);
        let time_between_pings = time::Duration::from_secs(self.interval_secs);

        let mut icmp = Icmp::new(self.host);
        if self.debug {
            println!("PING {}", self.host);
        }

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
                if self.debug {
                    println!("{} bytes from {}: icmp_seq={} time={:.1} ms", reply.len(), self.host, reply.seq(), time);
                }
            }

            // exit after we've made enough
            if i > self.num_pings {
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
}

