#import <Foundation/Foundation.h>
#include <stddef.h>
#include <stdint.h>

@interface SpinonRunner : NSObject
+ (NSString *)runSource:(NSString *)source;
+ (NSString *)runTaffyR10WithWidth:(float)width height:(float)height scale:(float)scale;
+ (void *)createR08WgpuWithUIKitView:(void *)view width:(uint32_t)width height:(uint32_t)height;
+ (BOOL)isS04IosFixtureEnabled;
+ (void *)createS04WgpuWithUIKitView:(void *)view width:(uint32_t)width height:(uint32_t)height density:(float)density surfaceGeneration:(uint64_t)surfaceGeneration;
+ (NSString *)drawS04Wgpu:(void *)renderer;
+ (NSString *)pollS04Readback:(void *)renderer;
+ (NSString *)hitTestS04Wgpu:(void *)renderer surfaceGeneration:(uint64_t)surfaceGeneration surfaceX:(float)surfaceX surfaceY:(float)surfaceY;
+ (int32_t)resizeS04Wgpu:(void *)renderer width:(uint32_t)width height:(uint32_t)height density:(float)density surfaceGeneration:(uint64_t)surfaceGeneration;
+ (int32_t)drawR08Wgpu:(void *)renderer activationCount:(uint32_t)activationCount;
+ (int32_t)resizeR08Wgpu:(void *)renderer width:(uint32_t)width height:(uint32_t)height;
+ (int32_t)injectR13Failure:(void *)renderer kind:(uint32_t)failureKind;
+ (void)destroyR08Wgpu:(void *)renderer;
+ (uint64_t)createRuntimeSession;
+ (NSString *)runRuntimePriorityProbe;
+ (NSString *)runRuntimePriorityFairnessProbe;
+ (NSString *)runRuntimeShutdownProbe;
+ (NSString *)runUaCascadeProbe;
+ (NSString *)evalRuntimeSession:(uint64_t)handle source:(NSString *)source;
+ (NSString *)dispatchRuntimeSession:(uint64_t)handle nodeID:(int32_t)nodeID;
+ (int32_t)cancelRuntimeSession:(uint64_t)handle;
+ (int32_t)notifyRuntimeMemoryPressure:(uint64_t)handle level:(int32_t)level;
+ (void)freeRuntimeSession:(uint64_t)handle;
+ (uint64_t)createRuntimeGpuHostWithRegisteredPropertiesFixture:(BOOL)enabled;
+ (uint64_t)beginRuntimeGpuPresentationUpdate:(uint64_t)handle;
+ (NSString *)setRuntimeGpuEnvironment:(uint64_t)handle width:(float)width
                                height:(float)height scale:(float)scale dark:(BOOL)dark;
+ (NSString *)evalRuntimeGpuFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuAuthorStylesheetsFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuCustomPropertiesFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuRegisteredPropertiesFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuRuntimeResultCacheFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuIncrementalRestyleFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuPercentageDimensionsFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuSpacingPercentagesFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuAbsoluteLengthsFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuFontRelativeUnitsFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuTypedCssMathFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuViewportUnitsFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuMinMaxSizingFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuBorderWidthFixture:(uint64_t)handle;
+ (NSString *)prepareRuntimeGpuWgpuSurface:(uint64_t)handle view:(void *)view;
+ (NSString *)createRuntimeGpuWgpu:(uint64_t)handle width:(uint32_t)width
                             height:(uint32_t)height;
/* CAMetalLayer surface configuration; call on the main thread. */
+ (NSString *)configureRuntimeGpuWgpuSurface:(uint64_t)handle;
+ (int32_t)resizeRuntimeGpuWgpu:(uint64_t)handle width:(uint32_t)width
                          height:(uint32_t)height;
+ (NSString *)drawRuntimeGpuWgpu:(uint64_t)handle;
+ (NSString *)injectNextRuntimeGpuDrawFailure:(uint64_t)handle;
+ (void)destroyRuntimeGpuRenderer:(uint64_t)handle;
+ (void)freeRuntimeGpuHost:(uint64_t)handle;
@end
