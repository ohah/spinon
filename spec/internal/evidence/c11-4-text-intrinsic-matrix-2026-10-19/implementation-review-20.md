# C11.4 텍스트 고유 크기 구현 적대 검토 20개

2026-10-19 현재 변경 코드와 두 simulator 실행 근거를 서로 다른 실패 경계 20개로 점검했다. 계획 검토를 재사용하지 않았으며, 이 문서는 구현과 실제 runtime assertion의 확인 결과를 기록한다.

## 관점별 결과

1. **scene 진입점 allowlist** — Rust fixture, Android, iOS의 C11.4 scene 목록이 같은 17개 ID를 허용한다. 그중 text 장면은 중복 없는 10개이며, 다른 C11 fixture route와 함께 들어오는 인수는 거부한다. 통과.
2. **문서 트리 소유권** — root 아래 단일 grid, 그 아래 단일 item, item 아래 direct-leaf text만 허용한다. 추가 형제·중첩 item·잘못된 parent는 layout 전에 실패한다. 통과.
3. **요소와 콘텐츠 신원** — root/item의 `div` tag와 fixture ID, 정해진 text 내용이 case table과 일치해야 한다. 임의 문서 내용이나 잘못된 ID를 해당 장면으로 해석하지 않는다. 통과.
4. **grid computed style** — `display`, box sizing, alignment, 크기, 여백·padding·border·gap을 제한한다. 지원하지 않은 box geometry를 조용히 측정하지 않고 거부한다. 통과.
5. **track 함수 선택** — `auto`, `min-content`, `max-content`, 고정 `fit-content(40px)`, calc 한 장면 이외의 열 track은 받지 않는다. 행 track·repeat·area·line name도 고정 계약을 벗어나면 거부한다. 통과.
6. **fit-content calc 분해** — `calc(10% + 5px)`에서 항이 정확히 하나의 10%와 하나의 5px인지 확인하고, 추가 항이나 다른 값은 통과시키지 않는다. 통과.
7. **텍스트 style 표면** — Ahem 10px, weight/style, line height, direction, line break, spacing, transform, writing mode 및 overflow-wrap을 computed style에서 확인한다. 측정에 영향을 주는 예상 밖 style은 fail-closed다. 통과.
8. **줄바꿈 모드** — normal, `break-all`, `nowrap`이 각각 `ParagraphStyle`과 computed property에 반영된다. fixture 세 입력에서 min/max-content width·height가 기대값으로 생성된다. 통과.
9. **box sizing과 가장자리** — content-box 장면의 좌우 padding·border와 border-box 장면을 각각 검사한다. 실제 item frame은 Chrome oracle과 10/10 일치한다. 통과.
10. **min/max width 경계** — `min-width:90px`와 `max-width:40px` 장면이 case별 값으로 제한되고, 그 외 min/max width 및 height는 기본값만 허용한다. 통과.
11. **font 결정성** — system font fallback 없이 fixture Ahem 파일을 등록해 측정한다. `fontRegistryRevision=1`이 두 앱의 구조화 입력과 전체 trace에 나타난다. 통과.
12. **intrinsic 결과 provenance** — width와 height의 min/max-content가 `gridItemIntrinsicInputs`에 text source와 revision을 붙여 직렬화된다. 두 플랫폼에서 10/10 장면의 네 측정값이 기대 입력과 일치한다. 통과.
13. **diagnostic trace 보존** — Columns·Rows sizing trace의 모든 chunk 번호를 복원하고 JSON을 파싱했다. 10/10은 정규화 후 의미상 같고, byte 차이 2개는 `AutomaticMinimum` Debug 공백뿐이다. 숫자·track stage·contribution 값은 바뀌지 않았다. 통과.
14. **C ABI scene 입력** — null, UTF-8이 아닌 ID, 64 byte 초과 ID, allowlist 밖 ID를 host 평가 전에 거부한다. host/output 인수도 경계에서 검사한다. 통과.
15. **C ABI pixel 인수** — 표본 수 1–64 제한, 곱셈 overflow, null 포인터, fallible allocation 및 serial renderer 호출 계약을 확인했다. feature 전용 C ABI 테스트가 잘못된 포인터·count 경로를 거부한다. 통과.
16. **Android intrinsic assertion** — 전체 report 문자열에서 숫자를 찾던 검사는 다른 trace 숫자로 잘못 통과할 수 있었다. 현재 구현은 진단 JSON을 파싱해 단일 grid/node/sourceOrder/kind와 네 dimension 및 font revision을 구조적으로 검사한다. 최신 AVD 10/10 run에서 통과.
17. **iOS intrinsic assertion** — JSON 문자열 내부의 escaped quote를 고려해 균형 object를 추출하고 JSONSerialization으로 같은 source identity와 측정값을 검사한다. 잘리거나 파싱할 수 없는 입력은 false가 된다. 최신 iOS Simulator 10/10 run에서 통과.
18. **native frame gate** — 두 앱이 root/item 크기뿐 아니라 x/y와 node 순서를 고정값과 비교한다. 10/10 scene에서 Chrome과 root/item frame 및 paint order가 정확히 일치했다. 통과.
19. **표면 표본 route** — Android PixelCopy와 iOS Metal drawable의 root 안 item, root 밖 item, root 배경 control 표본이 각각 3/3 exact다. Android 로그를 현재 JSON-input run ID로 갱신했다. iOS 수집기의 첫 predicate는 Metal marker를 누락했지만 Unified Log에 남은 현재 run의 3/3 기록으로 복구했다. 이는 수집 predicate 오류였고 앱 실행 실패가 아니다.
20. **fixture artifact 격리** — default release staticlib에는 C11.4 host/pixel symbol과 payload가 없고 opt-in fixture artifact에만 존재한다. 제품 기본 경로에 테스트용 surface API가 노출되지 않는다. 통과.

## 발견 및 조치

- 구현 점검 중 두 앱의 이전 intrinsic assertion이 report 전체의 문자열 포함 여부만 봐서 다른 diagnostic 숫자에 의해 거짓 통과할 수 있음을 확인했다. Android와 Swift 검사를 구조화 JSON의 단일 `gridItemIntrinsicInputs` 경로를 확인하도록 바꾸고 현재 simulator 빌드·10 scene 재실행에서 검증했다.
- 첫 iOS Metal 수집 스크립트는 `SPINON_C114_METAL_PIXEL` marker를 성공 predicate에 넣지 않아 timeout을 기록했다. 같은 run ID의 Unified Log에는 3개 표본과 `status=matched`가 남아 있었고 이를 evidence로 보존했다. README와 comparison JSON은 현재 JSON-input run ID를 사용한다.

## 근거

- 구현: `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_text.rs`, `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_text/cases.rs`, `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs`, Android/iOS C0410 fixture runner.
- 실행: `README.md`, `cross-platform-comparison.json`, `android-avd/runtime-*-verified.log`, `ios-simulator/runtime-*-verified.log`, `android-avd/pixel-matrix-min-content-text-break-all.log`, `ios-simulator/metal-pixel-min-content-text-break-all.log`.
- 검사: `workspace-test.log`, `workspace-clippy.log`, `rustfmt-check.log`, `ffi-c114-focused-tests.log`, `ffi-feature-clippy.log`, `artifact-boundary/` 및 두 simulator build 로그.

이 review는 고정 Ahem direct-leaf matrix 구현만 다룬다. 일반 font/script, nested/anonymous item, image contribution, glyph paint, WPT와 전체 C11.4 적합성은 완료하지 않는다.
