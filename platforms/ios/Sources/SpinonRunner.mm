#import "SpinonRunner.h"

#include <array>
#include <cstdint>
#include <cstring>
#include <memory>
#include <new>

#include "spinon_ffi.h"
#include "spinon_wgpu_r08.h"

#import <os/log.h>

#include <errno.h>
#include <pthread.h>
#include <sys/mman.h>
#include <unistd.h>

extern "C" int spinon_c124_capture_current_thread_stack(
    uint64_t *thread_id, uintptr_t *start_address, size_t *stack_size,
    size_t *guard_size, size_t *page_size, int *error_code) {
  if (thread_id == nullptr || start_address == nullptr || stack_size == nullptr ||
      guard_size == nullptr || page_size == nullptr || error_code == nullptr) {
    return -1;
  }
  *error_code = 0;
  const pthread_t thread = pthread_self();
  uint64_t observed_thread_id = 0;
  const int thread_status = pthread_threadid_np(thread, &observed_thread_id);
  const uintptr_t stack_top = reinterpret_cast<uintptr_t>(pthread_get_stackaddr_np(thread));
  const size_t observed_stack_size = pthread_get_stacksize_np(thread);
  const long observed_page_size = sysconf(_SC_PAGESIZE);
  if (thread_status != 0 || stack_top < observed_stack_size ||
      observed_stack_size == 0 || observed_page_size <= 0) {
    *error_code = thread_status != 0 ? thread_status :
        (errno != 0 ? errno : EINVAL);
    return -1;
  }
  *thread_id = observed_thread_id;
  *start_address = stack_top - observed_stack_size;
  *stack_size = observed_stack_size;
  *guard_size = 0;
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
                             stack_size, reinterpret_cast<char *>(page_flags));
  if (status != 0) *error_code = errno;
  return status;
}

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
                                            options:0
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

#if defined(SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST) && \
    SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST
NSUInteger C131LogHitTestSamples(NSString *report, NSUInteger expectedCount,
                                 const char *sampleLabel,
                                 const char *summaryLabel) {
  NSString *marker = @" hit_test_samples_json=[";
  NSRange markerRange = [report rangeOfString:marker];
  if (markerRange.location == NSNotFound) {
    C0410LogReport(OS_LOG_TYPE_ERROR, "SPINON_C131_HIT_TEST_ERROR",
                   @"marker=missing");
    return 0;
  }
  NSUInteger cursor = NSMaxRange(markerRange);
  NSRange closingRange = [report rangeOfString:@"]"
                                            options:NSBackwardsSearch
                                              range:NSMakeRange(cursor,
                                                                report.length - cursor)];
  if (closingRange.location == NSNotFound) {
    C0410LogReport(OS_LOG_TYPE_ERROR, "SPINON_C131_HIT_TEST_ERROR",
                   @"closing bracket is missing");
    return 0;
  }

  NSUInteger count = 0;
  NSUInteger matched = 0;
  while (cursor < closingRange.location) {
    NSRange objectStart = [report rangeOfString:@"{"
                                        options:0
                                          range:NSMakeRange(cursor,
                                                            closingRange.location - cursor)];
    if (objectStart.location == NSNotFound) break;
    NSRange objectEnd = [report rangeOfString:@"}"
                                      options:0
                                        range:NSMakeRange(NSMaxRange(objectStart),
                                                          closingRange.location - NSMaxRange(objectStart))];
    if (objectEnd.location == NSNotFound) {
      C0410LogReport(OS_LOG_TYPE_ERROR, "SPINON_C131_HIT_TEST_ERROR",
                     @"sample object is incomplete");
      return count;
    }
    NSString *sample = [report substringWithRange:
        NSMakeRange(objectStart.location, NSMaxRange(objectEnd) - objectStart.location)];
    C0410LogReport(OS_LOG_TYPE_INFO, sampleLabel, sample);
    count += 1;
    if ([sample containsString:@"\"status\":\"matched\""]) matched += 1;
    cursor = NSMaxRange(objectEnd);
  }
  NSString *summary = [NSString stringWithFormat:
      @"status=%@ matched=%lu total=%lu",
      matched == expectedCount && count == expectedCount ? @"ok" : @"error",
      (unsigned long)matched, (unsigned long)count];
  C0410LogReport(OS_LOG_TYPE_INFO, summaryLabel, summary);
  return count;
}
#endif

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
#if defined(SPINON_ENABLE_C04_UA_CASCADE_FIXTURE_PROBE) && \
    SPINON_ENABLE_C04_UA_CASCADE_FIXTURE_PROBE
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
#else
  return @"status=-8 ua_cascade_probe=DISABLED enable the internal fixture probe build flag";
#endif
}

+ (NSString *)runC141IntrinsicSizeFixture {
#if defined(SPINON_ENABLE_C14_1_APP_FIXTURE_TEST) && \
    SPINON_ENABLE_C14_1_APP_FIXTURE_TEST
  std::array<char, 4096> output{};
  const int32_t status = spinon_c14_1_app_intrinsic_size_fixture(
      output.data(), output.size());
  NSString *message = [NSString stringWithUTF8String:output.data()];
  if (status == 0) {
    os_log(OS_LOG_DEFAULT, "SPINON_C141_APP_FIXTURE %{public}s",
           message.UTF8String ?: "empty report");
  } else {
    os_log_error(OS_LOG_DEFAULT,
                 "SPINON_C141_APP_FIXTURE_ERROR status=%{public}d report=%{public}s",
                 status, message.UTF8String ?: "empty report");
  }
  return [NSString stringWithFormat:@"status=%d report=%@", status,
                                    message ?: @"empty report"];
#else
  return @"status=-8 report={\"status\":\"disabled\",\"detail\":\"fixture feature is off\"}";
#endif
}

+ (NSString *)runC142ImageRatioFixture {
#if defined(SPINON_ENABLE_C14_2_APP_FIXTURE_TEST) && \
    SPINON_ENABLE_C14_2_APP_FIXTURE_TEST
  std::array<char, 65536> output{};
  const int32_t status =
      spinon_c14_2_app_image_ratio_fixture(output.data(), output.size());
  NSString *message = [NSString stringWithUTF8String:output.data()];
  if (message == nil) message = @"empty report";
  const NSUInteger chunkSize = 600;
  const NSUInteger chunkCount =
      MAX(1, (message.length + chunkSize - 1) / chunkSize);
  for (NSUInteger chunk = 0; chunk < chunkCount; chunk++) {
    const NSUInteger offset = chunk * chunkSize;
    const NSUInteger length = MIN(chunkSize, message.length - offset);
    NSString *entry = [NSString
        stringWithFormat:@"SPINON_C142_APP_FIXTURE part=%lu/%lu payload=%@",
                         (unsigned long)(chunk + 1),
                         (unsigned long)chunkCount,
                         [message substringWithRange:NSMakeRange(offset, length)]];
    if (status == 0) {
      os_log(OS_LOG_DEFAULT, "%{public}s", entry.UTF8String ?: "empty report");
    } else {
      os_log_error(OS_LOG_DEFAULT, "%{public}s", entry.UTF8String ?: "empty report");
    }
  }
  return [NSString stringWithFormat:@"status=%d report=%@", status, message];
#else
  return @"status=-8 report={\"status\":\"disabled\",\"detail\":\"fixture feature is off\"}";
#endif
}

