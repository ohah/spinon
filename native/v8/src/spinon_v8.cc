#include "spinon_v8.h"

#include <libplatform/libplatform.h>
#include <v8.h>

#include <chrono>
#include <map>
#include <memory>
#include <mutex>
#include <new>
#include <string>
#include <utility>
#include <vector>

#if defined(__ANDROID__) || defined(__linux__)
#include <sys/syscall.h>
#include <unistd.h>
#elif defined(__APPLE__)
#include <pthread.h>
#endif

struct SpinonV8Runtime {
  v8::Isolate *isolate = nullptr;
  v8::ArrayBuffer::Allocator *allocator = nullptr;
  v8::Global<v8::Context> context;
  v8::Global<v8::Function> event_handler;
  SpinonNodeCallback node_callback = nullptr;
  SpinonTextCallback text_callback = nullptr;
  SpinonDocumentCommitCallback document_commit_callback = nullptr;
  SpinonDocumentQueryCallback document_query_callback = nullptr;
  SpinonDocumentCollectCallback document_collect_callback = nullptr;
  void *user_data = nullptr;
  void *document_user_data = nullptr;
  std::map<int32_t, v8::Global<v8::Object>> node_wrappers;
  std::vector<int32_t> wrapper_root_ids;
  std::vector<int32_t> reclaimed_node_ids;
  SpinonDocumentCollectionStats document_collection_stats{};
#if defined(SPINON_ENABLE_S03_DOM_GC_FIXTURE) && SPINON_ENABLE_S03_DOM_GC_FIXTURE
  bool request_collection_for_testing = false;
#endif
  std::string error;
  std::string collection_error;
  bool was_terminated = false;
  bool document_commit_active = false;
  bool document_collection_poisoned = false;
};

namespace {
std::once_flag platform_once;
std::unique_ptr<v8::Platform> platform;

void InitializeV8() {
  platform = v8::platform::NewDefaultPlatform();
  v8::V8::InitializePlatform(platform.get());
  v8::V8::Initialize();
}

void CreateNode(const v8::FunctionCallbackInfo<v8::Value> &args) {
  auto *runtime = static_cast<SpinonV8Runtime *>(args.GetIsolate()->GetData(0));
  if (args.Length() != 2 || !args[0]->IsInt32() || !args[1]->IsString()) {
    args.GetIsolate()->ThrowException(v8::String::NewFromUtf8Literal(
        args.GetIsolate(), "spinon.createNode(id, tag) requires an integer and a string"));
    return;
  }
  v8::String::Utf8Value tag(args.GetIsolate(), args[1]);
  if (runtime->node_callback) {
    runtime->node_callback(runtime->user_data,
                           args[0].As<v8::Int32>()->Value(), *tag ? *tag : "");
  }
}

void SetText(const v8::FunctionCallbackInfo<v8::Value> &args) {
  auto *runtime = static_cast<SpinonV8Runtime *>(args.GetIsolate()->GetData(0));
  if (args.Length() != 1 || !args[0]->IsString()) {
    args.GetIsolate()->ThrowException(v8::String::NewFromUtf8Literal(
        args.GetIsolate(), "spinon.setText(text) requires a string"));
    return;
  }
  v8::String::Utf8Value text(args.GetIsolate(), args[0]);
  if (runtime->text_callback) {
    runtime->text_callback(runtime->user_data, *text ? *text : "");
  }
}

struct OwnedDocumentOperation {
  SpinonDocumentOperation raw{};
  std::vector<uint16_t> namespace_utf16;
  std::vector<uint16_t> name_utf16;
  std::vector<uint16_t> value_utf16;
};

constexpr size_t kMaximumDocumentOperations = 256;
constexpr size_t kMaximumDocumentNodes = 16'384;
constexpr size_t kMaximumBatchStringUnits = 1'048'576;
constexpr int kMaximumNameUnits = 1024;
constexpr int kMaximumValueUnits = 1'048'576;
constexpr int kMaximumOperationTypeUnits = 32;
constexpr char16_t kHtmlNamespace[] = u"http://www.w3.org/1999/xhtml";
constexpr char kDomFacadeSource[] =
#include "spinon_dom_facade.inc"
    ;

class ScopedBoolean {
 public:
  explicit ScopedBoolean(bool *value) : value_(value) { *value_ = true; }
  ~ScopedBoolean() { *value_ = false; }
  ScopedBoolean(const ScopedBoolean &) = delete;
  ScopedBoolean &operator=(const ScopedBoolean &) = delete;

