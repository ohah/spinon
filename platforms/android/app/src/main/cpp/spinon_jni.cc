#include <android/log.h>
#include <android/native_window.h>
#include <android/native_window_jni.h>
#include <android/trace.h>
#include <jni.h>
#include <errno.h>
#include <pthread.h>
#include <sys/mman.h>
#include <sys/syscall.h>
#include <unistd.h>

#include "spinon_ffi.h"
#include "spinon_wgpu_r08.h"

#include <array>
#include <cstdint>
#include <cstdio>
#include <cstring>
#include <memory>
#include <new>
#include <string>
#include <vector>

extern "C" int spinon_c124_capture_current_thread_stack(
    uint64_t *thread_id, uintptr_t *start_address, size_t *stack_size,
    size_t *guard_size, size_t *page_size, int *error_code) {
  if (thread_id == nullptr || start_address == nullptr || stack_size == nullptr ||
      guard_size == nullptr || page_size == nullptr || error_code == nullptr) {
    return -1;
  }
  *error_code = 0;
  pthread_attr_t attributes{};
  const int attributes_status = pthread_getattr_np(pthread_self(), &attributes);
  if (attributes_status != 0) {
    *error_code = attributes_status;
    return -1;
  }
  void *stack_base = nullptr;
  size_t observed_stack_size = 0;
  const int stack_status = pthread_attr_getstack(
      &attributes, &stack_base, &observed_stack_size);
  size_t observed_guard_size = 0;
  const int guard_status = pthread_attr_getguardsize(&attributes, &observed_guard_size);
  const int destroy_status = pthread_attr_destroy(&attributes);
  if (stack_status != 0 || guard_status != 0 || destroy_status != 0 ||
      stack_base == nullptr || observed_stack_size == 0 ||
      observed_guard_size >= observed_stack_size) {
    *error_code = stack_status != 0 ? stack_status :
        (guard_status != 0 ? guard_status : (destroy_status != 0 ? destroy_status : EINVAL));
    if (observed_guard_size >= observed_stack_size) *error_code = EINVAL;
    return -1;
  }
  const long observed_page_size = sysconf(_SC_PAGESIZE);
  if (observed_page_size <= 0) {
    *error_code = errno != 0 ? errno : EINVAL;
    return -1;
  }
  *thread_id = static_cast<uint64_t>(syscall(SYS_gettid));
  // Bionic's pthread_attr_getstack range begins at the stack guard. /proc labels
  // only the writable stack-and-TLS mapping, which begins after that guard.
  *start_address = reinterpret_cast<uintptr_t>(stack_base);
  *stack_size = observed_stack_size;
  *guard_size = observed_guard_size;
  *page_size = static_cast<size_t>(observed_page_size);
  return 0;
}

extern "C" int spinon_c124_sample_stack_residency(
    uintptr_t start_address, size_t stack_size, uint8_t *page_flags,
    size_t page_capacity, size_t *page_size, int *error_code) {
  if (page_flags == nullptr || page_size == nullptr || error_code == nullptr) {
    return -1;
  }
  *error_code = 0;
  const long observed_page_size = sysconf(_SC_PAGESIZE);
  if (observed_page_size <= 0) {
    *error_code = errno != 0 ? errno : EINVAL;
    return -1;
  }
  const size_t current_page_size = static_cast<size_t>(observed_page_size);
  if (current_page_size == 0 || start_address % current_page_size != 0 ||
      stack_size % current_page_size != 0 ||
      stack_size / current_page_size != page_capacity) {
    *error_code = EINVAL;
    return -1;
  }
  *page_size = current_page_size;
  errno = 0;
  const int status = mincore(reinterpret_cast<void *>(start_address),
                             stack_size, page_flags);
  if (status != 0) *error_code = errno;
  return status;
}

namespace {
constexpr char kTag[] = "SpinonBootstrap";

class TraceSection final {
 public:
  explicit TraceSection(const char *name) { ATrace_beginSection(name); }
  ~TraceSection() { ATrace_endSection(); }
};

struct WgpuRendererContext {
  ANativeWindow *window;
  void *renderer;
};

#if SPINON_ENABLE_C04_RUNTIME_GPU
struct C0410RuntimeGpuContext {
  ANativeWindow *window;
  SpinonRuntimeGpuHost *host;
};
#endif

jbyteArray ToByteArray(JNIEnv *env, const std::string &value) {
  const auto length = static_cast<jsize>(value.size());
  jbyteArray output = env->NewByteArray(length);
  if (output == nullptr) return nullptr;
  env->SetByteArrayRegion(output, 0, length,
                          reinterpret_cast<const jbyte *>(value.data()));
  return env->ExceptionCheck() ? nullptr : output;
}

void LogC10NodeFrames(const std::string &report, const char *label) {
  constexpr char kMarker[] = " node_frames_css_px=[";
  const auto marker = report.find(kMarker);
  if (marker == std::string::npos) return;
  const auto start = marker + sizeof(kMarker) - 1;
  const auto end = report.find(']', start);
  if (end == std::string::npos) return;

  std::size_t frame_start = start;
  while (frame_start < end) {
    const auto separator = report.find(';', frame_start);
    const auto frame_end = separator == std::string::npos || separator > end
        ? end : separator;
    if (frame_end > frame_start) {
      const auto frame = report.substr(frame_start, frame_end - frame_start);
      __android_log_print(ANDROID_LOG_INFO, kTag, "%s %s", label, frame.c_str());
    }
    if (frame_end == end) break;
    frame_start = frame_end + 1;
  }
}

#if defined(SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST) && \
    SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST
void LogC131HitTestSamples(const std::string &report, std::size_t expected_count,
                           const char *sample_label, const char *summary_label) {
  constexpr char kMarker[] = " hit_test_samples_json=";
  const auto marker = report.find(kMarker);
  if (marker == std::string::npos) return;
  const auto array_start = report.find('[', marker + sizeof(kMarker) - 1);
  if (array_start == std::string::npos) return;
  const auto array_end = report.find(']', array_start + 1);
  if (array_end == std::string::npos) return;

  std::size_t sample_start = array_start + 1;
  std::size_t count = 0;
  std::size_t matched = 0;
  while (sample_start < array_end) {
    sample_start = report.find('{', sample_start);
    if (sample_start == std::string::npos || sample_start >= array_end) return;
    const auto sample_end = report.find('}', sample_start + 1);
    if (sample_end == std::string::npos || sample_end >= array_end) return;
    const auto sample = report.substr(sample_start, sample_end - sample_start + 1);
    __android_log_print(ANDROID_LOG_INFO, kTag, "%s %s", sample_label,
                        sample.c_str());
    if (sample.find("\"status\":\"matched\"") != std::string::npos) ++matched;
    ++count;
    sample_start = sample_end + 1;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag,
                      "%s status=%s matched=%zu total=%zu expected=%zu",
                      summary_label,
                      count == expected_count && matched == expected_count ? "ok" : "error",
                      matched, count, expected_count);
}
#endif

SpinonRuntimeSession *SessionFromHandle(jlong handle) {
  return reinterpret_cast<SpinonRuntimeSession *>(static_cast<uintptr_t>(handle));
}
}

#if SPINON_ENABLE_C04_RUNTIME_GPU
extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeCreateHost(
    JNIEnv *, jclass, jboolean registered_properties_fixture) {
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host = nullptr;
  if (registered_properties_fixture == JNI_TRUE) {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST) && \
    SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST
    host = spinon_runtime_gpu_host_new_registered_properties_fixture(
        output.data(), output.size());
#else
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_C052_HOST_ERROR=fixture feature disabled");
    return 0;
#endif
  } else {
    host = spinon_runtime_gpu_host_new(output.data(), output.size());
  }
  if (host == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "%s_HOST_ERROR=%s",
                        registered_properties_fixture == JNI_TRUE
                            ? "SPINON_C052" : "SPINON_C0410",
                        output.data());
    return 0;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag, "%s_HOST=%s",
                      registered_properties_fixture == JNI_TRUE
                          ? "SPINON_C052" : "SPINON_C0410",
                      output.data());
  return static_cast<jlong>(reinterpret_cast<uintptr_t>(host));
}

extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeCreateHostWithRootHtml(
    JNIEnv *env, jclass, jbyteArray source) {
  constexpr jsize kRootHtmlSourceMaxBytes = 1024 * 1024;
  std::array<char, 1024> output{};
  if (source == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_C0412_ROOT_HTML_DEFAULT_RENDER_HOST_ERROR=null_source");
    return 0;
  }
  const jsize source_length = env->GetArrayLength(source);
  if (source_length <= 0 || source_length > kRootHtmlSourceMaxBytes) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_C0412_ROOT_HTML_DEFAULT_RENDER_HOST_ERROR=source_length:%d",
                        source_length);
    return 0;
  }
  std::vector<uint8_t> source_bytes;
  try {
    source_bytes.resize(static_cast<size_t>(source_length));
  } catch (const std::bad_alloc &) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_C0412_ROOT_HTML_DEFAULT_RENDER_HOST_ERROR=allocation_failed");
    return 0;
  }
  env->GetByteArrayRegion(source, 0, source_length,
                          reinterpret_cast<jbyte *>(source_bytes.data()));
  if (env->ExceptionCheck()) return 0;
  SpinonRuntimeGpuHost *host = spinon_runtime_gpu_host_new_with_root_html(
      source_bytes.data(), source_bytes.size(), output.data(), output.size());
  if (host == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_C0412_ROOT_HTML_DEFAULT_RENDER_HOST_ERROR=%s",
                        output.data());
    return 0;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag,
                      "SPINON_C0412_ROOT_HTML_DEFAULT_RENDER_HOST=%s",
                      output.data());
  return static_cast<jlong>(reinterpret_cast<uintptr_t>(host));
}

#if defined(SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST) && \
    SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST
extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeCreateBlockPaintHost(
    JNIEnv *, jclass) {
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host = spinon_runtime_gpu_host_new_block_paint_fixture(
      output.data(), output.size());
  if (host == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_C08_HOST_ERROR=%s", output.data());
    return 0;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag,
                      "SPINON_C08_HOST=%s", output.data());
  return static_cast<jlong>(reinterpret_cast<uintptr_t>(host));
}

extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeCreateRootHtmlBlockPaintHost(
    JNIEnv *, jclass) {
#if defined(SPINON_ENABLE_C04_12_ROOT_HTML_BLOCK_PAINT_FIXTURE_TEST) && \
    SPINON_ENABLE_C04_12_ROOT_HTML_BLOCK_PAINT_FIXTURE_TEST
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_root_html_block_paint_fixture(
          output.data(), output.size());
  if (host == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_C0412_ROOT_HTML_BLOCK_PAINT_HOST_ERROR=%s",
                        output.data());
    return 0;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag,
                      "SPINON_C0412_ROOT_HTML_BLOCK_PAINT_HOST=%s",
                      output.data());
  return static_cast<jlong>(reinterpret_cast<uintptr_t>(host));
