# R05.3 Android SurfaceView 표시 신호 대조 계획

**상태:** 계획·구현·Android 시뮬레이터 검증 완료 · 실제 표시 시각과 frame 귀속 미완료

## 목적

API 37.2 wgpu/Vulkan probe에서 listener callback은 수신했지만 `present_time_ns`가 31/31건 모두 `-1`이었다. 같은 Android API·에뮬레이터·앱 SurfaceView 신호 API를 쓰는 OpenGL ES 2.0 경로를 대조해, 현재 결과가 해당 AVD의 SurfaceView 표시 신호 제한인지 wgpu/Vulkan 경로에 국한된 현상인지 좁힌다.

이 대조는 성능 비교가 아니다. renderer 구현, GPU backend, 제출 동작이 다르므로 callback 존재·timestamp sentinel·VSync ID 제공 여부만 비교한다. 양쪽의 시간을 빼거나 렌더러 우열을 주장하지 않는다.

## 비교 모델

| 항목 | wgpu 대상 | OpenGL ES 대조 | 고정 조건 |
|---|---|---|---|
| OS/API | Android 17.2 / API 37.2 | 동일 | 동일한 `spinon_api37_2` ARM64 AVD, 동일 system image와 실행 설정 |
| View/surface | `R08WgpuSurface : SurfaceView` | `R08GpuSurface : GLSurfaceView : SurfaceView` | 앱·화면 크기·orientation·refresh mode·SurfaceView listener API 동일 |
| renderer | wgpu / Vulkan · `Goldfish GFXStream (llvmpipe)` | GLES 2.0 API · `ANGLE / Vulkan SwiftShader` | 두 가상 GPU stack이 다르므로 capability만 비교하고 성능 순위를 만들지 않음 |
| 입력 | 중앙 단일 탭 30회 × 3 독립 block | 중앙 단일 탭 30회 × 3 독립 block | 같은 block 안의 arm 순서를 WGPU/GLES, GLES/WGPU, WGPU/GLES로 교대 |
| 수집 | `R05PresentTimingProbe` | 같은 probe 인스턴스와 정책 | API capability, callback batch·record 수, VSync ID, present timestamp, callback thread, surface generation 기록 |

비교 키는 실행별 `api_full`, AVD/image, renderer/backend, surface generation, 입력 수, 제출 요청 수, callback record 수, `vsync_id` sentinel 분포, `present_time_ns` sentinel 분포다. callback 배열 순서나 callback 도착 시각으로 frame을 추정하지 않는다.

## 구현 범위

1. 기존 R08 OpenGL ES surface를 R05 전용 대조 모드로 실행할 수 있게 한다. 일반 R08 기본 동작은 유지한다.
2. wgpu와 GLES 경로 모두 같은 `R05PresentTimingProbe`를 사용하고, 모든 로그에 실제 renderer 이름을 기록한다.
3. GLES 탭에서 단일 포인터 `ACTION_UP`의 sequence·event timestamp를 기록하고, `requestRender()` 호출은 frame 제출 완료로 부르지 않는다. 로그 이름은 `draw_requested`로 구분한다.
4. `R05PresentTimingProbe`가 wgpu/GLES 두 surface와 Activity 재생성 전체에서 공유하는 process-wide 단조 generation allocator를 제공한다. 증가 overflow는 generation 재사용 대신 등록 실패로 닫는다.
5. listener executor의 무제한 대기열을 고정 용량 64의 단일 worker 대기열로 바꾼다. executor가 수락하는 단위는 callback batch task임을 명시한다. 꽉 찼을 때 caller-runs나 무기록 폐기를 하지 않는다. callback batch 거부와 flush task 거부 수를 각각 보존하고, 거부된 flush는 `queued=false`로 기록한다. rejection handler는 counter 갱신만 하고 제출자 thread에서 callback을 실행하거나 로그 I/O를 하지 않는다.
6. `R08GpuSurface`는 `SurfaceHolder.Callback`을 추가 등록해 GLSurfaceView 내부 callback을 대체하지 않는다. surface 생성/소멸에 probe 등록·제거를 붙이고 공유 generation allocator를 쓴다.
7. 별도 내부 포화 fixture로 worker를 잠시 고정하고 callback task 64개를 대기열에 채운 뒤 callback task와 stale-safe flush task를 각각 추가 제출한다. 수락·거부 회계가 정확한지 API 37.2 시뮬레이터에서 확인한다. fixture 종료 때 worker를 반드시 풀고 정상 callback 처리를 재확인한다.

## 판정 기준

