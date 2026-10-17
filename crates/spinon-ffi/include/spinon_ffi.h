#ifndef SPINON_FFI_H
#define SPINON_FFI_H

#include <stddef.h>
#include <stdint.h>
#if defined(__APPLE__)
#include <TargetConditionals.h>
#endif
#ifdef __cplusplus
extern "C" {
#endif

/* 앱 바이너리에 포함된 기본 스타일 프로필입니다.
   식별자는 NUL 종료 UTF-8 문자열이고 데이터 포인터는 읽기 전용입니다.
   데이터 길이는 NUL을 포함하지 않으며 두 포인터는 프로세스 종료까지 유효합니다.
   메모리를 해제하지 마세요. 현재 이 자원은 CSS 렌더 경로에 아직 연결되지 않았습니다. */
const char *spinon_embedded_ua_stylesheet_profile_id(void);
const uint8_t *spinon_embedded_ua_stylesheet_data(void);
size_t spinon_embedded_ua_stylesheet_len(void);

/* 실험용 앱 시작 API. 성공 0, 인자 오류 -1, V8 생성 실패 -2,
   버퍼 부족 -3, JavaScript 평가·이벤트 오류 -4를 반환합니다. */
int32_t spinon_app_run(const char *source, char *output, size_t output_capacity);

#if defined(SPINON_ENABLE_C14_1_APP_FIXTURE_TEST) && \
    SPINON_ENABLE_C14_1_APP_FIXTURE_TEST
/* C14.1 고정 PNG 입력을 CSS px intrinsic size/layout snapshot으로 검증합니다.
   성공 0, 인자 오류 -1, fixture 오류 -2, 버퍼 부족 -3, oracle mismatch -4입니다. */
int32_t spinon_c14_1_app_intrinsic_size_fixture(char *output,
                                               size_t output_capacity);
#endif

#if defined(SPINON_ENABLE_C14_2_APP_FIXTURE_TEST) && \
    SPINON_ENABLE_C14_2_APP_FIXTURE_TEST
/* C14.2 live V8 Block 21-case + Flex 8-case image sizing fixture. */
int32_t spinon_c14_2_app_image_ratio_fixture(char *output,
                                            size_t output_capacity);
#endif

#if defined(SPINON_ENABLE_C14_3_APP_FIXTURE_TEST) && \
    SPINON_ENABLE_C14_3_APP_FIXTURE_TEST
/* C14.3 live V8 image intrinsic contribution 13-case fixture. */
int32_t spinon_c14_3_app_intrinsic_contributions_fixture(
    char *output, size_t output_capacity);
#endif

/* 개발용 시뮬레이터 진단: 실제 V8 우선순위 선택과 FIFO 순서를 검증합니다. */
int32_t spinon_runtime_priority_probe(char *output, size_t output_capacity);

#if defined(__ANDROID__) || (defined(__APPLE__) && TARGET_OS_IPHONE)
/* Android·iOS 개발용 시뮬레이터 진단: 높은 등급 유입 중 낮은 등급의 실행 순서를
   검증합니다. */
int32_t spinon_runtime_priority_fairness_probe(char *output,
                                               size_t output_capacity);
#endif

/* 개발용 시뮬레이터 진단: 실제 V8 세션 종료와 종료 중 명령 처리를 검증합니다.
   성공 0, 인자 오류 -1, 출력 버퍼 부족 -3, 검증 실패 -7을 반환합니다. */
int32_t spinon_runtime_shutdown_probe(char *output, size_t output_capacity);

/* 개발용 Taffy 실험 진입점입니다. SPINON_ENABLE_R10_EXPERIMENT=1로 빌드해야 활성화됩니다.
   성공 0, 인자 오류 -1, 레이아웃 오류 -2, 버퍼 부족 -3, 실험 기능 꺼짐 -4를 반환합니다. */
int32_t spinon_taffy_r10_run(float width, float height, float scale,
                             char *output, size_t output_capacity);

/* 플랫폼 호스트용 내부 C ABI입니다. 세션·작업 큐·V8 Isolate는 spinon-runtime이 소유하며,
   이 함수들은 경계 인자와 버퍼를 변환합니다. 제품 공개 API가 아닙니다.
   session_new는 초기화 보고 문자열을 출력하고, 실패하면 NULL을 반환합니다.
   eval/dispatch는 호출 스레드를 막으므로 UI 스레드에서 부르지 마세요.
   기본 eval은 user-visible, 기본 dispatch는 user-blocking입니다.
   우선순위 큐는 높은 등급을 먼저 고르고 같은 등급은 FIFO로 처리합니다.
   background는 명시적 우선순위 함수에서만 선택할 수 있습니다.
   메모리 압박 통지는 호출자가 선택한 단계를 V8에 전달하며 OS 콜백·임계값·시점은 연결하지 않습니다.
   성공 0, 인자 오류 -1, 출력 버퍼 부족 -3, JS 오류 -4, 큐 포화 -5,
   종료 중 -6, 실행기 오류 -7, 취소된 JS -8을 반환합니다.
   cancel은 취소 요청 0, 실행 중인 JS 없음 1, 오류는 음수를 반환합니다.
   free 전에 eval/dispatch/cancel/memory-pressure 호출을 모두 끝내야 합니다. */
typedef struct SpinonRuntimeSession SpinonRuntimeSession;
typedef struct SpinonRuntimeGpuHost SpinonRuntimeGpuHost;
typedef struct SpinonRuntimeRootHtmlEnvironment {
  float viewport_width_css_px;
  float viewport_height_css_px;
  float visual_viewport_width_css_px;
  float visual_viewport_height_css_px;
  float device_scale_factor;
  float page_scale;
  int32_t prefers_color_scheme; /* 0=light, 1=dark */
  int32_t document_color_scheme; /* 0=light, 1=dark */
  uint64_t environment_revision;
} SpinonRuntimeRootHtmlEnvironment;
typedef struct SpinonRuntimeRootHtmlCanvas {
  SpinonRuntimeRootHtmlEnvironment environment;
  uint8_t canvas_rgba[4];
  int32_t used_color_scheme; /* 0=light, 1=dark */
  uint64_t cascade_generation;
  uint64_t cascade_document_revision;
  uint64_t cascade_render_tree_revision;
  uint64_t cascade_style_revision;
  uint64_t cascade_environment_revision;
} SpinonRuntimeRootHtmlCanvas;
typedef enum SpinonTaskPriority {
  SPINON_TASK_PRIORITY_USER_BLOCKING = 0,
  SPINON_TASK_PRIORITY_USER_VISIBLE = 1,
  SPINON_TASK_PRIORITY_BACKGROUND = 2,
} SpinonTaskPriority;
typedef enum SpinonMemoryPressureLevel {
  SPINON_MEMORY_PRESSURE_NONE = 0,
  SPINON_MEMORY_PRESSURE_MODERATE = 1,
  SPINON_MEMORY_PRESSURE_CRITICAL = 2,
} SpinonMemoryPressureLevel;
SpinonRuntimeSession *spinon_runtime_session_new(char *output,
                                                 size_t output_capacity);
/* UTF-8 HTML document를 초기 HostDocument로 가져옵니다. source_length는 NUL을 제외하며
   최대 1,048,576 bytes입니다. script 실행·viewport/color-scheme 적용은 하지 않습니다.
   source는 source_length bytes를 읽을 수 있어야 하고 output은 output_capacity bytes를
   쓸 수 있어야 합니다. 두 범위는 겹치지 않아야 합니다. 실패하면 NULL입니다. */
SpinonRuntimeSession *spinon_runtime_session_new_with_root_html(
    const char *source, size_t source_length, char *output,
    size_t output_capacity);
#if defined(SPINON_ENABLE_C04_12_ROOT_HTML_FIXTURE_TEST) && \
    SPINON_ENABLE_C04_12_ROOT_HTML_FIXTURE_TEST
/* 실제 V8 documentElement/head/body 및 초기 HTML DOM을 검사합니다. */
int32_t spinon_c04_12_root_html_accessors_fixture(char *output,
                                                  size_t output_capacity);
/* 플랫폼 asset에서 읽은 UTF-8 HTML을 실제 V8 초기 document에 연결해 검사합니다. */
int32_t spinon_c04_12_root_html_asset_fixture(const char *source,
                                              size_t source_length,
                                              char *output,
                                              size_t output_capacity);
#endif
int32_t spinon_runtime_session_eval(SpinonRuntimeSession *session,
                                   const char *source, char *output,
                                   size_t output_capacity);
int32_t spinon_runtime_session_eval_with_priority(
    SpinonRuntimeSession *session, const char *source,
    SpinonTaskPriority priority, char *output, size_t output_capacity);
int32_t spinon_runtime_session_dispatch(SpinonRuntimeSession *session,
                                       int32_t node_id, char *output,
                                       size_t output_capacity);
int32_t spinon_runtime_session_dispatch_with_priority(
    SpinonRuntimeSession *session, int32_t node_id,
    SpinonTaskPriority priority, char *output, size_t output_capacity);
int32_t spinon_runtime_session_cancel(SpinonRuntimeSession *session);
/* 성공 0, 인자 또는 알 수 없는 단계 -1, 종료 중·종료된 세션 -6.
   JavaScript 취소·이벤트 실행을 요청하지 않으며, GC 수행 시점을 보장하지 않습니다.
   진행 중인 세션 호출과 free의 동시 실행은 금지합니다. */
int32_t spinon_runtime_session_notify_memory_pressure(
    SpinonRuntimeSession *session, SpinonMemoryPressureLevel level);
/* 내부 C04.8 진단 API입니다. 환경 설정은 최초 값 revision 0, 이후 변경마다 +1이며
   Stylo 계산 완료를 기다리지 않습니다. color_scheme: 0=light, 1=dark;
   primary_pointer: 0=none, 1=coarse, 2=fine; primary_hover는 0|1;
   all_pointer_flags bit0=coarse, bit1=fine, bit2=hover입니다.
   잘못된 값 -1, 종료 중 -6, CSS worker 불능 -7이며 성공 revision만 출력합니다.
   computed-style 조회는 JSON 비용이 있으므로 UI thread에서 호출하지 마세요.
   JSON 복사 성공 0, 인자 오류 -1, 짧은 버퍼 -3이며 required_capacity는 UTF-8 JSON의
   마지막 NUL을 포함합니다. 부족한 버퍼에는 부분 JSON 대신 NUL guard만 씁니다.
   schema는 spinon.runtime.ua-cascade.v1입니다. 두 API는 free와 병행할 수 없습니다. */
int32_t spinon_runtime_session_set_ua_cascade_environment(
    SpinonRuntimeSession *session, float width_css_px, float height_css_px,
    float device_scale_factor, int32_t color_scheme,
    int32_t primary_pointer, int32_t primary_hover,
    uint32_t all_pointer_flags, uint64_t *environment_revision);
/* 내부 C04.12 API입니다. prefers_color_scheme 입력과 두 출력 scheme은 0=light, 1=dark입니다.
   surface pixel 크기와 DPR을 전달하면 root HTML metadata로 CSS layout viewport와 초기 visual
   viewport를 계산하고 OS preference는 prefers-color-scheme으로 전달합니다. document_color_scheme은
   별도 initial scheme 결과로 반환하며 Canvas paint와 page_scale renderer transform은 적용하지 않습니다.
   이 호출은 root HTML 세션에서만 유효하고 성공 0, 잘못된 입력·root HTML 미설정 -1,
   종료 중 -6, CSS worker 불능 -7입니다. environment 출력과 session free를 병행하지 마세요. */
int32_t spinon_runtime_session_set_root_html_environment(
    SpinonRuntimeSession *session, float surface_width_px,
    float surface_height_px, float device_scale_factor,
    int32_t prefers_color_scheme,
    SpinonRuntimeRootHtmlEnvironment *environment);
/* 내부 C04.12 플랫폼 bridge입니다. 환경을 적용하고 CSS cascade 완료를 지정 시간까지 기다린 뒤
   실제 HTML root의 computed color-scheme으로 Chromium 155 Canvas sRGB 색과 revision key를
   반환합니다. 호출은 UI thread에서 하지 마세요. timeout은 0~60,000ms이며 root meta 중복
   -13, cascade timeout -10, stale snapshot -12, root/style 누락 -11,
   미지원 computed scheme -14를 반환합니다. 그 밖의 입력·환경 오류 -1, worker 오류 -7,
   종료 중 -6입니다. 출력·세션 해제와 병행 호출하지 마세요. */
int32_t spinon_runtime_session_resolve_root_html_canvas(
    SpinonRuntimeSession *session, float surface_width_px,
    float surface_height_px, float device_scale_factor,
    int32_t prefers_color_scheme, uint64_t timeout_millis,
    SpinonRuntimeRootHtmlCanvas *canvas);
int32_t spinon_runtime_session_copy_ua_cascade_json(
    SpinonRuntimeSession *session, char *output, size_t output_capacity,
    size_t *required_capacity);
/* 내부 C04 runtime CSS→Taffy 결과 조회입니다. UI thread에서 호출하지 마세요.
   기다리지 않고 UTF-8 JSON을 복사하며 성공 0, 인자 오류 -1, 짧은 버퍼 -3입니다.
   schema는 spinon.runtime.layout입니다. 상태와 revision, CSS px 프레임,
   Stylo parser 진단과 원문 CSS를 포함하지 않는 안정 오류 코드를 반환합니다.
   두 출력 범위는 서로 또는 session 저장 공간과 겹치면 안 되며 free와 병행할 수 없습니다. */
int32_t spinon_runtime_session_copy_layout_json(
    SpinonRuntimeSession *session, char *output, size_t output_capacity,
    size_t *required_capacity);
#if defined(SPINON_ENABLE_C04_UA_CASCADE_FIXTURE_PROBE) && \
    SPINON_ENABLE_C04_UA_CASCADE_FIXTURE_PROBE
/* Android emulator·iOS Simulator 내부 검증 fixture입니다. 실제 V8 DOM 변경 뒤 환경 setter,
   비동기 Stylo 결과와 JSON readback을 검증합니다. 성공 0, 인자 오류 -1, 생성 오류 -2,
   버퍼 부족 -3, cascade 검증 실패 -7을 반환합니다. 제품 렌더링 API가 아닙니다. */
int32_t spinon_runtime_ua_cascade_probe(char *output, size_t output_capacity);
#endif
void spinon_runtime_session_free(SpinonRuntimeSession *session);

#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new(char *output,
                                                  size_t output_capacity);
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST) && \
    SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST
/* C05.2 검증 fixture 전용 `@property` CSS profile로 V8 GPU host를 생성합니다. */
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_registered_properties_fixture(
    char *output, size_t output_capacity);
/* C08 Block·기본 페인트 검증 전용 CSS runtime host를 생성합니다. */
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_block_paint_fixture(
    char *output, size_t output_capacity);
/* C09.1 일반 Block 흐름·포함 블록·auto margin 검증 fixture 전용 host입니다. */
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_block_formatting_fixture(
    char *output, size_t output_capacity);
/* C12.1 정적·상대 위치 V8 GPU runtime fixture host입니다. */
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_c12_1_position_fixture(
    char *output, size_t output_capacity);
/* C12.2 Block absolute positioning V8 GPU runtime fixture host입니다. */
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_c12_2_absolute_block_fixture(
    char *output, size_t output_capacity);
/* C12.3 viewport fixed V8 GPU runtime fixture host입니다. */
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_c12_3_fixed_fixture(
    char *output, size_t output_capacity);
/* C12.4 쌓임 맥락 V8 GPU runtime fixture host입니다. */
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_c12_4_stacking_fixture(
    char *output, size_t output_capacity);
#if defined(SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST) && \
    SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST
/* C13.1 overflow clip 사전 비교 전용 V8 GPU runtime fixture host입니다. */
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_c13_1_overflow_clip_fixture(
    char *output, size_t output_capacity);