 private:
  bool *value_;
};

bool ReadProperty(v8::Isolate *isolate, v8::Local<v8::Context> context,
                  v8::Local<v8::Object> object,
                  const char *name, v8::Local<v8::Value> *value) {
  v8::Local<v8::String> key = v8::String::NewFromUtf8(
                                  isolate, name, v8::NewStringType::kNormal)
                                  .ToLocalChecked();
  return object->Get(context, key).ToLocal(value);
}

bool ReadRequiredInt32(v8::Isolate *isolate, v8::Local<v8::Context> context,
                       v8::Local<v8::Object> object, const char *name,
                       int32_t *result) {
  v8::Local<v8::Value> value;
  if (!ReadProperty(isolate, context, object, name, &value) || !value->IsInt32())
    return false;
  *result = value.As<v8::Int32>()->Value();
  return true;
}

bool ReadRequiredString(v8::Isolate *isolate, v8::Local<v8::Context> context,
                        v8::Local<v8::Object> object, const char *name,
                        int maximum_units, std::vector<uint16_t> *result) {
  v8::Local<v8::Value> value;
  if (!ReadProperty(isolate, context, object, name, &value) ||
      !value->IsString())
    return false;
  v8::Local<v8::String> string = value.As<v8::String>();
  const int length = string->Length();
  if (length < 0 || length > maximum_units) return false;
  result->resize(static_cast<size_t>(length));
  if (length > 0) {
    string->Write(isolate, 0, static_cast<uint32_t>(length), result->data());
  }
  return true;
}

bool ReadOptionalNamespace(v8::Isolate *isolate,
                           v8::Local<v8::Context> context,
                           v8::Local<v8::Object> object,
                           std::vector<uint16_t> *result) {
  v8::Local<v8::Value> value;
  if (!ReadProperty(isolate, context, object, "namespace", &value))
    return false;
  if (value->IsUndefined()) {
    result->reserve(sizeof(kHtmlNamespace) / sizeof(kHtmlNamespace[0]) - 1);
    for (size_t index = 0;
         index < sizeof(kHtmlNamespace) / sizeof(kHtmlNamespace[0]) - 1;
         ++index) {
      result->push_back(static_cast<uint16_t>(kHtmlNamespace[index]));
    }
    return true;
  }
  if (!value->IsString()) return false;
  v8::Local<v8::String> string = value.As<v8::String>();
  const int length = string->Length();
  if (length < 0 || length > kMaximumNameUnits) return false;
  result->resize(static_cast<size_t>(length));
  if (length > 0) {
    string->Write(isolate, 0, static_cast<uint32_t>(length), result->data());
  }
  return true;
}

bool ReadOperationType(v8::Isolate *isolate, v8::Local<v8::Context> context,
                       v8::Local<v8::Object> object, std::string *result) {
  v8::Local<v8::Value> value;
  if (!ReadProperty(isolate, context, object, "type", &value) ||
      !value->IsString())
    return false;
  if (value.As<v8::String>()->Length() > kMaximumOperationTypeUnits)
    return false;
  v8::String::Utf8Value text(isolate, value);
  if (*text == nullptr) return false;
  result->assign(*text, static_cast<size_t>(text.length()));
  return true;
}

bool ParseDocumentOperation(v8::Isolate *isolate,
                            v8::Local<v8::Context> context,
                            v8::Local<v8::Value> value,
                            OwnedDocumentOperation *operation) {
  if (!value->IsObject() || value->IsNull() || value->IsArray()) return false;
  v8::Local<v8::Object> object = value.As<v8::Object>();
  std::string type;
  if (!ReadOperationType(isolate, context, object, &type)) return false;

  if (type == "createElement") {
    operation->raw.kind = 1;
    if (!ReadRequiredInt32(isolate, context, object, "id", &operation->raw.node_id) ||
        !ReadRequiredString(isolate, context, object, "name", kMaximumNameUnits,
                            &operation->name_utf16) ||
        !ReadOptionalNamespace(isolate, context, object,
                               &operation->namespace_utf16)) {
      return false;
    }
  } else if (type == "createText") {
    operation->raw.kind = 2;
    if (!ReadRequiredInt32(isolate, context, object, "id", &operation->raw.node_id) ||
        !ReadRequiredString(isolate, context, object, "data", kMaximumValueUnits,
                            &operation->value_utf16)) {
      return false;
    }
  } else if (type == "append") {
    operation->raw.kind = 3;
    if (!ReadRequiredInt32(isolate, context, object, "parent", &operation->raw.parent_id) ||
        !ReadRequiredInt32(isolate, context, object, "node", &operation->raw.node_id)) {
      return false;
    }
  } else if (type == "insertBefore") {
    operation->raw.kind = 4;
    if (!ReadRequiredInt32(isolate, context, object, "parent", &operation->raw.parent_id) ||
        !ReadRequiredInt32(isolate, context, object, "node", &operation->raw.node_id) ||
        !ReadRequiredInt32(isolate, context, object, "before", &operation->raw.before_id)) {
      return false;
    }
  } else if (type == "remove") {
    operation->raw.kind = 5;
    if (!ReadRequiredInt32(isolate, context, object, "parent", &operation->raw.parent_id) ||
        !ReadRequiredInt32(isolate, context, object, "node", &operation->raw.node_id)) {
      return false;
    }
  } else if (type == "setText") {
    operation->raw.kind = 6;
    if (!ReadRequiredInt32(isolate, context, object, "node", &operation->raw.node_id) ||
        !ReadRequiredString(isolate, context, object, "data", kMaximumValueUnits,
                            &operation->value_utf16)) {
      return false;
    }
  } else if (type == "setAttribute") {
    operation->raw.kind = 7;
    if (!ReadRequiredInt32(isolate, context, object, "node", &operation->raw.node_id) ||
        !ReadRequiredString(isolate, context, object, "name", kMaximumNameUnits,
                            &operation->name_utf16) ||
        !ReadRequiredString(isolate, context, object, "value", kMaximumValueUnits,
                            &operation->value_utf16)) {
      return false;
    }
  } else if (type == "removeAttribute") {
    operation->raw.kind = 8;
    if (!ReadRequiredInt32(isolate, context, object, "node", &operation->raw.node_id) ||
        !ReadRequiredString(isolate, context, object, "name", kMaximumNameUnits,
                            &operation->name_utf16)) {
      return false;
    }
  } else {
    return false;
  }

  operation->raw.namespace_utf16 = operation->namespace_utf16.empty()
                                       ? nullptr
                                       : operation->namespace_utf16.data();
  operation->raw.namespace_length = operation->namespace_utf16.size();
  operation->raw.name_utf16 = operation->name_utf16.empty()
                                  ? nullptr
                                  : operation->name_utf16.data();
  operation->raw.name_length = operation->name_utf16.size();
  operation->raw.value_utf16 = operation->value_utf16.empty()
                                   ? nullptr
                                   : operation->value_utf16.data();
  operation->raw.value_length = operation->value_utf16.size();
  return true;
}

void ThrowDocumentError(v8::Isolate *isolate, const std::string &message,
                        bool type_error, const char *name = nullptr) {
  v8::Local<v8::String> text;
  if (!v8::String::NewFromUtf8(isolate, message.data(),
                               v8::NewStringType::kNormal,
                               static_cast<int>(message.size()))
           .ToLocal(&text)) {
    text = v8::String::NewFromUtf8Literal(isolate,
                                         "문서 변경 호출이 실패했습니다");
  }
  v8::Local<v8::Value> exception = type_error
                                      ? v8::Exception::TypeError(text)
                                      : v8::Exception::Error(text);
  if (!type_error && name != nullptr) {
    auto context = isolate->GetCurrentContext();
    auto key = v8::String::NewFromUtf8Literal(isolate, "name");
    auto value = v8::String::NewFromUtf8(isolate, name).ToLocalChecked();
    exception.As<v8::Object>()->Set(context, key, value).Check();
  }
  isolate->ThrowException(exception);
}

void CommitDocumentBatch(const v8::FunctionCallbackInfo<v8::Value> &args) {
  auto *runtime = static_cast<SpinonV8Runtime *>(args.GetIsolate()->GetData(0));
  v8::Isolate *isolate = args.GetIsolate();
  v8::Local<v8::Context> context = isolate->GetCurrentContext();
  if (runtime->document_commit_active) {
    ThrowDocumentError(isolate, "문서 변경 묶음은 중첩 호출할 수 없습니다", true);
    return;
  }
  ScopedBoolean active(&runtime->document_commit_active);
  v8::TryCatch try_catch(isolate);
  if (args.Length() != 1 || !args[0]->IsArray()) {
    ThrowDocumentError(isolate,
                       "commitDocumentBatch(operations)는 배열 하나를 받아야 합니다",
                       true);
    try_catch.ReThrow();
    return;
  }
  auto array = args[0].As<v8::Array>();
  const uint32_t operation_count = array->Length();
  if (operation_count > kMaximumDocumentOperations) {
    ThrowDocumentError(isolate,
                       "문서 변경 묶음은 최대 256개 작업까지 허용합니다",
                       true);
    try_catch.ReThrow();
    return;
  }

  std::vector<OwnedDocumentOperation> owned;
  owned.reserve(operation_count);
  size_t batch_string_units = 0;
  for (uint32_t index = 0; index < operation_count; ++index) {
    v8::Local<v8::Value> value;
    if (!array->Get(context, index).ToLocal(&value)) {
      try_catch.ReThrow();
      return;
    }
    OwnedDocumentOperation operation;
    if (!ParseDocumentOperation(isolate, context, value, &operation)) {
      if (try_catch.HasCaught()) {
        try_catch.ReThrow();
        return;
      }
      ThrowDocumentError(isolate,
                         "문서 변경 작업의 종류·필드·문자열이 잘못되었습니다",
                         true);
      try_catch.ReThrow();
      return;
    }
    const size_t operation_string_units =
        operation.namespace_utf16.size() + operation.name_utf16.size() +
        operation.value_utf16.size();
    if (operation_string_units >
        kMaximumBatchStringUnits - batch_string_units) {
      ThrowDocumentError(isolate,
                         "문서 변경 묶음 문자열은 UTF-16 코드 단위 1048576개까지 허용합니다",
                         true);
      try_catch.ReThrow();
      return;
    }
    batch_string_units += operation_string_units;
    owned.push_back(std::move(operation));
  }

  if (runtime->document_commit_callback == nullptr) {
    ThrowDocumentError(isolate, "문서 변경 callback이 등록되지 않았습니다", false);
    try_catch.ReThrow();
    return;
  }
  std::vector<SpinonDocumentOperation> operations;
  operations.reserve(owned.size());
  for (const auto &operation : owned) operations.push_back(operation.raw);
  SpinonDocumentReceipt receipt{};
  char error[1024] = {};
  const int32_t status = runtime->document_commit_callback(
      runtime->document_user_data, operations.data(), operations.size(), &receipt,
      error, sizeof(error));
  if (status != SPINON_DOCUMENT_CALLBACK_OK) {
    const char *name = status == SPINON_DOCUMENT_CALLBACK_QUOTA_EXCEEDED
                           ? "QuotaExceededError"
                           : nullptr;
    ThrowDocumentError(isolate,
                       error[0] == '\0' ? "문서 변경 묶음이 거부되었습니다"
                                        : std::string(error),
                       false, name);
    try_catch.ReThrow();
    return;
  }

  auto result = v8::Object::New(isolate);
  result->Set(context, v8::String::NewFromUtf8Literal(isolate, "changed"),
              v8::Boolean::New(isolate, receipt.changed != 0))
      .Check();
  result->Set(context,
              v8::String::NewFromUtf8Literal(isolate, "documentRevision"),
              v8::BigInt::NewFromUnsigned(isolate, receipt.document_revision))
      .Check();
  result->Set(context,
              v8::String::NewFromUtf8Literal(isolate, "renderTreeRevision"),
              v8::BigInt::NewFromUnsigned(isolate, receipt.render_tree_revision))
      .Check();
  result->Set(context, v8::String::NewFromUtf8Literal(isolate, "nodeCount"),
              v8::BigInt::NewFromUnsigned(isolate, receipt.node_count))
      .Check();
  args.GetReturnValue().Set(result);
}

void ReadDocument(const v8::FunctionCallbackInfo<v8::Value> &args) {
  auto *runtime = static_cast<SpinonV8Runtime *>(args.GetIsolate()->GetData(0));
  v8::Isolate *isolate = args.GetIsolate();
  v8::Local<v8::Context> context = isolate->GetCurrentContext();
  if (args.Length() < 2 || args.Length() > 4 || !args[0]->IsInt32() ||
      !args[1]->IsInt32() || (args.Length() > 2 && !args[2]->IsInt32()) ||
      (args.Length() > 3 && !args[3]->IsString())) {
    ThrowDocumentError(isolate,
                       "readDocument(kind, nodeId, index?, name?) 인자가 잘못되었습니다",
                       true);
    return;
  }
  if (runtime->document_query_callback == nullptr) {
    ThrowDocumentError(isolate, "문서 조회 callback이 등록되지 않았습니다", false);
    return;
  }

  std::vector<uint16_t> name;
  if (args.Length() > 3) {
    v8::Local<v8::String> input = args[3].As<v8::String>();
    if (input->Length() > 1024) {
      ThrowDocumentError(isolate, "조회 이름이 허용 길이를 초과했습니다", true);
      return;
    }
    v8::String::Value value(isolate, input);
    if (*value != nullptr) name.assign(*value, *value + value.length());
  }

  SpinonDocumentQuery query{};
  query.kind = args[0].As<v8::Int32>()->Value();
  query.node_id = args[1].As<v8::Int32>()->Value();
  query.index = args.Length() > 2 ? args[2].As<v8::Int32>()->Value() : 0;
  query.name_utf16 = name.empty() ? nullptr : name.data();
  query.name_length = name.size();
  SpinonDocumentQueryResult result{};
  char error[1024] = {};
  int32_t status = runtime->document_query_callback(
      runtime->document_user_data, &query, &result, error, sizeof(error));
  if (status != SPINON_DOCUMENT_CALLBACK_OK &&
      status != SPINON_DOCUMENT_CALLBACK_BUFFER_TOO_SMALL) {
    ThrowDocumentError(isolate,
                       error[0] == '\0' ? "문서 조회가 거부되었습니다"
                                        : std::string(error),
                       false);
    return;
  }
  if (result.output_length > 16'777'216) {
    ThrowDocumentError(isolate, "문서 조회 결과가 허용 길이를 초과했습니다", false);
    return;
  }
  std::vector<uint16_t> output;
  try {
    output.resize(result.output_length);
  } catch (const std::bad_alloc &) {
    ThrowDocumentError(isolate, "문서 조회 문자열 버퍼를 확보하지 못했습니다", false);
    return;
  }
  if (!output.empty()) {
    query.output_utf16 = output.data();
    query.output_capacity = output.size();
    status = runtime->document_query_callback(
        runtime->document_user_data, &query, &result, error, sizeof(error));
    if (status != SPINON_DOCUMENT_CALLBACK_OK) {
      ThrowDocumentError(isolate,
                         error[0] == '\0' ? "문서 조회 결과를 읽지 못했습니다"
                                          : std::string(error),
                         false);
      return;
    }
  }

  auto text = output.empty()
                  ? v8::String::Empty(isolate)
                  : v8::String::NewFromTwoByte(
                        isolate, output.data(), v8::NewStringType::kNormal,
                        static_cast<int>(output.size()));
  if (text.IsEmpty()) {
    ThrowDocumentError(isolate, "문서 조회 문자열을 만들지 못했습니다", false);
    return;
  }
  auto value = v8::Object::New(isolate);
  value->Set(context, v8::String::NewFromUtf8Literal(isolate, "exists"),
             v8::Boolean::New(isolate, result.exists != 0))
      .Check();
  value->Set(context, v8::String::NewFromUtf8Literal(isolate, "value"),
             v8::Int32::New(isolate, result.value))
      .Check();
  value->Set(context, v8::String::NewFromUtf8Literal(isolate, "text"),
             text.ToLocalChecked())
      .Check();
  args.GetReturnValue().Set(value);
}

void GetNodeWrapper(const v8::FunctionCallbackInfo<v8::Value> &args) {
  v8::Isolate *isolate = args.GetIsolate();
  auto *runtime = static_cast<SpinonV8Runtime *>(isolate->GetData(0));
  if (args.Length() != 1 || !args[0]->IsInt32() ||
      args[0].As<v8::Int32>()->Value() <= 0) {
    ThrowDocumentError(isolate, "getNodeWrapper(id) 인자가 잘못되었습니다", true);
    return;
  }
  const int32_t id = args[0].As<v8::Int32>()->Value();
  auto found = runtime->node_wrappers.find(id);
  if (found == runtime->node_wrappers.end() || found->second.IsEmpty()) {
    args.GetReturnValue().Set(v8::Undefined(isolate));
    return;
  }
  args.GetReturnValue().Set(found->second.Get(isolate));
}

void RegisterNodeWrapper(const v8::FunctionCallbackInfo<v8::Value> &args) {
  v8::Isolate *isolate = args.GetIsolate();
  auto *runtime = static_cast<SpinonV8Runtime *>(isolate->GetData(0));
  if (args.Length() != 2 || !args[0]->IsInt32() ||
      args[0].As<v8::Int32>()->Value() <= 0 || !args[1]->IsObject() ||
      args[1]->IsNull()) {
    ThrowDocumentError(isolate,
                       "registerNodeWrapper(id, wrapper) 인자가 잘못되었습니다",
                       true);
    return;
  }
  const int32_t id = args[0].As<v8::Int32>()->Value();
  auto wrapper = args[1].As<v8::Object>();
  auto found = runtime->node_wrappers.find(id);
  if (found != runtime->node_wrappers.end()) {
    if (!found->second.IsEmpty()) {
      args.GetReturnValue().Set(found->second == wrapper);
      return;
    }
    found->second.Reset(isolate, wrapper);
    found->second.SetWeak();
    args.GetReturnValue().Set(true);
    return;
  }
  if (runtime->node_wrappers.size() >= kMaximumDocumentNodes) {
    ThrowDocumentError(isolate,
                       "Node wrapper registry가 허용 개수를 초과했습니다",
                       false, "QuotaExceededError");
    return;
  }
  try {
    v8::Global<v8::Object> weak_wrapper(isolate, wrapper);
    weak_wrapper.SetWeak();
    runtime->node_wrappers.emplace(id, std::move(weak_wrapper));
  } catch (const std::bad_alloc &) {
    ThrowDocumentError(isolate, "Node wrapper registry 메모리를 확보하지 못했습니다",
                       false);
    return;
  }
  args.GetReturnValue().Set(true);
}

void UnregisterNodeWrapper(const v8::FunctionCallbackInfo<v8::Value> &args) {
  v8::Isolate *isolate = args.GetIsolate();
  auto *runtime = static_cast<SpinonV8Runtime *>(isolate->GetData(0));
  if (args.Length() != 2 || !args[0]->IsInt32() ||
      args[0].As<v8::Int32>()->Value() <= 0 || !args[1]->IsObject() ||
      args[1]->IsNull()) {
    ThrowDocumentError(isolate,
                       "unregisterNodeWrapper(id, wrapper) 인자가 잘못되었습니다",
                       true);
    return;
  }
  const int32_t id = args[0].As<v8::Int32>()->Value();
  auto found = runtime->node_wrappers.find(id);
  if (found != runtime->node_wrappers.end() && !found->second.IsEmpty() &&
      found->second == args[1].As<v8::Object>()) {
    found->second.Reset();
    runtime->node_wrappers.erase(found);
    args.GetReturnValue().Set(true);
    return;
  }
  args.GetReturnValue().Set(false);
}

#if defined(SPINON_ENABLE_S03_DOM_GC_FIXTURE) && SPINON_ENABLE_S03_DOM_GC_FIXTURE
void RequestLifecycleCollectionForTesting(
    const v8::FunctionCallbackInfo<v8::Value> &args) {
  auto *runtime = static_cast<SpinonV8Runtime *>(args.GetIsolate()->GetData(0));
  if (args.Length() != 0) {
    ThrowDocumentError(args.GetIsolate(),
                       "requestLifecycleCollectionForTesting()은 인자를 받지 않습니다",
                       true);
    return;
  }
  runtime->request_collection_for_testing = true;
}
#endif

void OnEvent(const v8::FunctionCallbackInfo<v8::Value> &args) {
  auto *runtime = static_cast<SpinonV8Runtime *>(args.GetIsolate()->GetData(0));
  if (args.Length() != 1 || !args[0]->IsFunction()) {
    args.GetIsolate()->ThrowException(v8::String::NewFromUtf8Literal(
        args.GetIsolate(), "spinon.onEvent(handler) requires a function"));
    return;
  }
  runtime->event_handler.Reset(args.GetIsolate(), args[0].As<v8::Function>());
}

std::string ExceptionText(v8::Isolate *isolate, v8::TryCatch &try_catch) {
  v8::String::Utf8Value text(isolate, try_catch.Exception());
  return *text ? *text : "unknown JavaScript error";
}

bool Enter(SpinonV8Runtime *runtime, v8::Local<v8::Context> *context) {
  if (!runtime || !runtime->isolate || runtime->context.IsEmpty()) return false;
  *context = runtime->context.Get(runtime->isolate);
  return !context->IsEmpty();
}

int32_t CollectDocumentAtSafePoint(SpinonV8Runtime *runtime) {
#if defined(SPINON_ENABLE_S03_DOM_GC_FIXTURE) && SPINON_ENABLE_S03_DOM_GC_FIXTURE
  if (runtime->request_collection_for_testing) {
    runtime->request_collection_for_testing = false;
    runtime->isolate->LowMemoryNotification();
  }
#endif
  using Clock = std::chrono::steady_clock;
  const auto scan_started = Clock::now();
  auto &stats = runtime->document_collection_stats;
  if (stats.scan_count != UINT64_MAX) ++stats.scan_count;
  const auto scan_started_ns = std::chrono::duration_cast<std::chrono::nanoseconds>(
                                   scan_started.time_since_epoch())
                                   .count();
  stats.last_scan_start_ns = scan_started_ns < 0
                                 ? 0
                                 : static_cast<uint64_t>(scan_started_ns);
  auto finish_scan = [&]() {
    const auto duration = std::chrono::duration_cast<std::chrono::microseconds>(
                              Clock::now() - scan_started)
                              .count();
    stats.last_scan_duration_us = duration < 0 ? 0 : static_cast<uint64_t>(duration);
  };
  auto defer = [&](const std::string &message) {
    runtime->collection_error = message;
    if (stats.deferred_count != UINT64_MAX) ++stats.deferred_count;
    finish_scan();
    return -1;
  };
  auto poison = [&](const std::string &message) {
    runtime->document_collection_poisoned = true;
    stats.runtime_poisoned = 1;
    return defer(message);
  };
  if (runtime->document_collect_callback == nullptr) {
    return defer("문서 회수 callback이 등록되지 않았습니다");
  }

  runtime->wrapper_root_ids.clear();
  stats.scanned_handle_count = runtime->node_wrappers.size();
  stats.live_handle_count = 0;
  stats.empty_handle_count = 0;
  for (const auto &entry : runtime->node_wrappers) {
    if (entry.second.IsEmpty()) {
      ++stats.empty_handle_count;
    } else {
      ++stats.live_handle_count;
    }
  }
  if (runtime->node_wrappers.size() > kMaximumDocumentNodes) {
    return defer("Node wrapper registry가 허용 개수를 초과했습니다");
  }
  for (const auto &entry : runtime->node_wrappers) {
    if (entry.second.IsEmpty()) continue;
    if (runtime->wrapper_root_ids.size() == kMaximumDocumentNodes) {
      return defer("Node wrapper root가 허용 개수를 초과했습니다");
    }
    runtime->wrapper_root_ids.push_back(entry.first);
  }

  size_t reclaimed_count = 0;
  char callback_error[1024] = {};
  const int32_t status = runtime->document_collect_callback(
      runtime->document_user_data,
      runtime->wrapper_root_ids.empty() ? nullptr
                                        : runtime->wrapper_root_ids.data(),
      runtime->wrapper_root_ids.size(), runtime->reclaimed_node_ids.data(),
      runtime->reclaimed_node_ids.size(), &reclaimed_count, callback_error,
      sizeof(callback_error));
  if (status != SPINON_DOCUMENT_CALLBACK_OK) {
    return defer(callback_error[0] == '\0'
                     ? "HostDocument weak wrapper 회수가 실패했습니다"
                     : callback_error);
  }
  if (reclaimed_count > runtime->reclaimed_node_ids.size()) {
    return poison("HostDocument 회수 callback이 출력 buffer 범위를 넘었습니다");
  }

  int32_t previous_id = 0;
  for (size_t index = 0; index < reclaimed_count; ++index) {
    const int32_t id = runtime->reclaimed_node_ids[index];
    if (id <= previous_id) {
      return poison("HostDocument 회수 결과 ID가 오름차순 고유값이 아닙니다");
    }
    previous_id = id;
    auto found = runtime->node_wrappers.find(id);
    if (found != runtime->node_wrappers.end() && !found->second.IsEmpty()) {
      return poison("HostDocument가 live JavaScript wrapper를 회수했습니다");
    }
  }
  for (size_t index = 0; index < reclaimed_count; ++index) {
    auto found = runtime->node_wrappers.find(runtime->reclaimed_node_ids[index]);
    if (found == runtime->node_wrappers.end()) continue;
    found->second.Reset();
    runtime->node_wrappers.erase(found);
  }
  finish_scan();
  return 0;
}
}  // namespace