+ (NSString *)runC143IntrinsicContributionsFixture {
#if defined(SPINON_ENABLE_C14_3_APP_FIXTURE_TEST) && \
    SPINON_ENABLE_C14_3_APP_FIXTURE_TEST
  std::array<char, 8192> output{};
  const int32_t status = spinon_c14_3_app_intrinsic_contributions_fixture(
      output.data(), output.size());
  NSString *message = [NSString stringWithUTF8String:output.data()];
  if (message == nil) message = @"empty report";
  const NSUInteger chunkSize = 600;
  const NSUInteger chunkCount =
      MAX(1, (message.length + chunkSize - 1) / chunkSize);
  for (NSUInteger chunk = 0; chunk < chunkCount; chunk++) {
    const NSUInteger offset = chunk * chunkSize;
    const NSUInteger length = MIN(chunkSize, message.length - offset);
    NSString *entry = [NSString
        stringWithFormat:@"SPINON_C143_APP_FIXTURE part=%lu/%lu payload=%@",
                         (unsigned long)(chunk + 1),
                         (unsigned long)chunkCount,
                         [message substringWithRange:NSMakeRange(offset, length)]];
    if (status == 0) {
      os_log(OS_LOG_DEFAULT, "%{public}s", entry.UTF8String ?: "empty report");
    } else {
      os_log_error(OS_LOG_DEFAULT, "%{public}s", entry.UTF8String ?: "empty report");
    }
  }
  return [NSString stringWithFormat:@"status=%d report=%@", status,
                                    message ?: @"empty report"];
#else
  return @"status=-8 report={\"status\":\"disabled\",\"detail\":\"fixture feature is off\"}";
#endif
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

+ (uint64_t)createRuntimeGpuHostWithC12_3FixedFixture {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host = spinon_runtime_gpu_host_new_c12_3_fixed_fixture(
      output.data(), output.size());
  if (host == nullptr) {
    os_log_error(OS_LOG_DEFAULT, "SPINON_C123_HOST_ERROR=%{public}s", output.data());
    return 0;
  }
  os_log(OS_LOG_DEFAULT, "SPINON_C123_HOST=%{public}s", output.data());
  return static_cast<uint64_t>(reinterpret_cast<uintptr_t>(host));
#else
  os_log_error(OS_LOG_DEFAULT,
               "SPINON_C123_DISABLED rebuild with SPINON_ENABLE_C04_RUNTIME_GPU=1");
  return 0;
#endif
}

+ (uint64_t)createRuntimeGpuHostWithC12_4StackingFixture {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host = spinon_runtime_gpu_host_new_c12_4_stacking_fixture(
      output.data(), output.size());
  if (host == nullptr) {
    os_log_error(OS_LOG_DEFAULT, "SPINON_C124_HOST_ERROR=%{public}s", output.data());
    return 0;
  }
  os_log(OS_LOG_DEFAULT, "SPINON_C124_HOST=%{public}s", output.data());
  return static_cast<uint64_t>(reinterpret_cast<uintptr_t>(host));
#else
  os_log_error(OS_LOG_DEFAULT,
               "SPINON_C124_DISABLED rebuild with SPINON_ENABLE_C04_RUNTIME_GPU=1");
  return 0;
#endif
}

+ (uint64_t)createRuntimeGpuHostWithC13_1OverflowClipFixture {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST) && \
    SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c13_1_overflow_clip_fixture(
          output.data(), output.size());
  if (host == nullptr) {
    os_log_error(OS_LOG_DEFAULT, "SPINON_C131_HOST_ERROR=%{public}s", output.data());
    return 0;
  }
  os_log(OS_LOG_DEFAULT, "SPINON_C131_HOST=%{public}s", output.data());
  return static_cast<uint64_t>(reinterpret_cast<uintptr_t>(host));
#else
  os_log_error(OS_LOG_DEFAULT,
               "SPINON_C131_DISABLED enable C04 runtime GPU and C13.1 fixture test");
  return 0;
#endif
}

+ (uint64_t)createRuntimeGpuHostWithC13_1CompactSurfaceMatrixFixture {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST) && \
    SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c13_1_compact_surface_matrix_fixture(
          output.data(), output.size());
  if (host == nullptr) {
    os_log_error(OS_LOG_DEFAULT, "SPINON_C131_COMPACT_HOST_ERROR=%{public}s", output.data());
    return 0;
  }
  os_log(OS_LOG_DEFAULT, "SPINON_C131_COMPACT_HOST=%{public}s", output.data());
  return static_cast<uint64_t>(reinterpret_cast<uintptr_t>(host));
#else
  os_log_error(OS_LOG_DEFAULT,
               "SPINON_C131_COMPACT_DISABLED enable C04 runtime GPU and C13.1 fixture test");
  return 0;
#endif
}

+ (uint64_t)createRuntimeGpuHostWithC13_1CompactSurfacePixelReadbackFixture {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST) && \
    SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c13_1_compact_surface_pixel_readback_fixture(
          output.data(), output.size());
  if (host == nullptr) {
    os_log_error(OS_LOG_DEFAULT, "SPINON_C131_METAL_HOST_ERROR=%{public}s", output.data());
    return 0;
  }
  os_log(OS_LOG_DEFAULT, "SPINON_C131_METAL_HOST=%{public}s", output.data());
  return static_cast<uint64_t>(reinterpret_cast<uintptr_t>(host));
#else
  os_log_error(OS_LOG_DEFAULT,
               "SPINON_C131_METAL_DISABLED enable C04 runtime GPU and C13.1 fixture test");
  return 0;
#endif
}

+ (uint64_t)createRuntimeGpuHostWithC13_1OverflowClipBorderFixture {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST) && \
    SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c13_1_overflow_clip_border_fixture(
          output.data(), output.size());
  if (host == nullptr) {
    os_log_error(OS_LOG_DEFAULT, "SPINON_C131_BORDER_HOST_ERROR=%{public}s", output.data());
    return 0;
  }
  os_log(OS_LOG_DEFAULT, "SPINON_C131_BORDER_HOST=%{public}s", output.data());
  return static_cast<uint64_t>(reinterpret_cast<uintptr_t>(host));
#else
  os_log_error(OS_LOG_DEFAULT,
               "SPINON_C131_BORDER_DISABLED enable C04 runtime GPU and C13.1 fixture test");
  return 0;
#endif
}