int32_t spinon_runtime_gpu_host_eval_c13_1_overflow_clip_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* C13.1 compact 13-case surface matrix 전용 fixture-test host입니다. */
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_c13_1_compact_surface_matrix_fixture(
    char *output, size_t output_capacity);
/* C13.1 compact fixture의 Metal drawable 표면 readback 전용 host입니다. */
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_c13_1_compact_surface_pixel_readback_fixture(
    char *output, size_t output_capacity);
int32_t spinon_runtime_gpu_host_eval_c13_1_compact_surface_matrix_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* C13.1 중첩 stacking descendant clip fixture-test host와 evaluator입니다. */
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_c13_1_stacking_descendant_clip_fixture(
    char *output, size_t output_capacity);
int32_t spinon_runtime_gpu_host_eval_c13_1_stacking_descendant_clip_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* 다음 C13.1 draw report에 포함할 최대 64개 Metal surface sample을 예약합니다. */
int32_t spinon_runtime_gpu_host_request_c13_1_surface_pixels_for_test(
    SpinonRuntimeGpuHost *host, const uint32_t *sample_xy, size_t sample_count,
    char *output, size_t output_capacity);
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_c13_1_overflow_clip_border_fixture(
    char *output, size_t output_capacity);
int32_t spinon_runtime_gpu_host_eval_c13_1_overflow_clip_border_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
#endif

