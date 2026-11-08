# C11.4 parsed Root HTML repeated intrinsic `grid-auto-rows` tail permutations

**상태:** 계획 게이트와 bounded 구현 완료. PR #1286은 `2026-11-08T09:10:52Z` Rebase and merge됐고 merge SHA는 `08da644ed876709798e118f0be9cefe79edfcd76`다. `fit-content(px|%)`로 시작하고 `min-content` 2개, `max-content` 2개, `auto` 1개를 뒤에 두는 30개 고유 순서를 선택했다. PR #1284 순서도 포함한다. Agent Browser Chrome 157 사전 비교는 120/120 scene과 9,480 track/item/payload checks에서 독립 기대값과 0 CSS px 차이였고, 계획 실패 관점 20/20 검토에서 누락은 없었다. 구현은 허용 판정을 단일 순서 일치에서 후행 multiset 검사로 넓혔다. Style projection의 30순서 × 4 cap form 120 typed cases, Layout 120 scene, RuntimeSession 120 scene은 독립 기대값과 일치했다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator가 같은 fixture SHA에서 각각 layout-ready 20 boxes를 기록했고 Android는 draw-presented를 기록했다. OS screenshots는 visual smoke이며 PixelCopy/Metal surface readback은 아니다. 앱 복원도 확인했다. 세 변경 crate의 all-target Clippy와 rustfmt가 통과했고 구현/runtime 20관점 및 PR diff 30관점 검토도 마쳤다. C11.4/C11 공식 체크와 `78/214`는 유지한다.

## 선택한 결손과 범위

PR #1284는 여섯-member 반복 intrinsic 목록의 다음 한 순서만 허용했다:

```css
fit-content(<finite non-negative px | direct percentage>)
  min-content max-content auto min-content max-content
```

CSS Grid의 `grid-auto-rows`는 하나 이상의 track-size 목록이며 implicit row가 더 필요하면 목록을 순서대로 반복한다. 현재 목록은 `fit-content()`가 첫 member이고 뒤 다섯 member는 `[min-content, min-content, max-content, max-content, auto]`의 한 permutation이다. 이 계획은 그 multiset의 **30개 고유 후행 순서**를 한정적으로 허용한다. Fit-content는 첫 위치에 고정해 여덟 implicit item 중 첫째와 일곱째 track에 cap/overflow 비교를 유지한다.

비교 topology는 기존과 같다: definite 40×100px border-box Root HTML Grid, 40px column, 10px explicit row, 8 implicit items, row sparse auto-flow, zero gaps, visible overflow, payload heights `[20, 10, 12, 6, 6, 6, 60, 6]px`, `align-content:start|stretch`. 각 list는 첫 6개 implicit row와 다음 2개 row에서 두 cycle 경계를 지난다. `20px`와 definite 100px block basis의 `20%` cap을 비교한다.

고정 px child의 intrinsic min/max contribution은 그 child의 고정 높이다. 독립 모델은 list index별로 payload 높이를 사용하고 두 번째 주기의 일곱째 fit-content row는 20px cap에 제한한다. Start에서 기대 used row 합계는 96px이고 free space 4px다. Stretch에서는 이 4px를 첫 8 implicit rows 안의 `auto` track occurrence에 균등하게 나눈다. 후행 순서 첫 member가 `auto`일 때만 두 번째 주기 안에 auto track 두 개가 나타난다. Browser observation은 이 계산을 검증하며 독립 모델을 생성하지 않는다.

## 독립 비교 관문

1. [CSS Grid Level 2 §7.6](https://www.w3.org/TR/css-grid-2/#auto-tracks)의 반복 규칙과 [§12.8](https://www.w3.org/TR/css-grid-2/#algo-stretch) auto-track stretch 규칙을 고정한다.
2. 30개 unique order × 2 cap forms × 2 alignment values, 총 120 scene의 Chromium observation을 수집한다.
3. 별도 모델로 each scene의 used row sizes, item frames, payload frames, computed `grid-auto-rows` order, cap overflow와 sum을 계산해 Chrome과 비교한다.
4. 관측 또는 독립 계산이 다르면 implementation 후보를 시작하지 않고 fixture/기대 모델을 원인부터 분리한다.
5. 계획 자체를 서로 다른 실패 관점 20개로 검토한 뒤에만 구현을 시작한다.

## 제한 admission 후보

- 기존 `RootHtmlFitContentAutoRowsV1` 기본 parsed Root HTML route만 사용한다.
- 목록 길이는 정확히 6이며 첫 member는 finite non-negative direct `fit-content(px|%)`다.
- 나머지는 `min-content` 2개, `max-content` 2개, `auto` 1개로 이루어진 30개 unique permutation 중 하나여야 한다.
- 단일 visible Grid owner, 지정된 fixed explicit tracks와 8개 payload item, `start|stretch`, visible overflow 조건을 유지한다.
- unresolved `calc()`, 다른 cap, 다른 member count/multiplicity, 임의 content contribution, 추가 child/owner, 다른 align/placement/style, full CSS admission 및 WPT는 계속 거부하거나 범위 밖이다.
- Unsupported revision은 partial layout/render를 publish하지 않고 같은 RuntimeSession에서 후속 유효 revision을 받아야 한다.
- 공개 API·FFI surface를 추가하지 않으며 내부 계약 `0.1.0`을 유지한다.

## 구현 뒤 관문

- Style projection은 typed track kind와 6-member 순서를 보존한다. 30개 허용 순서 모두와 nearby invalid multiplicity/order를 검사한다.
- Layout은 30개 순서를 입력 순서 그대로 반복 적용하고 explicit row를 implicit cycle에 포함하지 않는다. 8개 item frame, payload frame, start/stretch free-space distribution, fit-content overflow를 각각 독립 기대값과 비교한다.
- RuntimeSession은 px/% × start/stretch를 포함한 120 scene, error attribution, atomic rejection, same-session recovery를 검사한다.
- 기존 singleton/three-/four-member lists 및 PR #1284의 exact order를 회귀 검사한다.
- Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator의 기본 parsed Root HTML route에서 동일 fixture/hash를 실행한다. 화면은 visual smoke로 분류하고 surface readback을 했다고 확대하지 않는다.
- 구현/runtime 별도 20관점, PR diff 30관점 검토를 수행한다. 수정이 있으면 영향 관점을 재검토한다.
- 완료 뒤에만 `spec/STATUS.md` 완료 기록을 갱신한다. 이 bounded slice만으로 C11.4/C11 checkbox나 공식 `78/214`를 바꾸지 않는다.

## 현재 단계

bounded 구현은 PR #1286으로 `2026-11-08T09:10:52Z` Rebase and merge됐다(merge SHA `08da644ed876709798e118f0be9cefe79edfcd76`). Root HTML style projection과 layout의 허용 판정을 exact 한 순서에서 정해진 multiset의 30개 순서로 확장했다. projection·Layout·RuntimeSession 120-scene matrix는 독립 기대값과 일치했고, 기존 same-session 거부·복구 회귀도 통과했다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator는 같은 fixture SHA로 layout-ready 20 boxes를 기록했고 Android draw-presented도 확인했다. changed-crate Clippy와 rustfmt, 구현/runtime 20관점 및 PR diff 30관점 검토가 끝났다. C11.4/C11 공식 체크와 `78/214`는 유지한다.
