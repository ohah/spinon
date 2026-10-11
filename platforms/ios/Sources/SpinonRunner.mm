#import "SpinonRunner.h"

#include <array>
#include <cstdint>

#include "spinon_ffi.h"
#include "spinon_wgpu_r08.h"

#import <os/log.h>

namespace {

template <std::size_t N>
NSString *C0410Report(int32_t status, const std::array<char, N> &output) {
  NSString *detail = [NSString stringWithUTF8String:output.data()];
  if (detail == nil) detail = @"C ABI 보고가 유효한 UTF-8이 아닙니다";
  return [NSString stringWithFormat:@"status=%d %@", status, detail];
}

void C0410LogReport(os_log_type_t type, const char *label, NSString *report) {
  const char *detail = report.UTF8String;
  os_log_with_type(OS_LOG_DEFAULT, type, "%{public}s %{public}s", label,
                   detail != nullptr ? detail : "C ABI 보고를 UTF-8로 변환하지 못했습니다");
}

NSUInteger C10LogNodeFrames(NSString *report, const char *label) {
  NSString *marker = @" node_frames_css_px=[";
  NSRange markerRange = [report rangeOfString:marker];
  if (markerRange.location == NSNotFound) {
    NSString *detail = [NSString stringWithFormat:
        @"marker=missing report_length=%lu", (unsigned long)report.length];
    C0410LogReport(OS_LOG_TYPE_ERROR, "SPINON_C10_FRAME_REPORT_ERROR", detail);
    return 0;
  }
  NSUInteger framesStart = NSMaxRange(markerRange);
  NSRange closingRange = [report rangeOfString:@"]"
                                            options:NSBackwardsSearch
                                              range:NSMakeRange(framesStart,
                                                                report.length - framesStart)];
  if (closingRange.location == NSNotFound) {
    C0410LogReport(OS_LOG_TYPE_ERROR, "SPINON_C10_FRAME_REPORT_ERROR",
                   @"closing bracket is missing");
    return 0;
  }

  NSString *frames = [report substringWithRange:
      NSMakeRange(framesStart, closingRange.location - framesStart)];
  NSUInteger count = 0;
  for (NSString *frame in [frames componentsSeparatedByString:@";"]) {
    if (frame.length > 0) {
      C0410LogReport(OS_LOG_TYPE_INFO, label, frame);
      count += 1;
    }
  }
  NSString *detail = [NSString stringWithFormat:@"label=%s count=%lu", label,
                                                (unsigned long)count];
  C0410LogReport(OS_LOG_TYPE_INFO, "SPINON_C10_NODE_FRAME_COUNT", detail);
  return count;
}

}  // namespace

@implementation SpinonRunner

+ (NSString *)runSource:(NSString *)source {
  const char *source_utf8 = source.UTF8String;
  if (source_utf8 == nullptr) return @"invalid JavaScript source";

  char output[512] = {};
  const int32_t result = spinon_app_run(source_utf8, output, sizeof(output));
  NSString *message = [NSString stringWithUTF8String:output];
  if (result != 0) {
    os_log_error(OS_LOG_DEFAULT, "SPINON_BOOTSTRAP_ERROR code=%{public}d detail=%{public}@",
                 result, message);
  }
  return message ?: @"empty bootstrap result";
}

+ (NSString *)runTaffyR10WithWidth:(float)width height:(float)height scale:(float)scale {
  char output[2048] = {};
  const int32_t result =
      spinon_taffy_r10_run(width, height, scale, output, sizeof(output));
  NSString *message = [NSString stringWithUTF8String:output];
  if (result != 0) {
    os_log_error(OS_LOG_DEFAULT,
                 "SPINON_TAFFY_R10_ERROR code=%{public}d detail=%{public}@",
                 result, message);
  }
  return message ?: @"empty R10 report";
}

+ (void *)createR08WgpuWithUIKitView:(void *)view width:(uint32_t)width height:(uint32_t)height {
  char output[512] = {};
  void *renderer = spinon_wgpu_create_uikit(
      view, width, height, SPINON_WGPU_R08_METAL, output, sizeof(output));
  NSString *message = [NSString stringWithUTF8String:output];
  if (renderer == nullptr) {
    os_log_error(OS_LOG_DEFAULT, "SPINON_R08_WGPU_ERROR=%{public}@", message);
    return nullptr;
  }
  os_log(OS_LOG_DEFAULT, "SPINON_R08_WGPU=ready %{public}@", message);
  return renderer;
}

+ (BOOL)isS04IosFixtureEnabled {
#if defined(SPINON_ENABLE_S04_IOS_FIXTURE) && SPINON_ENABLE_S04_IOS_FIXTURE
  return YES;
#else
  return NO;
#endif
}

