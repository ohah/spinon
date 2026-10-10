#include <android/log.h>
#include <android/native_window.h>
#include <android/native_window_jni.h>
#include <android/trace.h>
#include <jni.h>

#include "spinon_ffi.h"
#include "spinon_wgpu_r08.h"

#include <array>
#include <cstdint>
#include <cstring>
#include <string>
#include <vector>

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

SpinonRuntimeSession *SessionFromHandle(jlong handle) {
  return reinterpret_cast<SpinonRuntimeSession *>(static_cast<uintptr_t>(handle));
}
}

#if SPINON_ENABLE_C04_RUNTIME_GPU
extern "C" JNIEXPORT jlong JNICALL
Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeCreateHost(
    JNIEnv *, jclass, jboolean registered_properties_fixture) {
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host = registered_properties_fixture == JNI_TRUE
      ? spinon_runtime_gpu_host_new_registered_properties_fixture(
            output.data(), output.size())
      : spinon_runtime_gpu_host_new(output.data(), output.size());
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
  std::array<char, 1024> output{};
  const int32_t status = spinon_runtime_gpu_host_draw(
      context->host, output.data(), output.size());
  const std::string report =
      "status=" + std::to_string(status) + " " + output.data();
  __android_log_print(status == 0 ? ANDROID_LOG_INFO : ANDROID_LOG_ERROR,
                      kTag, "SPINON_C0410_DRAW %s", report.c_str());
  return ToByteArray(env, report);
}

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
  const int32_t status =
      spinon_runtime_ua_cascade_probe(output.data(), output.size());
  const std::string report = output.data();
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
