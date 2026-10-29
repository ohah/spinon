# C11.4 · 자동 반복 `minmax(inflexible, fixed)` 트랙

**상태:** 구현 전 계획이다. Agent Browser HeadlessChrome 156의 49-scene 사전 비교와 별도 계획 실패 관점 20개 검토를 완료했다. 코드는 아직 변경하지 않았다. C11.4와 공식 집계 `72/213`은 미완료·유지 상태다.

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

## 목표

CSS Grid 자동 반복 문법의 두 번째 `<fixed-size>` 형식인 `minmax(<inflexible-breadth>, <fixed-breadth>)`를 기존 C11.4 opt-in profile에 연결한다. 반복 조각 안의 최소값은 `auto`, `min-content`, `max-content` 중 하나이고 최대값은 definite한 양의 `LengthPx`, definite 기준 크기를 가진 `Percentage`, 또는 현재 타입 계약이 허용하는 양의 길이 + 음이 아닌 백분율 `calc()`다. 열·행, `auto-fill`·`auto-fit`, 단일·복수 트랙 조각을 확인한다.

현재 layout 사전 검사 `auto_repeat_track_has_fixed_minimum()`은 반복 트랙의 최소값이 fixed size인 경우만 허용한다. 따라서 `minmax(auto, 40px)`, `minmax(min-content, 40px)`, `minmax(max-content, 40px)`는 Stylo가 계산값으로 보존해도 Taffy에 도달하기 전에 fail-closed한다. Taffy 0.15.0의 `TrackSizingFunction::has_fixed_component()`와 자동 반복 횟수 경로는 최대 측 fixed breadth를 계산할 수 있지만, 그것만으로 Spinon의 입력·축 기준 크기·scene 검증을 통과했다고 보지 않는다.

이번 작업은 위 고정 최대값 문법 분기의 제한 profile 구현이다. 임의 CSS author admission, indefinite 백분율 기준 크기, 임의 calc AST, flexible 최대값을 가진 잘못된 `<fixed-size>`, 여러 자동 반복 구성 요소, line-name 충돌, WPT 전체 실행과 C11.4 완료는 주장하지 않는다.

## 규범 기준과 독립 기대값

CSS Grid Level 1 2025 CRD §7.2.3.1의 `<fixed-size>`는 `<fixed-breadth>`, `minmax(<fixed-breadth>, <track-breadth>)`, `minmax(<inflexible-breadth>, <fixed-breadth>)`로 정의된다. `<inflexible-breadth>`에는 `auto`, `min-content`, `max-content`가 있고 `<fixed-breadth>`는 음수가 아닌 길이-백분율이다. §7.2.3.2는 preferred axis가 definite일 때 definite maximum track breadth로 자동 반복 수를 계산하고 트랙 사이 gap을 포함한다. `auto-fit`은 배치 후 빈 반복 트랙과 연결 gutter를 접는다.

고정 기준은 [49-scene 사전 비교](../spec/internal/evidence/c11-4-auto-repeat-inflexible-fixed-maximum-precomparison-2026-10-29/README.md)와 [fixture](../tests/fixtures/css/c11/grid-auto-repeat-fixed-maximum-c114-precomparison.html)다. 독립 계산은 반복 횟수 `n`이 다음 조건을 만족하는 가장 큰 양의 정수라는 것이다.

`n × repeated-pattern breadth + (n × repeated-track-count − 1) × gap ≤ definite content size`

| 입력 | 계산된 repeated breadth | definite axis | 예상 `auto-fill` 확장 |
| --- | ---: | ---: | --- |
| `minmax(auto, 40px)`, 1-track pattern | 40px | 129px / 130px | 2 / 3 tracks |
| `minmax(min-content, 30%)`, 1-track pattern | 39px | 130px | 3 tracks; 2 gaps 합계 10px |
| `minmax(max-content, calc(15px + 25%))`, 1-track pattern | 47.5px | 130px | 2 tracks |
| `minmax(auto, 40px) 20px`, 2-track pattern | 65px per pattern including internal gap | 135px | 2 patterns / 4 tracks |