+ (uint64_t)createRuntimeGpuHostWithC13_2ScrollFixture {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C132_SCROLL_FIXTURE_TEST) && \
    SPINON_ENABLE_C132_SCROLL_FIXTURE_TEST
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host = spinon_runtime_gpu_host_new_c13_2_scroll_fixture(
      output.data(), output.size());
  if (host == nullptr) {
    os_log_error(OS_LOG_DEFAULT, "SPINON_C132_HOST_ERROR=%{public}s", output.data());
    return 0;
  }
  os_log(OS_LOG_DEFAULT, "SPINON_C132_HOST=%{public}s", output.data());
  return static_cast<uint64_t>(reinterpret_cast<uintptr_t>(host));
#else
  os_log_error(OS_LOG_DEFAULT,
               "SPINON_C132_DISABLED enable C04 runtime GPU and C13.2 fixture test");
  return 0;
#endif
}

+ (uint64_t)createRuntimeGpuHostWithC13_2SurfacePixelReadbackFixture {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C132_SCROLL_FIXTURE_TEST) && \
    SPINON_ENABLE_C132_SCROLL_FIXTURE_TEST
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c13_2_scroll_surface_pixel_readback_fixture(
          output.data(), output.size());
  if (host == nullptr) {
    os_log_error(OS_LOG_DEFAULT, "SPINON_C132_METAL_PIXEL_HOST_ERROR=%{public}s",
                 output.data());
    return 0;
  }
  os_log(OS_LOG_DEFAULT, "SPINON_C132_METAL_PIXEL_HOST=%{public}s", output.data());
  return static_cast<uint64_t>(reinterpret_cast<uintptr_t>(host));
#else
  os_log_error(OS_LOG_DEFAULT,
               "SPINON_C132_METAL_PIXEL_DISABLED enable C04 runtime GPU and C13.2 fixture test");
  return 0;
#endif
}

#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST
+ (uint64_t)createRuntimeGpuHostWithC18_1ImagePaintFixture {
  std::array<char, 2048> output{};
  SpinonRuntimeGpuHost *host = spinon_runtime_gpu_host_new_c18_1_image_paint_fixture(
      output.data(), output.size());
  if (host == nullptr) {
    os_log_error(OS_LOG_DEFAULT, "SPINON_C181_HOST_ERROR=%{public}s", output.data());
    return 0;
  }
  os_log(OS_LOG_DEFAULT, "SPINON_C181_HOST=%{public}s", output.data());
  return static_cast<uint64_t>(reinterpret_cast<uintptr_t>(host));
}
#endif

#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST
+ (uint64_t)createRuntimeGpuHostWithC18_2AlphaImageFixture {
  std::array<char, 2048> output{};
  SpinonRuntimeGpuHost *host = spinon_runtime_gpu_host_new_c18_2_alpha_image_fixture(
      output.data(), output.size());
  if (host == nullptr) {
    NSLog(@"SPINON_C182_HOST_ERROR=%s", output.data());
    return 0;
  }
  NSLog(@"SPINON_C182_HOST=%s", output.data());
  return reinterpret_cast<uint64_t>(host);
}
#endif

#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C18_3_OBJECT_FIT_POSITION_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_3_OBJECT_FIT_POSITION_FIXTURE_TEST
+ (uint64_t)createRuntimeGpuHostWithC18_3ObjectFitPositionFixture {
  std::array<char, 2048> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c18_3_object_fit_position_fixture(
          output.data(), output.size());
  if (host == nullptr) {
    NSLog(@"SPINON_C183_HOST_ERROR=%s", output.data());
    return 0;
  }
  NSLog(@"SPINON_C183_HOST=%s", output.data());
  return reinterpret_cast<uint64_t>(host);
}
#endif

#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST
+ (uint64_t)createRuntimeGpuHostWithC18_4AsyncImageFixture {
  std::array<char, 2048> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c18_4_async_image_fixture(output.data(), output.size());
  if (host == nullptr) {
    NSLog(@"SPINON_C184_HOST_ERROR=%s", output.data());
    return 0;
  }
  NSLog(@"SPINON_C184_HOST=%s", output.data());
  return reinterpret_cast<uint64_t>(host);
}
#endif

#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C19_2_BOX_PAINT_OPACITY_FIXTURE_TEST) && \
    SPINON_ENABLE_C19_2_BOX_PAINT_OPACITY_FIXTURE_TEST
+ (uint64_t)createRuntimeGpuHostWithC19_2BoxPaintOpacityFixture {
  std::array<char, 2048> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c19_2_box_paint_opacity_fixture(
          output.data(), output.size());
  if (host == nullptr) {
    NSLog(@"SPINON_C192_HOST_ERROR=%s", output.data());
    return 0;
  }
  NSLog(@"SPINON_C192_HOST=%s", output.data());
  return reinterpret_cast<uint64_t>(host);
}
#endif

#if defined(SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST) && SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST
+ (uint64_t)createRuntimeGpuHostWithC13_3GestureFixture:(int32_t)scenarioID {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST) && \
    SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c13_3_gesture_fixture(
          scenarioID, output.data(), output.size());
  if (host == nullptr) {
    os_log_error(OS_LOG_DEFAULT, "SPINON_C133_HOST_ERROR=%{public}s", output.data());
    return 0;
  }
  os_log(OS_LOG_DEFAULT, "SPINON_C133_HOST=%{public}s", output.data());
  return static_cast<uint64_t>(reinterpret_cast<uintptr_t>(host));
#else
  os_log_error(OS_LOG_DEFAULT,
               "SPINON_C133_DISABLED enable C04 runtime GPU and C13.3 gesture fixture test");
  return 0;
#endif
}
#endif

+ (uint64_t)createRuntimeGpuHostWithC11_1GridFixture {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host = spinon_runtime_gpu_host_new_c11_1_grid_fixture(
      output.data(), output.size());
  if (host == nullptr) {
    os_log_error(OS_LOG_DEFAULT, "SPINON_C111_GRID_HOST_ERROR=%{public}s", output.data());
    return 0;
  }
  os_log(OS_LOG_DEFAULT, "SPINON_C111_GRID_HOST=%{public}s", output.data());
  return static_cast<uint64_t>(reinterpret_cast<uintptr_t>(host));
#else
  os_log_error(OS_LOG_DEFAULT,
               "SPINON_C111_GRID_DISABLED rebuild with SPINON_ENABLE_C04_RUNTIME_GPU=1");
  return 0;
#endif
}

+ (uint64_t)createRuntimeGpuHostWithC11_2_1GridPlacementFixture {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c11_2_1_grid_placement_fixture(
          output.data(), output.size());
  if (host == nullptr) {
    os_log_error(OS_LOG_DEFAULT, "SPINON_C1121_GRID_HOST_ERROR=%{public}s",
                 output.data());
    return 0;
  }
  os_log(OS_LOG_DEFAULT, "SPINON_C1121_GRID_HOST=%{public}s", output.data());
  return static_cast<uint64_t>(reinterpret_cast<uintptr_t>(host));
#else
  os_log_error(OS_LOG_DEFAULT,
               "SPINON_C1121_GRID_DISABLED rebuild with SPINON_ENABLE_C04_RUNTIME_GPU=1");
  return 0;
#endif
}

