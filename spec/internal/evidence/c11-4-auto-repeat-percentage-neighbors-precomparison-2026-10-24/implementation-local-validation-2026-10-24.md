# C11.4 percentage neighbor 구현 후보 로컬 검증

2026-10-24 현재 checkout의 미커밋 후보 변경을 고정한 Rust 검증 결과다. 이 기록은 `main`에 병합된 구현이나 Android/iOS 앱 runtime 검증을 뜻하지 않는다.

## 확인한 변경

- layout은 auto-repeat 앞뒤의 finite, nonnegative percentage track을 definite positive content-axis에서만 받아들인다. zero·indefinite·cyclic 축과 overflow가 나는 계산은 기존 fail-closed 경계를 유지한다.
- Stylo cascade는 fixed px와 nonnegative percentage 이웃을 track profile에 보존한다. layout preflight가 definite basis와 percentage 곱셈의 유한성을 검증한다.
- 빈 `auto-fit` 장면은 모든 반복 track이 collapse되어 Taffy가 `FixedTrackEarlyReturn`으로 종료한다. 13-scene 검증은 이 명시된 최종 종료 stage를 읽으며, 유연 track이 남은 장면은 `ExpandFlexibleTracks`를 읽는다.
- `col-fit-first`의 사용 track 규범 기대값은 200px이다. 빈 repeat track 양쪽 gutter collapse를 계산한 CSS Grid 2025 CRD §7.2.3.2 기준과 일치한다. Chrome 156 CSSOM의 196px 관측은 별도 호환성 관측으로 남긴다. end-alignment의 규범 236px / Chrome 232px 및 suffix item x/y=238px의 비판별성은 [규범·Chromium·Taffy 대조](normative-chromium-taffy-audit-2026-10-24.md)에 기록했다.

## 로컬 검사

| 명령 | 결과 |
|---|---|
| `cargo test --locked -p spinon-layout percentage_neighbors` | 2 passed, 0 failed. 고정 13-scene matrix와 indefinite, negative/non-finite percentage, basis 곱셈 overflow rejection을 포함한다. |
| `cargo test --locked -p spinon-style auto_repeat` | 16 passed, 0 failed. 이웃 track cascade와 기존 auto-repeat 제한 회귀를 포함한다. |
| `cargo fmt --all -- --check` | 통과 |

첫 matrix 실행은 빈 auto-fit 장면의 `ExpandFlexibleTracks` 부재를 잘못 실패로 처리했다. trace의 유효한 `FixedTrackEarlyReturn` 완료 기록을 선택하도록 helper를 보정한 뒤 13-scene matrix가 통과했다. 첫 style rejection 추가 사례의 trailing `1fr`는 CSS Grid 문법상 auto-repeat 이웃 track으로 허용되지 않는다. 런타임 cascade 결과를 직접 확인해 `columns=[]`, `columns_auto_repeat=None`임을 확인했으며, parser가 잘못된 전체 선언을 버리고 기본값을 사용한다. 이 invalid-declaration 사례는 runtime error assertion에서 제거했으며, 유효한 `calc()` neighbor rejection은 유지했다. 최종 targeted style 실행은 16/16 통과했다.

## 남은 경계

- Rust 변경은 현재 로컬 checkout의 미커밋 후보이며 `main`에는 없다.
- Android AVD, Android 실기기, iOS Simulator 및 iOS 실기기 runtime은 실행하지 않았다.
- 전체 workspace test, Clippy, FFI fixture, artifact boundary, 앱 화면/픽셀 비교는 이 로컬 검증에서 실행하지 않았다.
- C11.4는 미완료다. 공식 완료 집계는 `72/213`을 유지한다.