+ (void *)createS04WgpuWithUIKitView:(void *)view
                               width:(uint32_t)width
                              height:(uint32_t)height
                             density:(float)density
                   surfaceGeneration:(uint64_t)surfaceGeneration {
#if defined(SPINON_ENABLE_S04_IOS_FIXTURE) && SPINON_ENABLE_S04_IOS_FIXTURE
  std::array<char, 32768> output{};
  void *renderer = spinon_wgpu_create_uikit_s04(
      view, width, height, SPINON_WGPU_R08_METAL, density, surfaceGeneration,
      output.data(), output.size());
  if (renderer == nullptr) {
    os_log_error(OS_LOG_DEFAULT,
                 "SPINON_S04_RENDERER=failed surface_generation=%{public}llu detail=%{public}s",
                 surfaceGeneration, output.data());
    return nullptr;
  }
  os_log(OS_LOG_DEFAULT,
         "SPINON_S04_RENDERER=ready surface_generation=%{public}llu size=%{public}ux%{public}u density=%{public}.4f %{public}s",
         surfaceGeneration, width, height, density, output.data());
  return renderer;
#else
  (void)view;
  (void)width;
  (void)height;
  (void)density;
  (void)surfaceGeneration;
  os_log(OS_LOG_DEFAULT,
         "SPINON_S04_FIXTURE=disabled rebuild with SPINON_ENABLE_S04_IOS_FIXTURE=1");
  return nullptr;
#endif
}

+ (NSString *)drawS04Wgpu:(void *)renderer {
#if defined(SPINON_ENABLE_S04_IOS_FIXTURE) && SPINON_ENABLE_S04_IOS_FIXTURE
  std::array<char, 2048> output{};
  const int32_t status =
      spinon_wgpu_s04_draw(renderer, output.data(), output.size());
  NSString *message = [NSString stringWithUTF8String:output.data()];
  if (status == 0) {
    os_log(OS_LOG_DEFAULT, "SPINON_S04_FRAME status=%{public}d %{public}@",
           status, message ?: @"empty report");
  } else {
    os_log_error(OS_LOG_DEFAULT,
                 "SPINON_S04_FRAME status=%{public}d detail=%{public}@",
                 status, message ?: @"empty report");
  }
  return [NSString stringWithFormat:@"status=%d %@", status,
                                    message ?: @"empty report"];
#else
  (void)renderer;
  return @"status=-1 fixture-disabled";
#endif
}

+ (NSString *)pollS04Readback:(void *)renderer {
#if defined(SPINON_ENABLE_S04_IOS_FIXTURE) && SPINON_ENABLE_S04_IOS_FIXTURE
  std::array<char, 2048> output{};
  const int32_t status = spinon_wgpu_s04_poll_readback(
      renderer, output.data(), output.size());
  NSString *message = [NSString stringWithUTF8String:output.data()];
  if (status == 1) {
    os_log(OS_LOG_DEFAULT, "SPINON_S04_READBACK status=%{public}d %{public}@",
           status, message ?: @"empty report");
  } else if (status < 0) {
    os_log_error(OS_LOG_DEFAULT,
                 "SPINON_S04_READBACK status=%{public}d detail=%{public}@",
                 status, message ?: @"empty report");
  }
  return [NSString stringWithFormat:@"status=%d %@", status,
                                    message ?: @"empty report"];
#else
  (void)renderer;
  return @"status=-1 fixture-disabled";
#endif
}

+ (NSString *)hitTestS04Wgpu:(void *)renderer
         surfaceGeneration:(uint64_t)surfaceGeneration
                  surfaceX:(float)surfaceX
                  surfaceY:(float)surfaceY {
#if defined(SPINON_ENABLE_S04_IOS_FIXTURE) && SPINON_ENABLE_S04_IOS_FIXTURE
  std::array<char, 2048> output{};
  const int32_t status = spinon_wgpu_s04_hit_test(
      renderer, surfaceGeneration, surfaceX, surfaceY, output.data(), output.size());
  NSString *message = [NSString stringWithUTF8String:output.data()];
  if (status >= 0) {
    os_log(OS_LOG_DEFAULT, "SPINON_S04_HIT_TEST status=%{public}d %{public}@",
           status, message ?: @"empty report");
  } else {
    os_log_error(OS_LOG_DEFAULT,
                 "SPINON_S04_HIT_TEST status=%{public}d detail=%{public}@",
                 status, message ?: @"empty report");
  }
  return [NSString stringWithFormat:@"status=%d %@", status,
                                    message ?: @"empty report"];
#else
  (void)renderer;
  (void)surfaceGeneration;
  (void)surfaceX;
  (void)surfaceY;
  return @"status=-90 fixture-disabled";
#endif
}

+ (int32_t)resizeS04Wgpu:(void *)renderer
                   width:(uint32_t)width
                  height:(uint32_t)height
                 density:(float)density
       surfaceGeneration:(uint64_t)surfaceGeneration {
#if defined(SPINON_ENABLE_S04_IOS_FIXTURE) && SPINON_ENABLE_S04_IOS_FIXTURE
  const int32_t status = spinon_wgpu_s04_resize(
      renderer, width, height, density, surfaceGeneration);
  if (status != 0) {
    os_log_error(OS_LOG_DEFAULT,
                 "SPINON_S04_RESIZE status=%{public}d surface_generation=%{public}llu",
                 status, surfaceGeneration);
  }
  return status;
#else
  (void)renderer;
  (void)width;
  (void)height;
  (void)density;
  (void)surfaceGeneration;
  return -1;
#endif
}