+ (uint64_t)createRuntimeGpuHostWithC11_2_2GridAutoPlacementFixture {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c11_2_2_grid_auto_placement_fixture(
          output.data(), output.size());
  if (host == nullptr) {
    os_log_error(OS_LOG_DEFAULT, "SPINON_C1122_GRID_HOST_ERROR=%{public}s",
                 output.data());
    return 0;
  }
  os_log(OS_LOG_DEFAULT, "SPINON_C1122_GRID_HOST=%{public}s", output.data());
  return static_cast<uint64_t>(reinterpret_cast<uintptr_t>(host));
#else
  os_log_error(OS_LOG_DEFAULT,
               "SPINON_C1122_GRID_DISABLED rebuild with SPINON_ENABLE_C04_RUNTIME_GPU=1");
  return 0;
#endif
}

+ (uint64_t)createRuntimeGpuHostWithC11_2_3GridAutoPlacementFixture {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c11_2_3_grid_auto_placement_fixture(
          output.data(), output.size());
  if (host == nullptr) {
    os_log_error(OS_LOG_DEFAULT, "SPINON_C1123_GRID_HOST_ERROR=%{public}s",
                 output.data());
    return 0;
  }
  os_log(OS_LOG_DEFAULT, "SPINON_C1123_GRID_HOST=%{public}s", output.data());
  return static_cast<uint64_t>(reinterpret_cast<uintptr_t>(host));
#else
  os_log_error(OS_LOG_DEFAULT,
               "SPINON_C1123_GRID_DISABLED rebuild with SPINON_ENABLE_C04_RUNTIME_GPU=1");
  return 0;
#endif
}

+ (uint64_t)createRuntimeGpuHostWithC11_3_1GridRepeatFixture {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST) && \
    SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c11_3_1_grid_repeat_fixture(
          output.data(), output.size());
  if (host == nullptr) {
    os_log_error(OS_LOG_DEFAULT, "SPINON_C1131_GRID_HOST_ERROR=%{public}s",
                 output.data());
    return 0;
  }
  os_log(OS_LOG_DEFAULT, "SPINON_C1131_GRID_HOST=%{public}s", output.data());
  return static_cast<uint64_t>(reinterpret_cast<uintptr_t>(host));
#else
  os_log_error(OS_LOG_DEFAULT,
               "SPINON_C1131_GRID_DISABLED rebuild with SPINON_ENABLE_C04_RUNTIME_GPU=1");
  return 0;
#endif
}

+ (uint64_t)createRuntimeGpuHostWithC11_3_2GridNamedLinesFixture {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST) && \
    SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c11_3_2_grid_named_lines_fixture(
          output.data(), output.size());
  if (host == nullptr) {
    os_log_error(OS_LOG_DEFAULT, "SPINON_C1132_GRID_HOST_ERROR=%{public}s",
                 output.data());
    return 0;
  }
  os_log(OS_LOG_DEFAULT, "SPINON_C1132_GRID_HOST=%{public}s", output.data());
  return static_cast<uint64_t>(reinterpret_cast<uintptr_t>(host));
#else
  os_log_error(OS_LOG_DEFAULT,
               "SPINON_C1132_GRID_DISABLED rebuild with SPINON_ENABLE_C04_RUNTIME_GPU=1");
  return 0;
#endif
}

+ (uint64_t)createRuntimeGpuHostWithC11_3_3GridTemplateAreasFixture {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST) && \
    SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c11_3_3_grid_template_areas_fixture(
          output.data(), output.size());
  if (host == nullptr) {
    os_log_error(OS_LOG_DEFAULT, "SPINON_C1133_GRID_HOST_ERROR=%{public}s",
                 output.data());
    return 0;
  }
  os_log(OS_LOG_DEFAULT, "SPINON_C1133_GRID_HOST=%{public}s", output.data());
  return static_cast<uint64_t>(reinterpret_cast<uintptr_t>(host));
#else
  os_log_error(OS_LOG_DEFAULT,
               "SPINON_C1133_GRID_DISABLED rebuild with SPINON_ENABLE_C04_RUNTIME_GPU=1");
  return 0;
#endif
}

+ (uint64_t)createRuntimeGpuHostWithC11_1GridSurfacePixelReadbackFixture {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST) && \
    SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c11_1_grid_surface_pixel_readback_fixture(
          output.data(), output.size());
  if (host == nullptr) {
    os_log_error(OS_LOG_DEFAULT, "SPINON_C111_METAL_HOST_ERROR=%{public}s", output.data());
    return 0;
  }
  os_log(OS_LOG_DEFAULT, "SPINON_C111_METAL_HOST=%{public}s", output.data());
  return static_cast<uint64_t>(reinterpret_cast<uintptr_t>(host));
#else
  os_log_error(OS_LOG_DEFAULT,
               "SPINON_C111_METAL_DISABLED rebuild with the internal C11 pixel test flag");
  return 0;
#endif
}

+ (NSString *)runtimeGpuC12_4WorkerStackResidencySample:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C124_WORKER_STACK_RESIDENCY_TEST) && \
    SPINON_ENABLE_C124_WORKER_STACK_RESIDENCY_TEST
  std::array<char, 1024> output{};
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(static_cast<uintptr_t>(handle));
  const int32_t status = spinon_runtime_gpu_host_css_worker_stack_residency_for_test(
      host, output.data(), output.size());
  if (status != 0) {
    return [NSString stringWithFormat:
        @"{\"status\":\"error\",\"native_status\":%d,\"detail\":\"%s\"}",
        status, output.data()];
  }
  return [NSString stringWithUTF8String:output.data()];
#else
  return @"{\"status\":\"error\",\"detail\":\"probe disabled\"}";
#endif
}

+ (uint64_t)createRuntimeGpuHostWithC12_4SurfaceReadbackFixture {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C124_METAL_SURFACE_READBACK_TEST) && \
    SPINON_ENABLE_C124_METAL_SURFACE_READBACK_TEST
  std::array<char, 1024> output{};
  SpinonRuntimeGpuHost *host =
      spinon_runtime_gpu_host_new_c12_4_surface_readback_fixture(
          output.data(), output.size());
  if (host == nullptr) {
    os_log_error(OS_LOG_DEFAULT, "SPINON_C124_METAL_HOST_ERROR=%{public}s", output.data());
    return 0;
  }
  os_log(OS_LOG_DEFAULT, "SPINON_C124_METAL_HOST=%{public}s", output.data());
  return static_cast<uint64_t>(reinterpret_cast<uintptr_t>(host));
