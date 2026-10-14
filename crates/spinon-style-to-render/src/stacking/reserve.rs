use crate::StyleRenderError;

/// Centralizes fallible `Vec` reservations so runtime failures have one
/// diagnostic path and tests can inject deterministic reservation failures.
pub(super) trait FallibleReserve {
    fn reserve<T>(
        &mut self,
        values: &mut Vec<T>,
        additional: usize,
        stage: &'static str,
    ) -> Result<(), StyleRenderError>;
}

pub(super) struct SystemReserve;

impl FallibleReserve for SystemReserve {
    fn reserve<T>(
        &mut self,
        values: &mut Vec<T>,
        additional: usize,
        stage: &'static str,
    ) -> Result<(), StyleRenderError> {
        values
            .try_reserve(additional)
            .map_err(|_| StyleRenderError::AllocationFailed { stage })
    }
}

#[cfg(any(test, feature = "c13-1-overflow-clip-worker-test"))]
pub(super) struct FaultInjectingReserve {
    fail_at: Option<FailureTarget>,
    #[cfg(test)]
    attempts: usize,
    stages: Vec<&'static str>,
}

#[cfg(any(test, feature = "c13-1-overflow-clip-worker-test"))]
#[derive(Clone, Copy)]
enum FailureTarget {
    #[cfg(test)]
    Attempt(usize),
    StageOccurrence {
        stage: &'static str,
        occurrence: usize,
    },
}

#[cfg(any(test, feature = "c13-1-overflow-clip-worker-test"))]
impl FaultInjectingReserve {
    #[cfg(test)]
    pub(super) fn recording() -> Self {
        Self {
            fail_at: None,
            #[cfg(test)]
            attempts: 0,
            stages: Vec::new(),
        }
    }

    #[cfg(test)]
    pub(super) fn failing_at(index: usize) -> Self {
        assert!(index > 0, "예약 실패 인덱스는 1부터 시작합니다");
        Self {
            fail_at: Some(FailureTarget::Attempt(index)),
            #[cfg(test)]
            attempts: 0,
            stages: Vec::new(),
        }
    }

    #[cfg(any(test, feature = "c13-1-overflow-clip-worker-test"))]
    pub(super) fn failing_at_stage_occurrence(stage: &'static str, occurrence: usize) -> Self {
        assert!(occurrence > 0, "예약 stage occurrence는 1부터 시작합니다");
        Self {
            fail_at: Some(FailureTarget::StageOccurrence { stage, occurrence }),
            #[cfg(test)]
            attempts: 0,
            stages: Vec::new(),
        }
    }

    #[cfg(test)]
    pub(super) fn stages(&self) -> &[&'static str] {
        &self.stages
    }

    #[cfg(test)]
    pub(super) fn attempts(&self) -> usize {
        self.attempts
    }
}

#[cfg(any(test, feature = "c13-1-overflow-clip-worker-test"))]
impl FallibleReserve for FaultInjectingReserve {
    fn reserve<T>(
        &mut self,
        values: &mut Vec<T>,
        additional: usize,
        stage: &'static str,
    ) -> Result<(), StyleRenderError> {
        if additional > values.capacity().saturating_sub(values.len()) {
            #[cfg(test)]
            {
                self.attempts += 1;
            }
            self.stages.push(stage);
            let should_fail = match self.fail_at {
                #[cfg(test)]
                Some(FailureTarget::Attempt(attempt)) => attempt == self.attempts,
                Some(FailureTarget::StageOccurrence {
                    stage: failure_stage,
                    occurrence,
                }) => {
                    stage == failure_stage
                        && self
                            .stages
                            .iter()
                            .filter(|seen| **seen == failure_stage)
                            .count()
                            == occurrence
                }
                None => false,
            };
            if should_fail {
                return Err(StyleRenderError::AllocationFailed { stage });
            }
        }
        values
            .try_reserve(additional)
            .map_err(|_| StyleRenderError::AllocationFailed { stage })
    }
}