+ (int32_t)drawR08Wgpu:(void *)renderer activationCount:(uint32_t)activationCount {
  char output[512] = {};
  const int32_t result = spinon_wgpu_draw(renderer, activationCount, output, sizeof(output));
  if (result != 0) {
    NSString *message = [NSString stringWithUTF8String:output];
    os_log_error(OS_LOG_DEFAULT, "SPINON_R08_WGPU_DRAW_ERROR code=%{public}d detail=%{public}@",
                 result, message);
  }
  return result;
}

+ (int32_t)resizeR08Wgpu:(void *)renderer width:(uint32_t)width height:(uint32_t)height {
  return spinon_wgpu_resize(renderer, width, height);
}

+ (int32_t)injectR13Failure:(void *)renderer kind:(uint32_t)failureKind {
  return spinon_wgpu_r13_inject_failure(renderer, failureKind);
}

+ (void)destroyR08Wgpu:(void *)renderer {
  spinon_wgpu_destroy(renderer);
}

+ (uint64_t)createRuntimeSession {
  std::array<char, 512> output{};
  SpinonRuntimeSession *session =
      spinon_runtime_session_new(output.data(), output.size());
  if (session == nullptr) {
    os_log_error(OS_LOG_DEFAULT, "SPINON_RUNTIME_SESSION_CREATE_ERROR=%{public}s",
                 output.data());
    return 0;
  }
  os_log(OS_LOG_DEFAULT, "SPINON_RUNTIME_SESSION=%{public}s", output.data());
  return static_cast<uint64_t>(reinterpret_cast<uintptr_t>(session));
}

+ (NSString *)runRuntimePriorityProbe {
  std::array<char, 4096> output{};
  const int32_t status =
      spinon_runtime_priority_probe(output.data(), output.size());
  NSString *message = [NSString stringWithUTF8String:output.data()];
  if (status == 0) {
    os_log(OS_LOG_DEFAULT, "SPINON_PRIORITY_PROBE status=%{public}d %{public}@",
           status, message ?: @"empty report");
  } else {
    os_log_error(OS_LOG_DEFAULT,
                 "SPINON_PRIORITY_PROBE status=%{public}d %{public}@",
                 status, message ?: @"empty report");
  }
  return [NSString stringWithFormat:@"status=%d %@", status,
                                    message ?: @"empty report"];
}

+ (NSString *)runRuntimePriorityFairnessProbe {
  std::array<char, 4096> output{};
  const int32_t status =
      spinon_runtime_priority_fairness_probe(output.data(), output.size());
  NSString *message = [NSString stringWithUTF8String:output.data()];
  if (status == 0) {
    os_log(OS_LOG_DEFAULT,
           "SPINON_PRIORITY_FAIRNESS_PROBE status=%{public}d %{public}@",
           status, message ?: @"empty report");
  } else {
    os_log_error(OS_LOG_DEFAULT,
                 "SPINON_PRIORITY_FAIRNESS_PROBE status=%{public}d %{public}@",
                 status, message ?: @"empty report");
  }
  return [NSString stringWithFormat:@"status=%d %@", status,
                                    message ?: @"empty report"];
}

+ (NSString *)runRuntimeShutdownProbe {
  std::array<char, 4096> output{};
  const int32_t status =
      spinon_runtime_shutdown_probe(output.data(), output.size());
  NSString *message = [NSString stringWithUTF8String:output.data()];
  if (status == 0) {
    os_log(OS_LOG_DEFAULT, "SPINON_SHUTDOWN_PROBE status=%{public}d %{public}@",
           status, message ?: @"empty report");
  } else {
    os_log_error(OS_LOG_DEFAULT,
                 "SPINON_SHUTDOWN_PROBE status=%{public}d %{public}@",
                 status, message ?: @"empty report");
  }
  return [NSString stringWithFormat:@"status=%d %@", status,
                                    message ?: @"empty report"];
}

+ (NSString *)runUaCascadeProbe {
  std::array<char, 2048> output{};
  const int32_t status =
      spinon_runtime_ua_cascade_probe(output.data(), output.size());
  NSString *message = [NSString stringWithUTF8String:output.data()];
  if (status == 0) {
    os_log(OS_LOG_DEFAULT,
           "SPINON_C04_RUNTIME_LAYOUT_PROBE %{public}@",
           message ?: @"empty report");
  } else {
    os_log_error(OS_LOG_DEFAULT,
                 "SPINON_C04_RUNTIME_LAYOUT_PROBE %{public}@",
                 message ?: @"empty report");
  }
  return message ?: [NSString stringWithFormat:@"status=%d empty report", status];
}