#else
  os_log_error(OS_LOG_DEFAULT,
               "SPINON_C124_METAL_DISABLED rebuild with SPINON_ENABLE_C124_METAL_SURFACE_READBACK_TEST=1");
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

+ (NSString *)evalRuntimeGpuC12_3FixedFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c12_3_fixed_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C123_EVAL", report);
  return report;
#else
  (void)handle;
  return @"status=-90 feature-disabled";
#endif
}

+ (NSString *)evalRuntimeGpuC12_4StackingFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c12_4_stacking_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C124_EVAL", report);
  return report;
#else
  (void)handle;
  return @"status=-90 feature-disabled";
#endif
}

+ (NSString *)evalRuntimeGpuC13_1OverflowClipFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST) && \
    SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c13_1_overflow_clip_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C131_EVAL", report);
  if (status == 0) {
    const NSUInteger frameCount = C10LogNodeFrames(report, "SPINON_C131_NODE_FRAME");
    const NSUInteger hitCount = C131LogHitTestSamples(
        report, 15, "SPINON_C131_HIT_TEST", "SPINON_C131_HIT_TEST_SUMMARY");
    NSString *summary = [NSString stringWithFormat:
        @"status=%@ boxes=%lu hit_samples=%lu",
        frameCount == 13 && hitCount == 15 ? @"ok" : @"error",
        (unsigned long)frameCount, (unsigned long)hitCount];
    C0410LogReport(frameCount == 13 && hitCount == 15
                       ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                   "SPINON_C131_FIXTURE_SUMMARY", summary);
  }
  return report;
#else
  (void)handle;
  return @"status=-90 feature-disabled";
#endif
}

#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST
+ (NSString *)evalRuntimeGpuC18_1ImagePaintFixture:(uint64_t)handle {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c18_1_image_paint_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C181_EVAL", report);
  return report;
}
#endif

#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST
+ (NSString *)evalRuntimeGpuC18_2AlphaImageFixture:(uint64_t)handle {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c18_2_alpha_image_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = [NSString stringWithFormat:@"status=%d %s", status, output.data()];
  NSLog(@"SPINON_C182_EVAL %@", report);
  return report;
}
#endif

#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C18_3_OBJECT_FIT_POSITION_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_3_OBJECT_FIT_POSITION_FIXTURE_TEST
+ (NSString *)evalRuntimeGpuC18_3ObjectFitPositionFixture:(uint64_t)handle {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 32768> output{};
  const int32_t status =
      spinon_runtime_gpu_host_eval_c18_3_object_fit_position_fixture(
          host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C183_EVAL", report);
  return report;
}
#endif

#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST
+ (NSString *)evalRuntimeGpuC18_4AsyncImageFixture:(uint64_t)handle {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c18_4_async_image_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C184_EVAL", report);
  return report;
}
#endif

#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C19_2_BOX_PAINT_OPACITY_FIXTURE_TEST) && \
    SPINON_ENABLE_C19_2_BOX_PAINT_OPACITY_FIXTURE_TEST
+ (NSString *)evalRuntimeGpuC19_2BoxPaintOpacityFixture:(uint64_t)handle {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c19_2_box_paint_opacity_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C192_EVAL", report);
  return report;
}
#endif

+ (NSString *)evalRuntimeGpuC13_1CompactSurfaceMatrixFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST) && \
    SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 32768> output{};
  const int32_t status =
      spinon_runtime_gpu_host_eval_c13_1_compact_surface_matrix_fixture(
          host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C131_COMPACT_EVAL", report);
  if (status == 0) {
    const NSUInteger frameCount = C10LogNodeFrames(report, "SPINON_C131_COMPACT_NODE_FRAME");
    const NSUInteger hitCount = C131LogHitTestSamples(
        report, 52, "SPINON_C131_COMPACT_HIT_TEST",
        "SPINON_C131_COMPACT_HIT_TEST_SUMMARY");
    NSString *summary = [NSString stringWithFormat:
        @"status=%@ boxes=%lu hit_samples=%lu",
        frameCount == 41 && hitCount == 52 ? @"ok" : @"error",
        (unsigned long)frameCount, (unsigned long)hitCount];
    C0410LogReport(frameCount == 41 && hitCount == 52
                       ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                   "SPINON_C131_COMPACT_FIXTURE_SUMMARY", summary);
  }
  return report;
#else
  (void)handle;
  return @"status=-90 feature-disabled";
#endif
}

+ (NSString *)requestRuntimeGpuC13_1SurfacePixelsForTest:(uint64_t)handle
                                                   sampleXY:(const uint32_t *)sampleXY
                                                sampleCount:(NSUInteger)sampleCount {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST) && \
    SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 1024> output{};
  const int32_t status = spinon_runtime_gpu_host_request_c13_1_surface_pixels_for_test(
      host, sampleXY, sampleCount, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C131_METAL_PIXEL_REQUEST", report);
  return report;
#else
  (void)handle;
  (void)sampleXY;
  (void)sampleCount;
  return @"status=-90 C13.1 Metal surface pixel fixture is not enabled";
#endif
}

+ (NSString *)evalRuntimeGpuC13_1OverflowClipBorderFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST) && \
    SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c13_1_overflow_clip_border_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C131_BORDER_EVAL", report);
  if (status == 0) {
    const NSUInteger frameCount = C10LogNodeFrames(report, "SPINON_C131_BORDER_NODE_FRAME");
    const NSUInteger hitCount = C131LogHitTestSamples(
        report, 6, "SPINON_C131_BORDER_HIT_TEST",
        "SPINON_C131_BORDER_HIT_TEST_SUMMARY");
    NSString *summary = [NSString stringWithFormat:
        @"status=%@ boxes=%lu hit_samples=%lu",
        frameCount == 5 && hitCount == 6 ? @"ok" : @"error",
        (unsigned long)frameCount, (unsigned long)hitCount];
    C0410LogReport(frameCount == 5 && hitCount == 6
                       ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                   "SPINON_C131_BORDER_FIXTURE_SUMMARY", summary);
  }
  return report;
#else
  (void)handle;
  return @"status=-90 feature-disabled";
#endif
}

+ (NSString *)evalRuntimeGpuC13_2ScrollFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C132_SCROLL_FIXTURE_TEST) && \
    SPINON_ENABLE_C132_SCROLL_FIXTURE_TEST
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c13_2_scroll_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C132_EVAL", report);
  return report;
#else
  (void)handle;
  return @"status=-90 feature-disabled";
#endif
}

#if defined(SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST) && SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST
+ (NSString *)evalRuntimeGpuC13_3GestureFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST) && \
    SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c13_3_gesture_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C133_EVAL", report);
  return report;
#else
  (void)handle;
  return @"status=-90 feature-disabled";
#endif
}

