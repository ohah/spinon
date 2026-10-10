# C07.1 · 최소·최대 크기 구현 검토와 실행 근거

**상태:** 제한 runtime profile 구현·검증 완료 · 공개 CSS 지원 선언 아님 · 내부 계약 숫자 버전 `0.1.0` 유지

## 구현 실패 경로 점검

아래 항목은 계획 검토와 분리해 구현·테스트·플랫폼 경계를 다시 살핀 기록이다. 기준은 [고정 Chromium 사전 비교](./c07-1-min-max-sizing-precomparison-2026-10-10.md), [계획 검토](./c07-1-min-max-sizing-plan-review-2026-10-10.md), 실제 소스와 실행 결과다.

| # | 공격 입력·경계 | 확인한 실패 조건 | 결과와 증거 |
| ---: | --- | --- | --- |
| 1 | `width:60px; min-width:90px` | 최소 너비가 지정 너비보다 작거나 사라짐 | width `90px`; Rust frame이 Chrome과 일치. |
| 2 | `width:120px; max-width:90px` | 최대 너비가 무시되거나 잘못된 축에 적용 | width `90px`; Rust frame이 Chrome과 일치. |
| 3 | `height:20px; min-height:55px` | 최소 높이가 너비에 연결되거나 projection에서 누락 | height `55px`; Rust frame이 Chrome과 일치. |
| 4 | `height:80px; max-height:55px` | 최대 높이가 부모 너비 기준 또는 무제한 처리 | height `55px`; Rust frame이 Chrome과 일치. |
| 5 | `min-width:80px; max-width:50px` | Taffy projection에서 max가 min을 덮음 | Chrome·Rust 모두 width `80px`. |
| 6 | content-box, max `100px`, 좌우 padding `10px` | content-box 제한을 border-box 전체 크기에 잘못 적용 | 바깥 frame width `120px`, Chrome과 일치. |
| 7 | border-box, max/min `100px`, 좌우 padding `10px` | padding을 중복 포함하거나 제약에서 누락 | 상한과 하한 모두 frame width `100px`, Chrome과 일치. |
| 8 | border-box min `10px`, 좌우 padding `12px` | 음수 content box 또는 padding보다 작은 frame 생성 | frame width `24px`, Chrome과 일치. |
| 9 | 부모 width `200px`, `min-width:50%` | percentage가 viewport나 border-box 기준으로 해석 | min width `100px`, 부모 content width 기준과 일치. |
| 10 | 부모 width `200px`, `max-width:75%` | percentage 최대 크기 분율이 소실 | max width `150px`, Chrome과 일치. |
| 11 | 부모 height `120px`, `max-height:50%` | 세로 percentage가 부모 너비 또는 잘못된 indefinite basis 사용 | max height `60px`, Chrome과 일치. |
| 12 | `min-width:calc(20px + 15%)` | 계산 AST 또는 min-width property binding 누락 | definite `200px` 부모에서 width `50px`. |
| 13 | `max-width:min(120px, 40%)` | nested math의 percentage basis가 바뀜 | max width `80px`; 해당 속성 AST 연결을 확인. |
| 14 | custom property `var(--minimum)`으로 계산한 min | cascade 승자와 layout 수식이 다른 선언에서 옴 | width `60px`; `min-width` AST가 Stylo snapshot에 남음. |
| 15 | 0 크기 경계 | 0을 기본 auto/none이나 무효값으로 취급 | zero min/max가 보존되고 Chrome 기준과 일치. |
| 16 | 유효 min/max 뒤 음수 CSS 선언 | 잘못된 선언이 앞선 유효값을 지움 | CSS cascade는 음수 선언을 무시하고 앞선 값 `25px`/`100px` 유지. |
| 17 | min `auto`, max `none` | 서로 다른 CSS 의미가 내부 DTO에서 섞이거나 max가 0이 됨 | Stylo 값은 `Auto`/`None`으로 구분, Taffy 제한은 각각 자동/무제한. |
| 18 | Flex shrink: basis `100px` 두 개, 부모 `150px`, min `80px` | shrink 단계가 최소값을 무시 | 각 항목 width `80px`, overflow는 Chrome과 동일. |
| 19 | Flex shrink에서 각 `min-width:0` | 0이 auto minimum으로 바뀌어 줄어들지 않음 | 각 항목 width `75px`, Chrome과 일치. |
| 20 | Flex grow 두 항목에 `max-width:60px` | grow가 max를 초과하거나 남는 공간 위치가 달라짐 | 각각 width `60px`, 두 번째 x `140px`, Chrome과 일치. |