#if defined(SPINON_ENABLE_C132_SCROLL_FIXTURE_TEST) && \
    SPINON_ENABLE_C132_SCROLL_FIXTURE_TEST
/* C13.2 scroll state·paint·hit 비교 전용 opt-in runtime fixture host입니다. */
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_c13_2_scroll_fixture(
    char *output, size_t output_capacity);
int32_t spinon_runtime_gpu_host_eval_c13_2_scroll_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* C13.2 scroll 표면 표본 검증용 Metal drawable readback host입니다. */
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_c13_2_scroll_surface_pixel_readback_fixture(
    char *output, size_t output_capacity);
/* 다음 C13.2 draw report에 포함할 고정 5개 Metal surface 표본을 예약합니다. */
int32_t spinon_runtime_gpu_host_request_c13_2_surface_pixels_for_test(
    SpinonRuntimeGpuHost *host, const uint32_t *sample_xy, size_t sample_count,
    char *output, size_t output_capacity);
#endif

#if defined(SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST
/* C18.1 고정 PNG의 image paint·overflow surface 검증용 fixture-test host입니다. */
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_c18_1_image_paint_fixture(
    char *output, size_t output_capacity);
int32_t spinon_runtime_gpu_host_eval_c18_1_image_paint_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* 다음 C18.1 draw report에 포함할 정확히 9개 drawable sample을 예약합니다. */
int32_t spinon_runtime_gpu_host_request_c18_1_surface_pixels_for_test(
    SpinonRuntimeGpuHost *host, const uint32_t *sample_xy, size_t sample_count,
    char *output, size_t output_capacity);
#endif

#if defined(SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST
/* C18.2 고정 RGBA8 PNG source-over 합성 검증용 fixture host입니다. */
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_c18_2_alpha_image_fixture(
    char *output, size_t output_capacity);
int32_t spinon_runtime_gpu_host_eval_c18_2_alpha_image_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* 다음 C18.2 draw report에 포함할 정확히 10개 drawable sample을 예약합니다. */
int32_t spinon_runtime_gpu_host_request_c18_2_surface_pixels_for_test(
    SpinonRuntimeGpuHost *host, const uint32_t *sample_xy, size_t sample_count,
    char *output, size_t output_capacity);
#endif

#if defined(SPINON_ENABLE_C18_3_OBJECT_FIT_POSITION_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_3_OBJECT_FIT_POSITION_FIXTURE_TEST
/* C18.3 object-fit/object-position fixture is restricted to its opt-in build. */
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_c18_3_object_fit_position_fixture(
    char *output, size_t output_capacity);
int32_t spinon_runtime_gpu_host_eval_c18_3_object_fit_position_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* Reserves exactly 19 drawable samples for the next C18.3 draw. */
int32_t spinon_runtime_gpu_host_request_c18_3_surface_pixels_for_test(
    SpinonRuntimeGpuHost *host, const uint32_t *sample_xy, size_t sample_count,
    char *output, size_t output_capacity);
#endif

#if defined(SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST) && \
    SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST
/* C18.4 async image request fixture is restricted to its opt-in build. */
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_c18_4_async_image_fixture(
    char *output, size_t output_capacity);
int32_t spinon_runtime_gpu_host_eval_c18_4_async_image_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* Reserves exactly six drawable samples for the next C18.4 draw. */
int32_t spinon_runtime_gpu_host_request_c18_4_surface_pixels_for_test(
    SpinonRuntimeGpuHost *host, const uint32_t *sample_xy, size_t sample_count,
    char *output, size_t output_capacity);
#endif

#if defined(SPINON_ENABLE_C19_2_BOX_PAINT_OPACITY_FIXTURE_TEST) && \
    SPINON_ENABLE_C19_2_BOX_PAINT_OPACITY_FIXTURE_TEST
/* C19.2 box paint·opacity 통합 fixture 전용 opt-in host입니다. */
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_c19_2_box_paint_opacity_fixture(
    char *output, size_t output_capacity);
int32_t spinon_runtime_gpu_host_eval_c19_2_box_paint_opacity_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* 다음 C19.2 draw report에 CSS 고정 16점을 균일한 1~8 surface scale로 변환해 예약합니다. */
int32_t spinon_runtime_gpu_host_request_c19_2_surface_pixels_for_test(
    SpinonRuntimeGpuHost *host, const uint32_t *sample_xy, size_t sample_count,
    char *output, size_t output_capacity);
