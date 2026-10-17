use std::{error::Error, fmt, num::NonZeroU64, sync::Arc};

use spinon_core::NodeId;

use super::RuntimeRenderSnapshot;

const MAX_PENDING_FRAMES: usize = 8;

/// 한 registry 안에서 재사용하지 않는 submission frame ID입니다.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RuntimeFrameId(NonZeroU64);

impl RuntimeFrameId {
    /// Frame ID의 정수 표현을 반환합니다.
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

/// surface lifecycle 하나를 구분하는 양수 generation입니다.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RuntimeSurfaceGeneration(NonZeroU64);

impl RuntimeSurfaceGeneration {
    /// 0은 유효한 generation이 아니므로 `None`을 반환합니다.
    pub const fn new(value: u64) -> Option<Self> {
        match NonZeroU64::new(value) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Generation의 정수 표현을 반환합니다.
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

/// Registry가 발급한 frame submission과 surface lifecycle을 묶는 token입니다.
#[derive(Clone, Debug)]
pub struct RuntimePresentationToken {
    registry_identity: Arc<()>,
    frame_id: RuntimeFrameId,
    surface_generation: RuntimeSurfaceGeneration,
}

impl RuntimePresentationToken {
    /// Frame ID를 반환합니다.
    pub const fn frame_id(&self) -> RuntimeFrameId {
        self.frame_id
    }

    /// Submission 당시 surface generation을 반환합니다.
    pub const fn surface_generation(&self) -> RuntimeSurfaceGeneration {
        self.surface_generation
    }

    fn belongs_to(&self, registry_identity: &Arc<()>) -> bool {
        Arc::ptr_eq(&self.registry_identity, registry_identity)
    }
}

impl PartialEq for RuntimePresentationToken {
    fn eq(&self, other: &Self) -> bool {
        self.frame_id == other.frame_id
            && self.surface_generation == other.surface_generation
            && Arc::ptr_eq(&self.registry_identity, &other.registry_identity)
    }
}

impl Eq for RuntimePresentationToken {}

/// 플랫폼의 frame별 확인 통지를 받아 발행한 불변 입력 snapshot입니다.
#[derive(Clone, Debug)]
pub struct RuntimeConfirmedFrame {
    token: RuntimePresentationToken,
    snapshot: Arc<RuntimeRenderSnapshot>,
}

impl RuntimeConfirmedFrame {
    /// 확인된 submission token을 반환합니다.
    pub const fn token(&self) -> &RuntimePresentationToken {
        &self.token
    }

    /// 입력 handoff의 immutable render snapshot을 반환합니다.
    pub fn snapshot(&self) -> &RuntimeRenderSnapshot {
        &self.snapshot
    }

    /// 입력 worker에 snapshot 소유 참조를 넘길 때 공유 handle을 복제합니다.
    pub fn snapshot_handle(&self) -> Arc<RuntimeRenderSnapshot> {
        Arc::clone(&self.snapshot)
    }

    /// 기존 snapshot hit-test 규칙으로 확인된 frame의 CSS point target을 반환합니다.
    pub fn hit_test_css_point(&self, x: f32, y: f32) -> Option<NodeId> {
        self.snapshot
            .hit_test_css_point(x, y)
            .map(|render_box| render_box.node_id())
    }
}

/// Confirmed frame registry가 입력을 fail-closed할 때 반환하는 오류입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeConfirmedFrameError {
    InvalidPendingCapacity,
    AllocationFailed,
    PendingCapacityReached,
    FrameIdExhausted,
    ForeignRegistryToken,
    StaleSurfaceGeneration,
    SurfaceGenerationNotIncreasing,
    UnknownOrDiscardedFrame,
    FrameSuperseded,
    StaleOrDuplicateConfirmation,
    FrameAlreadyConfirmed,
}

impl fmt::Display for RuntimeConfirmedFrameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPendingCapacity => {
                formatter.write_str("presentation pending capacity는 1..=8이어야 합니다")
            }
            Self::AllocationFailed => {
                formatter.write_str("presentation pending storage를 예약하지 못했습니다")
            }
            Self::PendingCapacityReached => {
                formatter.write_str("presentation pending capacity에 도달했습니다")
            }
            Self::FrameIdExhausted => formatter.write_str("presentation FrameId가 소진됐습니다"),
            Self::ForeignRegistryToken => {
                formatter.write_str("다른 presentation registry의 token입니다")
            }
            Self::StaleSurfaceGeneration => {
                formatter.write_str("현재 surface generation과 다른 presentation token입니다")
            }
            Self::SurfaceGenerationNotIncreasing => {
                formatter.write_str("새 surface generation은 현재 값보다 커야 합니다")
            }
            Self::UnknownOrDiscardedFrame => {
                formatter.write_str("등록되지 않았거나 이미 폐기된 presentation frame입니다")
            }
            Self::FrameSuperseded => {
                formatter.write_str("최신 확인 frame보다 오래되어 대기열에서 밀려난 frame입니다")
            }
            Self::StaleOrDuplicateConfirmation => {
                formatter.write_str("오래됐거나 이미 반영한 presentation confirmation입니다")
            }
            Self::FrameAlreadyConfirmed => {
                formatter.write_str("이미 확인된 frame은 pending으로 폐기할 수 없습니다")
            }
        }
    }
}

