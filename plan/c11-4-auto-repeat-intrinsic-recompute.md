# C11.4 · auto-repeat intrinsic-size 변화 재계산 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 Grid 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** 구현 전 계획. 이전에 확인한 `auto-repeat + typed implicit track list`는 PR #391에서 이미 병합됐으므로 이번 범위에 포함하지 않는다. 이 작업은 WPT `grid-change-intrinsic-size-with-auto-repeat-tracks-001.html`의 같은 Grid를 연속 변경하는 경우만 다룬다. C11.4 체크와 공식 `72/213`은 그대로 둔다.

## 문제와 목표

고정된 auto-repeat track 입력을 한 번 계산하는 검증은 grid item의 intrinsic contribution이 바뀌었을 때 repeat count와 위치가 다시 계산되는지 증명하지 않는다. WPT는 같은 문서에서 item 폭을 `100px → 80px → 15px → 120px`로 바꾸고, Grid의 intrinsic width를 `max-content`에서 `min-content`로 바꾼 뒤 네 폭을 다시 적용한다. 각 변경은 auto-repeat 개수, track 폭, Grid item 및 absolute-positioned probe의 geometry를 다시 검사한다.

목표는 Rust runtime 경계에서 이 8단계를 같은 `HostDocument`와 같은 node handle로 순서대로 재계산하고, 각 상태가 새 문서/style revision을 사용했는지 확인하는 opt-in C11.4 fixture slice다. 별도 앱 검증은 동일한 여덟 고정 상태를 각각의 입력 scene으로 실행해 플랫폼 projection/render 연결을 확인한다. 앱의 독립 scene 결과는 동적 invalidation 증거로 확대 해석하지 않고, 같은 문서의 재계산 증거는 runtime sequence 검사에서 따로 확인한다.

## 고정된 기준 입력

기준 WPT는 Web Platform Tests revision `c999c58338ee1d223df5ad62e48be1202ced0d37`의 `css/css-grid/grid-definition/grid-change-intrinsic-size-with-auto-repeat-tracks-001.html`이다. 원본과 지원 파일 SHA-256은 [source manifest](../spec/internal/evidence/c11-4-grid-change-intrinsic-auto-repeat-precomparison-2026-10-27/source-manifest.json)에 고정했다. Agent Browser 0.31.1의 HeadlessChrome 156에서 WPT harness 8/8을 관찰했다. 이는 Spinon이 WPT를 통과했다는 뜻이 아니다.

Grid는 `display:grid`, `position:relative`, `min-width:30px`, `border:2px solid`, `padding-top:10px`, `grid-auto-columns:20px`, `grid-template-columns:repeat(auto-fill, 20px) minmax(min-content, 40px)`, `grid-template-rows:none`을 사용한다. in-flow item은 `grid-column:1 / -1`, 높이 30px이며, 네 absolute probe는 각각 `1/2`, `2/3`, `3/4`, `4/5` 열 line을 사용한다. 네 probe는 track sizing contribution에 참여하지 않지만 계산된 grid line과 containing block geometry를 노출한다.