+ (NSString *)evalRuntimeSession:(uint64_t)handle source:(NSString *)source {
  const char *sourceUTF8 = source.UTF8String;
  if (handle == 0 || sourceUTF8 == nullptr) return @"status=-1 invalid session or source";
  std::array<char, 2048> output{};
  const int32_t status = spinon_runtime_session_eval(
      reinterpret_cast<SpinonRuntimeSession *>(static_cast<uintptr_t>(handle)),
      sourceUTF8, output.data(), output.size());
  NSString *message = [NSString stringWithUTF8String:output.data()];
  if (status == 0) {
    os_log(OS_LOG_DEFAULT, "SPINON_RUNTIME_EVAL status=%{public}d %{public}@",
           status, message ?: @"empty report");
  } else {
    os_log_error(OS_LOG_DEFAULT, "SPINON_RUNTIME_EVAL status=%{public}d %{public}@",
                 status, message ?: @"empty report");
  }
  return [NSString stringWithFormat:@"status=%d %@", status,
                                    message ?: @"empty report"];
}

+ (NSString *)dispatchRuntimeSession:(uint64_t)handle nodeID:(int32_t)nodeID {
  if (handle == 0) return @"status=-1 invalid session";
  std::array<char, 2048> output{};
  const int32_t status = spinon_runtime_session_dispatch(
      reinterpret_cast<SpinonRuntimeSession *>(static_cast<uintptr_t>(handle)),
      nodeID, output.data(), output.size());
  NSString *message = [NSString stringWithUTF8String:output.data()];
  if (status == 0) {
    os_log(OS_LOG_DEFAULT, "SPINON_RUNTIME_DISPATCH status=%{public}d %{public}@",
           status, message ?: @"empty report");
  } else {
    os_log_error(OS_LOG_DEFAULT, "SPINON_RUNTIME_DISPATCH status=%{public}d %{public}@",
                 status, message ?: @"empty report");
  }
  return [NSString stringWithFormat:@"status=%d %@", status,
                                    message ?: @"empty report"];
}

+ (int32_t)cancelRuntimeSession:(uint64_t)handle {
  if (handle == 0) return -1;
  const int32_t status = spinon_runtime_session_cancel(
      reinterpret_cast<SpinonRuntimeSession *>(static_cast<uintptr_t>(handle)));
  os_log(OS_LOG_DEFAULT, "SPINON_RUNTIME_CANCEL status=%{public}d", status);
  return status;
}

+ (int32_t)notifyRuntimeMemoryPressure:(uint64_t)handle level:(int32_t)level {
  if (handle == 0) return -1;
  const int32_t status = spinon_runtime_session_notify_memory_pressure(
      reinterpret_cast<SpinonRuntimeSession *>(static_cast<uintptr_t>(handle)),
      static_cast<SpinonMemoryPressureLevel>(level));
  os_log(OS_LOG_DEFAULT,
         "SPINON_RUNTIME_MEMORY_PRESSURE level=%{public}d status=%{public}d",
         level, status);
  return status;
}

+ (void)freeRuntimeSession:(uint64_t)handle {
  if (handle == 0) return;
  os_log(OS_LOG_DEFAULT, "SPINON_RUNTIME_SESSION_FREE start");
  spinon_runtime_session_free(
      reinterpret_cast<SpinonRuntimeSession *>(static_cast<uintptr_t>(handle)));
  os_log(OS_LOG_DEFAULT, "SPINON_RUNTIME_SESSION_FREE done");
}

+ (uint64_t)createRuntimeGpuHostWithRegisteredPropertiesFixture:(BOOL)enabled {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host =
      enabled == YES
          ? spinon_runtime_gpu_host_new_registered_properties_fixture(
                output.data(), output.size())
          : spinon_runtime_gpu_host_new(output.data(), output.size());
  if (host == nullptr) {
    os_log_error(OS_LOG_DEFAULT, "%{public}s_HOST_ERROR=%{public}s",
                 enabled == YES ? "SPINON_C052" : "SPINON_C0410",
                 output.data());
    return 0;
  }
  os_log(OS_LOG_DEFAULT, "%{public}s_HOST=%{public}s",
         enabled == YES ? "SPINON_C052" : "SPINON_C0410", output.data());
  return static_cast<uint64_t>(reinterpret_cast<uintptr_t>(host));
#else
  (void)enabled;
  os_log_error(OS_LOG_DEFAULT,
               "SPINON_C0410_DISABLED rebuild with SPINON_ENABLE_C04_RUNTIME_GPU=1");
  return 0;
#endif
}

+ (uint64_t)createRuntimeGpuHostWithBlockPaintFixture {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host = spinon_runtime_gpu_host_new_block_paint_fixture(
      output.data(), output.size());
  if (host == nullptr) {
    os_log_error(OS_LOG_DEFAULT, "SPINON_C08_HOST_ERROR=%{public}s", output.data());
    return 0;
  }
  os_log(OS_LOG_DEFAULT, "SPINON_C08_HOST=%{public}s", output.data());
  return static_cast<uint64_t>(reinterpret_cast<uintptr_t>(host));
#else
  os_log_error(OS_LOG_DEFAULT,
               "SPINON_C08_DISABLED rebuild with SPINON_ENABLE_C04_RUNTIME_GPU=1");
  return 0;
#endif
}

