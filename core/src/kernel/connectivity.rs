use std::collections::VecDeque;
use std::time::Instant;

use parking_lot::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetState {
    Online,
    Degraded,
    Offline,
}

pub struct ConnectivityTracker {
    inner: Mutex<Inner>,
}

struct Inner {
    recent: VecDeque<bool>,
    last_success: Option<Instant>,
    last_attempt: Option<Instant>,
}

const WINDOW_SIZE: usize = 10;
const OFFLINE_RETRY_SECS: u64 = 30;

impl ConnectivityTracker {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(Inner {
                recent: VecDeque::with_capacity(WINDOW_SIZE),
                last_success: None,
                last_attempt: None,
            }),
        }
    }

    pub fn record(&self, success: bool) {
        let mut inner = self.inner.lock();
        if inner.recent.len() >= WINDOW_SIZE {
            inner.recent.pop_front();
        }
        inner.recent.push_back(success);
        inner.last_attempt = Some(Instant::now());
        if success {
            inner.last_success = Some(Instant::now());
        }
    }

    pub fn state(&self) -> NetState {
        let inner = self.inner.lock();
        if inner.recent.len() < 3 {
            return NetState::Online;
        }

        let failures = inner.recent.iter().filter(|&&s| !s).count();
        let total = inner.recent.len();

        if failures == total && total >= 5 {
            NetState::Offline
        } else if failures * 2 >= total {
            NetState::Degraded
        } else {
            NetState::Online
        }
    }

    pub fn should_attempt(&self) -> bool {
        let inner = self.inner.lock();
        let state = Self::state_inner(&inner);
        match state {
            NetState::Online | NetState::Degraded => true,
            NetState::Offline => inner
                .last_attempt
                .map(|t| t.elapsed().as_secs() >= OFFLINE_RETRY_SECS)
                .unwrap_or(true),
        }
    }

    fn state_inner(inner: &Inner) -> NetState {
        if inner.recent.len() < 3 {
            return NetState::Online;
        }
        let failures = inner.recent.iter().filter(|&&s| !s).count();
        let total = inner.recent.len();
        if failures == total && total >= 5 {
            NetState::Offline
        } else if failures * 2 >= total {
            NetState::Degraded
        } else {
            NetState::Online
        }
    }
}