WPT 관측과 독립 산술 기대값은 [Chrome observation](../spec/internal/evidence/c11-4-grid-change-intrinsic-auto-repeat-precomparison-2026-10-27/chrome-observation.json) 및 [expected JSON](../spec/internal/evidence/c11-4-grid-change-intrinsic-auto-repeat-precomparison-2026-10-27/expected.json)에 분리했다. 기준 규칙은 [CSS Grid Level 1 §7.2.3.2 및 §11](https://drafts.csswg.org/css-grid-1/#repeat-notation), [CSS Sizing Level 3 §5](https://drafts.csswg.org/css-sizing-3/#intrinsic-sizes)다. 여기서 산출한 expected values는 Chrome 측정값으로부터 복사하지 않는다. 고정 기대값 검증기는 8개 상태에서 root/item/probe geometry, used columns, repeat count와 WPT assertion을 비교한다.

| 상태 순서 | Grid width | item width | repeat count | used columns |
| --- | --- | ---: | ---: | --- |
| 1–4 | `max-content` | 100, 80, 15, 120px | 3, 2, 1, 4 | `[20,20,20,40]`, `[20,20,40]`, `[20,40]`, `[20,20,20,20,40]` |
| 5–8 | `min-content` | 100, 80, 15, 120px | 3, 2, 1, 4 | `[20,20,20,40]`, `[20,20,40]`, `[20,10]`, `[20,20,20,20,40]` |

세부 geometry와 tolerance는 expected JSON을 기준으로 한다. WPT의 root-relative coordinates와 앱/layout 절대 frame을 혼동하지 않도록 두 좌표계를 명시해 기록한다.

## 구현 범위와 경계

1. `c11-4-grid-track-sizing-fixture-test` opt-in profile에 고정 root ID, 정확한 DOM topology, CSS declarations, item placement와 absolute probes만 받는 fixture adapter를 추가한다. 다른 root/style/profile을 자동으로 이 테스트에 포함하지 않는다.
2. 하나의 `HostDocument`를 만든 뒤 WPT 순서대로 root width keyword와 item width를 바꾼다. 각 단계는 동일 generation·node handle을 보존하고, 문서 revision 및 `StyleRevision`을 증가시킨다. Style invalidation 결과가 변경을 포함하고, 이전 layout의 revision이 새 결과로 재사용되지 않았음을 각 단계에서 확인한다.
3. 각 상태를 C11.4 runtime cascade·layout 경로에 다시 넣어 computed track list, auto-repeat trace/count, node frame과 absolute probe frame을 관찰한다. 실패한 assertion도 trace/source revision을 보존한다.
4. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 여덟 고정 상태의 앱 fixture matrix를 실행한다. 각 scene의 frame·track·trace와 draw/present를 기록하고, 캡처/표본이 있으면 state ID·device/build/source hash에 연결한다. 이 matrix는 앱의 각 입력 상태 처리를 확인하지만 같은 앱 document에서의 mutation sequence는 증명하지 않는다. 시뮬레이터 결과는 실기기·hardware GPU 증거로 부르지 않는다.
5. 테스트 결과는 WPT oracle, 독립 기대값, Spinon runtime, Android simulator, iOS simulator로 분리한다. 미지원 CSS grammar와 전체 WPT/C11.4를 통과했다고 표시하지 않는다.

HostDocument mutation 경로에서 revision을 명시적으로 진행하거나 해당 opt-in fixture adapter를 추가할 수 없으면, 서로 다른 입력의 단일 계산으로 범위를 축소하지 않는다. 그 경우 런타임 문서/style revision 경계를 다시 조사하고 계획을 보정한 뒤 구현한다.

## 제외 범위

- 일반 JavaScript 실행, 임의 DOM mutation, 일반 CSS admission 및 class/stylesheet 전체 cascade conformance
- 복수 auto-repeat, `auto-fit`, 다른 intrinsic track grammar, arbitrary absolute positioning, 전체 Grid placement 및 모든 overflow 조합
- 전체 WPT 통과, C11.4 완료 또는 C11 완료 판정
- 공개 API 또는 출시된 내부 계약 변경

이 slice에는 호출 가능한 공개 API를 추가하지 않는다. Opt-in fixture가 기존 `0.1.0` 내부 계약을 변경한다고 주장하지 않으며, 공식 상태 체크박스나 `72/213`을 변경하지 않는다. 테스트 전용 결과와 한계는 [구현 전 비교 모델](../spec/0001-conformance.md#구현-전-비교-모델) 원칙에 따라 실행 evidence에 남긴다.

## 통과 기준

- 고정 WPT/source hash와 Chrome 156 8/8 기록이 독립 validator로 재생된다.
- Spinon runner가 8개 단계를 하나의 문서 generation 및 동일 node handle로 처리한다. source document revision과 style revision은 각 단계에서 증가하고, 환경 revision은 고정된다.
- 여덟 단계 모두 computed style, `auto-repeat` count, 모든 used column, item/probe geometry와 final trace가 독립 expected values와 일치한다. WPT 좌표의 허용오차는 `1/64 CSS px` 이하로 고정한다.
- root class/width 또는 item width mutation을 생략·건너뛰거나 반복 입력을 잘못 캐시하는 변형은 회귀 검사에서 실패한다. probe를 in-flow item으로 바꾸거나 item contribution에 포함하는 변형도 실패해야 한다.
- runtime sequence 검사와 두 simulator matrix가 같은 source revision의 고정 8개 기대값을 기록한다. 두 simulator의 여덟 scene은 각각 독립 입력이며 document mutation proof로 세지 않는다. 구현 범위 밖 플랫폼과 전체 conformance는 별도로 남긴다.
- 구현 뒤 새로운 20개 failure perspective 검토, PR 변경분에 서로 다른 30개 관점 검토를 통과한다.
