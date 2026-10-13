use spinon_render::RuntimeRenderSnapshot;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};

use super::lock;

pub(super) struct PublishedRuntimeScene {
    pub(super) sequence: u64,
    pub(super) snapshot: Arc<RuntimeRenderSnapshot>,
}

pub(super) struct PresentationScenes {
    sequence: AtomicU64,
    scene: Mutex<Option<PublishedRuntimeScene>>,
}

impl PresentationScenes {
    pub(super) fn new() -> Self {
        Self {
            sequence: AtomicU64::new(0),
            scene: Mutex::new(None),
        }
    }

    pub(super) fn invalidate(&self) -> Result<u64, String> {
        self.sequence
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |value| {
                value.checked_add(1)
            })
            .map(|value| value + 1)
            .map_err(|_| "화면 presentation sequence 값이 소진됐습니다".to_owned())
    }

    pub(super) fn sequence(&self) -> u64 {
        self.sequence.load(Ordering::Acquire)
    }

    pub(super) fn publish(&self, sequence: u64, snapshot: RuntimeRenderSnapshot) -> bool {
        let mut scene = lock(&self.scene);
        if self.sequence() != sequence {
            return false;
        }
        *scene = Some(PublishedRuntimeScene {
            sequence,
            snapshot: Arc::new(snapshot),
        });
        true
    }

    pub(super) fn lock_current(&self) -> (u64, MutexGuard<'_, Option<PublishedRuntimeScene>>) {
        let sequence = self.sequence();
        let mut scene = lock(&self.scene);
        if scene
            .as_ref()
            .is_some_and(|published| published.sequence != sequence)
        {
            *scene = None;
        }
        (sequence, scene)
    }
}