impl Error for RuntimeConfirmedFrameError {}

struct PendingFrame {
    token: RuntimePresentationToken,
    snapshot: Arc<RuntimeRenderSnapshot>,
}

/// 플랫폼 confirmation을 받은 submission만 입력용 immutable frame으로 발행합니다.
///
/// 이 registry는 단일 직렬 owner에서 사용합니다. `current_confirmed_frame`은 마지막으로
/// 반영한 platform callback이며 native event timestamp에서 실제 표시 frame을 역산하지 않습니다.
pub struct RuntimeConfirmedFrameRegistry {
    registry_identity: Arc<()>,
    surface_generation: RuntimeSurfaceGeneration,
    max_pending: usize,
    next_frame_id: Option<NonZeroU64>,
    pending: Vec<PendingFrame>,
    last_confirmed_frame_id: Option<RuntimeFrameId>,
    current_confirmed: Option<RuntimeConfirmedFrame>,
}

impl RuntimeConfirmedFrameRegistry {
    /// 양수 surface generation과 bounded pending capacity로 registry를 만듭니다.
    pub fn new(
        surface_generation: RuntimeSurfaceGeneration,
        max_pending: usize,
    ) -> Result<Self, RuntimeConfirmedFrameError> {
        if !(1..=MAX_PENDING_FRAMES).contains(&max_pending) {
            return Err(RuntimeConfirmedFrameError::InvalidPendingCapacity);
        }
        let mut pending = Vec::new();
        pending
            .try_reserve_exact(max_pending)
            .map_err(|_| RuntimeConfirmedFrameError::AllocationFailed)?;
        Ok(Self {
            registry_identity: Arc::new(()),
            surface_generation,
            max_pending,
            next_frame_id: NonZeroU64::new(1),
            pending,
            last_confirmed_frame_id: None,
            current_confirmed: None,
        })
    }

    /// 현재 surface generation의 immutable snapshot을 pending submission으로 등록합니다.
    pub fn submit(
        &mut self,
        snapshot: Arc<RuntimeRenderSnapshot>,
    ) -> Result<RuntimePresentationToken, RuntimeConfirmedFrameError> {
        if self.pending.len() >= self.max_pending {
            return Err(RuntimeConfirmedFrameError::PendingCapacityReached);
        }
        let frame_id = self
            .next_frame_id
            .map(RuntimeFrameId)
            .ok_or(RuntimeConfirmedFrameError::FrameIdExhausted)?;
        self.next_frame_id = frame_id.get().checked_add(1).and_then(NonZeroU64::new);
        let token = RuntimePresentationToken {
            registry_identity: Arc::clone(&self.registry_identity),
            frame_id,
            surface_generation: self.surface_generation,
        };
        self.pending.push(PendingFrame {
            token: token.clone(),
            snapshot,
        });
        Ok(token)
    }

