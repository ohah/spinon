# C11.4 parsed Root HTML implicit auto-row content alignment

**상태:** 제한 구현 candidate. 계획 PR [#1236](https://github.com/ohah/spinon/pull/1236)이 병합되어 같은 fixed Root HTML Grid profile 안에서 computed `align-content`의 위치·분배 정렬값을 검증하는 구현을 진행했다. `normal|stretch`는 기존 동작이며 새로운 값은 고정된 세 direct-item 형상에서만 통과시킨다. 전체 C11.4는 미완료다.

**구현 전 비교:** [HeadlessChrome 157 관측·독립 계산](../spec/internal/evidence/c11-root-html-auto-row-content-alignment-precomparison-2026-11-07/README.md) · [기대값](../spec/internal/evidence/c11-root-html-auto-row-content-alignment-precomparison-2026-11-07/expected-values.json) · [Agent Browser 관측](../spec/internal/evidence/c11-root-html-auto-row-content-alignment-precomparison-2026-11-07/chrome-observation.json) · [화면](../spec/internal/evidence/c11-root-html-auto-row-content-alignment-precomparison-2026-11-07/chrome/align-content-start.png)

## 결손과 목표

기존 `RootHtmlAutoRowStretchV1`은 computed `align-content:normal|stretch`만 통과한다. 이 scene의 initial row bases는 `[10,10,10]px`이며 definite 40px block axis에 free space 10px이 남는다. `normal|stretch`는 CSS Grid §11.8에 따라 두 implicit `auto` max tracks에 5px씩 더한다. Positional 및 distributed values는 auto tracks를 늘리지 않고 CSS Box Alignment의 content-distribution에 따라 track group을 40px 안에서 배치한다. Chrome 157의 관측과 독립 계산은 [precomparison evidence](../spec/internal/evidence/c11-root-html-auto-row-content-alignment-precomparison-2026-11-07/README.md)에 고정했다.

목표는 기존 제한 profile에서 아래 computed `align-content` 값들을 이 고정 형상에 한해 허용하는 것이다.

- `normal`, `stretch`
- `start`, `flex-start`, `end`, `flex-end`, `center`
- `space-between`, `space-around`, `space-evenly`

`flex-start`는 Grid에서 `start`, `flex-end`는 `end`와 같은 content-position 대체 동작을 사용한다. 이 값들은 하나의 root Grid, 한 explicit 10px row와 두 implicit `auto` rows, direct text `A`/`B`의 현재 고정 형상에서만 지원한다. 새로운 public CSS admission, runtime profile, API, FFI 또는 contract version은 추가하지 않는다.

규범 근거는 [CSS Grid Layout Module Level 1, 2025-03-26 CRD §§10.5, 11.8](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/) 및 [CSS Box Alignment Module Level 3 §4.1 positional alignment](https://www.w3.org/TR/css-align-3/#positional-values), [§4.3 distributed alignment](https://www.w3.org/TR/css-align-3/#distribution-values)다. CSS Grid §10.5는 grid tracks를 정렬 subject로 두고, §11.8은 `normal|stretch`의 auto-track growth를 규정한다. Chrome은 고정 관측자이며 기대값은 track sizes와 content-distribution 계산에서 독립적으로 산출한다.

## 고정 입력과 독립 기대값

Canonical source는 기존 [`root-html-multiple-auto-row-align-content-stretch-precomparison.html`](../tests/fixtures/css/c11/root-html-multiple-auto-row-align-content-stretch-precomparison.html), SHA-256 `44a3423cd3be20935b2436d7f9f3701e580e6d39205b445b5b2a8158ca3e7308`다. Grid content box는 40×40px, column은 40px, explicit row는 10px, `grid-auto-rows:auto`, gaps 0, row auto-flow, `align-items:normal`이다. 자식은 source order대로 fixed 40×10px empty item과 10px/10px monospace direct text `A`, `B` 두 개다. HeadlessChrome 157.0.0.0, Agent Browser 0.31.1, viewport 1280×633 CSS px, DPR 1에서 같은 document의 computed `align-content`만 순서대로 바꿨다.

두 text item은 각각 10px block-size minimum contribution을 가지므로 initial rows는 `[10,10,10]px`다. 남은 definite free space 10px에 대한 독립 기대값은 다음과 같다.

| Computed `align-content` | Used rows (px) | Child y positions (px) | 계산 |
|---|---|---|---|
| `normal`, `stretch` | `[10,15,15]` | `[0,10,25]` | 두 implicit auto max tracks에 각 5px stretch |
| `start`, `flex-start` | `[10,10,10]` | `[0,10,20]` | start edge, 남은 10px은 뒤에 둠 |
| `end`, `flex-end` | `[10,10,10]` | `[10,20,30]` | end edge, 앞에 10px 둠 |
| `center` | `[10,10,10]` | `[5,15,25]` | 10px을 양 끝에 절반씩 둠 |
| `space-between` | `[10,10,10]` | `[0,15,30]` | 두 내부 간격에 각 5px |
| `space-around` | `[10,10,10]` | `[5/3,15,85/3]` | 세 subject 주변에 같은 10/3px 간격, 외부는 절반 |
| `space-evenly` | `[10,10,10]` | `[2.5,15,27.5]` | 네 외부·내부 간격에 각 2.5px |

각 child frame의 x/width/height는 `[0,*,40,10]`이다. Agent Browser는 `space-around` positions `[1.65625,14.984375,28.3125]`를 관측했다. 이는 1/64 CSS px layout quantization 뒤의 값이며 독립 기대값과의 허용 차이는 각 좌표 최대 `1/32 CSS px`다. 나머지 표의 geometry는 CSS pixel 정확한 유리수/정수다. Computed serialization, browser 관측값과 renderer 한계는 machine-readable evidence로 보존한다.

## 제한과 거부 경계

- 기존 `RootHtmlAutoRowStretchV1` layout mode와 default parsed Root HTML runtime route만 재사용한다. Scene source, root owner, document/style revision, child topology, fixed dimensions, row/column tracks, gaps, writing direction, `align-items`, `justify-content`와 `justify-items` 검증을 약화하지 않는다.
- `align-content`는 위의 정확한 computed keyword set만 받는다. `baseline`, `first baseline`, `last baseline`, `safe|unsafe` modifier, `self-start`, `self-end`, CSS-wide 값 중 allowlist 밖의 computed value 및 CSS math는 계속 거부한다.
- `normal|stretch`만 auto-row stretch stage를 적용한다. 다른 허용 값은 row bases `[10,10,10]px`를 유지하고 track placement만 정렬한다. Trace에서 auto-track stretch stage의 적용 여부와 최종 positions를 분리 확인한다.
- Shorthand `place-content`는 computed `align-content`와 `justify-content` 각각을 검사한다. `justify-content`를 통해 새 축 지원이 새어 나오지 않게 한다.
- 다른 Root HTML intrinsic, mixed, flex, Grid fixture profile에 이 허용 범위를 전파하지 않는다. 이 한 장면을 일반 CSS admission, arbitrary children, non-zero gaps, other writing modes 또는 전체 C11.4 지원으로 표현하지 않는다.
- 변형 style/source, unsupported alignment, missing/extra/reordered children, stale revision 및 실패 뒤 partial layout/frame/draw가 없는지를 확인한다. 같은 RuntimeSession에서 unsupported style 뒤 valid style 요청이 성공해야 한다.

## 구현과 수용 관문

1. Style-to-layout admission과 runtime Root HTML candidate gate가 computed values 10개를 동일하게 허용해야 한다. Layout validator도 `normal|stretch`와 다른 8개 positioning/distribution values를 구분하고, 기존 generic intrinsic profile은 유지한다.
2. Focused layout test는 10개 값 모두에서 rows와 모든 child frames를 독립 기대값에 대조한다. `StretchAutoTracks` stage는 `normal|stretch`에서만 적용돼야 한다. Style projection 및 RuntimeSession은 전 값을 통과시키고, baseline/safety modifier/wrong axis·shape를 거부해야 한다.
3. Rejection은 frames, track result와 draw list를 부분 발행하지 않고 동일 session의 valid scene 회복을 보장한다. `place-content` shorthand의 content/justify 축 분리와 computed value 사용을 검사한다.
4. Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator에서 동일 source hash의 default Root HTML route를 실행한다. 최소한 `align-content:start`와 `space-between` scene을 각 platform에서 확인해 positioned/distributed alignment의 rows `[10,10,10]` 및 expected frames를 기록한다. Screenshot은 visual smoke이며 별도 표면 readback 없이 pixel equality를 주장하지 않는다.
5. Rust focused suites, affected crate Clippy, `cargo fmt --all -- --check`, simulator builds/runtime, release artifact fixture-boundary positive/negative controls 및 `git diff --check`를 실행한다. 실행하지 않은 CI/platform 결과는 통과로 기록하지 않는다.
6. 구현/runtime은 plan review와 겹치지 않는 새 실패 관점 20개로, 구현 PR diff는 별도 20개 관점으로 검토한다. 발견 결함을 수정하고 영향을 받은 항목을 재검증한다.
7. 구현 merge 뒤 내부 contract `0059`는 `0.1.0`을 유지하고, C11 plan·`spec/STATUS.md`·public roadmap과 지정 Tailnet preview를 실제 결과와 맞춘다. C11/C11.4 상위 check와 공식 완료 수 `78/214`는 유지한다.

## 구현 candidate 결과

- Computed allowlist 10개와 layout track/frame oracle을 연결했다. `normal|stretch`만 `StretchAutoTracks`를 적용하고 나머지 8개 값은 auto-row base `[10,10,10]px`를 유지한다.
- Style projection과 Root HTML runtime candidate, 두 layout validation gate를 같은 제한값으로 맞췄다. 두 text-row profile 밖의 기존 single-text shape는 positional alignment를 계속 거부하며 `place-content`의 `justify-content` 축도 허용값을 넓히지 않는다.
- Layout 1/1 및 auto-placement 92/92, multiple-row RuntimeSession 5/5, single-row regression 3/3, 전체 Root HTML auto-row group 24/24, style allowlist 1/1, scoped Clippy와 rustfmt가 통과했다. 전체 layout/style-to-layout suite에는 범위 밖 실패가 남아 있어 실행 로그와 test 이름을 [runtime evidence](../spec/internal/evidence/c11-root-html-auto-row-content-alignment-runtime-2026-11-07/README.md)에 구분해 기록했다.
- Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 동일 fixture hash로 `start`와 `space-between`을 각각 실행해 layout-ready, 6 boxes, 2 text draws 및 자산 복원을 확인했다. 화면 캡처는 visual smoke이며 PixelCopy/Metal surface readback은 아니다.
- 기본 release runtime/FFI artifact에는 새 unit-test marker가 없고 unit-test executable에는 marker가 있다. 구현/runtime 실패 관점 20개는 [검토 기록](../spec/internal/evidence/c11-root-html-auto-row-content-alignment-runtime-2026-11-07/implementation-runtime-review-20.md)에 있다.

이 결과는 제한 구현 candidate일 뿐이며 C11.4 또는 C11 전체 완료를 뜻하지 않는다. 공식 완료 `78/214`와 contract `0.1.0`을 유지한다.

## 제외 범위

Baseline 정렬, safe/unsafe modifier, `align-content` CSS math, 0이 아닌 gutter와 collapsed gutter, overflow, static이 아닌 Grid, 가변 root/item 크기, 변경된 track sizing, 여러 Grid, 임의 Root HTML/CSS admission, WPT, 실기기, hardware GPU, iOS compositor 이후 픽셀, C11.4 전체와 C11 완료는 제외한다.

## 계획 단계 리뷰

- 계획 실패 관점: [20개](./c11-root-html-auto-row-content-alignment-plan-review-20.md).
- 계획 PR 변경분 실패 관점: [20개](./c11-root-html-auto-row-content-alignment-plan-pr-diff-review-20.md).
