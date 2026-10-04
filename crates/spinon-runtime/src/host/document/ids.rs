use super::{DocumentBatchOperation, HostDocumentBridge};

impl HostDocumentBridge {
    pub(super) fn consume_creation_ids(
        &mut self,
        operations: &[DocumentBatchOperation],
    ) -> Result<(), String> {
        let mut validation_floor = self.next_external_id;
        let mut reused_id = None;
        for operation in operations {
            let Some(id) = created_external_id(operation) else {
                continue;
            };
            if id <= 0 || self.reserved_external_ids.contains(&id) {
                continue;
            }
            if validation_floor.is_some_and(|next| id >= next) {
                validation_floor = id.checked_add(1);
            } else {
                reused_id.get_or_insert(id);
            }
        }

        let mut consumption_floor = self.next_external_id;
        for operation in operations {
            let Some(id) = created_external_id(operation) else {
                continue;
            };
            if id <= 0 {
                continue;
            }
            if self.reserved_external_ids.remove(&id) {
                self.advance_external_id(id);
            } else if consumption_floor.is_some_and(|next| id >= next) {
                consumption_floor = id.checked_add(1);
                self.advance_external_id(id);
            }
        }

        if let Some(id) = reused_id {
            return Err(format!("노드 연결 키를 재사용했습니다: {id}"));
        }
        Ok(())
    }

    fn advance_external_id(&mut self, issued: i32) {
        let Some(after) = issued.checked_add(1) else {
            self.next_external_id = None;
            return;
        };
        if self.next_external_id.is_some_and(|current| after > current) {
            self.next_external_id = Some(after);
        }
    }
}

fn created_external_id(operation: &DocumentBatchOperation) -> Option<i32> {
    match operation {
        DocumentBatchOperation::CreateElement { id, .. }
        | DocumentBatchOperation::CreateText { id, .. } => Some(*id),
        DocumentBatchOperation::Append { .. }
        | DocumentBatchOperation::InsertBefore { .. }
        | DocumentBatchOperation::Remove { .. }
        | DocumentBatchOperation::SetText { .. }
        | DocumentBatchOperation::SetAttribute { .. }
        | DocumentBatchOperation::RemoveAttribute { .. } => None,
    }
}