+ (NSString *)applyRuntimeGpuC13_3StyleInvalidation:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST) && \
    SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_apply_c13_3_style_invalidation(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C133_STYLE_UPDATE", report);
  return report;
#else
  (void)handle;
  return @"status=-90 feature-disabled";
#endif
}

+ (NSString *)enqueueRuntimeGpuC13_3Gesture:(uint64_t)handle
                                      action:(int32_t)action
                           surfaceGeneration:(uint64_t)surfaceGeneration
                                   gestureID:(uint64_t)gestureID
                                    pointerID:(uint64_t)pointerID
                                  pointXCssPx:(float)pointXCssPx
                                  pointYCssPx:(float)pointYCssPx
                             coordinateScale:(float)coordinateScale
                                       reason:(int32_t)reason {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST) && \
    SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 2048> output{};
  const int32_t status = spinon_runtime_gpu_host_enqueue_c13_3_gesture(
      host, action, surfaceGeneration, gestureID, pointerID,
      pointXCssPx, pointYCssPx, coordinateScale, reason,
      output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C133_ENQUEUE", report);
  return report;
#else
  (void)handle;
  (void)action;
  (void)surfaceGeneration;
  (void)gestureID;
  (void)pointerID;
  (void)pointXCssPx;
  (void)pointYCssPx;
  (void)coordinateScale;
  (void)reason;
  return @"status=-90 feature-disabled";
#endif
}

+ (NSString *)enqueueRuntimeGpuC13_3Wheel:(uint64_t)handle
                                    intent:(int32_t)intent
                         surfaceGeneration:(uint64_t)surfaceGeneration
                              pointXCssPx:(float)pointXCssPx
                              pointYCssPx:(float)pointYCssPx
                              deltaXCssPx:(float)deltaXCssPx
                              deltaYCssPx:(float)deltaYCssPx
                                rawDeltaX:(float)rawDeltaX
                                rawDeltaY:(float)rawDeltaY
                               sourceUnit:(int32_t)sourceUnit
                          coordinateScale:(float)coordinateScale
                                   source:(int32_t)source
                            momentumPhase:(int32_t)momentumPhase
                            modifierFlags:(uint32_t)modifierFlags {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST) && \
    SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 2048> output{};
  const int32_t status = spinon_runtime_gpu_host_enqueue_c13_3_wheel(
      host, intent, surfaceGeneration, pointXCssPx, pointYCssPx,
      deltaXCssPx, deltaYCssPx, rawDeltaX, rawDeltaY, sourceUnit,
      coordinateScale, source, momentumPhase, modifierFlags,
      output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C133_WHEEL_ENQUEUE", report);
  return report;
#else
  (void)handle;
  (void)intent;
  (void)surfaceGeneration;
  (void)pointXCssPx;
  (void)pointYCssPx;
  (void)deltaXCssPx;
  (void)deltaYCssPx;
  (void)rawDeltaX;
  (void)rawDeltaY;
  (void)sourceUnit;
  (void)coordinateScale;
  (void)source;
  (void)momentumPhase;
  (void)modifierFlags;
  return @"status=-90 feature-disabled";
#endif
}

+ (NSString *)processRuntimeGpuC13_3GestureQueue:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST) && \
    SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_process_c13_3_gesture_queue(
      host, 64 + 8, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C133_INPUT_BATCH", report);
  return report;
#else
  (void)handle;
  return @"status=-90 feature-disabled";
#endif
}
#endif

+ (NSString *)requestRuntimeGpuC13_2SurfacePixels:(uint64_t)handle
                                             sampleXY:(const uint32_t *)sampleXY
                                          sampleCount:(NSUInteger)sampleCount {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C132_SCROLL_FIXTURE_TEST) && \
    SPINON_ENABLE_C132_SCROLL_FIXTURE_TEST
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 1024> output{};
  const int32_t status = spinon_runtime_gpu_host_request_c13_2_surface_pixels_for_test(
      host, sampleXY, sampleCount, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C132_METAL_PIXEL_REQUEST", report);
  return report;
#else
  (void)handle;
  (void)sampleXY;
  (void)sampleCount;
  return @"status=-90 feature-disabled";
#endif
}

