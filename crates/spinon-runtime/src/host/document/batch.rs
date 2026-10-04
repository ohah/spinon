use super::{
    DocumentBatchOperation, DocumentStage, HostDocumentBridge, MAX_BATCH_STRING_UNITS,
    MAX_DOCUMENT_STRING_UNITS, MAX_NAME_UNITS, MAX_VALUE_UNITS, NodeStringUsage,
    SpinonDocumentReceipt,
};
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, DocumentReceipt, DomString,
    HostNodeHandle, HostParent,
};
use std::collections::BTreeMap;

impl HostDocumentBridge {
    pub(super) fn build_and_commit(
        &mut self,
        operations: &[DocumentBatchOperation],
        stage: &mut DocumentStage,
    ) -> Result<DocumentReceipt, String> {
        let mut batch = DocumentChangeBatch::new(self.owner, self.document.document_revision());
        for (index, operation) in operations.iter().enumerate() {
            let operation = self.build_core_operation(operation, stage, index)?;
            batch.push(operation);
        }
        self.document
            .commit(batch)
            .map_err(|error| error.to_string())
    }

    fn build_core_operation(
        &mut self,
        operation: &DocumentBatchOperation,
        stage: &mut DocumentStage,
        index: usize,
    ) -> Result<DocumentOperation, String> {
        let at = |message: String| format!("문서 변경 {index}에서 거부했습니다: {message}");
        match operation {
            DocumentBatchOperation::CreateElement {
                id,
                namespace,
                name,
            } => {
                validate_new_id(*id, &stage.handles).map_err(at)?;
                let units = namespace.encode_utf16().count() + name.encode_utf16().count();
                add_string_units(&mut stage.string_units, units).map_err(at)?;
                stage.string_usage.insert(
                    *id,
                    NodeStringUsage {
                        base_units: units,
                        ..NodeStringUsage::default()
                    },
                );
                let handle = self
                    .document
                    .reserve_node_handle()
                    .map_err(|error| at(error.to_string()))?;
                stage.reserved.push(handle);
                stage.handles.insert(*id, handle);
                stage.external_ids.insert(handle, *id);
                Ok(DocumentOperation::CreateElement {
                    node: handle,
                    namespace: namespace.clone(),
                    local_name: name.clone(),
                })
            }
            DocumentBatchOperation::CreateText { id, data } => {
                validate_new_id(*id, &stage.handles).map_err(at)?;
                add_string_units(&mut stage.string_units, data.len()).map_err(at)?;
                stage.string_usage.insert(
                    *id,
                    NodeStringUsage {
                        text_units: data.len(),
                        ..NodeStringUsage::default()
                    },
                );
                let handle = self
                    .document
                    .reserve_node_handle()
                    .map_err(|error| at(error.to_string()))?;
                stage.reserved.push(handle);
                stage.handles.insert(*id, handle);
                stage.external_ids.insert(handle, *id);
                Ok(DocumentOperation::CreateText {
                    node: handle,
                    data: DomString::from_utf16(data.clone()),
                })
            }
            DocumentBatchOperation::Append { parent, node } => {
                let parent = core_parent(*parent, &stage.handles).map_err(at)?;
                let node = required_handle(*node, &stage.handles).map_err(at)?;
                Ok(DocumentOperation::InsertBefore {
                    parent,
                    node,
                    before: None,
                })
            }
            DocumentBatchOperation::InsertBefore {
                parent,
                node,
                before,
            } => {
                let parent = core_parent(*parent, &stage.handles).map_err(at)?;
                let node = required_handle(*node, &stage.handles).map_err(at)?;
                let before = if *before == 0 {
                    None
                } else {
                    Some(required_handle(*before, &stage.handles).map_err(at)?)
                };
                Ok(DocumentOperation::InsertBefore {
                    parent,
                    node,
                    before,
                })
            }
            DocumentBatchOperation::Remove { parent, node } => {
                let parent = core_parent(*parent, &stage.handles).map_err(at)?;
                let node = required_handle(*node, &stage.handles).map_err(at)?;
                Ok(DocumentOperation::RemoveChild { parent, node })
            }
            DocumentBatchOperation::SetText { node, data } => {
                let node = required_handle(*node, &stage.handles).map_err(at)?;
                let external_id = stage
                    .external_ids
                    .get(&node)
                    .copied()
                    .ok_or_else(|| at("문서 연결 키를 찾지 못했습니다".to_owned()))?;
                let usage = stage
                    .string_usage
                    .get_mut(&external_id)
                    .ok_or_else(|| at("문자열 사용량 항목이 없습니다".to_owned()))?;
                stage.string_units = stage.string_units.saturating_sub(usage.text_units);
                add_string_units(&mut stage.string_units, data.len()).map_err(at)?;
                usage.text_units = data.len();
                Ok(DocumentOperation::SetTextData {
                    node,
                    data: DomString::from_utf16(data.clone()),
                })
            }
            DocumentBatchOperation::SetAttribute { node, name, value } => {
                let node = required_handle(*node, &stage.handles).map_err(at)?;
                let external_id = stage
                    .external_ids
                    .get(&node)
                    .copied()
                    .ok_or_else(|| at("문서 연결 키를 찾지 못했습니다".to_owned()))?;
                let usage = stage
                    .string_usage
                    .get_mut(&external_id)
                    .ok_or_else(|| at("문자열 사용량 항목이 없습니다".to_owned()))?;
                let name_units = name.encode_utf16().count();
                let old_units = usage.attributes.get(name).copied().unwrap_or(0);
                stage.string_units = stage.string_units.saturating_sub(old_units);
                add_string_units(&mut stage.string_units, name_units + value.len()).map_err(at)?;
                usage
                    .attributes
                    .insert(name.clone(), name_units + value.len());
                let name = AttributeName::new(None, name.clone())
                    .ok_or_else(|| at("속성 이름이 잘못되었습니다".to_owned()))?;
                Ok(DocumentOperation::SetAttribute {
                    node,
                    name,
                    value: DomString::from_utf16(value.clone()),
                })
            }
            DocumentBatchOperation::RemoveAttribute { node, name } => {
                let node = required_handle(*node, &stage.handles).map_err(at)?;
                let external_id = stage
                    .external_ids
                    .get(&node)
                    .copied()
                    .ok_or_else(|| at("문서 연결 키를 찾지 못했습니다".to_owned()))?;
                let usage = stage
                    .string_usage
                    .get_mut(&external_id)
                    .ok_or_else(|| at("문자열 사용량 항목이 없습니다".to_owned()))?;
                let old_units = usage.attributes.remove(name).unwrap_or(0);
                stage.string_units = stage.string_units.saturating_sub(old_units);
                let name = AttributeName::new(None, name.clone())
                    .ok_or_else(|| at("속성 이름이 잘못되었습니다".to_owned()))?;
                Ok(DocumentOperation::RemoveAttribute { node, name })
            }
        }
    }
}

