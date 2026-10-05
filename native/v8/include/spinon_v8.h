#ifndef SPINON_V8_H
#define SPINON_V8_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct SpinonV8Runtime SpinonV8Runtime;
enum {
  SPINON_DOCUMENT_CALLBACK_OK = 0,
  SPINON_DOCUMENT_CALLBACK_BUFFER_TOO_SMALL = 1,
  SPINON_DOCUMENT_CALLBACK_INVALID = -1,
  SPINON_DOCUMENT_CALLBACK_REJECTED = -2,
  SPINON_DOCUMENT_CALLBACK_PANIC = -3,
  SPINON_DOCUMENT_CALLBACK_QUOTA_EXCEEDED = -4
};
typedef void (*SpinonNodeCallback)(void *user_data, int32_t node_id, const char *tag);
typedef void (*SpinonTextCallback)(void *user_data, const char *text);
typedef struct SpinonDocumentOperation {
  int32_t kind;
  int32_t node_id;
  int32_t parent_id;
  int32_t before_id;
  const uint16_t *namespace_utf16;
  size_t namespace_length;
  const uint16_t *name_utf16;
  size_t name_length;
  const uint16_t *value_utf16;
  size_t value_length;
} SpinonDocumentOperation;
typedef struct SpinonDocumentReceipt {
  uint64_t document_revision;
  uint64_t render_tree_revision;
  uint64_t node_count;
  int32_t changed;
} SpinonDocumentReceipt;
typedef int32_t (*SpinonDocumentCommitCallback)(
    void *document_user_data, const SpinonDocumentOperation *operations,
    size_t operation_count, SpinonDocumentReceipt *receipt,
    char *error_output, size_t error_capacity);
typedef struct SpinonDocumentQuery {
  int32_t kind;
  int32_t node_id;
  int32_t index;
  const uint16_t *name_utf16;
  size_t name_length;
  uint16_t *output_utf16;
  size_t output_capacity;
} SpinonDocumentQuery;
typedef struct SpinonDocumentQueryResult {
  int32_t exists;
  int32_t value;
  size_t output_length;
} SpinonDocumentQueryResult;
typedef struct SpinonDocumentCollectionStats {
  uint64_t scan_count;
  uint64_t deferred_count;
  uint64_t scanned_handle_count;
  uint64_t live_handle_count;
  uint64_t empty_handle_count;
  uint64_t last_scan_start_ns;
  uint64_t last_scan_duration_us;
  uint64_t wrapper_root_buffer_bytes;
  uint64_t reclaimed_node_buffer_bytes;
  uint32_t runtime_poisoned;
} SpinonDocumentCollectionStats;
typedef int32_t (*SpinonDocumentQueryCallback)(
    void *document_user_data, const SpinonDocumentQuery *query,
    SpinonDocumentQueryResult *result, char *error_output,
    size_t error_capacity);
typedef int32_t (*SpinonDocumentCollectCallback)(
    void *document_user_data, const int32_t *root_node_ids, size_t root_count,
    int32_t *reclaimed_node_ids, size_t reclaimed_capacity,
    size_t *reclaimed_count, char *error_output, size_t error_capacity);

SpinonV8Runtime *spinon_v8_runtime_new(SpinonNodeCallback node_callback,
                                      SpinonTextCallback text_callback,
                                      SpinonDocumentCommitCallback document_commit_callback,
                                      SpinonDocumentQueryCallback document_query_callback,
                                      SpinonDocumentCollectCallback document_collect_callback,
                                      void *user_data,
                                      void *document_user_data);
int32_t spinon_v8_runtime_eval(SpinonV8Runtime *runtime, const char *source);
int32_t spinon_v8_runtime_dispatch(SpinonV8Runtime *runtime, int32_t node_id);
const char *spinon_v8_runtime_last_error(SpinonV8Runtime *runtime);
const char *spinon_v8_runtime_last_collection_error(SpinonV8Runtime *runtime);
void spinon_v8_runtime_document_collection_stats(
    SpinonV8Runtime *runtime, SpinonDocumentCollectionStats *stats);
int32_t spinon_v8_runtime_was_terminated(SpinonV8Runtime *runtime);
/* TerminateExecution은 V8가 다른 스레드 호출을 허용하는 유일한 취소 경로입니다. */
void spinon_v8_runtime_terminate(SpinonV8Runtime *runtime);
void spinon_v8_runtime_cancel_termination(SpinonV8Runtime *runtime);
void spinon_v8_runtime_free(SpinonV8Runtime *runtime);
uint64_t spinon_v8_current_thread_id(void);

#ifdef __cplusplus
}
#endif

#endif