extern "C" SpinonV8Runtime *spinon_v8_runtime_new(
    SpinonNodeCallback node_callback, SpinonTextCallback text_callback,
    SpinonDocumentCommitCallback document_commit_callback,
    SpinonDocumentQueryCallback document_query_callback,
    SpinonDocumentCollectCallback document_collect_callback, void *user_data,
    void *document_user_data) {
  std::call_once(platform_once, InitializeV8);
  auto *runtime = new (std::nothrow) SpinonV8Runtime;
  if (runtime == nullptr) return nullptr;
  try {
    runtime->wrapper_root_ids.reserve(kMaximumDocumentNodes);
    // Rust callback의 출력 포인터 범위를 실제 vector 원소로 확보합니다.
    runtime->reclaimed_node_ids.resize(kMaximumDocumentNodes);
  } catch (const std::bad_alloc &) {
    delete runtime;
    return nullptr;
  }
  runtime->node_callback = node_callback;
  runtime->text_callback = text_callback;
  runtime->document_commit_callback = document_commit_callback;
  runtime->document_query_callback = document_query_callback;
  runtime->document_collect_callback = document_collect_callback;
  runtime->user_data = user_data;
  runtime->document_user_data = document_user_data;

  v8::Isolate::CreateParams params;
  runtime->allocator = v8::ArrayBuffer::Allocator::NewDefaultAllocator();
  params.array_buffer_allocator = runtime->allocator;
  runtime->isolate = v8::Isolate::New(params);
  if (!runtime->isolate) {
    delete runtime->allocator;
    delete runtime;
    return nullptr;
  }
  runtime->isolate->SetData(0, runtime);
  bool facade_initialized = false;
  {
    v8::Isolate::Scope isolate_scope(runtime->isolate);
    v8::HandleScope handle_scope(runtime->isolate);
    auto global = v8::ObjectTemplate::New(runtime->isolate);
    auto spinon = v8::ObjectTemplate::New(runtime->isolate);
    spinon->Set(runtime->isolate, "createNode",
                 v8::FunctionTemplate::New(runtime->isolate, CreateNode));
    spinon->Set(runtime->isolate, "setText",
                 v8::FunctionTemplate::New(runtime->isolate, SetText));
    spinon->Set(runtime->isolate, "onEvent",
                 v8::FunctionTemplate::New(runtime->isolate, OnEvent));
    auto internal = v8::ObjectTemplate::New(runtime->isolate);
    internal->Set(runtime->isolate, "commitDocumentBatch",
                  v8::FunctionTemplate::New(runtime->isolate,
                                            CommitDocumentBatch));
    internal->Set(runtime->isolate, "readDocument",
                  v8::FunctionTemplate::New(runtime->isolate, ReadDocument));
    internal->Set(runtime->isolate, "getNodeWrapper",
                  v8::FunctionTemplate::New(runtime->isolate, GetNodeWrapper));
    internal->Set(runtime->isolate, "registerNodeWrapper",
                  v8::FunctionTemplate::New(runtime->isolate,
                                            RegisterNodeWrapper));
    internal->Set(runtime->isolate, "unregisterNodeWrapper",
                  v8::FunctionTemplate::New(runtime->isolate,
                                            UnregisterNodeWrapper));
#if defined(SPINON_ENABLE_S03_DOM_GC_FIXTURE) && SPINON_ENABLE_S03_DOM_GC_FIXTURE
    internal->Set(runtime->isolate, "requestLifecycleCollectionForTesting",
                  v8::FunctionTemplate::New(
                      runtime->isolate,
                      RequestLifecycleCollectionForTesting));
#endif
    spinon->Set(runtime->isolate, "__internal", internal);
    global->Set(runtime->isolate, "spinon", spinon);
    runtime->context.Reset(
        runtime->isolate, v8::Context::New(runtime->isolate, nullptr, global));
    v8::Local<v8::Context> context = runtime->context.Get(runtime->isolate);
    v8::Context::Scope context_scope(context);
    auto source = v8::String::NewFromUtf8(runtime->isolate, kDomFacadeSource);
    v8::Local<v8::Script> script;
    if (source.IsEmpty() ||
        !v8::Script::Compile(context, source.ToLocalChecked()).ToLocal(&script) ||
        script->Run(context).IsEmpty()) {
      facade_initialized = false;
    } else {
      facade_initialized = true;
    }
  }
  if (!facade_initialized) {
    {
      v8::Isolate::Scope isolate_scope(runtime->isolate);
      runtime->context.Reset();
    }
    runtime->isolate->Dispose();
    delete runtime->allocator;
    delete runtime;
    return nullptr;
  }
  return runtime;
}