fn add_string_units(total: &mut usize, units: usize) -> Result<(), String> {
    *total = total
        .checked_add(units)
        .filter(|total| *total <= MAX_DOCUMENT_STRING_UNITS)
        .ok_or_else(|| {
            format!(
                "QuotaExceededError: 런타임 문서는 UTF-16 코드 단위 {MAX_DOCUMENT_STRING_UNITS}개까지 보존합니다"
            )
        })?;
    Ok(())
}

pub(super) fn validate_string_limits(operations: &[DocumentBatchOperation]) -> Result<(), String> {
    let mut total_units = 0usize;
    for (index, operation) in operations.iter().enumerate() {
        let at = |message: String| format!("문서 변경 {index}에서 거부했습니다: {message}");
        let (name_units, namespace_units, value_units) = match operation {
            DocumentBatchOperation::CreateElement {
                namespace, name, ..
            } => (
                name.encode_utf16().count(),
                namespace.encode_utf16().count(),
                0,
            ),
            DocumentBatchOperation::CreateText { data, .. }
            | DocumentBatchOperation::SetText { data, .. } => (0, 0, data.len()),
            DocumentBatchOperation::SetAttribute { name, value, .. } => {
                (name.encode_utf16().count(), 0, value.len())
            }
            DocumentBatchOperation::RemoveAttribute { name, .. } => {
                (name.encode_utf16().count(), 0, 0)
            }
            DocumentBatchOperation::Append { .. }
            | DocumentBatchOperation::InsertBefore { .. }
            | DocumentBatchOperation::Remove { .. } => (0, 0, 0),
        };
        if name_units > MAX_NAME_UNITS || namespace_units > MAX_NAME_UNITS {
            return Err(at(format!(
                "이름 또는 namespace는 UTF-16 코드 단위 {MAX_NAME_UNITS}개까지 허용합니다"
            )));
        }
        if value_units > MAX_VALUE_UNITS {
            return Err(at(format!(
                "문자열 값은 UTF-16 코드 단위 {MAX_VALUE_UNITS}개까지 허용합니다"
            )));
        }
        let operation_units = name_units + namespace_units + value_units;
        total_units = total_units
            .checked_add(operation_units)
            .filter(|units| *units <= MAX_BATCH_STRING_UNITS)
            .ok_or_else(|| {
                at(format!(
                    "문서 변경 묶음 문자열은 UTF-16 코드 단위 {MAX_BATCH_STRING_UNITS}개까지 허용합니다"
                ))
            })?;
    }
    Ok(())
}

impl From<DocumentReceipt> for SpinonDocumentReceipt {
    fn from(receipt: DocumentReceipt) -> Self {
        Self {
            document_revision: receipt.document_revision().get(),
            render_tree_revision: receipt.render_tree_revision().get(),
            node_count: 0,
            changed: i32::from(receipt.changed()),
        }
    }
}

fn validate_new_id(id: i32, handles: &BTreeMap<i32, HostNodeHandle>) -> Result<(), String> {
    if id <= 0 {
        return Err("새 노드 ID는 양의 32비트 정수여야 합니다".to_owned());
    }
    if handles.contains_key(&id) {
        return Err(format!("노드 연결 키를 재사용했습니다: {id}"));
    }
    Ok(())
}

fn required_handle(
    external_id: i32,
    handles: &BTreeMap<i32, HostNodeHandle>,
) -> Result<HostNodeHandle, String> {
    if external_id <= 0 {
        return Err("노드 ID는 양의 32비트 정수여야 합니다".to_owned());
    }
    handles
        .get(&external_id)
        .copied()
        .ok_or_else(|| format!("알 수 없는 노드 연결 키입니다: {external_id}"))
}

fn core_parent(
    external_id: i32,
    handles: &BTreeMap<i32, HostNodeHandle>,
) -> Result<HostParent, String> {
    if external_id == 0 {
        return Ok(HostParent::Root);
    }
    if external_id < 0 {
        return Err("부모 ID는 0 또는 양의 32비트 정수여야 합니다".to_owned());
    }
    required_handle(external_id, handles).map(HostParent::Node)
}