    /// 플랫폼에서 실제 frame별 presentation confirmation을 받은 token을 반영합니다.
    pub fn confirm_presented(
        &mut self,
        token: &RuntimePresentationToken,
    ) -> Result<RuntimeConfirmedFrame, RuntimeConfirmedFrameError> {
        self.validate_registry_and_generation(token)?;
        if self
            .last_confirmed_frame_id
            .is_some_and(|latest| token.frame_id <= latest)
        {
            return Err(RuntimeConfirmedFrameError::StaleOrDuplicateConfirmation);
        }
        let Some(index) = self
            .pending
            .iter()
            .position(|pending| pending.token.frame_id == token.frame_id)
        else {
            return Err(RuntimeConfirmedFrameError::UnknownOrDiscardedFrame);
        };
        if self.pending[index].token != *token {
            return Err(RuntimeConfirmedFrameError::UnknownOrDiscardedFrame);
        }
        let pending = self.pending.remove(index);
        self.pending
            .retain(|older| older.token.frame_id > token.frame_id);
        let confirmed = RuntimeConfirmedFrame {
            token: pending.token,
            snapshot: pending.snapshot,
        };
        self.last_confirmed_frame_id = Some(token.frame_id);
        self.current_confirmed = Some(confirmed.clone());
        Ok(confirmed)
    }

    /// 더는 표시될 수 없다고 host가 확정한 pending submission을 명시 폐기합니다.
    pub fn discard_unpresented(
        &mut self,
        token: &RuntimePresentationToken,
    ) -> Result<(), RuntimeConfirmedFrameError> {
        self.validate_registry_and_generation(token)?;
        if let Some(latest) = self.last_confirmed_frame_id {
            if token.frame_id == latest {
                return Err(RuntimeConfirmedFrameError::FrameAlreadyConfirmed);
            }
            if token.frame_id < latest {
                return Err(RuntimeConfirmedFrameError::FrameSuperseded);
            }
        }
        let Some(index) = self
            .pending
            .iter()
            .position(|pending| pending.token.frame_id == token.frame_id)
        else {
            return Err(RuntimeConfirmedFrameError::UnknownOrDiscardedFrame);
        };
        if self.pending[index].token != *token {
            return Err(RuntimeConfirmedFrameError::UnknownOrDiscardedFrame);
        }
        self.pending.remove(index);
        Ok(())
    }

    /// 이전 generation의 확인·대기 frame을 모두 폐기하고 새 surface lifecycle을 시작합니다.
    pub fn advance_surface_generation(
        &mut self,
        generation: RuntimeSurfaceGeneration,
    ) -> Result<(), RuntimeConfirmedFrameError> {
        if generation <= self.surface_generation {
            return Err(RuntimeConfirmedFrameError::SurfaceGenerationNotIncreasing);
        }
        self.surface_generation = generation;
        self.pending.clear();
        self.last_confirmed_frame_id = None;
        self.current_confirmed = None;
        Ok(())
    }

    /// 현재 surface lifecycle의 generation을 반환합니다.
    pub const fn surface_generation(&self) -> RuntimeSurfaceGeneration {
        self.surface_generation
    }

    /// 마지막으로 confirmation을 반영한 immutable frame을 복제해 반환합니다.
    pub fn current_confirmed_frame(&self) -> Option<RuntimeConfirmedFrame> {
        self.current_confirmed.clone()
    }

    /// Confirmation을 기다리는 pending submission 개수를 반환합니다.
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }

    #[cfg(test)]
    pub(super) fn set_next_frame_id_for_test(&mut self, value: u64) {
        self.next_frame_id = NonZeroU64::new(value);
    }

    fn validate_registry_and_generation(
        &self,
        token: &RuntimePresentationToken,
    ) -> Result<(), RuntimeConfirmedFrameError> {
        if !token.belongs_to(&self.registry_identity) {
            return Err(RuntimeConfirmedFrameError::ForeignRegistryToken);
        }
        if token.surface_generation != self.surface_generation {
            return Err(RuntimeConfirmedFrameError::StaleSurfaceGeneration);
        }
        Ok(())
    }
}
