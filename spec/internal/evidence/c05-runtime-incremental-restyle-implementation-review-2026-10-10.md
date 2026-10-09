# C05.4 runtime 하위 트리 cascade 구현 검토와 실행 근거

- **계약:** [0036 C05.4 runtime 하위 트리 cascade](../0036-c05-runtime-incremental-restyle.md), 내부 계약 `0.1.0` 고정
- **계획:** [C05.4 구현 계획](../../../plan/c05-runtime-incremental-restyle.md)
- **계획 공격 기록:** [계획 검토](c05-runtime-incremental-restyle-plan-review-2026-10-10.md)
- **비교 모델:** [Chromium 사전 비교](c05-runtime-incremental-restyle-precomparison-2026-10-10.md)
- **코드 기준:** C05.3 결과 재사용 병합 직후 `ce879f4`; 이 검토에선 해당 기준 위에 구현한다.
- **V8 기준:** 저장소가 고정한 revision `7b50b62cb18f28617959e8452e2cd18195b38bcf`
- **판정 범위:** 제한된 inline style 증분 cascade와 Android API 37 emulator/iOS 26.2 Simulator 실제 V8·WGPU 경로. 제품 공개 CSS 범위, GPU hardware 성능, 실기기 지원은 판정하지 않는다.

## 구현 중 발견해 수정한 사항

1. iOS `AppDelegate`는 C05.4 launch argument를 runtime GPU 화면으로 보내지 않아 빈 기본 화면이 나왔다. `--spinon-c053-runtime-result-cache`와 `--spinon-c054-incremental-restyle`를 runtime GPU 진입 분기에 연결하고 다시 빌드·실행했다.
2. 첫 release benchmark fixture에서 root element를 HostRoot에 삽입하지 않아 장면이 비어 있었다. root 연결을 추가하고 결과에 element 수·전체 oracle 일치를 검증하게 했다.
3. benchmark의 cascade/projection 계측값은 이미 정수 microseconds인데 nanoseconds로 간주해 1,000배 작게 출력했다. 각 입력 단위로 백분위를 계산하도록 수정하고 세 프로세스 측정을 다시 저장했다.
4. 전체 snapshot diff, 재사용 계획 및 스타일 출력 조립은 O(N)인데 하위 트리만 처리하는 전체 경로처럼 오해할 수 있었다. 계획·계약·상태 대장에 Stylo 호출만 줄고 full snapshot walk/copy와 layout/scene은 전체라는 한계를 명시했다.
5. inline style 제거와 연결 author stylesheet가 있는 상태의 fallback 비교가 부족했다. 삭제 뒤 full oracle 일치 테스트와 author stylesheet 존재 시 전체 재계산 counter 테스트를 추가했다.
6. `cascade.rs`와 runtime classifier test 파일이 500줄을 넘었다. incremental cascade·요소별 style 계산·분류기/runtime 테스트를 책임별 모듈로 나눴다.

## 적대적 구현 검토

