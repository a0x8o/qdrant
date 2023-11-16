use std::cmp;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::shards::shard::PeerId;

#[derive(Clone, Debug, Default)]
pub struct Registry {
    backoff: HashMap<PeerId, Backoff>,
}

impl Registry {
    pub fn retry_elapsed<T, I>(&mut self, peer_ids: I) -> T
    where
        I: IntoIterator<Item = PeerId>,
        T: FromIterator<PeerId>,
    {
        peer_ids
            .into_iter()
            .filter(|&peer_id| self.retry_if_elapsed(peer_id))
            .collect()
    }

    pub fn retry_if_elapsed(&mut self, peer_id: PeerId) -> bool {
        self.backoff.entry(peer_id).or_default().retry_if_elapsed()
    }

    pub fn reset(&mut self, peer_id: PeerId) {
        let _ = self.backoff.remove(&peer_id);
    }
}

#[derive(Copy, Clone, Debug)]
struct Backoff {
    last_attempt: Instant,
    delay: Duration,
}

impl Default for Backoff {
    fn default() -> Self {
        Self {
            last_attempt: Instant::now(),
            delay: Duration::ZERO,
        }
    }
}

impl Backoff {
    const MAX_DELAY: Duration = Duration::from_secs(10);

    pub fn retry_if_elapsed(&mut self) -> bool {
        let is_elapsed = self.is_elapsed();

        if is_elapsed {
            self.retry();
        }

        is_elapsed
    }

    fn is_elapsed(&self) -> bool {
        self.last_attempt.elapsed() >= self.delay
    }

    fn retry(&mut self) {
        self.last_attempt = Instant::now();

        self.delay = if self.delay.is_zero() {
            Duration::from_secs(1)
        } else {
            cmp::min(self.delay * 2, Self::MAX_DELAY)
        }
    }
}