+ (uint64_t)createRuntimeGpuHostWithBlockFormattingFixture {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host = spinon_runtime_gpu_host_new_block_formatting_fixture(
      output.data(), output.size());
  if (host == nullptr) {
    os_log_error(OS_LOG_DEFAULT, "SPINON_C09_BLOCK_HOST_ERROR=%{public}s", output.data());
    return 0;
  }
  os_log(OS_LOG_DEFAULT, "SPINON_C09_BLOCK_HOST=%{public}s", output.data());
  return static_cast<uint64_t>(reinterpret_cast<uintptr_t>(host));
#else
  os_log_error(OS_LOG_DEFAULT,
               "SPINON_C09_BLOCK_DISABLED rebuild with SPINON_ENABLE_C04_RUNTIME_GPU=1");
  return 0;
#endif
}

+ (uint64_t)createRuntimeGpuHostWithC12_1PositionFixture {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host = spinon_runtime_gpu_host_new_c12_1_position_fixture(
      output.data(), output.size());
  if (host == nullptr) {
    os_log_error(OS_LOG_DEFAULT, "SPINON_C121_HOST_ERROR=%{public}s", output.data());
    return 0;
  }
  os_log(OS_LOG_DEFAULT, "SPINON_C121_HOST=%{public}s", output.data());
  return static_cast<uint64_t>(reinterpret_cast<uintptr_t>(host));
#else
  os_log_error(OS_LOG_DEFAULT,
               "SPINON_C121_DISABLED rebuild with SPINON_ENABLE_C04_RUNTIME_GPU=1");
  return 0;
#endif
}

+ (uint64_t)createRuntimeGpuHostWithC12_2AbsoluteBlockFixture {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c12_2_absolute_block_fixture(
          output.data(), output.size());
  if (host == nullptr) {
    os_log_error(OS_LOG_DEFAULT, "SPINON_C122_HOST_ERROR=%{public}s", output.data());
    return 0;
  }
  os_log(OS_LOG_DEFAULT, "SPINON_C122_HOST=%{public}s", output.data());
  return static_cast<uint64_t>(reinterpret_cast<uintptr_t>(host));
#else
  os_log_error(OS_LOG_DEFAULT,
               "SPINON_C122_DISABLED rebuild with SPINON_ENABLE_C04_RUNTIME_GPU=1");
  return 0;
#endif
}

+ (uint64_t)createRuntimeGpuHostWithC10_3_5PositionedFlexFixture {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c10_3_5_positioned_flex_fixture(
          output.data(), output.size());
  if (host == nullptr) {
    os_log_error(OS_LOG_DEFAULT, "SPINON_C1035_HOST_ERROR=%{public}s", output.data());
    return 0;
  }
  os_log(OS_LOG_DEFAULT, "SPINON_C1035_HOST=%{public}s", output.data());
  return static_cast<uint64_t>(reinterpret_cast<uintptr_t>(host));
#else
  os_log_error(OS_LOG_DEFAULT,
               "SPINON_C1035_DISABLED rebuild with SPINON_ENABLE_C04_RUNTIME_GPU=1");
  return 0;
#endif
}

+ (uint64_t)beginRuntimeGpuPresentationUpdate:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  if (handle == 0) return 0;
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  const uint64_t sequence =
      spinon_runtime_gpu_host_begin_presentation_update(host);
  if (sequence == 0) {
    os_log_error(OS_LOG_DEFAULT,
                 "SPINON_C0410_PRESENTATION_INVALIDATION failed");
  }
  return sequence;
#else
  (void)handle;
  return 0;
#endif
}

+ (NSString *)setRuntimeGpuEnvironment:(uint64_t)handle
                                  width:(float)width
                                 height:(float)height
                                  scale:(float)scale
                                   dark:(BOOL)dark {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 32768> output{};
  os_log(OS_LOG_DEFAULT,
         "SPINON_C0410_ENVIRONMENT_FFI_BEGIN width=%{public}.1f height=%{public}.1f",
         width, height);
  const int32_t status = spinon_runtime_gpu_host_set_environment(
      host, width, height, scale, dark ? 1 : 0, 10000, output.data(),
      output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C0410_ENVIRONMENT", report);
  return report;
#else
  (void)handle; (void)width; (void)height; (void)scale; (void)dark;
  return @"status=-90 feature-disabled";
#endif
}

+ (NSString *)evalRuntimeGpuFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C0410_EVAL", report);
  return report;
#else
  (void)handle;
  return @"status=-90 feature-disabled";
#endif
}

+ (NSString *)evalRuntimeGpuAuthorStylesheetsFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_author_stylesheets_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C0411_EVAL", report);
  return report;
#else
  (void)handle;
  return @"status=-90 feature-disabled";
#endif
}

+ (NSString *)evalRuntimeGpuCustomPropertiesFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_custom_properties_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C051_EVAL", report);
  return report;
#else
  (void)handle;
  return @"status=-90 feature-disabled";
#endif
}