| # | 공격 관점 | 확인·수정 |
|---:|---|---|
| 1 | 기능 구현 중 제품·crate 버전이 올라갔는가 | 버전 파일은 바뀌지 않는다. Cargo crate 버전은 `0.1.0`; `package.json` 변경은 Chromium verifier script뿐이다. 계약 `0036`은 문서 ID다. |
| 2 | cascade 부분화를 전체 O(dirty subtree) 처리로 과장하는가 | 분류기와 출력 재조립은 전체 tree를 순회·복사하며 Taffy와 scene도 전체 계산한다. 계획·계약·상태 대장에 한계를 추가했다. |
| 3 | C05.4 iOS launch 인자가 빈 앱 화면으로 빠지는가 | 재현해서 원인을 찾았다. `AppDelegate` 진입 분기를 고치고 Simulator 화면을 다시 확인했다. |
| 4 | Android 확인 과정에서 연결된 실기기를 건드리는가 | 모든 설치·launch·로그·screenshot 명령은 `adb -s emulator-5554`로 지정했다. 연결된 실기기는 사용하지 않았다. |
| 5 | 단일 root가 아닌데 root 하나로 간주하는가 | 이전·현재 root count 및 root ID가 정확히 하나로 같아야 한다. 다른 경우 `Full`이다. |
| 6 | generation이 바뀐 뒤 같은 숫자 NodeId가 cache hit하는가 | classifier와 style cache 모두 generation을 검사한다. 세대 불일치는 전체 계산으로 간다. |
| 7 | owner·node kind·namespace·tag·element state 변화가 증분 경로에 들어오는가 | snapshot 비교에서 하나라도 달라지면 `Full`이다. |
| 8 | parent나 child 순서, attach/detach/move 변화로 selector·상속 결과가 낡는가 | parent 및 child handle 순서를 비교하고 다르면 전체 계산한다. reorder 테스트가 있다. |
| 9 | `class`, `id`, 기타 attribute나 text 변경을 style-only로 오인하는가 | inline style 이외 attribute·text·state 변화는 전체 계산한다. class와 text, order fallback 테스트를 확인했다. |
| 10 | 대소문자 변형이나 namespace가 있는 중복 `style` attribute가 모호하게 처리되는가 | HTML의 namespace 없는 이름만 ASCII 대소문자 무시로 비교하고, 중복·namespace 변경은 전체 fallback이다. 대소문자 동치와 중복 거부 테스트가 있다. |
| 11 | 연결 author stylesheet 또는 `<style>` 본문 변화가 partial 계산에 들어오는가 | 분류는 inline 변경을 표시해도 stylesheet가 수집되면 Stylo partial entrypoint가 거부하고 full cascade를 수행한다. 연결 stylesheet fallback 테스트를 추가했다. |
| 12 | 내장 UA CSS의 selector가 attribute/state/sibling dependency를 추가했는데 cache를 계속 쓰는가 | 고정 UA selector inventory test가 단순 type/namespace selector만 허용한다. 실패하면 부분 계산 불가 조건이다. |
| 13 | 상위 custom property 변경에서 깊은 descendant `var()` 값을 놓치는가 | dirty root의 전체 subtree를 Stylo cascade하고 ancestor computed context를 위에서 아래로 전달한다. full cascade와 NodeId별 style 비교가 있다. |
| 14 | 겹치는 변경 root가 중복 계산되거나 여러 sibling branch 중 하나가 빠지는가 | 현재 parent chain으로 가장 바깥 dirty root를 남긴다. 겹침·분리된 여러 root 테스트와 branch fixture를 확인했다. |
| 15 | style 제거가 이전 선언을 남기거나 `var()` fallback을 stale하게 만드는가 | style attribute 제거 뒤 partial/full style, diagnostics, frame이 같고 dirty branch만 재계산되는 테스트를 추가했다. |
| 16 | 이전 diagnostics를 복사해 제거한 parser 오류가 남는가 | diagnostics는 매 요청 현재 tree에서 다시 수집한다. partial 결과를 full oracle diagnostics와 비교한다. |
| 17 | pending request가 합쳐질 때 중간 revision을 잘못 기준으로 삼는가 | 이어진 revision만 가장 이른 baseline을 보존하고 gap은 force-full로 승격한다. coalescing 경계 테스트가 있다. |
| 18 | 이전 계산 중 도착한 새 request 뒤 stale 완료가 cache/published output을 덮는가 | `publish_result`가 최신 key를 먼저 검사하고 불일치 계산은 두 cache를 갱신하지 않는다. existing latest-wins tests와 source revision guard를 확인했다. |
| 19 | 오류·panic·layout 실패가 부분 출력 cache를 오염시키는가 | cascade 오류는 style cache를 비우고 panic은 worker 종료로 지역 cache를 drop한다. 임시 스타일 출력은 전체 계산 성공 전 교체하지 않는다. |
| 20 | reused counter를 GPU/frame 성능으로 해석하거나 platform fixture 불일치를 놓치는가 | counter는 Stylo 출력 요소 수다. 실제 WGPU 화면과 Chromium/Rust oracle을 따로 비교하고 cascade·projection·worker wall time을 나눠 기록했다. 16-node end-to-end 이득은 일관되지 않아 작은 tree 성능 향상으로 주장하지 않는다. |

## 고정 Chromium 비교와 Rust oracle

`mise exec -- bun run css:verify:c05-incremental-restyle-precomparison`은 pinned Chrome `154.0.8037.98`에서 before/after DOM의 computed width와 CSS px rectangle을 확인한다. 왼쪽 branch 자손은 `32px → 46px`, 오른쪽 branch의 `41px`와 x 좌표 `150px`는 그대로다. 좌표별 차이는 `0.5 CSS px` 허용치 안이어야 한다. [고정 reference](../../../tests/fixtures/css/references/c05-runtime-incremental-restyle-v1.json)와 Node hash test를 따른다.

Rust 테스트는 incremental 결과의 모든 `ComputedElementStyle`, diagnostics, layout frame을 같은 revision의 full-cascade oracle과 비교한다. inline style 제거 및 연결 author stylesheet 보유 조건도 시험한다. [사전 비교용 HTML](../../../tests/fixtures/css/c05/runtime-incremental-restyle.html), [브라우저 JavaScript](../../../tests/fixtures/css/c05/runtime-incremental-restyle.js), [실제 V8 JavaScript](../../../tests/fixtures/css/c05/runtime-incremental-restyle-runtime.js).

## Release worker 비교

세 번의 독립 release test process에서 각 장면별 30회 style mutation을 실행했다. full와 incremental 요청은 다른 runtime worker/document를 사용하고 실행 순서를 교대했다. NodeId별 style/diagnostics/frame이 매회 같지 않으면 benchmark는 실패한다. 아래 표는 세 프로세스에서 각 30-sample p50/p95를 낸 뒤 process-level p50/p95 값의 중앙값을 취했다.

