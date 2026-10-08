#import "SpinonRunner.h"

#include "spinon_wgpu_r08.h"
#import <os/log.h>

@implementation SpinonRunner

+ (void *)createR08WgpuWithUIKitView:(void *)view width:(uint32_t)width height:(uint32_t)height {
  char output[512] = {};
  void *renderer = spinon_wgpu_create_uikit(
      view, width, height, SPINON_WGPU_R08_METAL, output, sizeof(output));
  if (renderer == nullptr) {
    os_log_error(OS_LOG_DEFAULT, "SPINON_R05_SMOKE_WGPU_CREATE=failed %{public}s", output);
  } else {
    os_log(OS_LOG_DEFAULT, "SPINON_R05_SMOKE_WGPU_CREATE=ready %{public}s", output);
  }
  return renderer;
}

+ (int32_t)drawR08Wgpu:(void *)renderer activationCount:(uint32_t)activationCount {
  char output[512] = {};
  const int32_t result = spinon_wgpu_draw(renderer, activationCount, output, sizeof(output));
  os_log(OS_LOG_DEFAULT, "SPINON_R05_SMOKE_WGPU_DRAW result=%{public}d %{public}s", result, output);
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

@end