+ (NSString *)evalRuntimeGpuRegisteredPropertiesFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_registered_properties_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C052_EVAL", report);
  return report;
#else
  (void)handle;
  return @"status=-90 feature-disabled";
#endif
}

+ (NSString *)evalRuntimeGpuRuntimeResultCacheFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  if (handle == 0) return @"status=-1 runtime GPU host가 0입니다";
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_runtime_result_cache_fixture(
      host, 10000, output.data(), output.size());
  return [NSString stringWithFormat:@"status=%d %s", status, output.data()];
#else
  (void)handle;
  return @"status=-1 C04.10 GPU fixture 빌드가 비활성화되었습니다";
#endif
}

+ (NSString *)evalRuntimeGpuIncrementalRestyleFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  if (handle == 0) return @"status=-1 runtime GPU host가 0입니다";
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_incremental_restyle_fixture(
      host, 10000, output.data(), output.size());
  return [NSString stringWithFormat:@"status=%d %s", status, output.data()];
#else
  (void)handle;
  return @"status=-1 C04.10 GPU fixture 빌드가 비활성화되었습니다";
#endif
}

+ (NSString *)evalRuntimeGpuPercentageDimensionsFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  if (handle == 0) return @"status=-1 runtime GPU host가 0입니다";
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_percentage_dimensions_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C061_EVAL", report);
  return report;
#else
  (void)handle;
  return @"status=-1 C04.10 GPU fixture 빌드가 비활성화되었습니다";
#endif
}

+ (NSString *)evalRuntimeGpuSpacingPercentagesFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  if (handle == 0) return @"status=-1 runtime GPU host가 0입니다";
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_spacing_percentages_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C062_EVAL", report);
  return report;
#else
  (void)handle;
  return @"status=-1 C04.10 GPU fixture 빌드가 비활성화되었습니다";
#endif
}

+ (NSString *)evalRuntimeGpuAbsoluteLengthsFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  if (handle == 0) return @"status=-1 runtime GPU host가 0입니다";
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_absolute_lengths_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C063_EVAL", report);
  return report;
#else
  (void)handle;
  return @"status=-1 C04.10 GPU fixture 빌드가 비활성화되었습니다";
#endif
}

+ (NSString *)evalRuntimeGpuFontRelativeUnitsFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  if (handle == 0) return @"status=-1 runtime GPU host가 0입니다";
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_font_relative_units_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C064_EVAL", report);
  return report;
#else
  (void)handle;
  return @"status=-1 C04.10 GPU fixture 빌드가 비활성화되었습니다";
#endif
}

+ (NSString *)evalRuntimeGpuTypedCssMathFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  if (handle == 0) return @"status=-1 runtime GPU host가 0입니다";
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_typed_css_math_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C065_EVAL", report);
  return report;
#else
  (void)handle;
  return @"status=-1 C04.10 GPU fixture 빌드가 비활성화되었습니다";
#endif
}

+ (NSString *)evalRuntimeGpuViewportUnitsFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  if (handle == 0) return @"status=-1 runtime GPU host가 0입니다";
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_viewport_units_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C066A_EVAL", report);
  return report;
#else
  (void)handle;
  return @"status=-1 C04.10 GPU fixture 빌드가 비활성화되었습니다";
#endif
}

+ (NSString *)evalRuntimeGpuMinMaxSizingFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  if (handle == 0) return @"status=-1 runtime GPU host가 0입니다";
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_min_max_sizing_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C071_EVAL", report);
  return report;
#else
  (void)handle;
  return @"status=-1 C04.10 GPU fixture 빌드가 비활성화되었습니다";
#endif
}

+ (NSString *)evalRuntimeGpuBorderWidthFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  if (handle == 0) return @"status=-1 runtime GPU host가 0입니다";
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_border_width_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C072_EVAL", report);
  return report;
#else
  (void)handle;
  return @"status=-1 C04.10 GPU fixture 빌드가 비활성화되었습니다";
#endif
}

+ (NSString *)evalRuntimeGpuAspectRatioFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  if (handle == 0) return @"status=-1 runtime GPU host가 0입니다";
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_aspect_ratio_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C073_EVAL", report);
  return report;
#else
  (void)handle;
  return @"status=-1 C04.10 GPU fixture 빌드가 비활성화되었습니다";
#endif
}

+ (NSString *)evalRuntimeGpuBlockPaintFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  if (handle == 0) return @"status=-1 runtime GPU host가 0입니다";
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_block_paint_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C08_EVAL", report);
  return report;
#else
  (void)handle;
  return @"status=-1 C04.10 GPU fixture 빌드가 비활성화되었습니다";
#endif
}

+ (NSString *)evalRuntimeGpuBlockFormattingFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  if (handle == 0) return @"status=-1 runtime GPU host가 0입니다";
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_block_formatting_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C091_EVAL", report);
  return report;
#else
  (void)handle;
  return @"status=-1 C04.10 GPU fixture 빌드가 비활성화되었습니다";
#endif
}

+ (NSString *)evalRuntimeGpuC12_1PositionFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c12_1_position_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C121_EVAL", report);
  return report;