## 추가 경계 검토

- 수식 ID가 없거나 노드·property가 일치하지 않으면 부분 결과를 내지 않고 전체 layout을 실패시킨다. min/max 네 property 모두에 대한 binding negative test가 통과했다.
- `min-content`, `max-content`, `fit-content` 키워드는 노드·속성 문맥 오류로 fail-closed 한다. 음수, NaN, 무한대, 음수 percentage를 넣은 layout DTO는 Taffy 호출 전에 거부한다.
- 새 제약은 실제 runtime layout profile에서만 투영한다. 기존 profile은 기본 무제약 값을 유지한다. CSS inline allowlist와 stylesheet cascade가 같은 네 longhand를 허용하는지 별도 테스트했다.
- 고정 Chrome 154 기준 35개 노드의 `x/y/width/height`를 DPR 1·2에서 모두 대조했다. 양쪽 DPR 모두 최대 절대 오차 `0 CSS px`이며 computed typed 값과 frame이 DPR에 따라 바뀌지 않았다.
- Android API 37 emulator와 iPhone 17 Pro / iOS 26.2 Simulator에서 별도의 V8 JavaScript fixture를 실행했다. 양쪽 모두 `SPINON_C071_EVAL status=0`, `layout=ready`, `boxes=10`, `301×100 CSS px` 장면을 표시했다. 화면은 [Android](./c07-1-min-max-sizing/android-api37.png), [iOS](./c07-1-min-max-sizing/ios-26.2.png), 실행 로그는 [Android](./c07-1-min-max-sizing/android-c071-log.txt), [iOS](./c07-1-min-max-sizing/ios-log.txt)다. Android eval 한 줄은 JNI와 Java 로거 양쪽에서 출력되어 로그에 두 번 기록된다.
- 모바일 runtime log는 장면 준비 여부와 root frame만 제공하며 35개 oracle fixture의 노드별 mobile frame을 계측하지 않는다. 따라서 Chrome/Rust의 35-node 수치 비교를 모바일의 노드별 동일성으로 확대하지 않는다.
- 시뮬레이터 결과는 host simulator의 WGPU 경로 통합 확인이다. Android 실기기, 실제 GPU 성능, 텍스트 intrinsic minimum, 브라우저 전체 CSS 지원을 검증하지 않았다.

## 수정한 결함

1. 기존 layout 타입을 `style.rs`로 분리할 때 `LayoutJustifyContent`의 root 재수출이 빠져 컴파일이 실패했다. compiler error에 따라 재수출 목록을 복구했고 workspace 검사에 포함했다.
2. Stylo가 `min-height: calc(10px + 2px)`의 computed serialization을 `12px`로 내보내는 점을 테스트 기대값에 반영했다. 계산용 typed AST가 별도 map에 보존되는지 확인해 computed string과 layout 수식을 같은 표현으로 오해하지 않도록 했다.
3. C07.1 runtime 화면 검증 전에는 공통 Rust 비교만 완료되어 있었다. C07 전용 JS fixture와 FFI·JNI·Android launch flag·Obj-C runner·iOS launch argument를 잇고 두 simulator에서 실제로 실행했다.

## 실행 결과

| 검사 | 결과 |
| --- | --- |
| `cargo test --locked --workspace` | 통과, 239개 단위 테스트 및 doc-test |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` | 통과 |
| `cargo check --locked -p spinon-ffi --all-features` | 통과 |
| `cargo fmt --all -- --check` | 통과 |
| `node --test tools/css-reference/c07-min-max-sizing.test.mjs` | 통과, 6/6 |
| `mise exec -- env SPINON_ENABLE_C04_RUNTIME_GPU=1 tools/build-android-app.sh` | 통과, Android API 37 emulator용 debug APK |
| `SPINON_ENABLE_C04_RUNTIME_GPU=1 tools/build-ios-sim.sh` | 통과, iOS Simulator build |
| Android API 37 `sdk_gphone16k_arm64` | C07.1 fixture eval·layout·WGPU 화면 통과 |
| iPhone 17 Pro / iOS 26.2 Simulator | C07.1 fixture eval·layout·WGPU 화면 통과 |

crate·번들·내부 계약 숫자 버전은 `0.1.0`으로 유지했다. 이 slice 완료는 C07 전체 또는 공개 웹 CSS 지원 완료를 뜻하지 않는다.