- 대조의 필수 성공 조건은 API 37.2에서 두 renderer 모두 앱을 유지하고, arm별 3개 독립 block에서 각 block당 입력 30회·draw 요청과 callback 원시 record를 분리해 보존하는 것이다. 총 90회/arm의 입력 성공 여부를 합산하지 않고 block별 누락을 남긴다.
- `present_time_ns > 0`이면 해당 실행에서 public listener가 그 surface 경로에 유효 timestamp를 반환한 것으로만 기록한다. clock epoch와 revision/frame 상관이 입증되지 않았으므로 latency는 계산하지 않는다.
- `present_time_ns == -1`은 `unknown`, `0`은 `unset_or_not_presented`로 남긴다. VSync ID가 있더라도 Spinon `FrameId`로 취급하지 않는다.
- GLES만 양수 timestamp를 주면 wgpu/Vulkan 표면 제출 경로를 후속 조사한다. 두 경로 모두 unknown이면 현재 AVD/API 환경으로 actual-present 귀속을 증명하지 못한 결과다. 이것만으로 Android 기기 전체에서 API가 불가능하다고 일반화하지 않는다.
- callback queue 포화 fixture는 용량 64 수락, 초과 항목 1건 이상 명시적 거부, callback/flush 폐기 수 일치, worker 복구 후 후속 정상 작업 실행을 모두 만족해야 통과한다.
- API 36에서는 R05 listener를 등록하지 않고 GLES 화면·입력 경로가 유지되는 fallback smoke를 실행한다.
- 이 비교는 Android 전용이다. iOS 26.2 SDK의 `CAMetalDrawable`에는 표시 callback·timestamp 심볼이 없어 이전 native Metal type-check가 실패했으며, 이번 구현에서 iOS runtime 결과를 만들지 않았다.

## 실행 산출물

- 계획·구현 상태·API 37.2 실행 환경·각 arm의 원본 로그와 캡처·SHA-256 manifest.
- 계획과 구현을 서로 별도로 20개 실패 관점으로 검토한다. 실제 실행으로 확인한 행과 코드/계약 검토만 한 행을 구분한다.
- 결과와 한계를 저장소 `spec/STATUS.md`, 이 계획 및 R05.3 evidence에 반영한다. 개발 중 미리보기는 `allthatnba/spinon/roadmap.html`에 동기화하고 GitHub Pages는 배포하지 않는다.

## 계획 적대 검토 · 20개 독립 관점

| # | 실패 관점 | 계획에서 고정한 대응 |
|---:|---|---|
| 1 | native 대조군을 wgpu와 동등 성능 구현으로 오해 | 출력은 신호 capability 비교이며 성능 비교 금지로 명시 |
| 2 | API/AVD가 arm별로 달라짐 | 두 arm 모두 같은 API 37.2 AVD·image 사용 |
| 3 | GLES surface 유형이 다름 | `GLSurfaceView`가 `SurfaceView` 하위 타입인 점을 기록하고 실제 등록 객체를 로그에 남김 |
| 4 | listener 구현 차이가 renderer 결과에 섞임 | 두 arm에서 동일 `R05PresentTimingProbe`를 사용 |
| 5 | 로그 renderer label이 잘못됨 | 등록·flush·callback 로그에 인자로 받은 renderer를 일관 기록 |
| 6 | 화면 크기나 orientation 차이 | 같은 AVD, fullscreen surface, orientation 고정 및 surface 크기 기록 |
| 7 | refresh mode 변화를 숨김 | 실행 전후 refresh mode를 기록하고 달라지면 arm 비교를 무효화 |
| 8 | AVD 가상 GPU를 실기기 GPU로 일반화 | AVD renderer를 evidence에 남기고 hardware 성능 주장을 금지 |
| 9 | 빠른 첫 탭이 surface 생성보다 먼저 전달 | 앱 surface/listener ready 로그 후 안정 대기하고 입력 시작 |
| 10 | block별 탭 누락을 세 block 합계로 감춤 | arm별 3 block의 시도·input·draw 요청 수를 block별로 대조 |
| 11 | synthetic adb 입력을 물리 입력으로 분류 | 모든 표본을 synthetic으로 고정 |
| 12 | GLES `requestRender()`를 실제 present로 해석 | `draw_requested`로 기록하고 queue 요청과 present를 분리 |
| 13 | callback batch task 수를 JankData record 수로 잘못 세거나 frame 수라고 가정 | queue drop은 callback batch task 단위, 결과 record는 별도 count로 기록 |
| 14 | `-1` 또는 `0`을 0ms 성공으로 처리 | unknown·unset 상태를 분리하고 latency 계산 금지 |
| 15 | VSync ID를 Spinon frame identity로 간주 | 모든 callback에 `frame_id=unmatched`를 유지 |
| 16 | GLES와 wgpu의 generation이 충돌하거나 surface 회전 후 구 listener 결과를 새 세대에 연결 | 두 경로가 process-wide checked allocator를 사용하고 generation key·remove 요청·새 등록을 각각 기록 |
| 17 | callback queue가 무한히 증가하거나 batch와 record 손실 단위를 혼동 | 단일 worker + capacity 64, rejected callback batch task 수를 계수 |
| 18 | 포화 때 UI thread에서 callback을 실행하거나 거부를 숨김 | caller-runs 없이 callback/flush를 분리 계수, flush `queued=false` 기록 |
| 19 | 포화 실험 중 막은 worker를 풀지 못해 후속 test가 정지 | `finally`에서 release latch 해제, 후속 정상 task 복구 확인 |
| 20 | 한 platform/API arm 결과를 제품 전체 지원으로 과장 | 범위를 API 37.2 AVD capability 결과로 한정하고 iOS·실기기 결론 제외 |

