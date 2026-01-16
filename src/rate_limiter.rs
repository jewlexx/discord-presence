use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use parking_lot::Mutex;

use crate::models::rich_presence::SetActivityArgs;

#[derive(Default)]
struct RateLimiterState {
    last_update: Option<Instant>,
    queued: Option<SetActivityArgs>,
    is_sending: bool,
}

#[derive(Clone, Default)]
pub struct RateLimiter(Arc<Mutex<RateLimiterState>>);

impl RateLimiter {
    const RATE_LIMIT: Duration = Duration::from_secs(15);

    /// Queue an activity update to be sent later.
    pub(crate) fn queue(&self, args: SetActivityArgs) {
        let mut state = self.0.lock();
        state.queued = Some(args);
    }

    /// Mark that an activity update has been sent now.
    pub(crate) fn mark_sent(&self) {
        let mut state = self.0.lock();
        state.last_update = Some(Instant::now());
        state.queued = None;
        state.is_sending = false;
    }

    /// Release the send lock without updating timestamp.
    pub(crate) fn release_send(&self) {
        self.0.lock().is_sending = false;
    }

    /// Peek at the queued activity update if it can be sent now.
    pub(crate) fn peek_queued(&self) -> Option<SetActivityArgs> {
        let mut state = self.0.lock();
        let can_send = state
            .last_update
            .is_none_or(|t| t.elapsed() >= Self::RATE_LIMIT);

        if can_send && !state.is_sending {
            state.is_sending = true;
            state.queued.clone()
        } else {
            None
        }
    }

    /// Drop the queued activity update without sending it.
    pub(crate) fn drop_queued(&self) {
        let mut state = self.0.lock();
        state.queued = None;
    }
}
