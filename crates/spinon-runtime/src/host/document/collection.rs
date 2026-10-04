use super::{HostDocumentBridge, MAX_DOCUMENT_NODES, NodeStringUsage};
use spinon_core::HostNodeHandle;

impl HostDocumentBridge {
    #[allow(dead_code)] // V8 weak-handle safe-point 연결 전까지 내부 경로에서 호출하지 않습니다.
    pub(super) fn collect_unreachable(
        &mut self,
        roots: &[HostNodeHandle],
    ) -> Result<usize, String> {
        if roots.len() > MAX_DOCUMENT_NODES {
            return Err("노드 회수 root가 허용 개수를 초과했습니다".to_owned());
        }
        self.validate_collection_indexes()?;

        let plan = self
            .document
            .plan_collection(roots)
            .map_err(|error| error.to_string())?;
        let mut reclaimed_string_units = 0usize;
        for handle in plan.reclaimed_handles() {
            let external_id = self
                .external_ids
                .get(handle)
                .copied()
                .ok_or_else(|| "회수 대상의 외부 노드 ID가 없습니다".to_owned())?;
            if self.handles.get(&external_id) != Some(handle) {
                return Err("회수 대상의 양방향 노드 ID 매핑이 다릅니다".to_owned());
            }
            let usage = self
                .string_usage
                .get(&external_id)
                .and_then(NodeStringUsage::total_units)
                .ok_or_else(|| "회수 대상의 문자열 계수가 잘못되었습니다".to_owned())?;
            reclaimed_string_units = reclaimed_string_units
                .checked_add(usage)
                .ok_or_else(|| "회수 대상 문자열 계수가 넘쳤습니다".to_owned())?;
        }
        let remaining_string_units = self
            .string_units
            .checked_sub(reclaimed_string_units)
            .ok_or_else(|| "회수 뒤 문자열 계수가 음수가 됩니다".to_owned())?;
        let reclaimed_count = plan.reclaimed_node_count();

        self.document
            .commit_collection(&plan)
            .map_err(|error| error.to_string())?;
        for handle in plan.reclaimed_handles() {
            let external_id = self
                .external_ids
                .remove(handle)
                .expect("회수 전에 검증한 외부 노드 ID 매핑");
            self.handles.remove(&external_id);
            self.string_usage.remove(&external_id);
        }
        self.string_units = remaining_string_units;
        Ok(reclaimed_count)
    }

    fn validate_collection_indexes(&self) -> Result<(), String> {
        if self.document.node_count() != self.handles.len()
            || self.handles.len() != self.external_ids.len()
            || self.handles.len() != self.string_usage.len()
        {
            return Err("문서 회수 전에 노드 인덱스 수가 일치하지 않습니다".to_owned());
        }

        let mut total_string_units = 0usize;
        for (external_id, handle) in &self.handles {
            if *external_id <= 0
                || self.document.node(*handle).is_none()
                || self.external_ids.get(handle) != Some(external_id)
            {
                return Err("문서 회수 전에 노드 ID 매핑이 일치하지 않습니다".to_owned());
            }
            let units = self
                .string_usage
                .get(external_id)
                .and_then(NodeStringUsage::total_units)
                .ok_or_else(|| "문서 회수 전에 문자열 계수가 잘못되었습니다".to_owned())?;
            total_string_units = total_string_units
                .checked_add(units)
                .ok_or_else(|| "문서 회수 전 문자열 계수가 넘쳤습니다".to_owned())?;
        }
        if total_string_units != self.string_units {
            return Err("문서 회수 전 문자열 총계가 노드별 계수와 다릅니다".to_owned());
        }
        for (handle, external_id) in &self.external_ids {
            if self.handles.get(external_id) != Some(handle) {
                return Err("문서 회수 전에 역방향 노드 ID 매핑이 다릅니다".to_owned());
            }
        }
        Ok(())
    }
}