#else
  (void)handle;
  return @"status=-90 feature-disabled";
#endif
}

+ (NSString *)evalRuntimeGpuC12_2AbsoluteBlockFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c12_2_absolute_block_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C122_EVAL", report);
  return report;
#else
  (void)handle;
  return @"status=-90 feature-disabled";
#endif
}

+ (NSString *)evalRuntimeGpuC10_3_5PositionedFlexFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 32768> output{};
  const int32_t status =
      spinon_runtime_gpu_host_eval_c10_3_5_positioned_flex_fixture(
          host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C1035_EVAL", report);
  return report;
#else
  (void)handle;
  return @"status=-90 feature-disabled";
#endif
}

+ (NSString *)evalRuntimeGpuC12_1PositionState:(uint64_t)handle state:(uint32_t)state {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c12_1_position_state(
      host, state, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C121_STATE", report);
  return report;
#else
  (void)handle;
  (void)state;
  return @"status=-90 feature-disabled";
#endif
}

+ (NSString *)evalRuntimeGpuMarginCollapseFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  if (handle == 0) return @"status=-1 runtime GPU host가 0입니다";
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_margin_collapse_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C092_EVAL", report);
  return report;
#else
  (void)handle;
  return @"status=-1 C04.10 GPU fixture 빌드가 비활성화되었습니다";
#endif
}

+ (NSString *)evalRuntimeGpuFlowRootFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  if (handle == 0) return @"status=-1 runtime GPU host가 0입니다";
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_flow_root_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C093_EVAL", report);
  return report;
#else
  (void)handle;
  return @"status=-1 C04.10 GPU fixture 빌드가 비활성화되었습니다";
#endif
}

+ (NSString *)evalRuntimeGpuFlexWrapFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  if (handle == 0) return @"status=-1 runtime GPU host가 0입니다";
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_flex_wrap_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C101_EVAL", report);
  if (status == 0) C10LogNodeFrames(report, "SPINON_C101_NODE_FRAME");
  return report;
#else
  (void)handle;
  return @"status=-1 C04.10 GPU fixture 빌드가 비활성화되었습니다";
#endif
}

+ (NSString *)evalRuntimeGpuFlexDistributionFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  if (handle == 0) return @"status=-1 runtime GPU host가 0입니다";
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_flex_distribution_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C102_EVAL", report);
  if (status == 0) {
    const NSUInteger frameCount = C10LogNodeFrames(report, "SPINON_C102_NODE_FRAME");
    NSString *summary = [NSString stringWithFormat:
        @"frames=%lu marker=%@ report_length=%lu", (unsigned long)frameCount,
        [report containsString:@"node_frames_css_px=["] ? @"present" : @"missing",
        (unsigned long)report.length];
    C0410LogReport(OS_LOG_TYPE_INFO, "SPINON_C102_FRAME_SUMMARY", summary);
  }
  return report;
#else
  (void)handle;
  return @"status=-1 C04.10 GPU fixture 빌드가 비활성화되었습니다";
#endif
}

+ (NSString *)evalRuntimeGpuFlexReverseFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  if (handle == 0) return @"status=-1 runtime GPU host가 0입니다";
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_flex_reverse_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C1031_EVAL", report);
  if (status == 0) {
    const NSUInteger frameCount = C10LogNodeFrames(report, "SPINON_C1031_NODE_FRAME");
    NSString *summary = [NSString stringWithFormat:
        @"frames=%lu marker=%@ report_length=%lu", (unsigned long)frameCount,
        [report containsString:@"node_frames_css_px=["] ? @"present" : @"missing",
        (unsigned long)report.length];
    C0410LogReport(OS_LOG_TYPE_INFO, "SPINON_C1031_FRAME_SUMMARY", summary);
  }
  return report;
#else
  (void)handle;
  return @"status=-1 C04.10 GPU fixture 빌드가 비활성화되었습니다";
#endif
}

+ (NSString *)evalRuntimeGpuFlexOrderFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  if (handle == 0) return @"status=-1 runtime GPU host가 0입니다";
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_flex_order_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C1032_EVAL", report);
  if (status == 0) {
    const NSUInteger frameCount = C10LogNodeFrames(report, "SPINON_C1032_NODE_FRAME");
    NSString *summary = [NSString stringWithFormat:
        @"frames=%lu marker=%@ report_length=%lu", (unsigned long)frameCount,
        [report containsString:@"node_frames_css_px=["] ? @"present" : @"missing",
        (unsigned long)report.length];
    C0410LogReport(OS_LOG_TYPE_INFO, "SPINON_C1032_FRAME_SUMMARY", summary);
  }
  return report;
#else
  (void)handle;
  return @"status=-1 C04.10 GPU fixture 빌드가 비활성화되었습니다";
#endif
}

