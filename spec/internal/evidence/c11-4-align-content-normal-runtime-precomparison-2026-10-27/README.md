# C11.4 · `align-content: normal` 3행 auto-track stretch 런타임 사전 비교

## 목적과 범위

이 사전 비교는 기존 C11.4 `align-content: normal` Rust oracle의 row-axis 3행 geometry를 Android/iOS opt-in 앱 fixture로 연결하기 전에 고정한다. 이미 병합된 fit-content 옆 auto implicit row fixture는 `normal`이 auto row 하나를 늘리는 앱 경로를 확인하지만, 두 auto row에 free space를 균등 배분하면서 fixed row를 유지하는 이 입력은 다루지 않는다. 시각 캡처를 위해 이 fixture의 inline axis는 기존 Rust oracle의 `20px`에서 `160px`으로 넓혔다. root·유일한 column·모든 child의 폭을 같게 고정해 row-axis free-space 계산에는 inline 차이가 개입하지 않는다. 이번 비교는 새 CSS 지원을 주장하지 않는다.

규범 기준은 [CSS Grid Layout Module Level 1, 2025 CRD §11.8](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-stretch)이다. 입력과 독립 산술은 [`input.json`](./input.json)과 [`expected.json`](./expected.json)에 기록했다.

## 입력과 독립 계산

- Root와 viewport는 모두 definite `160×100px`이며 border/padding은 0이다. 이 입력은 pure Rust oracle의 `RootSizingPolicy::Match` root/viewport 불변 조건과 일치한다. 앱 Grid runtime profile은 viewport containing-block 정책(`ResolveWithin`)을 사용하지만 root 크기도 고정해 두 정책에서 같은 root frame을 기대한다.
- 행은 [`auto, auto, 5px`], 열은 [`160px`], `row-gap:10px`, `column-gap:0`, `align-content:normal`, `justify-content:flex-start`다. 기존 Rust oracle과 row-axis tracks, gaps, heights, alignment 값은 같고 inline-axis fixed width만 `20px → 160px`으로 확대했다.
- Direct children의 높이는 DOM 순서대로 [`10,20,5`]`px`이며 각 행에 하나씩 자동 배치된다.
- 두 내부 gutter를 빼고 계산하는 free space는 `100 - (10 + 20 + 5) - (10 + 10) = 45px`다. 대상 auto max track 두 개에 각각 `22.5px`을 더하면 최종 rows는 [`32.5,42.5,5`]`px`이고 마지막 fixed row는 유지된다.
- 기대 child frames는 `(0,0,160,10)`, `(0,42.5,160,20)`, `(0,95,160,5)`이다. `align-content:start` 대조군의 tracks [`10,20,5`]`px`와 child y [`0,20,50`]`px`는 [기존 Rust oracle test](../../../../crates/spinon-layout/src/grid_stretch_auto_tracks_oracle_tests.rs#L251)의 control에서 가져왔다. 이는 이번 Chrome 관찰에서 실행한 control이 아니다. 정상 auto-track 계산의 결과는 독립 산술과 Chrome 관찰로 별도 고정했다.

## Chrome 관찰

- Agent Browser `0.31.1`로 macOS `26.5.1` (build `25F80`, arm64)에서 viewport `160×100px`, DPR 1인 로컬 고정 fixture를 열었다. Chrome 정식 버전은 `156.0.8078.48`, revision은 `d0c02bfad08ec92a6ffcf989e53cd995e6381a4c`다. WPT harness는 실행하지 않았다.
- CSSOM `alignContent`는 `normal`, `justifyContent`는 `flex-start`, used `gridTemplateRows`는 `32.5px 42.5px 5px`였다. Root와 세 child의 raw `getBoundingClientRect()` 결과는 독립 기대값과 정확히 같았다. 전체 결과는 [`chrome156-observation.json`](./chrome156-observation.json), 원본 화면은 [`chrome156-normal-auto-track-rows.png`](./chrome156-normal-auto-track-rows.png)에 있다.
- 이 Chrome 관찰은 고정 입력의 독립 비교이며, Spinon 구현 결과나 WPT pass로 간주하지 않는다.
- 캡처 뒤 Agent Browser session info는 `active=false`였고 `session list`는 활성 session 0개를 반환했다. cleanup 상태는 [관찰 JSON](./chrome156-observation.json)에 기록했다.
- 저장소 루트에서 [SHA256SUMS](./SHA256SUMS)를 검사해 fixture, 입력·기대값, 관찰 JSON, README와 PNG의 bytes를 확인할 수 있다.

## 한계

이 입력은 block-axis의 3개 explicit row, 두 auto max track, 하나의 fixed row, definite free space만 다룬다. flex track, 다른 alignment 값, intrinsic contribution 조합, implicit track, auto-repeat, 일반 CSS admission, Android/iOS 앱 결과와 전체 C11.4는 이 사전 비교에 포함되지 않는다.
