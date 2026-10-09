# C04.3 Flex 정렬 전달 실행 근거

**날짜:** 2026-10-09  
**상태:** 내부 Rust fixture slice 구현·비교 완료  
**제품 지원:** 미완료 · 제품 runtime, GPU 화면, Android/iOS 실행 연결 없음

## 구현한 기능

새 `FlexAlignmentV1` cascade profile이 Stylo의 `align-items`·`justify-content` 계산값을 node별 `LayoutStyle`로 투영하고 Taffy에 전달한다. 지원 profile subset은 [내부 계약 0024](../0024-c04-flex-alignment.md)에 고정했다. 기존 C04.2 `FlexLayoutV1` entrypoint와 whitelist는 분리해 유지한다. `LayoutStyle` 기본 정렬값은 기존 `stretch`/`flex-start`를 유지한다.

미지원 computed 값, `align-self`·`align-content`·`place-items`, inline `style`, cascade 진단과 revision mismatch는 전체 요청을 실패시킨다. `place-items`가 CSS longhand로 확장되어 `justify-items`로 발견되는 실제 오류도 테스트에서 확인했다.

## Chromium 비교 기준

- Browser: Chrome `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`
- macOS `26.5.1` (`25F80`), arm64; Node `v24.20.0`
- viewport `301×40` CSS px, scale `1`, locale `en-US`, timezone `UTC`, media `screen/light`
- Chromium executable SHA-256: `ccffd5c5fe77ba3212e07c2c69091f8048b73c396b4647ecaa150e1e01a04954`
- 기준 수집기는 별도 Chromium 임시 profile과 headless process group을 사용하고, DevTools를 닫은 뒤 소유한 process group 종료를 확인한다.
- 16개 case에서 computed property 문자열을 정확 비교하고, 부모·세 자식 64개 frame의 각 x/y/width/height를 최대 절대 오차 `0.5 CSS px`로 비교한다. 평균 오차로 단일 frame 실패를 감추지 않는다.
- 관찰 범위: initial/`normal`, `stretch`, `center`, `flex-start`, `flex-end`; row·column; LTR·RTL; `space-between`, `space-around`, `space-evenly`.

고정 reference: [`c04-flex-alignment-v1.json`](../../../tests/fixtures/css/references/c04-flex-alignment-v1.json) · reference ID `chromium-darwin-arm64-Chrome-154.0.8037.98-245e40efcfab-b1f65c7f961c-4cfc56ce99dc-ccffd5c5fe77`.

| 입력 | SHA-256 |
|---|---|
| `tests/fixtures/css/c04/flex-alignment.v1.json` | `245e40efcfab0438330a6782f7b23dfefbeebf26238d58bfec24eef94bae28b8` |
| `tests/fixtures/css/c04/flex-alignment.html` | `7a44bb4d8d2c51eff7fceed896db18cbd999687e0a3c0df6a3d69b05715dc3a1` |
| `tests/fixtures/css/c04/style-layout-bridge.css` | `b1f65c7f961c3cd39a7841d921993c204408d86caf33c0b78c1970ab64b6ad2f` |
| `tools/css-reference/capture-c04-flex-alignment.mjs` | `4cfc56ce99dcfc3df03da76988c17dab16c935c073c76e9a72884df17e82b454` |
| `tools/css-reference/chromium-session.mjs` | `b010ada41ed75dc2d67ee25abd4266b17930f1de95fc683eebc5cdab6ad2cdc1` |
| Chromium reference JSON | `357e4def07283473decebaa6f9bb0e3ad7dfcdf4ab1567e6654de8e7dc88033b` |

## 실행 결과

| 명령 | 결과 |
|---|---|
| `mise exec -- node tools/css-reference/capture-c04-flex-alignment.mjs` | Chrome 기준 16 case·64 frame 수집, 기존 파일 덮어쓰기 없음 확인 |
| `mise exec -- node --test tools/css-reference/c04-flex-alignment.test.mjs` | reference schema, 입력·도구 hash, case/frame 구조 1/1 통과 |
| `mise exec -- cargo test --locked -p spinon-style-to-layout` | 12/12 통과. C04.2 기존 fixture, C04.3 16개 Chromium 비교, 미지원 값·property·inline style·진단·revision 경계 포함 |
| `mise exec -- bun run test` | 전체 통과: 예제, CSS reference, R05 터치 분석기와 Cargo workspace 테스트 |
| `mise exec -- cargo clippy --locked --workspace --all-targets -- -D warnings` | 경고 없이 통과 |
| `mise exec -- cargo fmt --all -- --check` | 통과 |
| `mise exec -- cargo check --locked -p spinon-style-to-layout --target aarch64-apple-ios-sim` | iOS Simulator 대상 컴파일 통과 |
| `mise exec -- cargo check --locked -p spinon-style-to-layout --target aarch64-apple-ios` | iOS 기기 대상 컴파일 통과 |
| `mise exec -- cargo check --locked -p spinon-style-to-layout --target aarch64-linux-android` | Android 대상 컴파일 통과 |
| `git diff --check` | 공백 오류 없이 통과 |