/* Re-renders the published C19.2 scene into an offscreen WGPU texture and returns CSS pixel samples. */
int32_t spinon_runtime_gpu_host_readback_c19_2_offscreen_for_test(
    SpinonRuntimeGpuHost *host, char *output, size_t output_capacity);
#endif

#if defined(SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST) && \
    SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST
/* C15.3 고정 글꼴 glyph mask를 WGPU paint command로 제출하는 앱 fixture-test host입니다. */
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_c15_3_glyph_paint_fixture(
    char *output, size_t output_capacity);
/* C15.3 fixture 소유 HostDocument text layout을 WGPU glyph paint로 잇는 host입니다. */
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_c15_3_hostdocument_text_paint_fixture(
    char *output, size_t output_capacity);
int32_t spinon_runtime_gpu_host_eval_c15_3_glyph_paint_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* 다음 C15.3 draw report에 overflow/scroll CSS glyph 표본 6점을 drawable pixel로 예약합니다. */
int32_t spinon_runtime_gpu_host_request_c15_3_surface_pixels_for_test(
    SpinonRuntimeGpuHost *host, const uint32_t *sample_xy, size_t sample_count,
    char *output, size_t output_capacity);
#endif

#if defined(SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST) && \
    SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST
/* C13.3 Android 입력 어댑터 전용 opt-in gesture fixture host입니다. */
enum {
    SPINON_C13_3_GESTURE_SCENARIO_AUTO_ZERO = 0,
    SPINON_C13_3_GESTURE_SCENARIO_AUTO_BOUNDARY = 1,
    SPINON_C13_3_GESTURE_SCENARIO_CONTAIN_X_AUTO_Y_BOUNDARY = 2,
    SPINON_C13_3_GESTURE_SCENARIO_STYLE_POLICY_INVALIDATION = 8,
    SPINON_C13_3_GESTURE_SCENARIO_GEOMETRY_INVALIDATION = 9,
    SPINON_C13_3_GESTURE_SCENARIO_PAINT_INVALIDATION = 10
};
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_c13_3_gesture_fixture(
    int32_t scenario_id, char *output, size_t output_capacity);
int32_t spinon_runtime_gpu_host_eval_c13_3_gesture_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* Active pan 중 style revision·geometry invalidation을 검증하는 고정 fixture update입니다. */
int32_t spinon_runtime_gpu_host_apply_c13_3_style_invalidation(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* UI callback에서 worker를 기다리지 않고 bounded gesture queue에 입력을 추가합니다. */
int32_t spinon_runtime_gpu_host_enqueue_c13_3_gesture(
    SpinonRuntimeGpuHost *host, int32_t action, uint64_t surface_generation,
    uint64_t gesture_id, uint64_t pointer_id, float x_or_cumulative_x_css_px,
    float y_or_cumulative_y_css_px, float coordinate_scale, int32_t reason,
    char *output, size_t output_capacity);
/* Opt-in fixture 전용 normalized CSS px wheel/trackpad sample입니다.
   intent 0은 scroll, 1은 host가 분류한 magnification/zoom 거부 입력입니다.
   source_unit: 0 CSS px, 1 Android px, 2 UIKit point, 3 line, 4 page.
   source: 0 mouse wheel, 1 trackpad scroll. phase: 0 none, 1 begin, 2 change, 3 end. */
int32_t spinon_runtime_gpu_host_enqueue_c13_3_wheel(
    SpinonRuntimeGpuHost *host, int32_t intent, uint64_t surface_generation,
    float point_x_css_px, float point_y_css_px, float delta_x_css_px,
    float delta_y_css_px, float raw_delta_x, float raw_delta_y,
    int32_t source_unit, float coordinate_scale, int32_t source,
    int32_t momentum_phase, uint32_t modifier_flags,
    char *output, size_t output_capacity);
/* 단일 background consumer가 ordered queue를 session에 적용하고 snapshot을 게시합니다. */
int32_t spinon_runtime_gpu_host_process_c13_3_gesture_queue(
    SpinonRuntimeGpuHost *host, size_t max_events,
    char *output, size_t output_capacity);
#endif
/* C11.1 고정 explicit Grid scene 전용 V8·Stylo·Taffy·WGPU 검증 host입니다.
   일반 앱의 display:grid CSS 지원을 선언하지 않습니다. */
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_c11_1_grid_fixture(
    char *output, size_t output_capacity);
/* C11.2.1 numeric explicit Grid placement fixture 전용 검증 host입니다. */
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_c11_2_1_grid_placement_fixture(
    char *output, size_t output_capacity);
/* C11.2.2 행 우선 sparse auto-placement와 양의 암시 행 전용 검증 host입니다. */
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_c11_2_2_grid_auto_placement_fixture(
    char *output, size_t output_capacity);
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST) && \
    SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST
/* C11.2.3 dense·column 자동 배치와 양의 암시 행·열 전용 검증 host입니다. */
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_c11_2_3_grid_auto_placement_fixture(
    char *output, size_t output_capacity);
#endif
/* C11.3.1 정수 repeat() 장면 host는 앱 fixture-test 빌드에서만 제공합니다. */
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_c11_3_1_grid_repeat_fixture(
    char *output, size_t output_capacity);
/* C11.3.2 named line 장면 host는 앱 fixture-test 빌드에서만 제공합니다. */
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_c11_3_2_grid_named_lines_fixture(
    char *output, size_t output_capacity);
