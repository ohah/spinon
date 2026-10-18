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
+ (uint64_t)createRuntimeSessionWithRootHTML:(NSString *)source;
+ (NSString *)resolveRuntimeSessionRootHtmlCanvas:(uint64_t)handle
                                     surfaceWidthPx:(float)surfaceWidthPx
                                    surfaceHeightPx:(float)surfaceHeightPx
                                 deviceScaleFactor:(float)deviceScaleFactor
                                      prefersDark:(BOOL)prefersDark;
#if defined(SPINON_ENABLE_C04_12_ROOT_HTML_FIXTURE_TEST) && \
    SPINON_ENABLE_C04_12_ROOT_HTML_FIXTURE_TEST
+ (NSString *)runC0412RootHtmlAccessorsFixture;
+ (NSString *)runC0412RootHtmlAssetFixture:(NSString *)source;
#endif
+ (NSString *)runRuntimePriorityProbe;
+ (NSString *)runRuntimePriorityFairnessProbe;
+ (NSString *)runRuntimeShutdownProbe;
+ (NSString *)runUaCascadeProbe;
+ (NSString *)runC141IntrinsicSizeFixture;
+ (NSString *)runC142ImageRatioFixture;
+ (NSString *)runC143IntrinsicContributionsFixture;
+ (NSString *)evalRuntimeSession:(uint64_t)handle source:(NSString *)source;
+ (NSString *)dispatchRuntimeSession:(uint64_t)handle nodeID:(int32_t)nodeID;
+ (int32_t)cancelRuntimeSession:(uint64_t)handle;
+ (int32_t)notifyRuntimeMemoryPressure:(uint64_t)handle level:(int32_t)level;
+ (void)freeRuntimeSession:(uint64_t)handle;
+ (uint64_t)createRuntimeGpuHostWithRegisteredPropertiesFixture:(BOOL)enabled;
+ (uint64_t)createRuntimeGpuHostWithBlockPaintFixture;
+ (uint64_t)createRuntimeGpuHostWithRootHtmlBlockPaintFixture;
+ (uint64_t)createRuntimeGpuHostWithBlockFormattingFixture;
+ (uint64_t)createRuntimeGpuHostWithC12_1PositionFixture;
+ (uint64_t)createRuntimeGpuHostWithC12_2AbsoluteBlockFixture;
+ (uint64_t)createRuntimeGpuHostWithC12_3FixedFixture;
+ (uint64_t)createRuntimeGpuHostWithC12_4StackingFixture;
+ (uint64_t)createRuntimeGpuHostWithC13_1OverflowClipFixture;
+ (uint64_t)createRuntimeGpuHostWithC13_1CompactSurfaceMatrixFixture;
+ (uint64_t)createRuntimeGpuHostWithC13_1CompactSurfacePixelReadbackFixture;
+ (uint64_t)createRuntimeGpuHostWithC13_1OverflowClipBorderFixture;
+ (uint64_t)createRuntimeGpuHostWithC13_1StackingDescendantClipFixture;
+ (uint64_t)createRuntimeGpuHostWithC13_2ScrollFixture;
+ (uint64_t)createRuntimeGpuHostWithC13_2SurfacePixelReadbackFixture;
#if defined(SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST
+ (uint64_t)createRuntimeGpuHostWithC18_1ImagePaintFixture;
#endif
#if defined(SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST
+ (uint64_t)createRuntimeGpuHostWithC18_2AlphaImageFixture;
#endif
#if defined(SPINON_ENABLE_C18_3_OBJECT_FIT_POSITION_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_3_OBJECT_FIT_POSITION_FIXTURE_TEST
+ (uint64_t)createRuntimeGpuHostWithC18_3ObjectFitPositionFixture;
#endif
#if defined(SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST
+ (uint64_t)createRuntimeGpuHostWithC18_4AsyncImageFixture;
#endif
#if defined(SPINON_ENABLE_C19_2_BOX_PAINT_OPACITY_FIXTURE_TEST) && \
    SPINON_ENABLE_C19_2_BOX_PAINT_OPACITY_FIXTURE_TEST
+ (uint64_t)createRuntimeGpuHostWithC19_2BoxPaintOpacityFixture;
+ (NSString *)evalRuntimeGpuC19_2BoxPaintOpacityFixture:(uint64_t)handle;
+ (NSString *)requestRuntimeGpuC19_2SurfacePixelsForTest:(uint64_t)handle
                                                   sampleXY:(const uint32_t *)sampleXY
                                                sampleCount:(NSUInteger)sampleCount;
#endif
#if defined(SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST) && \
    SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST
+ (uint64_t)createRuntimeGpuHostWithC15_3GlyphPaintFixture;
+ (uint64_t)createRuntimeGpuHostWithC15_3HostDocumentTextPaintFixture;
+ (NSString *)evalRuntimeGpuC15_3GlyphPaintFixture:(uint64_t)handle;
+ (NSString *)requestRuntimeGpuC15_3SurfacePixelsForTest:(uint64_t)handle
                                                   sampleXY:(const uint32_t *)sampleXY
                                                sampleCount:(NSUInteger)sampleCount;
