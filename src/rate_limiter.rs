use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use parking_lot::Mutex;

use crate::models::rich_presence::SetActivityArgs;

#[derive(Clone, Default)]
pub struct RateLimiter {
    last_activity_update: Arc<Mutex<Option<Instant>>>,
    queued_activity: Arc<Mutex<Option<SetActivityArgs>>>,
    claiming_send: Arc<AtomicBool>,
}

impl RateLimiter {
    /// Queue an activity update to be sent later.
    pub(crate) fn queue_activity(&self, args: SetActivityArgs) {
        let mut queued = self.queued_activity.lock();
        *queued = Some(args);
    }

    /// Can we send an activity update right now?
    pub(crate) fn can_send(&self) -> bool {
        const RATE_LIMIT: Duration = Duration::from_secs(15);
        let last_update = self.last_activity_update.lock();
        last_update
            .map(|t| t.elapsed() >= RATE_LIMIT)
            .unwrap_or(true)
    }

    /// Mark that we have just sent an activity update.
    pub(crate) fn mark_sent(&self) {
        let mut last_update = self.last_activity_update.lock();
        *last_update = Some(Instant::now());
        let mut queued = self.queued_activity.lock();
        *queued = None;
    }

    /// Take the queued activity update, if any.
    pub(crate) fn take_queued(&self) -> Option<SetActivityArgs> {
        let mut queued = self.queued_activity.lock();
        queued.take()
    }

    /// Try to claim the send lock. Returns true if successful.
    pub(crate) fn try_claim_send(&self) -> bool {
        !self.claiming_send.swap(true, Ordering::SeqCst)
    }

    /// Release the send lock.
    pub(crate) fn release_send(&self) {
        self.claiming_send.store(false, Ordering::SeqCst);
    }
}