모든 `<inflexible-breadth>` × maximum breadth 조합을 행·열 양축과 `auto-fill`·`auto-fit`으로 측정했다. item 하나를 둔 `auto-fit` 대조군은 차 있는 첫 트랙만 유지하고 나머지 반복 트랙과 gutter를 접는다. 10px 축에 20px item과 10px 최대값을 둔 대조군으로 intrinsic base size가 선언된 fixed maximum보다 큰 20px를 유지하는지도 확인한다.

## 구현과 검증 범위

1. `GridTrackSize::MinMax` layout preflight에 두 번째 valid fixed-size branch를 추가한다. Minimum은 정확히 `Auto | MinContent | MaxContent`만 허용하고 maximum은 `LengthPx | Percentage | Calc`만 허용한다. Fraction max, auto max, 잘못된 min/max 조합은 새 예외로 통과시키지 않는다.
2. Percentage 최대값은 해당 축에 definite한 양의 content-box 기준 크기가 있고 계산 결과가 유한할 때만 허용한다. Calc 최대값은 현재 `LayoutCssMath` 소유자/속성 identity와 기존의 정확한 2항 `length + percentage` 계약을 다시 검사하고, 임의 AST를 허용하지 않는다. 계산에서 0/max-below-min 바닥값과 UA의 1px 자동 반복 횟수 바닥값은 서로 다른 경계로 기록한다.
3. Taffy의 fixed-maximum repeat count, `MaximizeTracks`, `auto-fit` empty collapse 결과를 독립 CSS expectation과 대조한다. Taffy가 규범값과 다르면 관측값을 기준으로 고치지 말고 필요한 계산 코드를 보정하며 최소 회귀를 추가한다.
4. 49-scene 사전 비교를 implementation fixture로 연결한다: 세 intrinsic minimum × `40px`/`30%`/`calc(15px + 25%)` × 양축 × 두 repeat mode (36), max-below-minimum 6, 129/130px threshold 2, fixed-minimum positive control 1, two-track fragment 양축 × 두 repeat mode 4.
5. Rust direct layout, Stylo→typed projection, FFI opt-in dispatcher를 검사한다. 지원 밖 CSS value, indefinite basis, 잘못된 math owner, non-finite intermediate, unsupported fraction maximum은 기대 오류 또는 upstream CSS invalidation으로 분류하고 partial `LayoutOutput`이 없음을 확인한다.
6. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 같은 고정 scene ID별 track list, frame, item overflow, trace completeness, draw/present revision을 확인한다. PixelCopy와 compositor 제출 전 Metal drawable sample은 viewport/root 바깥까지 포함해 고정 좌표로 비교한다. AVD는 software-rendered일 수 있으므로 Android hardware GPU 증거로 확대하지 않는다.
7. 기본/제품 artifact에서 fixture symbol·payload가 빠지고 opt-in fixture artifact에는 positive control로 포함되는지 검사한다.
8. 구현/runtime 20개 관점 검토와 PR diff 20개 관점 검토는 서로 분리한다. PR plan-diff review도 계획 실패 검토표와 다른 20관점으로 남긴다. 리뷰 발견은 수정한 뒤 영향 관점을 다시 실행한다.
9. 내부 계약 0059, C11.4 상위 계획, `spec/STATUS.md`, public roadmap을 같은 용어와 범위로 갱신한다. C11.4와 C11 체크박스 및 공식 집계 `72/213`은 유지한다.

## 완료 조건

- 49개 Chrome oracle 사례의 예상 트랙 수, 사용 breadth, root/item frame 및 collapse 상태가 독립 계산과 일치한다.
- Rust, style projection, fixture FFI, Android AVD, iOS Simulator 행렬에서 모든 positive/negative case가 정해 둔 성공/오류 결과를 보인다.
- Browser/runtime evidence, artifact boundary, implementation review 20, PR diff review 20을 기록한다.
- PR의 `mergeStateStatus`, required checks 및 실제 merge 상태를 확인하고 Rebase and merge한다. Tailnet preview는 merge 뒤 갱신·렌더링 확인 후 Agent Browser 세션을 닫는다.

## 남는 범위

이 조각을 병합해도 일반 CSS 지원, 임의 calc 표현식, indefinite/cyclic 최대값, minmax 문법의 다른 모든 조합, repeat naming·placement 전체 조합, Spinon WPT 전체 적합성, C11.4 Stage A–D 종결, C11 상위 완료를 뜻하지 않는다.