extern "C" int32_t spinon_v8_runtime_eval(SpinonV8Runtime *runtime,
                                             const char *source) {
  if (!runtime || !source) return -1;
  runtime->error.clear();
  if (runtime->document_collection_poisoned) {
    runtime->error = "HostDocument 회수 경계가 손상되어 세션 재생성이 필요합니다";
    return -1;
  }
  runtime->was_terminated = false;
  v8::Isolate::Scope isolate_scope(runtime->isolate);
  const int32_t result = [&]() {
    v8::HandleScope handle_scope(runtime->isolate);
    v8::Local<v8::Context> context;
    if (!Enter(runtime, &context)) {
      runtime->error = "V8 Context가 준비되지 않았습니다";
      return -1;
    }
    v8::Context::Scope context_scope(context);
    v8::TryCatch try_catch(runtime->isolate);
    auto text = v8::String::NewFromUtf8(runtime->isolate, source);
    if (text.IsEmpty()) {
      runtime->error = "JavaScript source is not valid UTF-8";
      return -1;
    }
    v8::Local<v8::Script> script;
    if (!v8::Script::Compile(context, text.ToLocalChecked()).ToLocal(&script) ||
        script->Run(context).IsEmpty()) {
      runtime->was_terminated = try_catch.HasTerminated();
      runtime->error = ExceptionText(runtime->isolate, try_catch);
      return -1;
    }
    runtime->isolate->PerformMicrotaskCheckpoint();
    return 0;
  }();
  CollectDocumentAtSafePoint(runtime);
  return result;
}