/* C11.3.3 grid-template-areas 장면 host는 앱 fixture-test 빌드에서만 제공합니다. */
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_c11_3_3_grid_template_areas_fixture(
    char *output, size_t output_capacity);
#if defined(SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST) && \
    SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST
/* C11.1 고정 Grid surface pixel 내부 검증 전용 host입니다. */
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_c11_1_grid_surface_pixel_readback_fixture(
    char *output, size_t output_capacity);
int32_t spinon_runtime_gpu_host_request_c11_1_grid_surface_pixels_for_test(
    SpinonRuntimeGpuHost *host, const uint32_t *sample_xy, size_t sample_count,
    char *output, size_t output_capacity);
/* C11.2.2 행 우선 Grid fixture의 WGPU surface pixel 내부 검증 전용입니다. */
int32_t spinon_runtime_gpu_host_request_c11_2_2_grid_surface_pixels_for_test(
    SpinonRuntimeGpuHost *host, const uint32_t *sample_xy, size_t sample_count,
    char *output, size_t output_capacity);
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST) && \
    SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST
/* C11.2.3 dense·column Grid fixture의 WGPU surface pixel 내부 검증 전용입니다. */
int32_t spinon_runtime_gpu_host_request_c11_2_3_grid_surface_pixels_for_test(
    SpinonRuntimeGpuHost *host, const uint32_t *sample_xy, size_t sample_count,
    char *output, size_t output_capacity);
/* C11.3.1 integer repeat Grid fixture WGPU surface 표본 전용 진입점입니다. */
int32_t spinon_runtime_gpu_host_request_c11_3_1_grid_surface_pixels_for_test(
    SpinonRuntimeGpuHost *host, const uint32_t *sample_xy, size_t sample_count,
    char *output, size_t output_capacity);
/* C11.3.2 named line Grid fixture의 WGPU surface pixel 내부 검증 전용입니다. */
int32_t spinon_runtime_gpu_host_request_c11_3_2_grid_surface_pixels_for_test(
    SpinonRuntimeGpuHost *host, const uint32_t *sample_xy, size_t sample_count,
    char *output, size_t output_capacity);
/* C11.3.3 template area Grid fixture의 WGPU surface pixel 내부 검증 전용입니다. */
int32_t spinon_runtime_gpu_host_request_c11_3_3_grid_surface_pixels_for_test(
    SpinonRuntimeGpuHost *host, const uint32_t *sample_xy, size_t sample_count,
    char *output, size_t output_capacity);
#endif
#endif
#if defined(SPINON_ENABLE_C124_METAL_SURFACE_READBACK_TEST) && \
    SPINON_ENABLE_C124_METAL_SURFACE_READBACK_TEST
/* iOS 시뮬레이터 전용 Metal 표면 진단 host입니다. */
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_c12_4_surface_readback_fixture(
    char *output, size_t output_capacity);
#endif
/* C10.3.5 positioned Flex child V8 GPU runtime fixture host입니다. */
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_c10_3_5_positioned_flex_fixture(
    char *output, size_t output_capacity);
#endif
/* UI event에서 viewport·색상 체계·surface 변경을 플랫폼 큐에 넣기 전에
   호출합니다. 잠금·대기를 하지 않으며 실패는 0입니다. 반환값은 내부 무효화 순번이며
   호출자가 다른 함수에 전달하지 않습니다. 성공 여부 확인 외 용도로 보관하지 마세요. */
uint64_t spinon_runtime_gpu_host_begin_presentation_update(
    SpinonRuntimeGpuHost *host);
/* `layout_timeout_millis`는 CSS 계산 결과 대기만 제한합니다. */
int32_t spinon_runtime_gpu_host_set_environment(
    SpinonRuntimeGpuHost *host, float width_css_px, float height_css_px,
    float device_scale_factor, int32_t dark, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* JavaScript는 동기 실행합니다. `layout_timeout_millis`는 평가 후 CSS 계산 결과 대기만
   제한하며 JavaScript 실행 자체를 종료하지 않습니다. */
int32_t spinon_runtime_gpu_host_eval(SpinonRuntimeGpuHost *host,
                                    const char *source,
                                    uint64_t layout_timeout_millis, char *output,
                                    size_t output_capacity);
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST) && \
    SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST
int32_t spinon_runtime_gpu_host_eval_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* C04.11: 연결된 HTML style 요소의 CSS 본문을 실제 V8 DOM에서 평가하는 fixture입니다. */
int32_t spinon_runtime_gpu_host_eval_author_stylesheets_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* 기존 C04.10 HostDocument에 C05.1 `--*`·`var()` style 값을 적용하는 내부 검증 fixture입니다. */
int32_t spinon_runtime_gpu_host_eval_custom_properties_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* C05.2: 연결 HTML style 요소의 @property 등록을 처리하는 검증 전용 fixture입니다. */
int32_t spinon_runtime_gpu_host_eval_registered_properties_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* C05.3: detached mutation cache hit, 연결·분리 cache miss 검증 fixture입니다. */
int32_t spinon_runtime_gpu_host_eval_runtime_result_cache_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* C05.4: 실제 V8 inline style 하위 트리 재계산 fixture입니다. */
int32_t spinon_runtime_gpu_host_eval_incremental_restyle_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* C06.1: percentage width·height·flex-basis를 실제 V8·Taffy 경로에서 확인합니다. */
int32_t spinon_runtime_gpu_host_eval_percentage_dimensions_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* C06.2: percentage margin·padding·gap을 실제 V8·Stylo·Taffy 경로에서 확인합니다. */
int32_t spinon_runtime_gpu_host_eval_spacing_percentages_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* C06.3: 절대 CSS 길이를 실제 V8·Stylo·Taffy 경로에서 확인합니다. */
int32_t spinon_runtime_gpu_host_eval_absolute_lengths_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* C06.4: em/rem cascade와 Taffy/WGPU runtime 경로를 확인합니다. */
int32_t spinon_runtime_gpu_host_eval_font_relative_units_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* C06.5: calc/min/max/clamp를 실제 V8·Stylo·Taffy 경로에서 확인합니다. */
int32_t spinon_runtime_gpu_host_eval_typed_css_math_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* C06.6a: viewport length 단위를 실제 V8·Stylo·Taffy 경로에서 확인합니다. */
int32_t spinon_runtime_gpu_host_eval_viewport_units_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* C07.1: 물리 축 min/max sizing을 실제 V8·Stylo·Taffy 경로에서 확인합니다. */
int32_t spinon_runtime_gpu_host_eval_min_max_sizing_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* C07.2: border width layout을 실제 V8·Stylo·Taffy 경로에서 확인합니다. */
int32_t spinon_runtime_gpu_host_eval_border_width_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* C07.3: aspect-ratio를 실제 V8·Stylo·Taffy 경로에서 확인합니다. */
int32_t spinon_runtime_gpu_host_eval_aspect_ratio_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* C08: 제한 Block 흐름과 배경 페인트를 실제 V8·Stylo·Taffy 경로에서 확인합니다. */
int32_t spinon_runtime_gpu_host_eval_block_paint_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* C09.1 actual V8 fixture; 보고에는 DOM preorder node별 CSS frame이 포함됩니다. */
int32_t spinon_runtime_gpu_host_eval_block_formatting_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* C12.1 정적·상대 위치 CSS를 실제 V8·Stylo·Taffy 경로에서 평가합니다. */
int32_t spinon_runtime_gpu_host_eval_c12_1_position_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* C12.2 Block absolute CSS를 실제 V8·Stylo·Taffy 경로에서 평가합니다. */
int32_t spinon_runtime_gpu_host_eval_c12_2_absolute_block_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* C12.3 지원 viewport fixed subset을 실제 V8·Stylo·Taffy 경로에서 평가합니다. */
int32_t spinon_runtime_gpu_host_eval_c12_3_fixed_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* C12.4 제한 쌓임 맥락 CSS를 실제 V8·Stylo·Taffy 경로에서 평가합니다. */
int32_t spinon_runtime_gpu_host_eval_c12_4_stacking_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* C11.1 fixed-tracks-gap fixture를 V8 document 변경부터 GPU scene까지 평가합니다.
   UI thread에서 부르지 마세요. layout timeout은 JS 실행을 취소하지 않고 결과 대기만 제한합니다. */
int32_t spinon_runtime_gpu_host_eval_c11_1_grid_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* C11.1 내부 검증용 고정 matrix scene 선택 API입니다. 유효 index는 0..9이며 JS 문자열은 받지 않습니다. */
int32_t spinon_runtime_gpu_host_eval_c11_1_grid_scene_fixture(
    SpinonRuntimeGpuHost *host, uint32_t scene_index,
    uint64_t layout_timeout_millis, char *output, size_t output_capacity);
/* 고정 C11.2.1 scene ID 하나를 실제 V8·Stylo·Taffy 경로에서 평가합니다.
   허용되는 ID는 내장 matrix의 10개뿐이며 임의 JavaScript는 받지 않습니다. */
