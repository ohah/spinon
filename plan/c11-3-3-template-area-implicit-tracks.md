# C11.3.3 · `grid-template-areas` 누락 track 생성

**상태:** Chrome 156 사전 비교와 독립 기대값을 고정했다. 이 계획은 C11.3.3의 제한 `RuntimeBlockFlexGridTemplateV1`에서 area 차원과 authored track 수가 다를 때 CSS Grid가 만드는 implicit track을 연결한다. 계획 PR을 병합한 뒤 별도 구현 PR로 진행한다. C11.3.3 candidate, C11 및 공식 완료 `72/213`은 이 계획만으로 완료 처리하지 않는다.

## 구현 진행 상황 · 2026-10-29

- 구현 후보는 같은 checkout의 `feat/c11-3-3-area-auto-tracks` 브랜치에서 진행 중이다. 구현 PR [#807](https://github.com/ohah/spinon/pull/807)을 열었고, GitHub에는 아직 check run이 보고되지 않았다. 공식 완료 상태는 `72/213` 그대로다.
- Style projection의 dimension-equality gate, no-auto-repeat template-area 전달 경로, `grid-template: none` cascade gate 및 TemplateGrid track 검증을 좁혀 area가 요구하는 track을 만들도록 연결했다. Authored track은 보존하고 부족한 track은 기존 단일 `grid-auto-*` `auto`/px 값으로 채운다. `area-track-count-mismatch`는 성공 geometry 대조군으로 전환했다. Android 검증에서 implicit-placement 모드 전환 뒤 area metadata가 잘못 거부되던 경계를 수정하고 회귀 테스트를 추가했다.
- Runtime matrix는 기존 Chrome 154 26 scene·65 node를 보존하고 Agent Browser/Chrome 156에서 새 5 scene·25 node를 추가했다. 후보 집계는 31 scene·90 node, success 25, cascade fallback 5, expected rejection 1이다. Android API 37 ARM64 AVD/SwiftShader와 iPhone 17 Pro/iOS 26.2 Simulator에서 최신 코드로 각 31/31 scene이 통과했다. 양쪽 모두 visible node 88개의 최대 geometry 오차는 0 CSS px, 기본 PixelCopy/Metal 표본은 2/2 exact다. 별도 unknown-area overflow 1 scene은 양쪽에서 geometry 오차 0 CSS px·표면 표본 3/3 exact, leading named-line 7 scene·14 node는 오차 0 CSS px·표본 7/7 exact다. Android의 두 negative control도 readback/draw 전 거부됐다. 이 결과는 simulator fixture 증거이며 실기기·hardware GPU·WPT 적합성을 증명하지 않는다.
- [2026-10-29 Chrome 보충 근거](../spec/internal/evidence/c11-3-3-area-auto-tracks-implementation-2026-10-29/README.md)에는 fixture, CSSOM used tracks, root-relative frames, screenshot과 browser closure를 연결했다. 기존 26-scene Chrome 154 reference와 구분한다.
- Rust layout/style focused tests, fixture FFI matrix test, CSS reference tests와 `cargo fmt --check`가 통과했다. 기본 `spinon-ffi` static archive에는 C11.3.3 FFI symbol/payload가 없고 `c04-runtime-gpu-fixture-test` opt-in archive에는 둘 다 포함됨을 확인했다. [archive 경계 근거](../spec/internal/evidence/c11-3-3-area-auto-tracks-implementation-2026-10-29/ffi-artifact-boundary.json). 시작 시 pinned V8 checkout이 없어 저장소 pin `7b50b62cb18f28617959e8452e2cd18195b38bcf`로 새 checkout과 Android/iOS simulator archive를 만들었다. 구현 PR [#807](https://github.com/ohah/spinon/pull/807)이 열려 있다. 구현/runtime 실패 관점 20개와 별도 PR diff 실패 관점 20개를 기록했다. 프로젝트 규칙의 병합 전 PR 적대 검토 30개는 [검토 기록](../spec/internal/evidence/c11-3-3-area-auto-tracks-implementation-2026-10-29/pr-review-30.md)에 남겼다. merge 전 최종 PR state 확인이 남아 있으며 현재 check run은 보고되지 않았다. 이 검증만으로 C11.3.3 candidate·C11·공식 완료 수를 올리지 않는다. [Chrome 보충 근거](../spec/internal/evidence/c11-3-3-area-auto-tracks-implementation-2026-10-29/README.md) · [Android API 37 AVD 실행](../spec/internal/evidence/c11-3-3-grid-template-areas-runtime-2026-10-29/android-avd-c1133-matrix-20261029T132906693Z-b13baf02/README.md) · [iOS Simulator 실행](../spec/internal/evidence/c11-3-3-grid-template-areas-runtime-2026-10-29/ios-simulator-c1133-ios-matrix-20261029T131950859Z-f6d1bd47/README.md).

**상위:** [C11 계획](./c11-css-grid.md) · [C11.3 `repeat()`·named line·template areas](./c11-3-template-repeat-named-lines-areas.md) · [공식 상태](../spec/STATUS.md#c11-css-grid) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**규범:** [CSS Grid Layout Module Level 1 · 2025-03-26 CRD §7.3](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-template-areas-property), §7.3.2, §8.5.

## 결손과 목표

현재 C11.3.3은 `grid-template-areas`의 각 행·열 수가 explicit `grid-template-rows`·`grid-template-columns` 수와 정확히 같아야 한다고 검사한다. 다르면 cascade에서 무효인 declaration으로 되돌아간 경우와 구분해 `grid-template-areas` 오류로 거부한다. 이 판정은 area가 요구하는 track을 implicit하게 만들고 `grid-auto-rows`·`grid-auto-columns`로 크기를 정하는 CSS 동작을 막는다.

CSS Grid §7.3은 area 문자열마다 row를 만들고 각 cell마다 column을 만든다. §7.3 note는 explicit grid가 없는 축의 행·열이 implicit하게 생성되며 `grid-auto-*`로 크기가 정해진다고 규정한다. Chrome 156의 고정 사전 비교는 area 차원이 explicit track보다 큰 각 축에서 추가 track이 생성되고, explicit track이 더 많은 경우 기존 trailing track이 보존되는 것을 확인한다.

이 작업은 C11.3의 기존 `RuntimeBlockFlexGridTemplateV1` opt-in route에 한정한다. style projection은 해당 profile에서 validated `template_areas` metadata를 layout 입력까지 보존한다. Taffy의 area expansion으로 필요한 implicit track을 만들고 기존 `grid-auto-rows`·`grid-auto-columns` 값으로 크기를 정한다. 새 CSS parser, 일반 author admission, 새 ABI나 공개 support 선언을 추가하지 않는다.

## 고정 Chrome 비교

[고정 fixture와 Chrome 156 관측](../spec/internal/evidence/c11-3-3-area-auto-tracks-precomparison-2026-10-29/README.md)은 같은 2×2 named-area matrix, `120×80px` content box, `align-content:start`, `justify-content:start`를 사용한다. Chrome CSSOM의 used `grid-template-*` 값과 모든 item의 root-relative frame을 함께 비교한다.

Chrome 비교 key는 `none`, `short-row`, `short-col`, `auto`, `long`, `width-mismatch`다. 구현 matrix에서는 앞 다섯 장면을 `area-auto-tracks-none`, `area-auto-tracks-short-row`, `area-auto-tracks-short-col`, `area-auto-tracks-auto`, `area-auto-tracks-long`으로 추가하고 `width-mismatch` 결과는 기존 `area-track-count-mismatch` scene에 적용한다.

| 장면 | Authored row / column tracks | `grid-auto-*` | Chrome used rows / columns |
| --- | --- | --- | --- |
| `none` | none / none | `7px` / `9px` | `7px 7px` / `9px 9px` |
| `short-row` | `12px` / none | `7px` / `9px` | `12px 7px` / `9px 9px` |
| `short-col` | none / `13px` | `7px` / `9px` | `7px 7px` / `13px 9px` |
| `auto` | none / none | `auto` / `auto` | `10px 20px` / `20px 30px` |
| `long` | `5px 6px 7px` / `30px 40px 50px` | `7px` / `9px` | `5px 6px 7px` / `30px 40px 50px` |
| 기존 `area-track-count-mismatch` | `60px 60px` / `40px 120px` | `auto` / `auto` | `60px 60px` / `40px 120px 0px` |

기존 `area-track-count-mismatch` runtime 장면을 마지막 행의 의미 회귀 대조군으로 사용한다. `grid-template-areas: "a b c" "d e f"`와 authored column 두 개의 조합은 유효하며 Chrome은 세 번째 implicit column을 만든다. 장면 기대값은 정규화한 CSS px 좌표를 사용하고 browser 문서상 offset은 제외한다.

## 구현 계약과 경계

1. `RuntimeBlockFlexGridTemplateV1`에서는 어느 축에도 auto-repeat이 없어도 검증된 computed area metadata를 `GridContainerInput`까지 전달한다. 기존 TrackSizing+auto-repeat 경로를 보존하고 관계없는 profile에 `grid-template-areas`를 열지 않는다.
2. Template profile에서는 area 차원이 authored track list보다 크거나 작다는 이유만으로 거부하지 않는다. Authored 목록을 그대로 보존한다. Taffy는 area가 요구하는 부족한 implicit track만 만들고, 초과 authored track을 유지하며, 생성 track을 기존 computed `grid-auto-rows`/`grid-auto-columns` 값으로 크기 결정해야 한다.
3. 허용하는 `grid-auto-*` subset은 `auto` 또는 현재의 fixed-pixel 형식으로 유지한다. 두 축 각각에서 `auto` content contribution과 fixed-pixel 확장을 검증한다. track-list, intrinsic, percentage, `calc()` auto-track 문법은 넓히지 않는다.
4. area 경계, 직사각형 유효성, line-name metadata 한도, `GRID_MAX_TRACKS`, finite 검사와 실패 원자성을 유지한다. 잘못된 authored area CSS는 계속 Stylo cascade fallback을 따르게 하며 layout adapter에서 재해석하지 않는다.
5. explicit track이 없는 경우, authored row 목록이 짧은 경우, column 목록이 짧은 경우, `auto`로 생성한 track, area matrix보다 authored 목록이 긴 경우, 기존 한 축 mismatch 회귀를 포함한다. used track 목록, named-area item 배치, node/source 순서, revision, 부정 입력의 partial output 부재와 root 밖 overflow를 확인한다.
6. 기존 C11.3.3 26-scene 행렬에 success 장면 다섯 개를 추가하고 현재 `area-track-count-mismatch` 거부를 success로 바꿔 base 행렬을 31 scene·90 node로 확장한다. 기대 집계는 success 25, cascade fallback 5, expected rejection 1이다. 별도 unknown-area overflow scene과 leading named implicit-line 장면 일곱 개도 다시 실행한다. 수정된 scene 수 및 success/fallback/rejection 집계를 맞추고 기존 26-scene 근거는 구분해 보존한다.
7. 최종 구현 revision에서 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 행렬을 실행한다. PixelCopy와 compositor 제출 전 Metal 근거를 분리하고 fixture FFI 기본/opt-in artifact 경계를 확인한다.
8. 서로 다른 실패 관점 20개의 새 구현/runtime 검토와 별도 20관점 PR diff 검토를 기록한다. 수정으로 영향받은 관점은 다시 실행한다. PR checks, merge state, 병합 뒤 roadmap 동기화는 각각 별도 근거로 남긴다.

## 완료 조건

- 모든 고정 장면의 Chrome CSSOM rows/columns와 item frame이 독립 기대값 및 pinned browser 캡처와 일치한다.
- style projection과 layout 테스트가 부족한 area track은 `grid-auto-*`를 쓰고 authored track은 보존되며 초과 explicit track도 유지함을 입증한다.
- 구현 commit의 Android/iOS Simulator 행렬, artifact 경계, format/Clippy와 별도 20+20 적대 검토를 통과한다.
- 기존 mismatch 거부는 같은 revision의 simulator/runtime 근거에서 생성 implicit track과 item geometry를 확인한 뒤에만 성공 결과로 바꾼다.
- 내부 계약 0059, `spec/STATUS.md`, C11.3 plan과 공개 roadmap에 제한 결과 및 남은 범위를 반영한다. 고정 장면만으로 C11.3.3 또는 C11을 체크하지 않는다. 별도 공식 완료 기준을 만족하지 않으면 `72/213`을 유지한다.

## 명시적 미포함

이 slice는 C11.3.3 전체, 일반 `grid-template-areas`, auto-placement, 임의 implicit track-size 목록, auto-repeat과 area dimension mismatch 결합, subgrid, writing mode/RTL, 전체 WPT suite, C11.4 track sizing, 공개 CSS 지원, Android 실기기, hardware GPU 또는 iOS compositor 이후 표시를 완료하지 않는다.