extern "C" int32_t spinon_v8_runtime_dispatch(SpinonV8Runtime *runtime,
                                                 int32_t node_id) {
  if (!runtime) return -1;
  runtime->error.clear();
  if (runtime->document_collection_poisoned) {
    runtime->error = "HostDocument 회수 경계가 손상되어 세션 재생성이 필요합니다";
    return -1;
  }
  runtime->was_terminated = false;
  v8::Isolate::Scope isolate_scope(runtime->isolate);
  const int32_t result = [&]() {
    v8::HandleScope handle_scope(runtime->isolate);
    v8::Local<v8::Context> context;
    if (!Enter(runtime, &context)) {
      runtime->error = "V8 Context가 준비되지 않았습니다";
      return -1;
    }
    v8::Context::Scope context_scope(context);
    if (runtime->event_handler.IsEmpty()) {
      runtime->error = "JavaScript registered no event handler";
      return -1;
    }
    v8::TryCatch try_catch(runtime->isolate);
    auto handler = runtime->event_handler.Get(runtime->isolate);
    v8::Local<v8::Value> arguments[] = {
        v8::Int32::New(runtime->isolate, node_id)};
    if (handler->Call(context, context->Global(), 1, arguments).IsEmpty()) {
      runtime->was_terminated = try_catch.HasTerminated();
      runtime->error = ExceptionText(runtime->isolate, try_catch);
      return -1;
    }
    runtime->isolate->PerformMicrotaskCheckpoint();
    return 0;
  }();
  CollectDocumentAtSafePoint(runtime);
  return result;
}

