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

#[cfg(test)]
pub(super) struct FaultInjectingReserve {
    fail_at: Option<usize>,
    attempts: usize,
    stages: Vec<&'static str>,
}

#[cfg(test)]
impl FaultInjectingReserve {
    pub(super) fn recording() -> Self {
        Self {
            fail_at: None,
            attempts: 0,
            stages: Vec::new(),
        }
    }

    pub(super) fn failing_at(index: usize) -> Self {
        assert!(index > 0, "예약 실패 인덱스는 1부터 시작합니다");
        Self {
            fail_at: Some(index),
            attempts: 0,
            stages: Vec::new(),
        }
    }

    pub(super) fn stages(&self) -> &[&'static str] {
        &self.stages
    }

    pub(super) fn attempts(&self) -> usize {
        self.attempts
    }
}

#[cfg(test)]
impl FallibleReserve for FaultInjectingReserve {
    fn reserve<T>(
        &mut self,
        values: &mut Vec<T>,
        additional: usize,
        stage: &'static str,
    ) -> Result<(), StyleRenderError> {
        if additional > values.capacity().saturating_sub(values.len()) {
            self.attempts += 1;
            self.stages.push(stage);
            if self.fail_at == Some(self.attempts) {
                return Err(StyleRenderError::AllocationFailed { stage });
            }
        }
        values
            .try_reserve(additional)
            .map_err(|_| StyleRenderError::AllocationFailed { stage })
    }
}
