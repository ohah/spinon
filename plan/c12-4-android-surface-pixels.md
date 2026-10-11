# C12.4 Android AVD 실제 surface 픽셀 비교 계획

**상위 계획:** [C12.4 쌓임 맥락](c12-4-stacking-context.md)
**비교 기준:** [고정 Chromium 154 fixture](../tests/fixtures/css/references/c12-4-stacking-context-v1.json)
**구현 전 실패 관점 검토:** [surface 픽셀 계획 검토](../spec/internal/evidence/c12-4-android-surface-pixels-plan-review-2026-10-11.md)
**상태:** Android API 37 AVD 검증 완료 · 실기기 검증은 범위 밖

## 목표와 범위

고정 Chromium screenshot의 내부 표본과 Android API 37 AVD의 실제 `SurfaceView` buffer를 `PixelCopy`로 비교한다. 실행은 2026-10-11에 완료했다. iOS Metal surface pixel readback은 [별도 계획과 8-case 결과](c12-4-ios-metal-surface-pixels.md)에서 완료했으며 C12.4 전체 지원은 미완료다.

- 전체 26개 fixture 중 지원 장면 21개, 원본 요소 64개, 고정 색상 표본 22개를 렌더하고 검사한다.
- 나머지 5개는 현재 Android runtime matrix에서 기대 진단과 함께 거부되는지 계속 확인한다. 거부 장면을 강제로 그리거나 픽셀 성공 수에 포함하지 않는다.
- 각 지원 장면은 별도 앱 프로세스와 새 `HostDocument`에서 실행한다. 이전 장면의 document, style cache, render snapshot을 재사용하지 않는다.
- 캡처는 실제 Android `SurfaceView`를 대상으로 하는 `PixelCopy`로 얻는다. macOS headless WGPU texture readback 및 Chrome `elementsFromPoint`는 대체 증거가 아니다.
- 실행 대상은 Android API 37 AVD `emulator-5554`다. 연결된 Android 실기기는 사용하지 않는다. ANGLE/SwiftShader software renderer 여부를 실행 metadata와 log로 기록하며 hardware GPU 성능이나 실기기 동작으로 일반화하지 않는다.

## 입력·기준·판정

1. 입력 fixture는 `tests/fixtures/css/c12/stacking-context-inventory.json`, 생성 앱 asset은 `build/spinon/bootstrap/c12-4-runtime-matrix.json`, 픽셀 oracle은 `tests/fixtures/css/references/c12-4-stacking-context-v1.json`이다. 실행 metadata에 inventory SHA-256, reference SHA-256, 앱 asset의 inventory hash를 보존하고 세 값이 다르면 실행을 실패시킨다.
2. 색상 기준은 고정 Chrome screenshot의 `deviceScaleFactor: 1` 표본 `expectedRgba`다. 표본 위치·expected color는 fixture inventory와 reference screenshot metadata 간에 먼저 일치하는지 검증한다.
3. 앱 runtime density와 PixelCopy bitmap의 실제 크기를 기록한다. 현재 AVD density 2.625를 기준으로 320×240 CSS surface는 840×630 physical pixels여야 한다. 기기 설정이 바뀌면 크기를 추정하지 않고 metadata에서 산출하며, CSS viewport 크기와 bitmap 간 비율이 일치하지 않으면 실패한다.
4. fixture 표본의 surface pixel 좌표는 `floor((fixture origin + local CSS coordinate + 0.5) × runtime density)`로 계산한다. fixture origin은 `(10, 10)` CSS px다. 비교기는 같은 공식으로 좌표를 독립 계산해 앱 로그 좌표와 대조한다.
5. 22개 표본 각각의 RGBA 채널 절대 오차가 최대 1 이내여야 한다. 평균 오차나 다수결로 개별 표본 실패를 숨기지 않는다. 불투명 배경 sample alpha는 255여야 한다.
6. 지원 장면 수·표본 수·case ID가 정확히 21·22·fixture에 선언된 집합과 일치해야 한다. 누락·중복·추가 case, PixelCopy 실패, 진단 오류, 잘못된 크기, stale capture 파일은 실패다.
7. 최종 산출물은 case별 surface PNG, raw Android log, pixel 비교 JSON, 기기·렌더러·density·build metadata, 입력 hash다. 모든 검사 뒤 산출물과 상태를 동기화한다.

## 구현 단계와 실행 결과