#else
  __android_log_print(ANDROID_LOG_ERROR, kTag,
                      "SPINON_C0412_ROOT_HTML_BLOCK_PAINT_DISABLED");
  return 0;
#endif
}

extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeCreateRootHtmlTextPaintHost(
    JNIEnv *, jclass) {
#if defined(SPINON_ENABLE_C04_12_ROOT_HTML_TEXT_PAINT_FIXTURE_TEST) && \
    SPINON_ENABLE_C04_12_ROOT_HTML_TEXT_PAINT_FIXTURE_TEST
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_root_html_text_paint_fixture(
          output.data(), output.size());
  if (host == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_C0412_ROOT_HTML_TEXT_PAINT_HOST_ERROR=%s",
                        output.data());
    return 0;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag,
                      "SPINON_C0412_ROOT_HTML_TEXT_PAINT_HOST=%s",
                      output.data());
  return static_cast<jlong>(reinterpret_cast<uintptr_t>(host));
#else
  __android_log_print(ANDROID_LOG_ERROR, kTag,
                      "SPINON_C0412_ROOT_HTML_TEXT_PAINT_DISABLED");
  return 0;
#endif
}

extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeCreateBlockFormattingHost(
    JNIEnv *, jclass) {
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host = spinon_runtime_gpu_host_new_block_formatting_fixture(
      output.data(), output.size());
  if (host == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_C09_BLOCK_HOST_ERROR=%s", output.data());
    return 0;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag,
                      "SPINON_C09_BLOCK_HOST=%s", output.data());
  return static_cast<jlong>(reinterpret_cast<uintptr_t>(host));
}

extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeCreateC121PositionHost(
    JNIEnv *, jclass) {
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host = spinon_runtime_gpu_host_new_c12_1_position_fixture(
      output.data(), output.size());
  if (host == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_C121_HOST_ERROR=%s", output.data());
    return 0;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag,
                      "SPINON_C121_HOST=%s", output.data());
  return static_cast<jlong>(reinterpret_cast<uintptr_t>(host));
}

extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeCreateC122AbsoluteBlockHost(
    JNIEnv *, jclass) {
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host = spinon_runtime_gpu_host_new_c12_2_absolute_block_fixture(
      output.data(), output.size());
  if (host == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_C122_HOST_ERROR=%s", output.data());
    return 0;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag,
                      "SPINON_C122_HOST=%s", output.data());
  return static_cast<jlong>(reinterpret_cast<uintptr_t>(host));
}

extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeCreateC123FixedHost(
    JNIEnv *, jclass) {
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c12_3_fixed_fixture(output.data(), output.size());
  if (host == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_C123_HOST_ERROR=%s", output.data());
    return 0;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag,
                      "SPINON_C123_HOST=%s", output.data());
  return static_cast<jlong>(reinterpret_cast<uintptr_t>(host));
}

extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeCreateC124StackingHost(
    JNIEnv *, jclass) {
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host = spinon_runtime_gpu_host_new_c12_4_stacking_fixture(
      output.data(), output.size());
  if (host == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_C124_HOST_ERROR=%s", output.data());
    return 0;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag,
                      "SPINON_C124_HOST=%s", output.data());
  return static_cast<jlong>(reinterpret_cast<uintptr_t>(host));
}

extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeCreateC131OverflowClipHost(
    JNIEnv *, jclass) {
#if defined(SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST) && \
    SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c13_1_overflow_clip_fixture(
          output.data(), output.size());
  if (host == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_C131_HOST_ERROR=%s", output.data());
    return 0;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag,
                      "SPINON_C131_HOST=%s", output.data());
  return static_cast<jlong>(reinterpret_cast<uintptr_t>(host));
#else
  __android_log_print(ANDROID_LOG_ERROR, kTag,
                      "SPINON_C131_DISABLED fixture-test feature is off");
  return 0;
#endif
}

extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeCreateC131CompactSurfaceMatrixHost(
    JNIEnv *, jclass) {
#if defined(SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST) && \
    SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c13_1_compact_surface_matrix_fixture(
          output.data(), output.size());
  if (host == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_C131_COMPACT_HOST_ERROR=%s", output.data());
    return 0;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag,
                      "SPINON_C131_COMPACT_HOST=%s", output.data());
  return static_cast<jlong>(reinterpret_cast<uintptr_t>(host));
#else
  __android_log_print(ANDROID_LOG_ERROR, kTag,
                      "SPINON_C131_COMPACT_DISABLED fixture-test feature is off");
  return 0;
#endif
}

extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeCreateC131StackingDescendantClipHost(
    JNIEnv *, jclass) {
#if defined(SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST) && \
    SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c13_1_stacking_descendant_clip_fixture(
          output.data(), output.size());
  if (host == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_C131_STACKING_CLIP_HOST_ERROR=%s", output.data());
    return 0;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag,
                      "SPINON_C131_STACKING_CLIP_HOST=%s", output.data());
  return static_cast<jlong>(reinterpret_cast<uintptr_t>(host));
#else
  __android_log_print(ANDROID_LOG_ERROR, kTag,
                      "SPINON_C131_STACKING_CLIP_DISABLED fixture-test feature is off");
  return 0;
#endif
}

extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeCreateC131OverflowClipBorderHost(
    JNIEnv *, jclass) {
#if defined(SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST) && \
    SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c13_1_overflow_clip_border_fixture(
          output.data(), output.size());
  if (host == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_C131_BORDER_HOST_ERROR=%s", output.data());
    return 0;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag,
                      "SPINON_C131_BORDER_HOST=%s", output.data());
  return static_cast<jlong>(reinterpret_cast<uintptr_t>(host));
#else
  __android_log_print(ANDROID_LOG_ERROR, kTag,
                      "SPINON_C131_BORDER_DISABLED fixture-test feature is off");
  return 0;
#endif
}

#if defined(SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST) && \
    SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST
extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_GlyphPaintFixtureNativeAdapter_createHost(JNIEnv *, jclass) {
  std::array<char, 2048> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c15_3_glyph_paint_fixture(output.data(), output.size());
  if (host == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag, "SPINON_C153_HOST_ERROR=%s", output.data());
    return 0;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag, "SPINON_C153_HOST=%s", output.data());
  return static_cast<jlong>(reinterpret_cast<uintptr_t>(host));
}

extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_GlyphPaintFixtureNativeAdapter_createHostDocumentTextPaintHost(
    JNIEnv *, jclass) {
  std::array<char, 2048> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c15_3_hostdocument_text_paint_fixture(
          output.data(), output.size());
  if (host == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_C153_HOSTDOCUMENT_HOST_ERROR=%s", output.data());
    return 0;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag,
                      "SPINON_C153_HOSTDOCUMENT_HOST=%s", output.data());
  return static_cast<jlong>(reinterpret_cast<uintptr_t>(host));
}
#endif

#if defined(SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST) && \
    SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST
extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_GlyphPaintFixtureNativeAdapter_evalFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c15_3_glyph_paint_fixture(
      host, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C153_EVAL %s", report.c_str());
  return ToByteArray(env, report);
}
#endif

#if defined(SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST
extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_ImagePaintFixtureNativeAdapter_createHost(JNIEnv *, jclass) {
  std::array<char, 2048> output{};
  SpinonRuntimeGpuHost *host = spinon_runtime_gpu_host_new_c18_1_image_paint_fixture(
      output.data(), output.size());
  if (host == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag, "IMAGE_PAINT_HOST_ERROR=%s", output.data());
    return 0;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag, "IMAGE_PAINT_HOST=%s", output.data());
  return static_cast<jlong>(reinterpret_cast<uintptr_t>(host));
}
#endif

#if defined(SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST
extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_AlphaImageFixtureNativeAdapter_createHost(JNIEnv *, jclass) {
  std::array<char, 2048> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c18_2_alpha_image_fixture(output.data(), output.size());
  if (host == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag, "SPINON_C182_HOST_ERROR=%s", output.data());
    return 0;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag, "SPINON_C182_HOST=%s", output.data());
  return static_cast<jlong>(reinterpret_cast<uintptr_t>(host));
}
#endif

#if defined(SPINON_ENABLE_C19_2_BOX_PAINT_OPACITY_FIXTURE_TEST) && \
    SPINON_ENABLE_C19_2_BOX_PAINT_OPACITY_FIXTURE_TEST
extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_BoxPaintOpacityFixtureNativeAdapter_createHost(JNIEnv *, jclass) {
  std::array<char, 2048> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c19_2_box_paint_opacity_fixture(
          output.data(), output.size());
  if (host == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag, "SPINON_C192_HOST_ERROR=%s", output.data());
    return 0;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag, "SPINON_C192_HOST=%s", output.data());
  return static_cast<jlong>(reinterpret_cast<uintptr_t>(host));
}
#endif

#if defined(SPINON_ENABLE_C18_3_OBJECT_FIT_POSITION_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_3_OBJECT_FIT_POSITION_FIXTURE_TEST
extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_ObjectFitPositionFixtureNativeAdapter_createHost(JNIEnv *, jclass) {
  std::array<char, 2048> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c18_3_object_fit_position_fixture(
          output.data(), output.size());
  if (host == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag, "SPINON_C183_HOST_ERROR=%s", output.data());
    return 0;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag, "SPINON_C183_HOST=%s", output.data());
  return static_cast<jlong>(reinterpret_cast<uintptr_t>(host));
}
#endif

#if defined(SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST
extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_AsyncImageFixtureNativeAdapter_createHost(JNIEnv *, jclass) {
  std::array<char, 2048> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c18_4_async_image_fixture(output.data(), output.size());
  if (host == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag, "SPINON_C184_HOST_ERROR=%s", output.data());
    return 0;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag, "SPINON_C184_HOST=%s", output.data());
  return static_cast<jlong>(reinterpret_cast<uintptr_t>(host));
}
#endif

extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeCreateC132ScrollHost(
    JNIEnv *, jclass) {
#if defined(SPINON_ENABLE_C132_SCROLL_FIXTURE_TEST) && \
    SPINON_ENABLE_C132_SCROLL_FIXTURE_TEST
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host = spinon_runtime_gpu_host_new_c13_2_scroll_fixture(
      output.data(), output.size());
  if (host == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_C132_HOST_ERROR=%s", output.data());
    return 0;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag,
                      "SPINON_C132_HOST=%s", output.data());
  return static_cast<jlong>(reinterpret_cast<uintptr_t>(host));
#else
  __android_log_print(ANDROID_LOG_ERROR, kTag,
                      "SPINON_C132_DISABLED fixture-test feature is off");
  return 0;
#endif
}

extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeCreateC133GestureHost(
    JNIEnv *, jclass, jint scenario_id) {
#if defined(SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST) && \
    SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host = spinon_runtime_gpu_host_new_c13_3_gesture_fixture(
      static_cast<int32_t>(scenario_id), output.data(), output.size());
  if (host == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_C133_HOST_ERROR=%s", output.data());
    return 0;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag,
                      "SPINON_C133_HOST=%s", output.data());
  return static_cast<jlong>(reinterpret_cast<uintptr_t>(host));
#else
  __android_log_print(ANDROID_LOG_ERROR, kTag,
                      "SPINON_C133_DISABLED fixture-test feature is off");
  return 0;
#endif
}

extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeCreateC11GridHost(
    JNIEnv *, jclass) {
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host = spinon_runtime_gpu_host_new_c11_1_grid_fixture(
      output.data(), output.size());
  if (host == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_C111_GRID_HOST_ERROR=%s", output.data());
    return 0;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag,
                      "SPINON_C111_GRID_HOST=%s", output.data());
  return static_cast<jlong>(reinterpret_cast<uintptr_t>(host));
}

extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeCreateC1121GridPlacementHost(
    JNIEnv *, jclass) {
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c11_2_1_grid_placement_fixture(
          output.data(), output.size());
  if (host == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_C1121_GRID_HOST_ERROR=%s", output.data());
    return 0;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag,
                      "SPINON_C1121_GRID_HOST=%s", output.data());
  return static_cast<jlong>(reinterpret_cast<uintptr_t>(host));
}

extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeCreateC1122GridAutoPlacementHost(
    JNIEnv *, jclass) {
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c11_2_2_grid_auto_placement_fixture(
          output.data(), output.size());
  if (host == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_C1122_GRID_HOST_ERROR=%s", output.data());
    return 0;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag,
                      "SPINON_C1122_GRID_HOST=%s", output.data());
  return static_cast<jlong>(reinterpret_cast<uintptr_t>(host));
}

extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeCreateC1123GridAutoPlacementHost(
    JNIEnv *, jclass) {
#if defined(SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST) && \
    SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c11_2_3_grid_auto_placement_fixture(
          output.data(), output.size());
  if (host == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_C1123_GRID_HOST_ERROR=%s", output.data());
    return 0;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag,
                      "SPINON_C1123_GRID_HOST=%s", output.data());
  return static_cast<jlong>(reinterpret_cast<uintptr_t>(host));
#else
  __android_log_print(ANDROID_LOG_ERROR, kTag,
                      "SPINON_C1123_GRID_DISABLED fixture feature is off");
  return 0;
#endif
}

extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeCreateC1131GridRepeatHost(
    JNIEnv *, jclass) {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST) && \
    SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c11_3_1_grid_repeat_fixture(
          output.data(), output.size());
  if (host == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_C1131_GRID_HOST_ERROR=%s", output.data());
    return 0;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag,
                      "SPINON_C1131_GRID_HOST=%s", output.data());
  return static_cast<jlong>(reinterpret_cast<uintptr_t>(host));
#else
  __android_log_print(ANDROID_LOG_ERROR, kTag,
                      "SPINON_C1131_GRID_DISABLED fixture-test feature is off");
  return 0;
#endif
}

extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeCreateC1132GridNamedLinesHost(
    JNIEnv *, jclass) {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST) && \
    SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c11_3_2_grid_named_lines_fixture(
          output.data(), output.size());
  if (host == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_C1132_GRID_HOST_ERROR=%s", output.data());
    return 0;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag,
                      "SPINON_C1132_GRID_HOST=%s", output.data());
  return static_cast<jlong>(reinterpret_cast<uintptr_t>(host));
#else
  __android_log_print(ANDROID_LOG_ERROR, kTag,
                      "SPINON_C1132_GRID_DISABLED fixture-test feature is off");
  return 0;
#endif
}

extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeCreateC1133GridTemplateAreasHost(
    JNIEnv *, jclass) {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST) && \
    SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c11_3_3_grid_template_areas_fixture(
          output.data(), output.size());
  if (host == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_C1133_GRID_HOST_ERROR=%s", output.data());
    return 0;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag,
                      "SPINON_C1133_GRID_HOST=%s", output.data());
  return static_cast<jlong>(reinterpret_cast<uintptr_t>(host));
#else
  __android_log_print(ANDROID_LOG_ERROR, kTag,
                      "SPINON_C1133_GRID_DISABLED fixture-test feature is off");
  return 0;
#endif
}

#if defined(SPINON_ENABLE_C124_WORKER_STACK_RESIDENCY_TEST) && \
    SPINON_ENABLE_C124_WORKER_STACK_RESIDENCY_TEST
extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeC124WorkerStackResidencySample(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 1024> output{};
  const int32_t status = spinon_runtime_gpu_host_css_worker_stack_residency_for_test(
      host, output.data(), output.size());
  const std::string report = status == 0
      ? output.data()
      : "{\"status\":\"error\",\"native_status\":" +
            std::to_string(status) + ",\"detail\":\"" + output.data() + "\"}";
  return ToByteArray(env, report);
}
#endif

extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeCreateC1035PositionedFlexHost(
    JNIEnv *, jclass) {
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host = spinon_runtime_gpu_host_new_c10_3_5_positioned_flex_fixture(
      output.data(), output.size());
  if (host == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_C1035_HOST_ERROR=%s", output.data());
    return 0;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag,
                      "SPINON_C1035_HOST=%s", output.data());
  return static_cast<jlong>(reinterpret_cast<uintptr_t>(host));
}
#endif

extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeBeginPresentationUpdate(
    JNIEnv *, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  return static_cast<jlong>(spinon_runtime_gpu_host_begin_presentation_update(host));
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeSetEnvironment(
    JNIEnv *env, jclass, jlong host_handle, jfloat width_css_px,
    jfloat height_css_px, jfloat scale, jboolean dark) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_set_environment(
      host, width_css_px, height_css_px, scale, dark == JNI_TRUE ? 1 : 0,
      10000, output.data(), output.size());
  const std::string report =
      "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C0410_ENVIRONMENT %s", report.c_str());
  return ToByteArray(env, report);
}

#if defined(SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST) && \
    SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST
extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_fixture(
      host, 10000, output.data(), output.size());
  const std::string report =
      "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C0410_EVAL %s", report.c_str());
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalAuthorStylesheetsFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_author_stylesheets_fixture(
      host, 10000, output.data(), output.size());
  const std::string report =
      "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C0411_EVAL %s", report.c_str());
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalCustomPropertiesFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_custom_properties_fixture(
      host, 10000, output.data(), output.size());
  const std::string report =
      "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C051_EVAL %s", report.c_str());
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalRegisteredPropertiesFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_registered_properties_fixture(
      host, 10000, output.data(), output.size());
  const std::string report =
      "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C052_EVAL %s", report.c_str());
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalRuntimeResultCacheFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_runtime_result_cache_fixture(
      host, 10000, output.data(), output.size());
  const std::string report =
      "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C053_EVAL %s", report.c_str());
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalIncrementalRestyleFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_incremental_restyle_fixture(
      host, 10000, output.data(), output.size());
  const std::string report =
      "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C054_EVAL %s", report.c_str());
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalPercentageDimensionsFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 4096> output{};
  const int32_t status =
      spinon_runtime_gpu_host_eval_percentage_dimensions_fixture(
          host, 10000, output.data(), output.size());
  const std::string report =
      "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C061_EVAL %s", report.c_str());
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalSpacingPercentagesFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 4096> output{};
  const int32_t status =
      spinon_runtime_gpu_host_eval_spacing_percentages_fixture(
          host, 10000, output.data(), output.size());
  const std::string report =
      "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C062_EVAL %s", report.c_str());
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalAbsoluteLengthsFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 4096> output{};
  const int32_t status =
      spinon_runtime_gpu_host_eval_absolute_lengths_fixture(
          host, 10000, output.data(), output.size());
  const std::string report =
      "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C063_EVAL %s", report.c_str());
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalFontRelativeUnitsFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 4096> output{};
  const int32_t status =
      spinon_runtime_gpu_host_eval_font_relative_units_fixture(
          host, 10000, output.data(), output.size());
  const std::string report =
      "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C064_EVAL %s", report.c_str());
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalTypedCssMathFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_typed_css_math_fixture(
      host, 10000, output.data(), output.size());
  const std::string report =
      "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C065_EVAL %s", report.c_str());
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalViewportUnitsFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_viewport_units_fixture(
      host, 10000, output.data(), output.size());
  const std::string report =
      "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C066A_EVAL %s", report.c_str());
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalMinMaxSizingFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_min_max_sizing_fixture(
      host, 10000, output.data(), output.size());
  const std::string report =
      "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C071_EVAL %s", report.c_str());
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalBorderWidthFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_border_width_fixture(
      host, 10000, output.data(), output.size());
  const std::string report =
      "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C072_EVAL %s", report.c_str());
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalAspectRatioFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_aspect_ratio_fixture(
      host, 10000, output.data(), output.size());
  const std::string report =
      "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C073_EVAL %s", report.c_str());
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalBlockPaintFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_block_paint_fixture(
      host, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C08_EVAL %s", report.c_str());
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalRootHtmlBlockPaintFixture(
    JNIEnv *env, jclass, jlong host_handle) {
#if defined(SPINON_ENABLE_C04_12_ROOT_HTML_BLOCK_PAINT_FIXTURE_TEST) && \
    SPINON_ENABLE_C04_12_ROOT_HTML_BLOCK_PAINT_FIXTURE_TEST
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_root_html_block_paint_fixture(
      host, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C0412_ROOT_HTML_BLOCK_PAINT_EVAL %s",
                      report.c_str());
  return ToByteArray(env, report);
#else
  return ToByteArray(env,
      "status=-90 C04.12 root HTML Block paint fixture 빌드가 비활성화되었습니다");
#endif
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalRootHtmlTextPaintFixture(
    JNIEnv *env, jclass, jlong host_handle) {
#if defined(SPINON_ENABLE_C04_12_ROOT_HTML_TEXT_PAINT_FIXTURE_TEST) && \
    SPINON_ENABLE_C04_12_ROOT_HTML_TEXT_PAINT_FIXTURE_TEST
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_root_html_text_paint_fixture(
      host, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C0412_ROOT_HTML_TEXT_PAINT_EVAL %s",
                      report.c_str());
  return ToByteArray(env, report);
#else
  return ToByteArray(env,
      "status=-90 C04.12 root HTML text paint fixture 빌드가 비활성화되었습니다");
#endif
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalBlockFormattingFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_block_formatting_fixture(
      host, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C091_EVAL %s", report.c_str());
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalC121PositionFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c12_1_position_fixture(
      host, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C121_EVAL %s", report.c_str());
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalC122AbsoluteBlockFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c12_2_absolute_block_fixture(
      host, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C122_EVAL %s", report.c_str());
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalC123FixedFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c12_3_fixed_fixture(
      host, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C123_EVAL %s", report.c_str());
  if (status == 0) LogC10NodeFrames(report, "SPINON_C123_NODE_FRAME");
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalC124StackingFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c12_4_stacking_fixture(
      host, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C124_EVAL %s", report.c_str());
  if (status == 0) LogC10NodeFrames(report, "SPINON_C124_NODE_FRAME");
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalC131OverflowClipFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
#if defined(SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST) && \
    SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c13_1_overflow_clip_fixture(
      host, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C131_EVAL %s", report.c_str());
  if (status == 0) {
    LogC10NodeFrames(report, "SPINON_C131_NODE_FRAME");
    LogC131HitTestSamples(report, 15, "SPINON_C131_HIT_TEST",
                          "SPINON_C131_HIT_TEST_SUMMARY");
  }
  return ToByteArray(env, report);
#else
  return ToByteArray(env, "status=-1 C13.1 fixture-test feature is off");
#endif
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalC131StackingDescendantClipFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
#if defined(SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST) && \
    SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST
  std::array<char, 32768> output{};
  const int32_t status =
      spinon_runtime_gpu_host_eval_c13_1_stacking_descendant_clip_fixture(
          host, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C131_STACKING_CLIP_EVAL %s", report.c_str());
  if (status == 0) {
    LogC10NodeFrames(report, "SPINON_C131_STACKING_CLIP_NODE_FRAME");
    LogC131HitTestSamples(report, 18, "SPINON_C131_STACKING_CLIP_HIT_TEST",
                          "SPINON_C131_STACKING_CLIP_HIT_TEST_SUMMARY");
  }
  return ToByteArray(env, report);
#else
  return ToByteArray(env, "status=-1 C13.1 fixture-test feature is off");
#endif
}

#if defined(SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST
extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_ImagePaintFixtureNativeAdapter_evalFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c18_1_image_paint_fixture(
      host, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "IMAGE_PAINT_EVAL %s", report.c_str());
  return ToByteArray(env, report);
}
#endif

#if defined(SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST
extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_AlphaImageFixtureNativeAdapter_evalFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c18_2_alpha_image_fixture(
      host, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C182_EVAL %s", report.c_str());
  return ToByteArray(env, report);
}
#endif

#if defined(SPINON_ENABLE_C19_2_BOX_PAINT_OPACITY_FIXTURE_TEST) && \
    SPINON_ENABLE_C19_2_BOX_PAINT_OPACITY_FIXTURE_TEST
extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_BoxPaintOpacityFixtureNativeAdapter_evalFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c19_2_box_paint_opacity_fixture(
      host, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C192_EVAL %s", report.c_str());
  return ToByteArray(env, report);
}
#endif

#if defined(SPINON_ENABLE_C18_3_OBJECT_FIT_POSITION_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_3_OBJECT_FIT_POSITION_FIXTURE_TEST
extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_ObjectFitPositionFixtureNativeAdapter_evalFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 32768> output{};
  const int32_t status =
      spinon_runtime_gpu_host_eval_c18_3_object_fit_position_fixture(
          host, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C183_EVAL %s", report.c_str());
  return ToByteArray(env, report);
}
#endif

#if defined(SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST
extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_AsyncImageFixtureNativeAdapter_evalFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c18_4_async_image_fixture(
      host, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C184_EVAL %s", report.c_str());
  return ToByteArray(env, report);
}
#endif

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalC131CompactSurfaceMatrixFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
#if defined(SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST) && \
    SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST
  std::array<char, 32768> output{};
  const int32_t status =
      spinon_runtime_gpu_host_eval_c13_1_compact_surface_matrix_fixture(
          host, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C131_COMPACT_EVAL %s", report.c_str());
  if (status == 0) {
    LogC10NodeFrames(report, "SPINON_C131_COMPACT_NODE_FRAME");
    LogC131HitTestSamples(report, 52, "SPINON_C131_COMPACT_HIT_TEST",
                          "SPINON_C131_COMPACT_HIT_TEST_SUMMARY");
  }
  return ToByteArray(env, report);
#else
  return ToByteArray(env, "status=-1 C13.1 fixture-test feature is off");
#endif
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalC131OverflowClipBorderFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
#if defined(SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST) && \
    SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c13_1_overflow_clip_border_fixture(
      host, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C131_BORDER_EVAL %s", report.c_str());
  if (status == 0) {
    LogC10NodeFrames(report, "SPINON_C131_BORDER_NODE_FRAME");
    LogC131HitTestSamples(report, 6, "SPINON_C131_BORDER_HIT_TEST",
                          "SPINON_C131_BORDER_HIT_TEST_SUMMARY");
  }
  return ToByteArray(env, report);
#else
  return ToByteArray(env, "status=-1 C13.1 fixture-test feature is off");
#endif
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalC132ScrollFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
#if defined(SPINON_ENABLE_C132_SCROLL_FIXTURE_TEST) && \
    SPINON_ENABLE_C132_SCROLL_FIXTURE_TEST
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c13_2_scroll_fixture(
      host, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C132_EVAL %s", report.c_str());
  return ToByteArray(env, report);
#else
  return ToByteArray(env, "status=-1 C13.2 fixture-test feature is off");
#endif
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalC133GestureFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
#if defined(SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST) && \
    SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c13_3_gesture_fixture(
      host, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C133_EVAL %s", report.c_str());
  return ToByteArray(env, report);
#else
  return ToByteArray(env, "status=-1 C13.3 fixture-test feature is off");
#endif
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeApplyC133StyleInvalidation(
    JNIEnv *env, jclass, jlong host_handle) {
#if defined(SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST) && \
    SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_apply_c13_3_style_invalidation(
      host, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  return ToByteArray(env, report);
#else
  (void)host_handle;
  return ToByteArray(env, "status=-1 C13.3 fixture-test feature is off");
#endif
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEnqueueC133Gesture(
    JNIEnv *env, jclass, jlong host_handle, jint action,
    jlong surface_generation, jlong gesture_id, jlong pointer_id,
    jfloat x_or_cumulative_x_css_px, jfloat y_or_cumulative_y_css_px,
    jfloat coordinate_scale, jint reason) {
#if defined(SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST) && \
    SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 1024> output{};
  const int32_t status = spinon_runtime_gpu_host_enqueue_c13_3_gesture(
      host, action, static_cast<uint64_t>(surface_generation),
      static_cast<uint64_t>(gesture_id), static_cast<uint64_t>(pointer_id),
      x_or_cumulative_x_css_px, y_or_cumulative_y_css_px, coordinate_scale,
      reason, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_WARN,
                      kTag, "SPINON_C133_ENQUEUE %s", report.c_str());
  return ToByteArray(env, report);
#else
  (void)host_handle;
  (void)action;
  (void)surface_generation;
  (void)gesture_id;
  (void)pointer_id;
  (void)x_or_cumulative_x_css_px;
  (void)y_or_cumulative_y_css_px;
  (void)coordinate_scale;
  (void)reason;
  return ToByteArray(env, "status=-1 C13.3 fixture-test feature is off");
#endif
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEnqueueC133Wheel(
    JNIEnv *env, jclass, jlong host_handle, jint intent,
    jlong surface_generation, jfloat point_x_css_px, jfloat point_y_css_px,
    jfloat delta_x_css_px, jfloat delta_y_css_px, jfloat raw_delta_x,
    jfloat raw_delta_y, jint source_unit, jfloat coordinate_scale,
    jint source, jint momentum_phase, jint modifier_flags) {
#if defined(SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST) && \
    SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 2048> output{};
  const int32_t status = spinon_runtime_gpu_host_enqueue_c13_3_wheel(
      host, static_cast<int32_t>(intent),
      static_cast<uint64_t>(surface_generation), point_x_css_px,
      point_y_css_px, delta_x_css_px, delta_y_css_px, raw_delta_x,
      raw_delta_y, static_cast<int32_t>(source_unit), coordinate_scale,
      static_cast<int32_t>(source), static_cast<int32_t>(momentum_phase),
      static_cast<uint32_t>(modifier_flags), output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_WARN,
                      kTag, "SPINON_C133_WHEEL_ENQUEUE %s", report.c_str());
  return ToByteArray(env, report);
#else
  (void)host_handle;
  (void)intent;
  (void)surface_generation;
  (void)point_x_css_px;
  (void)point_y_css_px;
  (void)delta_x_css_px;
  (void)delta_y_css_px;
  (void)raw_delta_x;
  (void)raw_delta_y;
  (void)source_unit;
  (void)coordinate_scale;
  (void)source;
  (void)momentum_phase;
  (void)modifier_flags;
  return ToByteArray(env, "status=-1 C13.3 fixture-test feature is off");
#endif
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeProcessC133GestureQueue(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
#if defined(SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST) && \
    SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_process_c13_3_gesture_queue(
      host, 64 + 8, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C133_PROCESS %s", report.c_str());
  return ToByteArray(env, report);
#else
  return ToByteArray(env, "status=-1 C13.3 fixture-test feature is off");
#endif
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalC11GridFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c11_1_grid_fixture(
      host, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C111_GRID_EVAL %s", report.c_str());
  if (status == 0) LogC10NodeFrames(report, "SPINON_C111_GRID_NODE_FRAME");
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalC11GridSceneFixture(
    JNIEnv *env, jclass, jlong host_handle, jint scene_index) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  if (scene_index < 0) {
    const std::string report = "status=-1 scene_index=" + std::to_string(scene_index)
        + " error=negative-scene-index";
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_C111_GRID_EVAL %s", report.c_str());
    return ToByteArray(env, report);
  }
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c11_1_grid_scene_fixture(
      host, static_cast<uint32_t>(scene_index), 10000,
      output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C111_GRID_EVAL %s", report.c_str());
  if (status == 0) LogC10NodeFrames(report, "SPINON_C111_GRID_NODE_FRAME");
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalC1121GridPlacementFixture(
    JNIEnv *env, jclass, jlong host_handle, jstring scene_id_value,
    jstring run_id_value) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  const char *scene_id = scene_id_value == nullptr
      ? nullptr : env->GetStringUTFChars(scene_id_value, nullptr);
  const char *run_id = run_id_value == nullptr
      ? nullptr : env->GetStringUTFChars(run_id_value, nullptr);
  if (scene_id_value != nullptr && scene_id == nullptr) return nullptr;
  if (run_id_value != nullptr && run_id == nullptr) {
    if (scene_id != nullptr) env->ReleaseStringUTFChars(scene_id_value, scene_id);
    return nullptr;
  }
  std::array<char, 32768> output{};
  const int32_t status =
      spinon_runtime_gpu_host_eval_c11_2_1_grid_placement_fixture(
          host, scene_id, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  const std::string log_report = "run_id=" + std::string(run_id == nullptr ? "missing" : run_id)
      + " scene_id=" + std::string(scene_id == nullptr ? "missing" : scene_id)
      + " " + report;
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C1121_GRID_EVAL %s", log_report.c_str());
  if (status == 0) LogC10NodeFrames(report, "SPINON_C1121_GRID_NODE_FRAME");
  if (scene_id != nullptr) env->ReleaseStringUTFChars(scene_id_value, scene_id);
  if (run_id != nullptr) env->ReleaseStringUTFChars(run_id_value, run_id);
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalC1122GridAutoPlacementFixture(
    JNIEnv *env, jclass, jlong host_handle, jstring scene_id_value,
    jstring run_id_value) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  const char *scene_id = scene_id_value == nullptr
      ? nullptr : env->GetStringUTFChars(scene_id_value, nullptr);
  const char *run_id = run_id_value == nullptr
      ? nullptr : env->GetStringUTFChars(run_id_value, nullptr);
  if (scene_id_value != nullptr && scene_id == nullptr) return nullptr;
  if (run_id_value != nullptr && run_id == nullptr) {
    if (scene_id != nullptr) env->ReleaseStringUTFChars(scene_id_value, scene_id);
    return nullptr;
  }
  std::array<char, 32768> output{};
  const int32_t status =
      spinon_runtime_gpu_host_eval_c11_2_2_grid_auto_placement_fixture(
          host, scene_id, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  const std::string log_report = "run_id=" + std::string(run_id == nullptr ? "missing" : run_id)
      + " scene_id=" + std::string(scene_id == nullptr ? "missing" : scene_id)
      + " " + report;
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C1122_GRID_EVAL %s", log_report.c_str());
  if (status == 0) LogC10NodeFrames(report, "SPINON_C1122_GRID_NODE_FRAME");
  if (scene_id != nullptr) env->ReleaseStringUTFChars(scene_id_value, scene_id);
  if (run_id != nullptr) env->ReleaseStringUTFChars(run_id_value, run_id);
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalC1123GridAutoPlacementFixture(
    JNIEnv *env, jclass, jlong host_handle, jstring scene_id_value,
    jstring run_id_value) {
#if defined(SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST) && \
    SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  const char *scene_id = scene_id_value == nullptr
      ? nullptr : env->GetStringUTFChars(scene_id_value, nullptr);
  const char *run_id = run_id_value == nullptr
      ? nullptr : env->GetStringUTFChars(run_id_value, nullptr);
  if (scene_id_value != nullptr && scene_id == nullptr) return nullptr;
  if (run_id_value != nullptr && run_id == nullptr) {
    if (scene_id != nullptr) env->ReleaseStringUTFChars(scene_id_value, scene_id);
    return nullptr;
  }
  std::array<char, 32768> output{};
  const int32_t status =
      spinon_runtime_gpu_host_eval_c11_2_3_grid_auto_placement_fixture(
          host, scene_id, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  const std::string log_report = "run_id=" + std::string(run_id == nullptr ? "missing" : run_id)
      + " scene_id=" + std::string(scene_id == nullptr ? "missing" : scene_id)
      + " " + report;
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C1123_GRID_EVAL %s", log_report.c_str());
  if (status == 0) LogC10NodeFrames(report, "SPINON_C1123_GRID_NODE_FRAME");
  if (scene_id != nullptr) env->ReleaseStringUTFChars(scene_id_value, scene_id);
  if (run_id != nullptr) env->ReleaseStringUTFChars(run_id_value, run_id);
  return ToByteArray(env, report);
#else
  (void)host_handle;
  (void)scene_id_value;
  (void)run_id_value;
  return ToByteArray(env, "status=-90 feature-disabled");
#endif
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalC1131GridRepeatFixture(
    JNIEnv *env, jclass, jlong host_handle, jstring scene_id_value,
    jstring run_id_value) {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST) && \
    SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  const char *scene_id = scene_id_value == nullptr
      ? nullptr : env->GetStringUTFChars(scene_id_value, nullptr);
  const char *run_id = run_id_value == nullptr
      ? nullptr : env->GetStringUTFChars(run_id_value, nullptr);
  if (scene_id_value != nullptr && scene_id == nullptr) return nullptr;
  if (run_id_value != nullptr && run_id == nullptr) {
    if (scene_id != nullptr) env->ReleaseStringUTFChars(scene_id_value, scene_id);
    return nullptr;
  }
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c11_3_1_grid_repeat_fixture(
      host, scene_id, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  const std::string log_report = "run_id=" + std::string(run_id == nullptr ? "missing" : run_id)
      + " scene_id=" + std::string(scene_id == nullptr ? "missing" : scene_id)
      + " " + report;
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C1131_GRID_EVAL %s", log_report.c_str());
  if (status == 0) LogC10NodeFrames(report, "SPINON_C1131_GRID_NODE_FRAME");
  if (scene_id != nullptr) env->ReleaseStringUTFChars(scene_id_value, scene_id);
  if (run_id != nullptr) env->ReleaseStringUTFChars(run_id_value, run_id);
  return ToByteArray(env, report);
#else
  return ToByteArray(env, "status=-90 C11.3.1 fixture-test feature is disabled");
#endif
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalC1132GridNamedLinesFixture(
    JNIEnv *env, jclass, jlong host_handle, jstring scene_id_value,
    jstring run_id_value) {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST) && \
    SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  const char *scene_id = scene_id_value == nullptr
      ? nullptr : env->GetStringUTFChars(scene_id_value, nullptr);
  const char *run_id = run_id_value == nullptr
      ? nullptr : env->GetStringUTFChars(run_id_value, nullptr);
  if (scene_id_value != nullptr && scene_id == nullptr) return nullptr;
  if (run_id_value != nullptr && run_id == nullptr) {
    if (scene_id != nullptr) env->ReleaseStringUTFChars(scene_id_value, scene_id);
    return nullptr;
  }
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c11_3_2_grid_named_lines_fixture(
      host, scene_id, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  const std::string log_report = "run_id=" + std::string(run_id == nullptr ? "missing" : run_id)
      + " scene_id=" + std::string(scene_id == nullptr ? "missing" : scene_id)
      + " " + report;
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C1132_GRID_EVAL %s", log_report.c_str());
  if (status == 0) LogC10NodeFrames(report, "SPINON_C1132_GRID_NODE_FRAME");
  if (scene_id != nullptr) env->ReleaseStringUTFChars(scene_id_value, scene_id);
  if (run_id != nullptr) env->ReleaseStringUTFChars(run_id_value, run_id);
  return ToByteArray(env, report);
#else
  return ToByteArray(env, "status=-90 C11.3.2 fixture-test feature is disabled");
#endif
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalC1133GridTemplateAreasFixture(
    JNIEnv *env, jclass, jlong host_handle, jstring scene_id_value,
    jstring run_id_value) {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST) && \
    SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  const char *scene_id = scene_id_value == nullptr
      ? nullptr : env->GetStringUTFChars(scene_id_value, nullptr);
  const char *run_id = run_id_value == nullptr
      ? nullptr : env->GetStringUTFChars(run_id_value, nullptr);
  if (scene_id_value != nullptr && scene_id == nullptr) return nullptr;
  if (run_id_value != nullptr && run_id == nullptr) {
    if (scene_id != nullptr) env->ReleaseStringUTFChars(scene_id_value, scene_id);
    return nullptr;
  }
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c11_3_3_grid_template_areas_fixture(
      host, scene_id, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  const std::string log_report = "run_id=" + std::string(run_id == nullptr ? "missing" : run_id)
      + " scene_id=" + std::string(scene_id == nullptr ? "missing" : scene_id)
      + " " + report;
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C1133_GRID_EVAL %s", log_report.c_str());
  if (status == 0) LogC10NodeFrames(report, "SPINON_C1133_GRID_NODE_FRAME");
  if (scene_id != nullptr) env->ReleaseStringUTFChars(scene_id_value, scene_id);
  if (run_id != nullptr) env->ReleaseStringUTFChars(run_id_value, run_id);
  return ToByteArray(env, report);
#else
  return ToByteArray(env, "status=-90 C11.3.3 fixture-test feature is disabled");
#endif
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalC124Source(
    JNIEnv *env, jclass, jlong host_handle, jstring source_value) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  if (source_value == nullptr) return nullptr;
  const char *source = env->GetStringUTFChars(source_value, nullptr);
  if (source == nullptr) return nullptr;
  constexpr std::size_t kOutputCapacity = 256 * 1024;
  std::unique_ptr<char[]> output(new (std::nothrow) char[kOutputCapacity]{});
  if (output == nullptr) {
    return ToByteArray(env, "status=-3 C12.4 진단 보고서 버퍼 할당 실패");
  }
  const int32_t status = spinon_runtime_gpu_host_eval(
      host, source, 60000, output.get(), kOutputCapacity);
  env->ReleaseStringUTFChars(source_value, source);
  const std::string report = "status=" + std::to_string(status) + " " + output.get();
  return ToByteArray(env, report);
}
#endif

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalRootHtmlApplicationScript(
    JNIEnv *env, jclass, jlong host_handle, jstring source_value) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  if (host == nullptr || source_value == nullptr) {
    return ToByteArray(env, "status=-1 root-app.js host 또는 source가 null입니다");
  }
  const char *source = env->GetStringUTFChars(source_value, nullptr);
  if (source == nullptr) return nullptr;
  constexpr std::size_t kOutputCapacity = 256 * 1024;
  std::unique_ptr<char[]> output(new (std::nothrow) char[kOutputCapacity]{});
  if (output == nullptr) {
    env->ReleaseStringUTFChars(source_value, source);
    return ToByteArray(env, "status=-3 root-app.js 보고 버퍼 할당 실패");
  }
  const int32_t status = spinon_runtime_gpu_host_eval(
      host, source, 60000, output.get(), kOutputCapacity);
  env->ReleaseStringUTFChars(source_value, source);
  const std::string report = "status=" + std::to_string(status) + " " + output.get();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_ROOT_HTML_DEFAULT_APP_EVAL %s", report.c_str());
  return ToByteArray(env, report);
}

#if defined(SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST) && \
    SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST
extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalC1035PositionedFlexFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c10_3_5_positioned_flex_fixture(
      host, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C1035_EVAL %s", report.c_str());
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalC121PositionState(
    JNIEnv *env, jclass, jlong host_handle, jint state) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c12_1_position_state(
      host, static_cast<uint32_t>(state), 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C121_STATE status=%d state=%d %s", status,
                      state, output.data());
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalMarginCollapseFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_margin_collapse_fixture(
      host, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C092_EVAL %s", report.c_str());
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalFlowRootFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_flow_root_fixture(
      host, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C093_EVAL %s", report.c_str());
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalFlexWrapFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_flex_wrap_fixture(
      host, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C101_EVAL %s", report.c_str());
  if (status == 0) LogC10NodeFrames(report, "SPINON_C101_NODE_FRAME");
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalFlexDistributionFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_flex_distribution_fixture(
      host, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C102_EVAL %s", report.c_str());
  if (status == 0) LogC10NodeFrames(report, "SPINON_C102_NODE_FRAME");
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalFlexReverseFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_flex_reverse_fixture(
      host, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C1031_EVAL %s", report.c_str());
  if (status == 0) LogC10NodeFrames(report, "SPINON_C1031_NODE_FRAME");
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalFlexOrderFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_flex_order_fixture(
      host, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C1032_EVAL %s", report.c_str());
  if (status == 0) LogC10NodeFrames(report, "SPINON_C1032_NODE_FRAME");
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalFlexAlignmentFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_flex_alignment_fixture(
      host, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C1033_EVAL %s", report.c_str());
  if (status == 0) LogC10NodeFrames(report, "SPINON_C1033_NODE_FRAME");
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalFlexBaselineFixture(
    JNIEnv *env, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_flex_baseline_fixture(
      host, 10000, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C1034_EVAL %s", report.c_str());
  if (status == 0) LogC10NodeFrames(report, "SPINON_C1034_NODE_FRAME");
  return ToByteArray(env, report);
}
#endif

extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeCreateSurface(
    JNIEnv *env, jclass, jlong host_handle, jobject surface, jint width,
    jint height, jint backend) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  ANativeWindow *window = ANativeWindow_fromSurface(env, surface);
  if (host == nullptr || window == nullptr || width <= 0 || height <= 0) {
    if (window != nullptr) ANativeWindow_release(window);
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_C0410_SURFACE_ERROR=invalid surface input");
    return 0;
  }
  std::array<char, 1024> output{};
  void *created = spinon_runtime_gpu_host_create_android(
      host, window, static_cast<uint32_t>(width), static_cast<uint32_t>(height),
      static_cast<uint32_t>(backend), output.data(), output.size());
  if (created == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_C0410_SURFACE_ERROR=%s", output.data());
    ANativeWindow_release(window);
    return 0;
  }
  auto *context = new C0410RuntimeGpuContext{window, host};
  __android_log_print(ANDROID_LOG_INFO, kTag, "SPINON_C0410_RENDERER=%s",
                      output.data());
  return static_cast<jlong>(reinterpret_cast<uintptr_t>(context));
}

extern "C" JNIEXPORT jint JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeResizeSurface(
    JNIEnv *, jclass, jlong context_handle, jint width, jint height) {
  auto *context = reinterpret_cast<C0410RuntimeGpuContext *>(
      static_cast<uintptr_t>(context_handle));
  if (context == nullptr) return -1;
  __android_log_print(ANDROID_LOG_INFO, kTag,
                      "SPINON_C0410_NATIVE_WINDOW requested=%dx%d before=%dx%d",
                      width, height, ANativeWindow_getWidth(context->window),
                      ANativeWindow_getHeight(context->window));
  std::array<char, 1024> output{};
  const int32_t status = spinon_runtime_gpu_host_resize(
      context->host, static_cast<uint32_t>(width),
      static_cast<uint32_t>(height), output.data(), output.size());
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C0410_RESIZE status=%d %s", status,
                      output.data());
  __android_log_print(ANDROID_LOG_INFO, kTag,
                      "SPINON_C0410_NATIVE_WINDOW after=%dx%d",
                      ANativeWindow_getWidth(context->window),
                      ANativeWindow_getHeight(context->window));
  return status;
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeDrawSurface(
    JNIEnv *env, jclass, jlong context_handle) {
  auto *context = reinterpret_cast<C0410RuntimeGpuContext *>(
      static_cast<uintptr_t>(context_handle));
  if (context == nullptr) return ToByteArray(env, "status=-1 null renderer");
  // 표면 픽셀 표본 JSON을 포함하는 draw 보고서가 1 KiB를 넘을 수 있습니다.
  std::array<char, 16384> output{};
  const int32_t status = spinon_runtime_gpu_host_draw(
      context->host, output.data(), output.size());
  const std::string report =
      "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C0410_DRAW %s", report.c_str());
  return ToByteArray(env, report);
}

#if defined(SPINON_ENABLE_C19_2_BOX_PAINT_OPACITY_FIXTURE_TEST) && \
    SPINON_ENABLE_C19_2_BOX_PAINT_OPACITY_FIXTURE_TEST
extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeRequestC192SurfacePixels(
    JNIEnv *env, jclass, jlong context_handle, jintArray sample_xy) {
  auto *context = reinterpret_cast<C0410RuntimeGpuContext *>(
      static_cast<uintptr_t>(context_handle));
  if (context == nullptr || sample_xy == nullptr || env->GetArrayLength(sample_xy) != 32) {
    return ToByteArray(env, "status=-1 invalid C19.2 direct pixel request");
  }
  std::array<jint, 32> java_coordinates{};
  env->GetIntArrayRegion(sample_xy, 0,
                         static_cast<jsize>(java_coordinates.size()),
                         java_coordinates.data());
  if (env->ExceptionCheck()) return nullptr;
  std::array<uint32_t, 32> sample_coordinates{};
  for (size_t index = 0; index < java_coordinates.size(); ++index) {
    if (java_coordinates[index] < 0) {
      return ToByteArray(env, "status=-1 negative C19.2 direct pixel coordinate");
    }
    sample_coordinates[index] = static_cast<uint32_t>(java_coordinates[index]);
  }
  std::array<char, 2048> output{};
  const int32_t status = spinon_runtime_gpu_host_request_c19_2_surface_pixels_for_test(
      context->host, sample_coordinates.data(), sample_coordinates.size() / 2,
      output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C192_ANDROID_DIRECT_REQUEST %s", report.c_str());
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeReadbackC192Offscreen(
    JNIEnv *env, jclass, jlong context_handle) {
  auto *context = reinterpret_cast<C0410RuntimeGpuContext *>(
      static_cast<uintptr_t>(context_handle));
  if (context == nullptr) return ToByteArray(env, "status=-1 null C19.2 renderer");
  std::array<char, 8192> output{};
  const int32_t status = spinon_runtime_gpu_host_readback_c19_2_offscreen_for_test(
      context->host, output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C192_OFFSCREEN_REQUEST %s", report.c_str());
  return ToByteArray(env, report);
}
#endif

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeInjectNextDrawFailure(
    JNIEnv *env, jclass, jlong context_handle) {
  auto *context = reinterpret_cast<C0410RuntimeGpuContext *>(
      static_cast<uintptr_t>(context_handle));
  if (context == nullptr) return ToByteArray(env, "status=-1 null renderer");
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU_FAILURE_FIXTURE) && \
    SPINON_ENABLE_C04_RUNTIME_GPU_FAILURE_FIXTURE
  std::array<char, 1024> output{};
  const int32_t status = spinon_runtime_gpu_host_inject_next_draw_failure_for_test(
      context->host, output.data(), output.size());
  const std::string report =
      "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C0410_DRAW_FAILURE_HOOK %s", report.c_str());
  return ToByteArray(env, report);
#else
  return ToByteArray(env, "status=-90 failure fixture is not enabled in this build");
#endif
}

extern "C" JNIEXPORT void JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeDestroySurface(
    JNIEnv *, jclass, jlong context_handle) {
  auto *context = reinterpret_cast<C0410RuntimeGpuContext *>(
      static_cast<uintptr_t>(context_handle));
  if (context == nullptr) return;
  std::array<char, 1024> output{};
  const int32_t status = spinon_runtime_gpu_host_destroy_renderer(
      context->host, output.data(), output.size());
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C0410_RENDERER_DESTROY status=%d %s",
                      status, output.data());
  ANativeWindow_release(context->window);
  delete context;
}

extern "C" JNIEXPORT void JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeFreeHost(
    JNIEnv *, jclass, jlong host_handle) {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(host_handle));
  spinon_runtime_gpu_host_free(host);
}
#endif

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_MainActivity_nativeRun(JNIEnv *env, jclass,
                                                  jbyteArray source, jfloat width,
                                                  jfloat height, jfloat density,
                                                  jboolean run_r10) {
  if (source == nullptr) return nullptr;
  const jsize source_length = env->GetArrayLength(source);
  std::vector<char> source_utf8(static_cast<size_t>(source_length) + 1);
  env->GetByteArrayRegion(source, 0, source_length,
                          reinterpret_cast<jbyte *>(source_utf8.data()));
  if (env->ExceptionCheck()) return nullptr;
  source_utf8[static_cast<size_t>(source_length)] = '\0';

  std::array<char, 512> output{};
  const int32_t result = spinon_app_run(source_utf8.data(), output.data(), output.size());
  std::string result_text(output.data());

  if (result != 0) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_BOOTSTRAP_ERROR code=%d detail=%s", result,
                        output.data());
  }
  if (run_r10 == JNI_TRUE) {
    std::array<char, 2048> layout_output{};
    const int32_t layout_result = spinon_taffy_r10_run(
        width, height, density, layout_output.data(), layout_output.size());
    if (layout_result == 0) {
      __android_log_print(ANDROID_LOG_INFO, kTag,
                          "SPINON_TAFFY_R10_RESULT=%s", layout_output.data());
      result_text += "\n\nSPINON_TAFFY_R10_RESULT=";
      result_text += layout_output.data();
    } else {
      __android_log_print(ANDROID_LOG_ERROR, kTag,
                          "SPINON_TAFFY_R10_ERROR code=%d detail=%s",
                          layout_result, layout_output.data());
      result_text += "\n\nSPINON_TAFFY_R10_ERROR=";
      result_text += layout_output.data();
    }
  }
  const auto output_length = static_cast<jsize>(result_text.size());
  jbyteArray result_array = env->NewByteArray(output_length);
  if (result_array == nullptr) return nullptr;
  env->SetByteArrayRegion(result_array, 0, output_length,
                          reinterpret_cast<const jbyte *>(result_text.data()));
  if (env->ExceptionCheck()) return nullptr;
  return result_array;
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_MainActivity_nativeC141IntrinsicSizeFixture(
    JNIEnv *env, jclass) {
  std::array<char, 4096> output{};
  std::string report;
#if defined(SPINON_ENABLE_C14_1_APP_FIXTURE_TEST) && \
    SPINON_ENABLE_C14_1_APP_FIXTURE_TEST
  const int32_t status =
      spinon_c14_1_app_intrinsic_size_fixture(output.data(), output.size());
  report = output.data();
  if (status != 0) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_C141_NATIVE_ERROR code=%d report=%s", status,
                        report.c_str());
  }
#else
  report = "{\"status\":\"disabled\",\"detail\":\"fixture feature is off\"}";
#endif
  const auto report_length = static_cast<jsize>(report.size());
  jbyteArray encoded = env->NewByteArray(report_length);
  if (encoded == nullptr) return nullptr;
  env->SetByteArrayRegion(encoded, 0, report_length,
                          reinterpret_cast<const jbyte *>(report.data()));
  if (env->ExceptionCheck()) return nullptr;
  return encoded;
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_MainActivity_nativeC142ImageRatioFixture(JNIEnv *env,
                                                                  jclass) {
  std::array<char, 65536> output{};
  std::string report;
#if defined(SPINON_ENABLE_C14_2_APP_FIXTURE_TEST) && \
    SPINON_ENABLE_C14_2_APP_FIXTURE_TEST
  const int32_t status =
      spinon_c14_2_app_image_ratio_fixture(output.data(), output.size());
  report = output.data();
  if (status != 0) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_C142_NATIVE_ERROR code=%d report=%s", status,
                        report.c_str());
  }
#else
  report = "{\"status\":\"disabled\",\"detail\":\"fixture feature is off\"}";
#endif
  const auto report_length = static_cast<jsize>(report.size());
  jbyteArray encoded = env->NewByteArray(report_length);
  if (encoded == nullptr) return nullptr;
  env->SetByteArrayRegion(encoded, 0, report_length,
                          reinterpret_cast<const jbyte *>(report.data()));
  if (env->ExceptionCheck()) return nullptr;
  return encoded;
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_MainActivity_nativeC143IntrinsicContributionsFixture(
    JNIEnv *env, jclass) {
  std::array<char, 8192> output{};
  std::string report;
#if defined(SPINON_ENABLE_C14_3_APP_FIXTURE_TEST) && \
    SPINON_ENABLE_C14_3_APP_FIXTURE_TEST
  const int32_t status = spinon_c14_3_app_intrinsic_contributions_fixture(
      output.data(), output.size());
  report = output.data();
  if (status != 0) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_C143_NATIVE_ERROR code=%d report=%s", status,
                        report.c_str());
  }
#else
  report = "{\"status\":\"disabled\",\"detail\":\"fixture feature is off\"}";
#endif
  const auto report_length = static_cast<jsize>(report.size());
  jbyteArray encoded = env->NewByteArray(report_length);
  if (encoded == nullptr) return nullptr;
  env->SetByteArrayRegion(encoded, 0, report_length,
                          reinterpret_cast<const jbyte *>(report.data()));
  if (env->ExceptionCheck()) return nullptr;
  return encoded;
}

extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_R08WgpuSurface_nativeCreate(JNIEnv *env, jclass,
                                                       jobject surface,
                                                       jint width, jint height,
                                                       jint backend) {
  ANativeWindow *window = ANativeWindow_fromSurface(env, surface);
  if (window == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_R08_WGPU_ERROR=ANativeWindow unavailable");
    return 0;
  }
  char output[512] = {};
  void *renderer = spinon_wgpu_create_android(
      window, static_cast<uint32_t>(width), static_cast<uint32_t>(height),
      static_cast<uint32_t>(backend), output, sizeof(output));
  if (renderer == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_R08_WGPU_ERROR=%s", output);
    ANativeWindow_release(window);
    return 0;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag,
                      "SPINON_R08_WGPU=ready %s", output);
  auto *context = new WgpuRendererContext{window, renderer};
  return reinterpret_cast<jlong>(context);
}

#if defined(SPINON_ENABLE_S04_ANDROID_FIXTURE)
extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_R08WgpuSurface_nativeCreateS04(
    JNIEnv *env, jclass, jobject surface, jint width, jint height, jint backend,
    jfloat density, jlong surface_generation) {
  ANativeWindow *window = ANativeWindow_fromSurface(env, surface);
  if (window == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_S04_RENDERER_ERROR=ANativeWindow unavailable");
    return 0;
  }
  char output[1024] = {};
  void *renderer = spinon_wgpu_create_android_s04(
      window, static_cast<uint32_t>(width), static_cast<uint32_t>(height),
      static_cast<uint32_t>(backend), density,
      static_cast<uint64_t>(surface_generation), output, sizeof(output));
  if (renderer == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_S04_RENDERER_ERROR=%s", output);
    ANativeWindow_release(window);
    return 0;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag,
                      "SPINON_S04_RENDERER=ready surface_generation=%lld size=%dx%d density=%.4f %s",
                      static_cast<long long>(surface_generation), width, height,
                      static_cast<double>(density), output);
  auto *context = new WgpuRendererContext{window, renderer};
  return reinterpret_cast<jlong>(context);
}
#else
extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_R08WgpuSurface_nativeCreateS04(JNIEnv *, jclass,
                                                          jobject, jint, jint,
                                                          jint, jfloat, jlong) {
  __android_log_print(ANDROID_LOG_ERROR, kTag,
                      "SPINON_S04_FIXTURE=disabled rebuild with SPINON_ENABLE_S04_ANDROID_FIXTURE=1");
  return 0;
}
#endif

extern "C" JNIEXPORT jint JNICALL
Java_dev_spinon_bootstrap_R08WgpuSurface_nativeDraw(JNIEnv *, jclass,
                                                     jlong handle,
                                                     jint activation_count) {
  auto *context = reinterpret_cast<WgpuRendererContext *>(handle);
  if (context == nullptr) return -1;
  char output[512] = {};
  const int32_t result = spinon_wgpu_draw(
      context->renderer, static_cast<uint32_t>(activation_count), output,
      sizeof(output));
  if (result != 0) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_R08_WGPU_DRAW_ERROR code=%d detail=%s",
                        result, output);
  }
  return result;
}

#if defined(SPINON_ENABLE_S04_ANDROID_FIXTURE)
extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_R08WgpuSurface_nativeDrawS04(JNIEnv *env, jclass,
                                                        jlong handle) {
  auto *context = reinterpret_cast<WgpuRendererContext *>(handle);
  if (context == nullptr) return ToByteArray(env, "status=-1 null renderer");
  std::array<char, 1024> output{};
  const int32_t result = spinon_wgpu_s04_draw(
      context->renderer, output.data(), output.size());
  const std::string report =
      "status=" + std::to_string(result) + " " + output.data();
  __android_log_print(result == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR, kTag,
                      "SPINON_S04_FRAME %s", report.c_str());
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_R08WgpuSurface_nativePollS04Readback(JNIEnv *env,
                                                                jclass,
                                                                jlong handle) {
  auto *context = reinterpret_cast<WgpuRendererContext *>(handle);
  if (context == nullptr) return ToByteArray(env, "status=-1 null renderer");
  std::array<char, 1024> output{};
  const int32_t result = spinon_wgpu_s04_poll_readback(
      context->renderer, output.data(), output.size());
  const std::string report =
      "status=" + std::to_string(result) + " " + output.data();
  if (result != 0) {
    __android_log_print(result > 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR, kTag,
                        "SPINON_S04_READBACK %s", report.c_str());
  }
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_R08WgpuSurface_nativeHitTestS04(
    JNIEnv *env, jclass, jlong handle, jlong surface_generation,
    jfloat surface_x, jfloat surface_y) {
  auto *context = reinterpret_cast<WgpuRendererContext *>(handle);
  if (context == nullptr) return ToByteArray(env, "status=-1 null renderer");
  std::array<char, 1024> output{};
  const int32_t result = spinon_wgpu_s04_hit_test(
      context->renderer, static_cast<uint64_t>(surface_generation), surface_x,
      surface_y, output.data(), output.size());
  const std::string report = "status=" + std::to_string(result) + " " + output.data();
  __android_log_print(result >= 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR, kTag,
                      "SPINON_S04_HIT_TEST %s", report.c_str());
  return ToByteArray(env, report);
}
#else
extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_R08WgpuSurface_nativeDrawS04(JNIEnv *env, jclass,
                                                        jlong) {
  return ToByteArray(env, "status=-90 S04 Android fixture is disabled");
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_R08WgpuSurface_nativePollS04Readback(JNIEnv *env,
                                                                jclass,
                                                                jlong) {
  return ToByteArray(env, "status=-90 S04 Android fixture is disabled");
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_R08WgpuSurface_nativeHitTestS04(JNIEnv *env, jclass,
                                                           jlong, jlong,
                                                           jfloat, jfloat) {
  return ToByteArray(env, "status=-90 S04 Android fixture is disabled");
}
#endif

extern "C" JNIEXPORT jint JNICALL
Java_dev_spinon_bootstrap_R08WgpuSurface_nativeResize(JNIEnv *, jclass,
                                                       jlong handle,
                                                       jint width, jint height) {
  auto *context = reinterpret_cast<WgpuRendererContext *>(handle);
  if (context == nullptr) return -1;
  return spinon_wgpu_resize(context->renderer, static_cast<uint32_t>(width),
                            static_cast<uint32_t>(height));
}

#if defined(SPINON_ENABLE_S04_ANDROID_FIXTURE)
extern "C" JNIEXPORT jint JNICALL
Java_dev_spinon_bootstrap_R08WgpuSurface_nativeResizeS04(
    JNIEnv *, jclass, jlong handle, jint width, jint height, jfloat density,
    jlong surface_generation) {
  auto *context = reinterpret_cast<WgpuRendererContext *>(handle);
  if (context == nullptr) return -1;
  return spinon_wgpu_s04_resize(context->renderer,
                                static_cast<uint32_t>(width),
                                static_cast<uint32_t>(height), density,
                                static_cast<uint64_t>(surface_generation));
}
#else
extern "C" JNIEXPORT jint JNICALL
Java_dev_spinon_bootstrap_R08WgpuSurface_nativeResizeS04(JNIEnv *, jclass,
                                                          jlong, jint, jint,
                                                          jfloat, jlong) {
  return -90;
}
#endif

extern "C" JNIEXPORT jint JNICALL
Java_dev_spinon_bootstrap_R08WgpuSurface_nativeInjectFailure(JNIEnv *, jclass,
                                                              jlong handle,
                                                              jint failure_kind) {
  auto *context = reinterpret_cast<WgpuRendererContext *>(handle);
  if (context == nullptr) return -1;
  return spinon_wgpu_r13_inject_failure(context->renderer,
                                         static_cast<uint32_t>(failure_kind));
}

extern "C" JNIEXPORT void JNICALL
Java_dev_spinon_bootstrap_R08WgpuSurface_nativeDestroy(JNIEnv *, jclass,
                                                        jlong handle) {
  auto *context = reinterpret_cast<WgpuRendererContext *>(handle);
  if (context == nullptr) return;
  spinon_wgpu_destroy(context->renderer);
  ANativeWindow_release(context->window);
  delete context;
}

extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_MainActivity_nativeSessionCreate(JNIEnv *, jclass) {
  std::array<char, 512> output{};
  SpinonRuntimeSession *session =
      spinon_runtime_session_new(output.data(), output.size());
  if (session == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_RUNTIME_SESSION_CREATE_ERROR=%s", output.data());
    return 0;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag, "SPINON_RUNTIME_SESSION=%s",
                      output.data());
  return static_cast<jlong>(reinterpret_cast<uintptr_t>(session));
}

extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_MainActivity_nativeSessionCreateWithRootHtml(
    JNIEnv *env, jclass, jbyteArray source_utf8) {
  if (source_utf8 == nullptr) return 0;
  const jsize source_length = env->GetArrayLength(source_utf8);
  constexpr size_t kMaxRootHtmlSourceBytes = 1'048'576;
  if (source_length < 0 ||
      static_cast<size_t>(source_length) > kMaxRootHtmlSourceBytes) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_ROOT_HTML_SESSION_ERROR reason=source-too-large");
    return 0;
  }
  std::vector<char> source(static_cast<size_t>(source_length));
  if (source_length > 0) {
    env->GetByteArrayRegion(source_utf8, 0, source_length,
                            reinterpret_cast<jbyte *>(source.data()));
    if (env->ExceptionCheck()) return 0;
  }

  std::array<char, 512> output{};
  SpinonRuntimeSession *session = spinon_runtime_session_new_with_root_html(
      source.data(), source.size(), output.data(), output.size());
  if (session == nullptr) {
    __android_log_print(ANDROID_LOG_ERROR, kTag,
                        "SPINON_ROOT_HTML_SESSION_ERROR=%s", output.data());
    return 0;
  }
  __android_log_print(ANDROID_LOG_INFO, kTag,
                      "SPINON_ROOT_HTML_SESSION=%s", output.data());
  return static_cast<jlong>(reinterpret_cast<uintptr_t>(session));
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_MainActivity_nativeSessionSetRootHtmlEnvironment(
    JNIEnv *env, jclass, jlong session, jfloat surface_width_px,
    jfloat surface_height_px, jfloat device_scale_factor, jboolean prefers_dark) {
  SpinonRuntimeRootHtmlCanvas canvas{};
  const int32_t status = spinon_runtime_session_resolve_root_html_canvas(
      reinterpret_cast<SpinonRuntimeSession *>(static_cast<uintptr_t>(session)),
      surface_width_px, surface_height_px, device_scale_factor,
      prefers_dark == JNI_TRUE ? 1 : 0, 5000, &canvas);
  std::string report = "status=" + std::to_string(status);
  if (status == 0) {
    const SpinonRuntimeRootHtmlEnvironment &environment = canvas.environment;
    char canvas_color[8]{};
    std::snprintf(canvas_color, sizeof(canvas_color), "#%02x%02x%02x",
                  static_cast<unsigned int>(canvas.canvas_rgba[0]),
                  static_cast<unsigned int>(canvas.canvas_rgba[1]),
                  static_cast<unsigned int>(canvas.canvas_rgba[2]));
    report += " viewport_css_px=" +
              std::to_string(environment.viewport_width_css_px) + "x" +
              std::to_string(environment.viewport_height_css_px) +
              " visual_viewport_css_px=" +
              std::to_string(environment.visual_viewport_width_css_px) + "x" +
              std::to_string(environment.visual_viewport_height_css_px) +
              " device_scale_factor=" +
              std::to_string(environment.device_scale_factor) +
              " page_scale=" + std::to_string(environment.page_scale) +
              " prefers_color_scheme=" +
              std::to_string(environment.prefers_color_scheme) +
              " document_color_scheme=" +
              std::to_string(environment.document_color_scheme) +
              " environment_revision=" +
              std::to_string(environment.environment_revision) +
              " canvas_color=" + canvas_color +
              " used_color_scheme=" + std::to_string(canvas.used_color_scheme) +
              " cascade_generation=" + std::to_string(canvas.cascade_generation) +
              " cascade_document_revision=" +
              std::to_string(canvas.cascade_document_revision) +
              " cascade_render_tree_revision=" +
              std::to_string(canvas.cascade_render_tree_revision) +
              " cascade_style_revision=" +
              std::to_string(canvas.cascade_style_revision) +
              " cascade_environment_revision=" +
              std::to_string(canvas.cascade_environment_revision);
  }
  jbyteArray output = env->NewByteArray(static_cast<jsize>(report.size()));
  if (output == nullptr) return nullptr;
  env->SetByteArrayRegion(output, 0, static_cast<jsize>(report.size()),
                          reinterpret_cast<const jbyte *>(report.data()));
  return env->ExceptionCheck() ? nullptr : output;
}

#if defined(SPINON_ENABLE_C04_12_ROOT_HTML_FIXTURE_TEST) && \
    SPINON_ENABLE_C04_12_ROOT_HTML_FIXTURE_TEST
extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0412RootHtmlFixtureNativeAdapter_run(JNIEnv *env,
                                                                jclass) {
  std::array<char, 2048> output{};
  const int32_t status = spinon_c04_12_root_html_accessors_fixture(
      output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  jbyteArray result = env->NewByteArray(static_cast<jsize>(report.size()));
  if (result == nullptr) return nullptr;
  env->SetByteArrayRegion(result, 0, static_cast<jsize>(report.size()),
                          reinterpret_cast<const jbyte *>(report.data()));
  return result;
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_C0412RootHtmlFixtureNativeAdapter_runAsset(
    JNIEnv *env, jclass, jbyteArray source_utf8) {
  auto make_result = [env](const std::string &report) -> jbyteArray {
    jbyteArray result = env->NewByteArray(static_cast<jsize>(report.size()));
    if (result == nullptr) return nullptr;
    env->SetByteArrayRegion(result, 0, static_cast<jsize>(report.size()),
                            reinterpret_cast<const jbyte *>(report.data()));
    return env->ExceptionCheck() ? nullptr : result;
  };

  if (source_utf8 == nullptr) return make_result("status=-1 root_html_asset=null");
  const jsize source_length = env->GetArrayLength(source_utf8);
  constexpr size_t kMaxRootHtmlSourceBytes = 1'048'576;
  if (source_length < 0 ||
      static_cast<size_t>(source_length) > kMaxRootHtmlSourceBytes) {
    return make_result("status=-1 root_html_asset=oversized");
  }

  std::vector<char> source(static_cast<size_t>(source_length));
  if (source_length > 0) {
    env->GetByteArrayRegion(source_utf8, 0, source_length,
                            reinterpret_cast<jbyte *>(source.data()));
    if (env->ExceptionCheck()) return nullptr;
  }

  std::array<char, 1024> output{};
  const int32_t status = spinon_c04_12_root_html_asset_fixture(
      source.data(), source.size(), output.data(), output.size());
  return make_result(std::string("status=") + std::to_string(status) + " " +
                     output.data());
}
#endif

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_MainActivity_nativeSessionEval(JNIEnv *env, jclass,
                                                          jlong handle,
                                                          jbyteArray source) {
  if (handle == 0 || source == nullptr) return nullptr;
  const jsize source_length = env->GetArrayLength(source);
  std::vector<char> source_utf8(static_cast<size_t>(source_length) + 1);
  env->GetByteArrayRegion(source, 0, source_length,
                          reinterpret_cast<jbyte *>(source_utf8.data()));
  if (env->ExceptionCheck()) return nullptr;
  source_utf8[static_cast<size_t>(source_length)] = '\0';
  std::array<char, 2048> output{};
  const int32_t status = spinon_runtime_session_eval(
      SessionFromHandle(handle), source_utf8.data(), output.data(), output.size());
  const std::string report = "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_WARN, kTag,
                      "SPINON_RUNTIME_EVAL=%s", report.c_str());
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_MainActivity_nativeSessionPriorityProbe(JNIEnv *env, jclass) {
  std::array<char, 4096> output{};
  const int32_t status =
      spinon_runtime_priority_probe(output.data(), output.size());
  const std::string report =
      "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR, kTag,
                      "SPINON_PRIORITY_PROBE %s", report.c_str());
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_MainActivity_nativeSessionPriorityFairnessProbe(JNIEnv *env,
                                                                           jclass) {
  std::array<char, 4096> output{};
  const int32_t status =
      spinon_runtime_priority_fairness_probe(output.data(), output.size());
  const std::string report =
      "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR, kTag,
                      "SPINON_PRIORITY_FAIRNESS_PROBE %s", report.c_str());
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_MainActivity_nativeSessionShutdownProbe(JNIEnv *env,
                                                                   jclass) {
  std::array<char, 4096> output{};
  const int32_t status =
      spinon_runtime_shutdown_probe(output.data(), output.size());
  const std::string report =
      "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR, kTag,
                      "SPINON_SHUTDOWN_PROBE %s", report.c_str());
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_MainActivity_nativeSessionUaCascadeProbe(JNIEnv *env,
                                                                   jclass) {
  std::array<char, 32768> output{};
#if defined(SPINON_ENABLE_C04_UA_CASCADE_FIXTURE_PROBE) && \
    SPINON_ENABLE_C04_UA_CASCADE_FIXTURE_PROBE
  const int32_t status =
      spinon_runtime_ua_cascade_probe(output.data(), output.size());
  const std::string report = output.data();
#else
  const int32_t status = -8;
  const std::string report =
      "status=-8 ua_cascade_probe=DISABLED enable the internal fixture probe build flag";
#endif
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR, kTag,
                      "SPINON_C04_RUNTIME_LAYOUT_PROBE %s", report.c_str());
  return ToByteArray(env, report);
}

extern "C" JNIEXPORT jbyteArray JNICALL
Java_dev_spinon_bootstrap_MainActivity_nativeSessionDispatch(JNIEnv *env, jclass,
                                                              jlong handle,
                                                              jint node_id) {
  TraceSection dispatch_section("SpinonR05:native-session-dispatch");
  if (handle == 0) return nullptr;
  std::array<char, 2048> output{};
  int32_t status;
  {
    TraceSection ffi_section("SpinonR05:native-session-ffi");
    status = spinon_runtime_session_dispatch(
        SessionFromHandle(handle), node_id, output.data(), output.size());
  }
  std::string report;
  {
    TraceSection report_section("SpinonR05:native-session-report");
    report = "status=" + std::to_string(status) + " " + output.data();
  }
  jbyteArray result;
  {
    TraceSection byte_array_section("SpinonR05:native-session-byte-array");
    result = ToByteArray(env, report);
  }
  return result;
}

extern "C" JNIEXPORT jint JNICALL
Java_dev_spinon_bootstrap_MainActivity_nativeSessionCancel(JNIEnv *, jclass,
                                                            jlong handle) {
  if (handle == 0) return -1;
  const int32_t status = spinon_runtime_session_cancel(SessionFromHandle(handle));
  __android_log_print(ANDROID_LOG_INFO, kTag, "SPINON_RUNTIME_CANCEL status=%d",
                      status);
  return status;
}

extern "C" JNIEXPORT jint JNICALL
Java_dev_spinon_bootstrap_MainActivity_nativeSessionMemoryPressure(
    JNIEnv *, jclass, jlong handle, jint level) {
  if (handle == 0) return -1;
  const int32_t status = spinon_runtime_session_notify_memory_pressure(
      SessionFromHandle(handle), static_cast<SpinonMemoryPressureLevel>(level));
  __android_log_print(ANDROID_LOG_INFO, kTag,
                      "SPINON_RUNTIME_MEMORY_PRESSURE level=%d status=%d",
                      level, status);
  return status;
}

extern "C" JNIEXPORT void JNICALL
Java_dev_spinon_bootstrap_MainActivity_nativeSessionFree(JNIEnv *, jclass,
                                                          jlong handle) {
  if (handle == 0) return;
  __android_log_print(ANDROID_LOG_INFO, kTag, "SPINON_RUNTIME_SESSION_FREE start");
  spinon_runtime_session_free(SessionFromHandle(handle));
  __android_log_print(ANDROID_LOG_INFO, kTag, "SPINON_RUNTIME_SESSION_FREE done");
}
