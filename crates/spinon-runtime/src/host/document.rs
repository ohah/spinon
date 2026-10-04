#[cfg(test)]
use spinon_core::HostDocumentSnapshot;
use spinon_core::{HostDocument, HostNodeHandle, OwnerId};
use std::collections::{BTreeMap, BTreeSet};
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

pub(crate) const MAX_BATCH_OPERATIONS: usize = 256;
const MAX_DOCUMENT_NODES: usize = 16_384;
const MAX_PENDING_EXTERNAL_IDS: usize = MAX_BATCH_OPERATIONS;
const MAX_DOCUMENT_STRING_UNITS: usize = 16_777_216;
const MAX_NAME_UNITS: usize = 1024;
const MAX_VALUE_UNITS: usize = 1_048_576;
const MAX_BATCH_STRING_UNITS: usize = 1_048_576;

mod batch;
mod callback;
mod collection;
mod ids;
mod query;
pub(crate) use callback::{DocumentCommitCallback, SpinonDocumentReceipt, commit_callback};
#[cfg(test)]
pub(crate) use callback::{
    OP_APPEND, OP_CREATE_ELEMENT, OP_CREATE_TEXT, OP_SET_ATTRIBUTE, OP_SET_TEXT,
    SpinonDocumentOperation,
};
pub(crate) use collection::{DocumentCollectCallback, collect_callback};
pub(crate) use query::{DocumentQueryCallback, query_callback};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum DocumentBatchOperation {
    CreateElement {
        id: i32,
        namespace: String,
        name: String,
    },
    CreateText {
        id: i32,
        data: Vec<u16>,
    },
    Append {
        parent: i32,
        node: i32,
    },
    InsertBefore {
        parent: i32,
        node: i32,
        before: i32,
    },
    Remove {
        parent: i32,
        node: i32,
    },
    SetText {
        node: i32,
        data: Vec<u16>,
    },
    SetAttribute {
        node: i32,
        name: String,
        value: Vec<u16>,
    },
    RemoveAttribute {
        node: i32,
        name: String,
    },
}

#[derive(Debug)]
pub(crate) struct HostDocumentBridge {
    document: HostDocument,
    owner: OwnerId,
    handles: BTreeMap<i32, HostNodeHandle>,
    external_ids: BTreeMap<HostNodeHandle, i32>,
    string_usage: BTreeMap<i32, NodeStringUsage>,
    string_units: usize,
    next_external_id: Option<i32>,
    reserved_external_ids: BTreeSet<i32>,
}

#[derive(Clone, Debug, Default)]
struct NodeStringUsage {
    base_units: usize,
    text_units: usize,
    attributes: BTreeMap<String, usize>,
}

impl NodeStringUsage {
    fn total_units(&self) -> Option<usize> {
        self.base_units
            .checked_add(self.text_units)
            .and_then(|total| {
                self.attributes
                    .values()
                    .try_fold(total, |sum, units| sum.checked_add(*units))
            })
    }
}

struct DocumentStage {
    handles: BTreeMap<i32, HostNodeHandle>,
    external_ids: BTreeMap<HostNodeHandle, i32>,
    string_usage: BTreeMap<i32, NodeStringUsage>,
    string_units: usize,
    reserved: Vec<HostNodeHandle>,
}

impl HostDocumentBridge {
    pub(crate) fn new() -> Result<Self, String> {
        let document = HostDocument::new().map_err(|error| error.to_string())?;
        let owner = OwnerId::new(1).ok_or_else(|| "문서 소유자 ID가 잘못되었습니다".to_owned())?;
        Ok(Self {
            document,
            owner,
            handles: BTreeMap::new(),
            external_ids: BTreeMap::new(),
            string_usage: BTreeMap::new(),
            string_units: 0,
            next_external_id: Some(1),
            reserved_external_ids: BTreeSet::new(),
        })
    }

    pub(crate) fn commit(
        &mut self,
        operations: &[DocumentBatchOperation],
    ) -> Result<SpinonDocumentReceipt, String> {
        self.consume_creation_ids(operations)?;
        if operations.len() > MAX_BATCH_OPERATIONS {
            return Err(format!(
                "문서 변경 묶음은 최대 {MAX_BATCH_OPERATIONS}개 작업까지 허용합니다"
            ));
        }
        batch::validate_string_limits(operations)?;

        let new_nodes = operations
            .iter()
            .filter(|operation| {
                matches!(
                    operation,
                    DocumentBatchOperation::CreateElement { .. }
                        | DocumentBatchOperation::CreateText { .. }
                )
            })
            .count();
        if self.handles.len().saturating_add(new_nodes) > MAX_DOCUMENT_NODES {
            return Err(format!(
                "QuotaExceededError: 런타임 문서는 최대 {MAX_DOCUMENT_NODES}개 노드를 보존합니다"
            ));
        }

        let mut stage = DocumentStage {
            handles: self.handles.clone(),
            external_ids: self.external_ids.clone(),
            string_usage: self.string_usage.clone(),
            string_units: self.string_units,
            reserved: Vec::with_capacity(operations.len()),
        };
        let result = catch_unwind(AssertUnwindSafe(|| {
            self.build_and_commit(operations, &mut stage)
        }));
        match result {
            Ok(Ok(receipt)) => {
                self.handles = stage.handles;
                self.external_ids = stage.external_ids;
                self.string_usage = stage.string_usage;
                self.string_units = stage.string_units;
                let mut receipt = SpinonDocumentReceipt::from(receipt);
                receipt.node_count = self.handles.len() as u64;
                Ok(receipt)
            }
            Ok(Err(error)) => {
                for handle in stage.reserved {
                    let _ = self.document.cancel_node_handle_reservation(handle);
                }
                Err(error)
            }
            Err(payload) => {
                for handle in stage.reserved {
                    let _ = self.document.cancel_node_handle_reservation(handle);
                }
                resume_unwind(payload)
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn snapshot(&self) -> HostDocumentSnapshot {
        self.document.snapshot()
    }

    pub(crate) fn document_revision(&self) -> u64 {
        self.document.document_revision().get()
    }

    pub(crate) fn render_tree_revision(&self) -> u64 {
        self.document.render_tree_revision().get()
    }

    pub(crate) fn node_count(&self) -> u64 {
        self.handles.len() as u64
    }

    pub(crate) fn external_id(&self, handle: HostNodeHandle) -> Option<i32> {
        self.external_ids.get(&handle).copied()
    }

    pub(crate) fn next_external_id(&mut self) -> Result<i32, String> {
        if self.reserved_external_ids.len() >= MAX_PENDING_EXTERNAL_IDS {
            return Err(format!(
                "QuotaExceededError: 대기 중인 노드 ID는 {MAX_PENDING_EXTERNAL_IDS}개까지 예약할 수 있습니다"
            ));
        }
        let id = self.next_external_id.ok_or_else(|| {
            "QuotaExceededError: 노드 연결 키 공간을 모두 사용했습니다".to_owned()
        })?;
        self.next_external_id = id.checked_add(1);
        self.reserved_external_ids.insert(id);
        Ok(id)
    }

    pub(crate) fn handle(&self, external_id: i32) -> Option<HostNodeHandle> {
        self.handles.get(&external_id).copied()
    }

    #[cfg(test)]
    pub(crate) fn string_units(&self) -> usize {
        self.string_units
    }
}

#[cfg(test)]
mod callback_tests;
#[cfg(test)]
mod collection_tests;
#[cfg(test)]
mod lifecycle_contract_tests;
#[cfg(test)]
mod limit_tests;
#[cfg(test)]
mod tests;