1. 생성된 matrix asset에 각 scene의 표본과 fixture origin을 포함하고 source hash 및 21/5 분류를 검증한다.
2. Android 앱에 단일 지원 scene 전용 internal intent를 추가한다. case ID를 고정 matrix allowlist에서만 찾고, 누락·지원 거부 case·잘못된 ID는 화면 렌더 전에 실패한다. 선택한 case는 새 HostDocument에 한 번 평가한다.
3. 첫 성공 draw 뒤 다음 UI frame 경계에서 `PixelCopy.request(SurfaceView, ...)`를 호출한다. surface 크기와 bitmap 크기를 검증하고, `ERROR_SOURCE_NO_DATA` 등 재시도 가능한 결과만 제한적으로 재시도한다. 결과를 고유 실행 ID가 포함된 cache 경로에 저장해 이전 PNG를 새 결과로 읽지 않게 한다.
4. 캡처 bitmap/저장 PNG에서 fixture 표본을 읽어 case ID, sample ID, 실제 x/y, RGBA를 로그로 남긴다. 캡처 실패, 범위 밖 표본, 결과 저장 실패는 성공 marker를 내지 않는다.
5. AVD 전용 실행 script가 고정 APK를 설치/실행하고 각 21개 case를 새 프로세스로 순차 실행한다. 실기기 serial이면 즉시 중단한다. case별 process ID 로그에서 현재 run marker를 기다린 뒤 PNG를 회수하고 이전 log/cache 결과는 사용하지 않는다.
6. 독립 Node 비교기가 고정 Chrome reference와 Android 표본을 case/sample 집합·좌표·색상·오차 기준으로 검증한다. 올바른 캡처에 대한 변형, 누락 표본, 오래된 run ID, 한 채널 초과, 잘못된 해상도에 대한 negative control이 반드시 실패하는지 확인한다.
7. [x] APK 빌드, runtime matrix 26/26, 새 surface 픽셀 비교 21/21 및 기존 workspace 검증을 실행했다. case별 로그와 자동 pixel 비교 결과를 보존했고, reverse fixture 캡처도 육안 확인했다.
8. [x] 상위 C12.4 계획, `spec/STATUS.md`, 내부 계약/evidence index와 Tailnet preview 원본을 결과에 맞게 동기화한다. GitHub Pages 배포, commit, PR, merge는 이 계획의 일부가 아니다.

## 실패·복구 기준

- AVD가 offline이거나 emulator가 아닌 serial이면 실기기로 대체하지 않고 작업을 중단한다.
- 잘못된 scene ID, unsupported scene, asset/reference hash 불일치, renderer 준비 실패, JS eval 실패, stale generation은 모두 캡처 전 실패로 남긴다.
- PixelCopy callback이 surface보다 오래된 세대를 가리키거나 bitmap의 가로·세로가 기대 surface와 다르면 그 캡처를 폐기하고 실행을 실패 처리한다.
- PixelCopy timeout/no-data는 제한된 횟수만 재시도한다. 최종 실패를 정상 색상으로 대체하거나 offscreen 결과로 바꾸지 않는다.
- 색상 차이는 먼저 runtime scale, target format/color space, surface generation, WGPU shader output 및 PixelCopy 변환을 조사한다. 근거 없이 tolerance를 넓히지 않는다.
- AVD가 ANGLE/SwiftShader일 경우 판정은 이 software path에만 한정한다. 별도 실기기 검증이 없다는 사실은 계속 남긴다.

## 완료 결과

- [x] 고정 입력 hash와 AVD metadata를 기록했고, `emulator-5554`/API 37 장치 guard를 통과했다.
- [x] 21개 지원 scene을 새 runtime host와 Android `SurfaceView`에서 그려 `PixelCopy` PNG를 회수했다.
- [x] 22개 표본의 좌표·RGBA 비교가 모두 통과했다. 최대 channel error는 0이다. comparator negative controls도 통과했다.
- [x] 기존 26-case runtime matrix에서 21 ready / 5 expected rejection 결과를 확인했다.
- [x] case별 PNG·raw log·comparison JSON을 보존하고 문서/evidence index와 Tailnet preview를 동기화했다.

실행: [비교 JSON](../spec/internal/evidence/c12-4-android-surface-pixels-2026-10-11/run-c124-20261011T100934321Z-68d7a5c4/comparison.json) · [reverse Flex 캡처](../spec/internal/evidence/c12-4-android-surface-pixels-2026-10-11/run-c124-20261011T100934321Z-68d7a5c4/flex-row-reverse-paint.png) · [raw case log](../spec/internal/evidence/c12-4-android-surface-pixels-2026-10-11/run-c124-20261011T100934321Z-68d7a5c4/flex-row-reverse-paint.log).