int32_t spinon_runtime_gpu_host_eval_c11_2_1_grid_placement_fixture(
    SpinonRuntimeGpuHost *host, const char *scene_id,
    uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* 고정 C11.2.2 scene ID만 허용하며 arbitrary JavaScript는 받지 않습니다. */
int32_t spinon_runtime_gpu_host_eval_c11_2_2_grid_auto_placement_fixture(
    SpinonRuntimeGpuHost *host, const char *scene_id,
    uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU && \
    defined(SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST) && \
    SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST
/* 고정 C11.2.3 scene ID만 허용하며 임의 JavaScript는 받지 않습니다. */
int32_t spinon_runtime_gpu_host_eval_c11_2_3_grid_auto_placement_fixture(
    SpinonRuntimeGpuHost *host, const char *scene_id,
    uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
#endif
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST) && \
    SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST
/* C11.3.1 고정 정수 repeat scene ID 평가 진입점은 앱 fixture-test 빌드 전용입니다. */
int32_t spinon_runtime_gpu_host_eval_c11_3_1_grid_repeat_fixture(
    SpinonRuntimeGpuHost *host, const char *scene_id,
    uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* C11.3.2 고정 named line scene ID만 허용하며 임의 JavaScript는 받지 않습니다. */
int32_t spinon_runtime_gpu_host_eval_c11_3_2_grid_named_lines_fixture(
    SpinonRuntimeGpuHost *host, const char *scene_id,
    uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* C11.3.3 고정 template area scene ID만 허용하며 임의 JavaScript는 받지 않습니다. */
int32_t spinon_runtime_gpu_host_eval_c11_3_3_grid_template_areas_fixture(
    SpinonRuntimeGpuHost *host, const char *scene_id,
    uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
#endif
/* C12.4 내부 테스트: 실제 CSS worker stack 범위와 순간 mincore 상주 페이지 표본입니다. */
int32_t spinon_runtime_gpu_host_css_worker_stack_residency_for_test(
    SpinonRuntimeGpuHost *host, char *output, size_t output_capacity);
/* C12.1 inventory의 고정 mutation 상태: 1=target-relative, 2=ancestor-relative. */
int32_t spinon_runtime_gpu_host_eval_c12_1_position_state(
    SpinonRuntimeGpuHost *host, uint32_t state, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* C09.2 실제 V8 fixture: 부호 있는 세로 margin collapse와 노드별 CSS frame입니다. */
int32_t spinon_runtime_gpu_host_eval_margin_collapse_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* C09.3: flow-root 내부 BFC와 외부 margin collapse를 실제 V8 경로에서 확인합니다. */
int32_t spinon_runtime_gpu_host_eval_flow_root_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* C10.1: flex-wrap 줄바꿈과 gap을 실제 V8·Stylo·Taffy·WGPU 경로에서 확인합니다. */
int32_t spinon_runtime_gpu_host_eval_flex_wrap_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* C10.3.1: flex-direction reverse와 flex-wrap reverse를 실제 V8·Stylo·Taffy·WGPU 경로에서 확인합니다. */
int32_t spinon_runtime_gpu_host_eval_flex_reverse_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* C10.3.2: Flex order-modified layout과 WGPU paint list를 실제 V8 경로에서 확인합니다. */
int32_t spinon_runtime_gpu_host_eval_flex_order_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
int32_t spinon_runtime_gpu_host_eval_flex_distribution_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* C10.3.3: align-items/self/content와 place-* shorthand를 실제 V8·Stylo·Taffy·WGPU 경로에서 확인합니다. */
int32_t spinon_runtime_gpu_host_eval_flex_alignment_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* C10.3.4: Flex first/last baseline 및 중첩 container 전파를 확인합니다. */
int32_t spinon_runtime_gpu_host_eval_flex_baseline_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
/* C10.3.5 positioned Flex child runtime subset; DOM preorder frame을 포함합니다. */
int32_t spinon_runtime_gpu_host_eval_c10_3_5_positioned_flex_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
#endif
/* Android backend: 0=Vulkan 실패 뒤 GL 순차 재시도, 1=Vulkan 강제, 2=GL 강제. iOS는 3=Metal. */
void *spinon_runtime_gpu_host_create_android(
    SpinonRuntimeGpuHost *host, void *native_window, uint32_t width,
    uint32_t height, uint32_t backend, char *output, size_t output_capacity);
/* UIKit surface 준비는 UIView.layer 접근 때문에 메인 스레드에서 호출합니다. */
int32_t spinon_runtime_gpu_host_prepare_uikit_surface(
    SpinonRuntimeGpuHost *host, void *view, uint32_t backend, char *output,
    size_t output_capacity);
/* 준비된 surface의 GPU 장치와 pipeline 초기화는 전용 render queue에서 호출합니다.
   UIKit 표면 구성은 다음 configure 함수를 메인 스레드에서 호출합니다. */
int32_t spinon_runtime_gpu_host_create_uikit(
    SpinonRuntimeGpuHost *host, uint32_t width, uint32_t height, char *output,
    size_t output_capacity);
/* UIKit CAMetalLayer의 surface 속성을 갱신하므로 메인 스레드에서 호출합니다. */
int32_t spinon_runtime_gpu_host_configure_uikit_surface(
    SpinonRuntimeGpuHost *host, char *output, size_t output_capacity);
/* Android는 render queue, UIKit은 메인 스레드에서 호출합니다. */
int32_t spinon_runtime_gpu_host_resize(SpinonRuntimeGpuHost *host,
                                       uint32_t width, uint32_t height,
                                       char *output, size_t output_capacity);
int32_t spinon_runtime_gpu_host_draw(SpinonRuntimeGpuHost *host,
                                     char *output, size_t output_capacity);
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST) && \
    SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST && \
    defined(SPINON_ENABLE_C124_METAL_SURFACE_READBACK_TEST) && \
    SPINON_ENABLE_C124_METAL_SURFACE_READBACK_TEST
int32_t spinon_runtime_gpu_host_request_c12_4_surface_pixel_for_test(
    SpinonRuntimeGpuHost *host, uint32_t x, uint32_t y, char *output,
    size_t output_capacity);
int32_t spinon_runtime_gpu_host_request_c12_4_surface_pixels_for_test(
    SpinonRuntimeGpuHost *host, const uint32_t *sample_xy, size_t sample_count,
    char *output, size_t output_capacity);
#endif
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST) && \
    SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST && \
    defined(SPINON_ENABLE_C04_RUNTIME_GPU_FAILURE_FIXTURE) && \
    SPINON_ENABLE_C04_RUNTIME_GPU_FAILURE_FIXTURE
/* 내부 검증 빌드에서 다음 draw 오류를 한 번 주입합니다. */
int32_t spinon_runtime_gpu_host_inject_next_draw_failure_for_test(
    SpinonRuntimeGpuHost *host, char *output, size_t output_capacity);
#endif
int32_t spinon_runtime_gpu_host_destroy_renderer(
    SpinonRuntimeGpuHost *host, char *output, size_t output_capacity);
void spinon_runtime_gpu_host_free(SpinonRuntimeGpuHost *host);
#endif

#ifdef __cplusplus
}
#endif

#endif