### 검토 결과 반영

검토에서 GLES 대조를 성능 기준으로 오해할 수 있어 signal capability 비교로 범위를 제한했고, API 37.2 AVD 결과를 실제 기기에 일반화하지 않도록 했다. queue 포화는 수용 수만으로 합격 처리하지 않고 rejected callback·flush 수와 worker 복구까지 확인하도록 했다. 단일 30회 block은 일시적인 무수신을 놓칠 수 있어 3개 독립 block으로 늘리고 block마다 arm 순서를 교대한다. 이는 signal capability 확인을 위한 반복이며 성능 순위를 만드는 계획은 아니다.

## 실행 결과

최종 Android debug APK를 같은 Android 17.2/API 37.2 ARM64 AVD(`1080×1920`, 60 Hz)에 설치했다. 실행 순서는 WGPU/GLES, GLES/WGPU, WGPU/GLES였고 arm마다 30개 synthetic 중앙 탭을 세 block으로 반복했다. 모든 block에서 입력과 제출 요청은 각각 30/30이었다. WGPU는 각 block에서 31개 callback record를, GLES는 31·31·32개 GL draw와 callback record를 남겼다. 각 record의 `present_time_ns`는 전부 `-1`; positive timestamp는 두 renderer 모두 0개다. callback의 OS TID는 등록한 main thread TID와 달랐고 일반 arm에서 queue 거부는 0이었다.

GLES surface의 첫 연결에서 하위 클래스가 `GLSurfaceView`의 자체 `SurfaceHolder.Callback`을 덮어써 실제 GL thread가 그리지 않는 결함을 발견했다. callback을 별도 객체로 추가 등록하도록 바꿨고, 최종 실행에서 `onSurfaceCreated`, GL draw, 색상 변경을 확인했다. 초기 잘못된 화면과 로그는 [결함 재현](../spec/internal/evidence/r05-presentation-signal-probe-2026-10-08/gles_block_1-pre-callback-fix.log)으로 보존했다. 코드 검토에서는 generation counter 최대치가 예외를 던져 앱을 종료시킬 수 있음을 추가로 발견해 무효 sentinel을 반환하고 listener 등록만 생략하도록 수정했다. 최종 APK 재빌드 뒤 API 37.2 WGPU/GLES, API 36 fallback, queue saturation, 충돌 모드 smoke를 다시 확인했다.

queue 포화 fixture에서는 64개 callback task를 수용하고 추가 callback 및 flush task를 각각 거부했다. 거부 callback은 호출자 thread에서 실행되지 않았고, 거부 delta는 각 1, worker는 차단 해제 후 후속 작업을 처리했으며 queue depth는 0으로 돌아왔다. API 36 fallback에서는 listener를 등록하지 않은 상태로 GLES 입력 3/3·제출 3/3을 확인했다. 회전에서는 generation 1→2→3, 이전 두 listener의 제거 요청, 새 surface 등록을 확인하고 원래 AVD orientation 설정을 복구했다.

이는 API 37.2 AVD의 신호 capability 결과다. wgpu는 llvmpipe, GLES API는 ANGLE/SwiftShader를 사용했으므로 성능 대조로 해석할 수 없다. iOS simulator, 실기기, 광학 표시 시각, exact frame/revision 귀속은 측정하지 않았다. 원본 로그·캡처·최종 코드용 별도 실패 관점 검토는 [R05.3 후속 evidence](../spec/internal/evidence/r05-presentation-signal-probe-2026-10-08.md#api-372-android-gles-signal-control)에 있다. R05.3과 상위 R05는 미완료다.
