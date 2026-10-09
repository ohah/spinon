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
/* Android emulator·iOS Simulator 내부 검증 fixture입니다. 실제 V8 DOM 변경 뒤 환경 setter,
   비동기 Stylo 결과와 JSON readback을 검증합니다. 성공 0, 인자 오류 -1, 생성 오류 -2,
   버퍼 부족 -3, cascade 검증 실패 -7을 반환합니다. 제품 렌더링 API가 아닙니다. */
int32_t spinon_runtime_ua_cascade_probe(char *output, size_t output_capacity);
void spinon_runtime_session_free(SpinonRuntimeSession *session);

#if defined(SPINON_ENABLE_C04_RUNTIME_GPU) && SPINON_ENABLE_C04_RUNTIME_GPU
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new(char *output,
                                                  size_t output_capacity);
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
#if defined(SPINON_ENABLE_C04_RUNTIME_GPU_FAILURE_FIXTURE) && \
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