+ (NSString *)evalRuntimeGpuFlexAlignmentFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  if (handle == 0) return @"status=-1 runtime GPU host가 0입니다";
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_flex_alignment_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C1033_EVAL", report);
  if (status == 0) {
    const NSUInteger frameCount = C10LogNodeFrames(report, "SPINON_C1033_NODE_FRAME");
    NSString *summary = [NSString stringWithFormat:
        @"frames=%lu marker=%@ report_length=%lu", (unsigned long)frameCount,
        [report containsString:@"node_frames_css_px=["] ? @"present" : @"missing",
        (unsigned long)report.length];
    C0410LogReport(OS_LOG_TYPE_INFO, "SPINON_C1033_FRAME_SUMMARY", summary);
  }
  return report;
#else
  (void)handle;
  return @"status=-1 C04.10 GPU fixture 빌드가 비활성화되었습니다";
#endif
}

+ (NSString *)evalRuntimeGpuFlexBaselineFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  if (handle == 0) return @"status=-1 runtime GPU host가 0입니다";
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 4096> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_flex_baseline_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C1034_EVAL", report);
  if (status == 0) {
    const NSUInteger frameCount = C10LogNodeFrames(report, "SPINON_C1034_NODE_FRAME");
    NSString *summary = [NSString stringWithFormat:
        @"frames=%lu marker=%@ report_length=%lu", (unsigned long)frameCount,
        [report containsString:@"node_frames_css_px=["] ? @"present" : @"missing",
        (unsigned long)report.length];
    C0410LogReport(OS_LOG_TYPE_INFO, "SPINON_C1034_FRAME_SUMMARY", summary);
  }
  return report;
#else
  (void)handle;
  return @"status=-1 C04.10 GPU fixture 빌드가 비활성화되었습니다";
#endif
}

+ (NSString *)prepareRuntimeGpuWgpuSurface:(uint64_t)handle view:(void *)view {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  if (![NSThread isMainThread]) {
    return @"status=-1 UIKit surface preparation requires the main thread";
  }
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 2048> output{};
  const int32_t status = spinon_runtime_gpu_host_prepare_uikit_surface(
      host, view, SPINON_WGPU_R08_METAL, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C0410_SURFACE_PREPARE", report);
  return report;
#else
  (void)handle; (void)view;
  return @"status=-90 feature-disabled";
#endif
}

+ (NSString *)createRuntimeGpuWgpu:(uint64_t)handle width:(uint32_t)width
                             height:(uint32_t)height {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 2048> output{};
  const int32_t status = spinon_runtime_gpu_host_create_uikit(
      host, width, height, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C0410_RENDERER", report);
  return report;
#else
  (void)handle; (void)width; (void)height;
  return @"status=-90 feature-disabled";
#endif
}

+ (NSString *)configureRuntimeGpuWgpuSurface:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  if (![NSThread isMainThread]) {
    return @"status=-1 UIKit surface configuration requires the main thread";
  }
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 2048> output{};
  const int32_t status = spinon_runtime_gpu_host_configure_uikit_surface(
      host, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C0410_SURFACE_CONFIGURE", report);
  return report;
#else
  (void)handle;
  return @"status=-90 feature-disabled";
#endif
}

+ (int32_t)resizeRuntimeGpuWgpu:(uint64_t)handle width:(uint32_t)width
                          height:(uint32_t)height {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  if (![NSThread isMainThread]) return -1;
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 1024> output{};
  const int32_t status = spinon_runtime_gpu_host_resize(
      host, width, height, output.data(), output.size());
  if (status != 0) {
    os_log_error(OS_LOG_DEFAULT,
                 "SPINON_C0410_RESIZE status=%{public}d %{public}s", status,
                 output.data());
  }
  return status;
#else
  (void)handle; (void)width; (void)height;
  return -90;
#endif
}

+ (NSString *)drawRuntimeGpuWgpu:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 1024> output{};
  const int32_t status = spinon_runtime_gpu_host_draw(
      host, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C0410_DRAW", report);
  return report;
#else
  (void)handle;
  return @"status=-90 feature-disabled";
#endif
}

+ (NSString *)injectNextRuntimeGpuDrawFailure:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C04_RUNTIME_GPU_FAILURE_FIXTURE) && \
    SPINON_ENABLE_C04_RUNTIME_GPU_FAILURE_FIXTURE
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 1024> output{};
  const int32_t status = spinon_runtime_gpu_host_inject_next_draw_failure_for_test(
      host, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C0410_DRAW_FAILURE_HOOK", report);
  return report;
#else
  (void)handle;
  return @"status=-90 failure fixture is not enabled in this build";
#endif
}

+ (void)destroyRuntimeGpuRenderer:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  std::array<char, 1024> output{};
  const int32_t status = spinon_runtime_gpu_host_destroy_renderer(
      reinterpret_cast<SpinonRuntimeGpuHost *>(static_cast<uintptr_t>(handle)),
      output.data(), output.size());
  os_log_with_type(OS_LOG_DEFAULT,
                   status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                   "SPINON_C0410_RENDERER_DESTROY status=%{public}d %{public}s",
                   status, output.data());
#else
  (void)handle;
#endif
}

+ (void)freeRuntimeGpuHost:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  spinon_runtime_gpu_host_free(reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle)));
#else
  (void)handle;
#endif
}

@end
