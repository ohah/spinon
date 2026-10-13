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
+ (uint64_t)createRuntimeGpuHostWithBlockPaintFixture;
+ (uint64_t)createRuntimeGpuHostWithBlockFormattingFixture;
+ (uint64_t)createRuntimeGpuHostWithC12_1PositionFixture;
+ (uint64_t)createRuntimeGpuHostWithC12_2AbsoluteBlockFixture;
+ (uint64_t)createRuntimeGpuHostWithC12_3FixedFixture;
+ (uint64_t)createRuntimeGpuHostWithC12_4StackingFixture;
+ (uint64_t)createRuntimeGpuHostWithC11_1GridFixture;
+ (uint64_t)createRuntimeGpuHostWithC11_1GridSurfacePixelReadbackFixture;
+ (NSString *)runtimeGpuC12_4WorkerStackResidencySample:(uint64_t)handle;
+ (uint64_t)createRuntimeGpuHostWithC12_4SurfaceReadbackFixture;
+ (uint64_t)createRuntimeGpuHostWithC10_3_5PositionedFlexFixture;
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
+ (NSString *)evalRuntimeGpuAspectRatioFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuBlockPaintFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuBlockFormattingFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuC12_1PositionFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuC12_2AbsoluteBlockFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuC12_3FixedFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuC12_4StackingFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuC11_1GridFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuC11_1GridSceneFixture:(uint64_t)handle
                                       sceneIndex:(uint32_t)sceneIndex;
+ (uint64_t)createRuntimeGpuHostWithC11_2_1GridPlacementFixture;
+ (NSString *)evalRuntimeGpuC11_2_1GridPlacementFixture:(uint64_t)handle
                                                sceneID:(NSString *)sceneID
                                                  runID:(NSString *)runID;
+ (uint64_t)createRuntimeGpuHostWithC11_2_2GridAutoPlacementFixture;
+ (NSString *)evalRuntimeGpuC11_2_2GridAutoPlacementFixture:(uint64_t)handle
                                                    sceneID:(NSString *)sceneID
                                                      runID:(NSString *)runID;
+ (uint64_t)createRuntimeGpuHostWithC11_2_3GridAutoPlacementFixture;
+ (NSString *)evalRuntimeGpuC11_2_3GridAutoPlacementFixture:(uint64_t)handle
                                                    sceneID:(NSString *)sceneID
                                                      runID:(NSString *)runID;
+ (uint64_t)createRuntimeGpuHostWithC11_3_1GridRepeatFixture;
+ (NSString *)evalRuntimeGpuC11_3_1GridRepeatFixture:(uint64_t)handle
                                               sceneID:(NSString *)sceneID
                                                 runID:(NSString *)runID;
+ (uint64_t)createRuntimeGpuHostWithC11_3_2GridNamedLinesFixture;
+ (NSString *)evalRuntimeGpuC11_3_2GridNamedLinesFixture:(uint64_t)handle
                                                  sceneID:(NSString *)sceneID
                                                    runID:(NSString *)runID;
+ (NSString *)evalRuntimeGpuC12_4Source:(uint64_t)handle source:(NSString *)source;
+ (NSString *)evalRuntimeGpuC10_3_5PositionedFlexFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuC12_1PositionState:(uint64_t)handle state:(uint32_t)state;
+ (NSString *)evalRuntimeGpuMarginCollapseFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuFlowRootFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuFlexWrapFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuFlexDistributionFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuFlexReverseFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuFlexOrderFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuFlexAlignmentFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuFlexBaselineFixture:(uint64_t)handle;
+ (NSString *)prepareRuntimeGpuWgpuSurface:(uint64_t)handle view:(void *)view;
+ (NSString *)createRuntimeGpuWgpu:(uint64_t)handle width:(uint32_t)width
                             height:(uint32_t)height;
/* CAMetalLayer surface configuration; call on the main thread. */
+ (NSString *)configureRuntimeGpuWgpuSurface:(uint64_t)handle;
+ (int32_t)resizeRuntimeGpuWgpu:(uint64_t)handle width:(uint32_t)width
                          height:(uint32_t)height;
+ (NSString *)drawRuntimeGpuWgpu:(uint64_t)handle;
+ (NSString *)requestRuntimeGpuSurfacePixelReadbackForTest:(uint64_t)handle
                                                        x:(uint32_t)x y:(uint32_t)y;
+ (NSString *)requestRuntimeGpuC11_1GridSurfacePixelsForTest:(uint64_t)handle
                                                     sampleXY:(const uint32_t *)sampleXY
                                                  sampleCount:(NSUInteger)sampleCount;
+ (NSString *)requestRuntimeGpuC11_2_2GridSurfacePixelsForTest:(uint64_t)handle
                                                       sampleXY:(const uint32_t *)sampleXY
                                                    sampleCount:(NSUInteger)sampleCount;
+ (NSString *)requestRuntimeGpuC11_2_3GridSurfacePixelsForTest:(uint64_t)handle
                                                       sampleXY:(const uint32_t *)sampleXY
                                                    sampleCount:(NSUInteger)sampleCount;
+ (NSString *)requestRuntimeGpuC11_3_1GridSurfacePixelsForTest:(uint64_t)handle
                                                       sampleXY:(const uint32_t *)sampleXY
                                                    sampleCount:(NSUInteger)sampleCount;
+ (NSString *)requestRuntimeGpuC11_3_2GridSurfacePixelsForTest:(uint64_t)handle
                                                     sampleXY:(const uint32_t *)sampleXY
                                                  sampleCount:(NSUInteger)sampleCount;
+ (NSString *)injectNextRuntimeGpuDrawFailure:(uint64_t)handle;
+ (void)destroyRuntimeGpuRenderer:(uint64_t)handle;
+ (void)freeRuntimeGpuHost:(uint64_t)handle;
@end