extern "C" const char *spinon_v8_runtime_last_error(
    SpinonV8Runtime *runtime) {
  return runtime ? runtime->error.c_str() : "null runtime";
}

extern "C" const char *spinon_v8_runtime_last_collection_error(
    SpinonV8Runtime *runtime) {
  return runtime ? runtime->collection_error.c_str() : "null runtime";
}

extern "C" void spinon_v8_runtime_document_collection_stats(
    SpinonV8Runtime *runtime, SpinonDocumentCollectionStats *stats) {
  if (stats == nullptr) return;
  *stats = runtime ? runtime->document_collection_stats
                   : SpinonDocumentCollectionStats{};
}

extern "C" int32_t spinon_v8_runtime_was_terminated(
    SpinonV8Runtime *runtime) {
  return runtime && runtime->was_terminated ? 1 : 0;
}

extern "C" void spinon_v8_runtime_terminate(SpinonV8Runtime *runtime) {
  if (runtime && runtime->isolate) runtime->isolate->TerminateExecution();
}

extern "C" void spinon_v8_runtime_cancel_termination(
    SpinonV8Runtime *runtime) {
  if (runtime && runtime->isolate) runtime->isolate->CancelTerminateExecution();
}

extern "C" uint64_t spinon_v8_current_thread_id() {
#if defined(__ANDROID__) || defined(__linux__)
  return static_cast<uint64_t>(syscall(SYS_gettid));
#elif defined(__APPLE__)
  uint64_t thread_id = 0;
  if (pthread_threadid_np(nullptr, &thread_id) == 0) return thread_id;
  return static_cast<uint64_t>(reinterpret_cast<uintptr_t>(pthread_self()));
#else
  return 0;
#endif
}

extern "C" void spinon_v8_runtime_free(SpinonV8Runtime *runtime) {
  if (!runtime) return;
  {
    v8::Isolate::Scope isolate_scope(runtime->isolate);
    runtime->node_wrappers.clear();
    runtime->event_handler.Reset();
    runtime->context.Reset();
  }
  runtime->isolate->Dispose();
  delete runtime->allocator;
  delete runtime;
}
