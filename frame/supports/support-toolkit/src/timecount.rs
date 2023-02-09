use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct TimeCount {
    count: u32,
    time: Vec<u128>,
    max_count: u32,
    max_time_interval: Duration,
    sleep_interval: Duration,
}

impl Default for TimeCount {
    fn default() -> Self {
        Self::custom(3, Duration::from_secs(60 * 2), Duration::from_secs(60 * 10))
    }
}

impl TimeCount {
    pub fn custom(max_count: u32, max_time_interval: Duration, sleep_interval: Duration) -> Self {
        Self {
            count: 0,
            time: vec![timestamp()],
            max_count,
            max_time_interval,
            sleep_interval,
        }
    }

    pub fn simple(sleep_interval: Duration) -> Self {
        Self::custom(3, Duration::from_secs(60 * 2), sleep_interval)
    }

    pub fn new() -> Self {
        Self::default()
    }
}

impl TimeCount {
    pub fn reset(&mut self) -> &mut Self {
        self.count = 0;
        self.time = vec![timestamp()];
        self
    }

    pub fn plus_and_check(&mut self) -> Result<&mut Self, Duration> {
        let now = timestamp();
        let last_time = self.time.get(self.count as usize).copied().unwrap_or(now);
        let interval = now - last_time;

        // if check interval grather than max_time_interval, reset count.
        if interval > self.max_time_interval.as_millis() {
            return Ok(self.reset());
        }

        let next_count = self.count + 1;

        if next_count < self.max_count {
            self.count = next_count;
            self.time.push(now);
            return Ok(self);
        }

        self.reset();

        Err(self.sleep_interval)
    }
}

fn timestamp() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let ts = since_the_epoch.as_secs() * 1000u64
        + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as u64;
    ts as u128
}