#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST
+ (NSString *)requestRuntimeGpuC18_1SurfacePixelsForTest:(uint64_t)handle
                                                   sampleXY:(const uint32_t *)sampleXY
                                                sampleCount:(NSUInteger)sampleCount {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 1024> output{};
  const int32_t status = spinon_runtime_gpu_host_request_c18_1_surface_pixels_for_test(
      host, sampleXY, sampleCount, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C181_METAL_PIXEL_REQUEST", report);
  return report;
}
#endif

#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST
+ (NSString *)requestRuntimeGpuC18_2SurfacePixelsForTest:(uint64_t)handle
                                                   sampleXY:(const uint32_t *)sampleXY
                                                sampleCount:(NSUInteger)sampleCount {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 2048> output{};
  const int32_t status = spinon_runtime_gpu_host_request_c18_2_surface_pixels_for_test(
      host, sampleXY, sampleCount, output.data(), output.size());
  NSString *report = [NSString stringWithFormat:@"status=%d %s", status, output.data()];
  NSLog(@"SPINON_C182_METAL_PIXEL_REQUEST %@", report);
  return report;
}
#endif

#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C18_3_OBJECT_FIT_POSITION_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_3_OBJECT_FIT_POSITION_FIXTURE_TEST
+ (NSString *)requestRuntimeGpuC18_3SurfacePixelsForTest:(uint64_t)handle
                                                   sampleXY:(const uint32_t *)sampleXY
                                                sampleCount:(NSUInteger)sampleCount {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 2048> output{};
  const int32_t status = spinon_runtime_gpu_host_request_c18_3_surface_pixels_for_test(
      host, sampleXY, sampleCount, output.data(), output.size());
  NSString *report = [NSString stringWithFormat:@"status=%d %s", status, output.data()];
  NSLog(@"SPINON_C183_METAL_PIXEL_REQUEST %@", report);
  return report;
}
#endif

#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST
+ (NSString *)requestRuntimeGpuC18_4SurfacePixelsForTest:(uint64_t)handle
                                                   sampleXY:(const uint32_t *)sampleXY
                                                sampleCount:(NSUInteger)sampleCount {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 2048> output{};
  const int32_t status = spinon_runtime_gpu_host_request_c18_4_surface_pixels_for_test(
      host, sampleXY, sampleCount, output.data(), output.size());
  NSString *report = [NSString stringWithFormat:@"status=%d %s", status, output.data()];
  NSLog(@"SPINON_C184_METAL_PIXEL_REQUEST %@", report);
  return report;
}
#endif

#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C19_2_BOX_PAINT_OPACITY_FIXTURE_TEST) && \
    SPINON_ENABLE_C19_2_BOX_PAINT_OPACITY_FIXTURE_TEST
+ (NSString *)requestRuntimeGpuC19_2SurfacePixelsForTest:(uint64_t)handle
                                                   sampleXY:(const uint32_t *)sampleXY
                                                sampleCount:(NSUInteger)sampleCount {
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 2048> output{};
  const int32_t status = spinon_runtime_gpu_host_request_c19_2_surface_pixels_for_test(
      host, sampleXY, sampleCount, output.data(), output.size());
  NSString *report = [NSString stringWithFormat:@"status=%d %s", status, output.data()];
  NSLog(@"SPINON_C192_METAL_PIXEL_REQUEST %@", report);
  return report;
}
#endif

+ (NSString *)evalRuntimeGpuC11_1GridFixture:(uint64_t)handle {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c11_1_grid_fixture(
      host, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C111_GRID_EVAL", report);
  return report;
#else
  (void)handle;
  return @"status=-90 feature-disabled";
#endif
}

+ (NSString *)evalRuntimeGpuC11_1GridSceneFixture:(uint64_t)handle
                                       sceneIndex:(uint32_t)sceneIndex {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c11_1_grid_scene_fixture(
      host, sceneIndex, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C111_GRID_EVAL", report);
  if (status == 0) C10LogNodeFrames(report, "SPINON_C111_GRID_NODE_FRAME");
  return report;
#else
  (void)handle;
  (void)sceneIndex;
  return @"status=-90 feature-disabled";
#endif
}

+ (NSString *)evalRuntimeGpuC11_2_1GridPlacementFixture:(uint64_t)handle
                                                sceneID:(NSString *)sceneID
                                                  runID:(NSString *)runID {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  const char *scene_id = sceneID.UTF8String;
  if (scene_id == nullptr) return @"status=-1 invalid-scene-id";
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 32768> output{};
  const int32_t status = spinon_runtime_gpu_host_eval_c11_2_1_grid_placement_fixture(
      host, scene_id, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  NSString *runReport = [NSString stringWithFormat:@"run_id=%@ scene_id=%@ %@",
                                                   runID ?: @"missing", sceneID, report];
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C1121_GRID_EVAL", runReport);
  if (status == 0) C10LogNodeFrames(report, "SPINON_C1121_GRID_NODE_FRAME");
  return report;
#else
  (void)handle;
  (void)sceneID;
  (void)runID;
  return @"status=-90 feature-disabled";
#endif
}

+ (NSString *)evalRuntimeGpuC11_2_2GridAutoPlacementFixture:(uint64_t)handle
                                                    sceneID:(NSString *)sceneID
                                                      runID:(NSString *)runID {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  const char *scene_id = sceneID.UTF8String;
  if (scene_id == nullptr) return @"status=-1 invalid-scene-id";
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 32768> output{};
  const int32_t status =
      spinon_runtime_gpu_host_eval_c11_2_2_grid_auto_placement_fixture(
          host, scene_id, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  NSString *runReport = [NSString stringWithFormat:@"run_id=%@ scene_id=%@ %@",
                                                   runID ?: @"missing", sceneID, report];
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C1122_GRID_EVAL", runReport);
  if (status == 0) C10LogNodeFrames(report, "SPINON_C1122_GRID_NODE_FRAME");
  return report;
#else
  (void)handle;
  (void)sceneID;
  (void)runID;
  return @"status=-90 feature-disabled";
#endif
}

+ (NSString *)evalRuntimeGpuC11_2_3GridAutoPlacementFixture:(uint64_t)handle
                                                    sceneID:(NSString *)sceneID
                                                      runID:(NSString *)runID {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  const char *scene_id = sceneID.UTF8String;
  if (scene_id == nullptr) return @"status=-1 invalid-scene-id";
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 32768> output{};
  const int32_t status =
      spinon_runtime_gpu_host_eval_c11_2_3_grid_auto_placement_fixture(
          host, scene_id, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  NSString *runReport = [NSString stringWithFormat:@"run_id=%@ scene_id=%@ %@",
                                                   runID ?: @"missing", sceneID, report];
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C1123_GRID_EVAL", runReport);
  if (status == 0) C10LogNodeFrames(report, "SPINON_C1123_GRID_NODE_FRAME");
  return report;
#else
  (void)handle;
  (void)sceneID;
  (void)runID;
  return @"status=-90 feature-disabled";
#endif
}

+ (NSString *)evalRuntimeGpuC11_3_1GridRepeatFixture:(uint64_t)handle
                                               sceneID:(NSString *)sceneID
                                                 runID:(NSString *)runID {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST) && \
    SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST
  const char *scene_id = sceneID.UTF8String;
  if (scene_id == nullptr) return @"status=-1 invalid-scene-id";
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 32768> output{};
  const int32_t status =
      spinon_runtime_gpu_host_eval_c11_3_1_grid_repeat_fixture(
          host, scene_id, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  NSString *runReport = [NSString stringWithFormat:@"run_id=%@ scene_id=%@ %@",
                                                   runID ?: @"missing", sceneID, report];
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C1131_GRID_EVAL", runReport);
  if (status == 0) C10LogNodeFrames(report, "SPINON_C1131_GRID_NODE_FRAME");
  return report;
#else
  (void)handle;
  (void)sceneID;
  (void)runID;
  return @"status=-90 feature-disabled";
#endif
}

+ (NSString *)evalRuntimeGpuC11_3_2GridNamedLinesFixture:(uint64_t)handle
                                                  sceneID:(NSString *)sceneID
                                                    runID:(NSString *)runID {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST) && \
    SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST
  const char *scene_id = sceneID.UTF8String;
  if (scene_id == nullptr) return @"status=-1 invalid-scene-id";
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 32768> output{};
  const int32_t status =
      spinon_runtime_gpu_host_eval_c11_3_2_grid_named_lines_fixture(
          host, scene_id, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  NSString *runReport = [NSString stringWithFormat:@"run_id=%@ scene_id=%@ %@",
                                                   runID ?: @"missing", sceneID, report];
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C1132_GRID_EVAL", runReport);
  if (status == 0) C10LogNodeFrames(report, "SPINON_C1132_GRID_NODE_FRAME");
  return report;
#else
  (void)handle;
  (void)sceneID;
  (void)runID;
  return @"status=-90 feature-disabled";
#endif
}

+ (NSString *)evalRuntimeGpuC11_3_3GridTemplateAreasFixture:(uint64_t)handle
                                                    sceneID:(NSString *)sceneID
                                                      runID:(NSString *)runID {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST) && \
    SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST
  const char *scene_id = sceneID.UTF8String;
  if (scene_id == nullptr) return @"status=-1 invalid-scene-id";
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 32768> output{};
  const int32_t status =
      spinon_runtime_gpu_host_eval_c11_3_3_grid_template_areas_fixture(
          host, scene_id, 10000, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  NSString *runReport = [NSString stringWithFormat:@"run_id=%@ scene_id=%@ %@",
                                                   runID ?: @"missing", sceneID, report];
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C1133_GRID_EVAL", runReport);
  if (status == 0) C10LogNodeFrames(report, "SPINON_C1133_GRID_NODE_FRAME");
  return report;
#else
  (void)handle;
  (void)sceneID;
  (void)runID;
  return @"status=-90 feature-disabled";
#endif
}

+ (NSString *)evalRuntimeGpuC12_4Source:(uint64_t)handle source:(NSString *)source {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
  const char *source_utf8 = source.UTF8String;
  if (source_utf8 == nullptr) return @"status=-1 invalid-source";
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  constexpr std::size_t kOutputCapacity = 256 * 1024;
  std::unique_ptr<char[]> output(new (std::nothrow) char[kOutputCapacity]{});
  if (output == nullptr) return @"status=-3 C12.4 진단 보고서 버퍼 할당 실패";
  const int32_t status = spinon_runtime_gpu_host_eval(
      host, source_utf8, 60000, output.get(), kOutputCapacity);
  NSString *detail = [NSString stringWithUTF8String:output.get()];
  if (detail == nil) detail = @"C ABI 보고가 유효한 UTF-8이 아닙니다";
  NSString *report = [NSString stringWithFormat:@"status=%d %@", status, detail];
  NSString *summary = [NSString stringWithFormat:@"status=%d bytes=%lu", status,
                                                   (unsigned long)strlen(output.get())];
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C124_SOURCE", summary);
  return report;
#else
  (void)handle; (void)source;
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
  // Surface-pixel 검증 JSON을 포함하면 7개 이상 표본에서 1 KiB를 넘을 수 있습니다.
  std::array<char, 16384> output{};
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

+ (NSString *)requestRuntimeGpuSurfacePixelReadbackForTest:(uint64_t)handle
                                                        x:(uint32_t)x y:(uint32_t)y {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C124_METAL_SURFACE_READBACK_TEST) && \
    SPINON_ENABLE_C124_METAL_SURFACE_READBACK_TEST
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 1024> output{};
  const int32_t status = spinon_runtime_gpu_host_request_c12_4_surface_pixel_for_test(
      host, x, y, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C124_METAL_PIXEL_REQUEST", report);
  return report;
#else
  (void)handle; (void)x; (void)y;
  return @"status=-90 Metal surface readback fixture is not enabled";
#endif
}


+ (NSString *)requestRuntimeGpuC11_1GridSurfacePixelsForTest:(uint64_t)handle
                                                     sampleXY:(const uint32_t *)sampleXY
                                                  sampleCount:(NSUInteger)sampleCount {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST) && \
    SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 16384> output{};
  const int32_t status =
      spinon_runtime_gpu_host_request_c11_1_grid_surface_pixels_for_test(
          host, sampleXY, sampleCount, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C111_METAL_PIXEL_REQUEST", report);
  return report;
#else
  (void)handle;
  (void)sampleXY;
  (void)sampleCount;
  return @"status=-90 C11 Metal surface pixel fixture is not enabled";
#endif
}

+ (NSString *)requestRuntimeGpuC11_2_2GridSurfacePixelsForTest:(uint64_t)handle
                                                       sampleXY:(const uint32_t *)sampleXY
                                                    sampleCount:(NSUInteger)sampleCount {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST) && \
    SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 16384> output{};
  const int32_t status =
      spinon_runtime_gpu_host_request_c11_2_2_grid_surface_pixels_for_test(
          host, sampleXY, sampleCount, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C1122_METAL_PIXEL_REQUEST", report);
  return report;
#else
  (void)handle;
  (void)sampleXY;
  (void)sampleCount;
  return @"status=-90 C11.2.2 Metal surface pixel fixture is not enabled";
#endif
}

+ (NSString *)requestRuntimeGpuC11_2_3GridSurfacePixelsForTest:(uint64_t)handle
                                                       sampleXY:(const uint32_t *)sampleXY
                                                    sampleCount:(NSUInteger)sampleCount {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST) && \
    SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 16384> output{};
  const int32_t status =
      spinon_runtime_gpu_host_request_c11_2_3_grid_surface_pixels_for_test(
          host, sampleXY, sampleCount, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C1123_METAL_PIXEL_REQUEST", report);
  return report;
#else
  (void)handle;
  (void)sampleXY;
  (void)sampleCount;
  return @"status=-90 C11.2.3 Metal surface pixel fixture is not enabled";
#endif
}

+ (NSString *)requestRuntimeGpuC11_3_1GridSurfacePixelsForTest:(uint64_t)handle
                                                       sampleXY:(const uint32_t *)sampleXY
                                                    sampleCount:(NSUInteger)sampleCount {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST) && \
    SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST && \
    defined(SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST) && \
    SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 16384> output{};
  const int32_t status =
      spinon_runtime_gpu_host_request_c11_3_1_grid_surface_pixels_for_test(
          host, sampleXY, sampleCount, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C1131_METAL_PIXEL_REQUEST", report);
  return report;
#else
  (void)handle;
  (void)sampleXY;
  (void)sampleCount;
  return @"status=-90 C11.3.1 Metal surface pixel fixture is not enabled";
#endif
}

+ (NSString *)requestRuntimeGpuC11_3_2GridSurfacePixelsForTest:(uint64_t)handle
                                                     sampleXY:(const uint32_t *)sampleXY
                                                  sampleCount:(NSUInteger)sampleCount {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST) && \
    SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST && \
    defined(SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST) && \
    SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 16384> output{};
  const int32_t status =
      spinon_runtime_gpu_host_request_c11_3_2_grid_surface_pixels_for_test(
          host, sampleXY, sampleCount, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C1132_METAL_PIXEL_REQUEST", report);
  return report;
#else
  (void)handle;
  (void)sampleXY;
  (void)sampleCount;
  return @"status=-90 C11.3.2 Metal surface pixel fixture is not enabled";
#endif
}

+ (NSString *)requestRuntimeGpuC11_3_3GridSurfacePixelsForTest:(uint64_t)handle
                                                     sampleXY:(const uint32_t *)sampleXY
                                                  sampleCount:(NSUInteger)sampleCount {
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST) && \
    SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST && \
    defined(SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST) && \
    SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST
  auto *host = reinterpret_cast<SpinonRuntimeGpuHost *>(
      static_cast<uintptr_t>(handle));
  std::array<char, 16384> output{};
  const int32_t status =
      spinon_runtime_gpu_host_request_c11_3_3_grid_surface_pixels_for_test(
          host, sampleXY, sampleCount, output.data(), output.size());
  NSString *report = C0410Report(status, output);
  C0410LogReport(status == 0 ? OS_LOG_TYPE_INFO : OS_LOG_TYPE_ERROR,
                 "SPINON_C1133_METAL_PIXEL_REQUEST", report);
  return report;
#else
  (void)handle;
  (void)sampleXY;
  (void)sampleCount;
  return @"status=-90 C11.3.3 Metal surface pixel fixture is not enabled";
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