#endif
#if defined(SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST) && SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST
+ (uint64_t)createRuntimeGpuHostWithC13_3GestureFixture:(int32_t)scenarioID;
#endif
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
+ (NSString *)evalRuntimeGpuRootHtmlBlockPaintFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuBlockFormattingFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuC12_1PositionFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuC12_2AbsoluteBlockFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuC12_3FixedFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuC12_4StackingFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuC13_1OverflowClipFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuC13_1CompactSurfaceMatrixFixture:(uint64_t)handle;
+ (NSString *)requestRuntimeGpuC13_1SurfacePixelsForTest:(uint64_t)handle
                                                   sampleXY:(const uint32_t *)sampleXY
                                                sampleCount:(NSUInteger)sampleCount;
+ (NSString *)evalRuntimeGpuC13_1OverflowClipBorderFixture:(uint64_t)handle;
+ (NSString *)evalRuntimeGpuC13_1StackingDescendantClipFixture:(uint64_t)handle;
#if defined(SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST
+ (NSString *)evalRuntimeGpuC18_1ImagePaintFixture:(uint64_t)handle;
#endif
#if defined(SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST
+ (NSString *)evalRuntimeGpuC18_2AlphaImageFixture:(uint64_t)handle;
#endif
#if defined(SPINON_ENABLE_C18_3_OBJECT_FIT_POSITION_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_3_OBJECT_FIT_POSITION_FIXTURE_TEST
+ (NSString *)evalRuntimeGpuC18_3ObjectFitPositionFixture:(uint64_t)handle;
#endif
#if defined(SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST
+ (NSString *)evalRuntimeGpuC18_4AsyncImageFixture:(uint64_t)handle;
#endif
+ (NSString *)evalRuntimeGpuC13_2ScrollFixture:(uint64_t)handle;
#if defined(SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST) && SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST
+ (NSString *)evalRuntimeGpuC13_3GestureFixture:(uint64_t)handle;
+ (NSString *)applyRuntimeGpuC13_3StyleInvalidation:(uint64_t)handle;
+ (NSString *)enqueueRuntimeGpuC13_3Gesture:(uint64_t)handle
                                      action:(int32_t)action
                           surfaceGeneration:(uint64_t)surfaceGeneration
                                   gestureID:(uint64_t)gestureID
                                    pointerID:(uint64_t)pointerID
                                  pointXCssPx:(float)pointXCssPx
                                  pointYCssPx:(float)pointYCssPx
                             coordinateScale:(float)coordinateScale
                                       reason:(int32_t)reason;
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
                            modifierFlags:(uint32_t)modifierFlags;
+ (NSString *)processRuntimeGpuC13_3GestureQueue:(uint64_t)handle;
#endif
+ (NSString *)requestRuntimeGpuC13_2SurfacePixels:(uint64_t)handle
                                             sampleXY:(const uint32_t *)sampleXY
                                          sampleCount:(NSUInteger)sampleCount;
#if defined(SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST
+ (NSString *)requestRuntimeGpuC18_1SurfacePixelsForTest:(uint64_t)handle
                                                   sampleXY:(const uint32_t *)sampleXY
                                                sampleCount:(NSUInteger)sampleCount;
#endif
#if defined(SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST
+ (NSString *)requestRuntimeGpuC18_2SurfacePixelsForTest:(uint64_t)handle
                                                   sampleXY:(const uint32_t *)sampleXY
                                                sampleCount:(NSUInteger)sampleCount;
#endif
#if defined(SPINON_ENABLE_C18_3_OBJECT_FIT_POSITION_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_3_OBJECT_FIT_POSITION_FIXTURE_TEST
+ (NSString *)requestRuntimeGpuC18_3SurfacePixelsForTest:(uint64_t)handle
                                                   sampleXY:(const uint32_t *)sampleXY
                                                sampleCount:(NSUInteger)sampleCount;
#endif
#if defined(SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST
+ (NSString *)requestRuntimeGpuC18_4SurfacePixelsForTest:(uint64_t)handle
                                                   sampleXY:(const uint32_t *)sampleXY
                                                sampleCount:(NSUInteger)sampleCount;
#endif
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
+ (uint64_t)createRuntimeGpuHostWithC11_3_3GridTemplateAreasFixture;
+ (NSString *)evalRuntimeGpuC11_3_3GridTemplateAreasFixture:(uint64_t)handle
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
+ (NSString *)requestRuntimeGpuC11_3_3GridSurfacePixelsForTest:(uint64_t)handle
                                                     sampleXY:(const uint32_t *)sampleXY
                                                  sampleCount:(NSUInteger)sampleCount;
+ (NSString *)injectNextRuntimeGpuDrawFailure:(uint64_t)handle;
+ (void)destroyRuntimeGpuRenderer:(uint64_t)handle;
+ (void)freeRuntimeGpuHost:(uint64_t)handle;
@end