Android·iOS 앱의 화면 실행, 실기기 터치, 시뮬레이터 UI 검증은 이 내부 Rust adapter 변경에 포함하지 않았다. 위 교차 컴파일은 Rust target에서의 컴파일 가능성만 확인한다.

## 구현 후 적대 검토에서 확인한 실패 경계

| 검토 관점 | 확인 결과와 반영 |
|---|---|
| CSS initial value가 누락되어 parser가 기본값을 못 받음 | author declaration 없는 case에서 `normal` computed string과 expected initial layout을 비교했다. |
| `align-items: normal`을 CSSOM 값부터 `stretch`로 덮어씀 | computed string을 그대로 두고 Taffy 입력에서만 `stretch`로 정규화했다. |
| `justify-content: normal`을 `stretch`로 잘못 매핑 | 별도 case의 CSSOM 문자열과 frame이 `flex-start` 배치에 맞는지 확인했다. |
| auto cross-size에서 stretch가 적용되지 않음 | `normal` 및 `stretch`와 자동 높이를 둔 row case의 40 CSS px 높이를 비교했다. |
| `align-items:flex-start`가 cross-axis start가 아님 | row의 개별 자식 y 좌표를 Chromium과 비교했다. |
| `align-items:flex-end`가 cross-axis end가 아님 | row의 개별 자식 y 좌표를 Chromium과 비교했다. |
| `align-items:center`가 부모의 주축에 적용됨 | row case에서 자식 높이 10, y 15를 대조했다. |
| column의 교차축이 세로로 잘못 선택됨 | column case에서 x 130.5와 y 방향 순서를 Chromium과 비교했다. |
| LTR `justify-content:flex-start`가 시작점을 유지하지 못함 | x 0/45/90과 DOM child order를 비교했다. |
| `justify-content:flex-end`가 여유 공간을 잘못 계산 | x 171/216/261을 좌표별로 비교했다. |
| `justify-content:center`가 소수 여백을 반올림 | x 85.5/130.5/175.5를 허용 오차로 개별 비교했다. |
| `space-between`이 기존 column-gap을 여유 간격에서 잘못 처리 | gap 5를 포함한 x 0/130.5/261을 비교했다. |
| `space-around`의 양 끝 여백 비율이 잘못됨 | outer spacing과 child x 28.5/130.5/232.5를 비교했다. |
| `space-evenly`의 4분할이 정수로 반올림됨 | 분수 위치 x 42.75/130.5/218.25를 그대로 비교했다. |
| RTL `flex-start`가 LTR의 physical left에 고정됨 | Chromium/Taffy가 모두 x 261/216/171과 역순 시각 배치를 보였다. |
| flex 성장분이 justify 결과를 가려 비교가 무의미해짐 | computed grow/shrink를 0, basis를 고정해 자유 공간을 분리했다. |
| `baseline`을 임의의 중앙/끝 정렬로 대체 | Stylo의 computed 값은 보존하되 adapter가 node/property/value 오류로 거부한다. |
| overflow 안전 modifier를 버리고 unsafe 동작으로 진행 | `safe center` computed value를 허용하지 않고 전체 실패시켰다. |
| 논리 `start`를 `flex-start`로 가정 | `align-items`와 `justify-content`의 `start`를 모두 거부했다. |
| 개별 정렬 속성이 부모 값으로 섞여 들어옴 | `align-self`, `align-content`와 확장된 `place-items`를 whitelist 단계에서 실패시켰다. |
| shorthand 오류가 입력된 이름과 다르다고 실패 검사 누락 | 실제 Stylo 진단 feature `justify-items`를 확인하도록 검사를 수정했다. |
| 새 profile 값이 C04.2 호출자의 허용 범위를 넓힘 | C04.2 entrypoint에 `align-items`를 넣으면 거부되는 회귀를 확인했다. |
| inline style이 stylesheet profile을 우회 | 새 entrypoint에서 `style` 속성을 가진 HostDocument를 전체 실패시켰다. |
| 새 계산에서 revision stamp가 바뀌거나 다른 문서 view가 섞임 | nonzero style/environment revision echo와 foreign generation 거부를 새 entrypoint에서 확인했다. |
| fixture·도구가 바뀌어도 예전 Chromium 기준을 조용히 재사용 | Node 검사가 input·HTML·CSS·캡처기·DevTools helper hash와 case 순서를 대조한다. |
| Stylo parse 진단이 있어도 일부 프레임이 반환됨 | `display:grid` 진단 입력이 전체 `CascadeDiagnostic` 실패로 끝나는 테스트를 추가했다. |

위 표는 변경 코드와 실제 fixture를 검토한 실패 경계 및 수정사항이다. 별도 검토표는 [작업 계획](../../../plan/c04-flex-alignment.md)에 둔다.

## 남은 한계

CSS negative free space, flex wrapping/reverse, baseline, safe/unsafe overflow, writing mode, intrinsic measurement, 텍스트, 제품 UA stylesheet, dynamic invalidation, 앱 runtime, GPU 표시, Android/iOS 화면과 성능은 미검증이다. 공개 API 지원 수는 계속 0개다. C04.3 하위 fixture 완료는 C04 또는 S02 전체 완료를 뜻하지 않는다.