| 연결 element 수 | Cascade full → incremental p50/p95 (µs) | Taffy·frame·scene projection full → incremental p50/p95 (µs) | Request wall full → incremental p50/p95 (µs) | 요소 수: full → incremental |
|---:|---:|---:|---:|---:|
| 16 | 73/105 → 73/115 | 20/39 → 20/40 | 105.4/168.4 → 106.1/178.7 | 16 → 2 재계산, 14 재사용 |
| 256 | 944/1014 → 839/903 | 419/481 → 406/486 | 1499.1/1649.4 → 1376.5/1558.9 | 256 → 2 재계산, 254 재사용 |
| 2048 | 7613/7861 → 6583/6959 | 4028/4123 → 4079/4244 | 12926.9/13200.6 → 11959.1/12442.3 | 2048 → 2 재계산, 2046 재사용 |

context 조상은 `reused` count에 들어간다. 각 partial 요청에서 그 수는 1이다. 측정은 Apple M4 Max / macOS 26.5.1의 Rust release worker다. request wall timer는 HostDocument mutation 및 snapshot 생성 뒤 `register_document_snapshot` 직전부터 cascade/layout 상태가 terminal이 될 때까지다. `projectionDurationUs`는 Taffy layout, frame mapping, renderer scene snapshot 생성의 합산 timer다. 실제 Metal/GLES GPU 제출, 화면표시, V8↔native input, 실기기 성능은 측정하지 않았다.

큰 tree에서 cascade p50와 request wall p50/p95는 세 process 모두 감소했다. 16-element request는 일관된 이득이 없고 p95도 변동한다. layout/scene은 전체 처리이므로 개선되지 않았다. 이 결과는 snapshot diff와 출력 조립 cost를 포함한 현재 전체 worker path에서의 비교일 뿐, 모바일 성능이나 프레임 향상 보장이 아니다. 원본 로그: [process 1](c05-runtime-incremental-restyle/release-process-1.log) · [process 2](c05-runtime-incremental-restyle/release-process-2.log) · [process 3](c05-runtime-incremental-restyle/release-process-3.log).

## Android·iOS Simulator 실제 V8 실행

각 앱에서 초기 화면을 띄운 뒤 실제 V8 fixture의 왼쪽 branch inline style을 버튼으로 바꾸고 WGPU scene을 다시 제출했다. 두 플랫폼 초기 snapshot은 `cascadeRecomputedStyleElements=5`, `cascadeReusedStyleElements=0`; 변경 후 snapshot은 `recomputed=2`, `reused=3`, `context=1`이었다. 변경 후 오른쪽 green branch는 그대로고 왼쪽 blue tile만 커진다.

| 플랫폼 | 빌드·실행 환경 | 근거 |
| --- | --- | --- |
| Android | API 37 ARM64 emulator `emulator-5554`; ANGLE/SwiftShader EGL 경로 | [초기 화면](c05-runtime-incremental-restyle/android-api37-initial.png) · [변경 화면](c05-runtime-incremental-restyle/android-api37-step-1.png) · [logcat](c05-runtime-incremental-restyle/android-api37.log) |
| iOS | iPhone 17 Pro / iOS 26.2 Simulator; Metal surface | [초기 화면](c05-runtime-incremental-restyle/ios-26.2-initial.png) · [변경 화면](c05-runtime-incremental-restyle/ios-26.2-step-1.png) · [unified log](c05-runtime-incremental-restyle/ios-26.2.log) |

설치와 입력은 Android emulator serial 및 booted iOS Simulator를 지정했다. Android 실기기는 이 변경에서 사용하지 않았다. 캡처는 실제 GPU device benchmark가 아니라 화면과 runtime result의 기능 근거다.

## 자동·수동 검증

| 확인 | 결과 |
| --- | --- |
| `mise exec -- cargo fmt --all -- --check` | 통과 |
| `git diff --check` | 통과 |
| `mise exec -- bun run css:verify:c05-incremental-restyle-precomparison` | 통과: Chromium before/after, sibling geometry 보존 |
| `mise exec -- node --test tools/css-reference/c05-runtime-incremental-restyle.test.mjs` | 통과: pinned fixture/reference hashes |
| `mise exec -- cargo test --locked --workspace --no-fail-fast` | 통과: 전체 workspace unit·doc tests |
| `mise exec -- cargo test --locked -p spinon-runtime --no-fail-fast` | 통과: runtime unit 113개, integration 19개; release-only benchmark 2개는 의도적 ignore |
| `mise exec -- cargo clippy --workspace --all-targets --locked -- -D warnings` | 통과 |
| `mise exec -- cargo check --locked -p spinon-ffi --all-features` | 통과 |
| `mise exec -- bun run test:css-reference` | 통과: 28개 |
| Android `build:android` + API 37 emulator install/launch | 최종 모듈 구조에서 빌드·실행 통과; 초기 5/0/0, 변경 2/3/1 |
| iOS `build:ios-sim` + iPhone 17 Pro / iOS 26.2 Simulator install/launch | 최종 모듈 구조에서 빌드·실행 통과; 초기 5/0/0, 변경 2/3/1 |
| release-only full vs incremental worker benchmark | 독립 process 3회 통과, 각 장면 30회, 모든 output/frame 비교 통과 |

실기기 성능, author stylesheet selector invalidation, partial Taffy/paint/GPU submission, 앱 전체 프레임 효율은 미검증이다.
