# C11.2.1 인덱스 후보 통합 검증 · 2026-10-13

## 검증 대상

기준 커밋 `975d3356c9e8837834d4dd633fb3699084fad71a`와 그 시점의 staged index를 별도 임시 복사본으로 재구성해 빌드·시험했다. 검증한 index tree는 [`source-tree.txt`](./source-tree.txt)에 기록했다. 후보에는 C11 변경만 포함했고 작업 트리의 미커밋 C12.4 변경은 포함하지 않았다. 문서와 이번 실행 증거는 검증 뒤 추가했다.

통합 검증에서 세 오류를 찾아 수정했다. 검토 중 500줄을 넘던 신규 Grid 테스트 파일 두 개는 cascade, 배치 성공, 숨김 경계, 거부 경계별 하위 모듈로 나눴다. 테스트 본문과 제품 동작은 바꾸지 않았으며, 이 분리를 포함한 최종 인덱스 tree는 `fb9411cb7cc7f767699447d29a202a8fadca1ece`다.

1. iOS C11 후보에 C12.4 전용 Grid scene 분기만 부분적으로 들어가 심볼이 정의되지 않았다. 그 분기를 C11 index에서 제거하고 C12 작업 트리는 보존했다.
2. 첫 iOS readback 실행에서 draw 결과에 snapshot revision과 다중 sample JSON이 없어 표본 검증이 실패했다. 공용 readback 보고서에 해당 revision과 순서 보존 JSON을 추가했다. 시험용 readback이 없을 때는 sample JSON을 만들지 않는다.
3. batch API로 바꾼 뒤 기존 C12.4 단일 표본 FFI 호출이 이전 인수를 사용했다. C12.4 동작은 유지하고 해당 호출을 단일 좌표 batch 형태로 갱신했다. C12.4 전용 feature와 C11/C12 동시 활성화 검사도 추가 실행했다.

## 결과

| 검증 | 결과 |
| --- | --- |
| `mise exec -- bun run test` | JavaScript 2/2, CSS reference 304/304, Android 계측 도구 29/29, Rust 623 통과·4 ignored·0 실패, Release depth 1/1 (분리 후 정확한 인덱스 복사본 재실행) |
| C11 Metal readback feature FFI 경계 검사 | 8/8 통과 |
| C12.4 Metal readback feature 회귀 | 37 통과·2 ignored·0 실패 |
| C11·C12.4 feature 동시 활성화 | 39 통과·2 ignored·0 실패 |
| C11.1 pixel matrix 생성기 고정 검사 | 10 scene·36 sample 일치 |
| C11.2.1 pixel matrix 생성기 고정 검사 | 10 scene·6 pixel scene 일치 |
| locked workspace Clippy `-D warnings` | 통과 |
| `cargo fmt --all -- --check` | 통과 |
| Android 16/API 36 SM-S731N 실기기 runtime/PixelCopy | 10/10 scene, 성공 6·기대 거부 4, 21 node, geometry 최대 오차 0 CSS px, 표면 sample 20/20 exact |
| iPhone 17 Pro/iOS 26.2 Simulator runtime/Metal drawable | 10/10 scene, 성공 6·기대 거부 4, 21 node, geometry 최대 오차 0 CSS px, 표면 sample 20/20 exact |

전체 저장소 실행은 [`full-test.log`](./full-test.log), FFI·fixture 생성 검사는 [`ffi-and-fixture-checks.log`](./ffi-and-fixture-checks.log)와 [`fixture-generator-checks.log`](./fixture-generator-checks.log), C12 호환성은 [`c12-feature-compat.log`](./c12-feature-compat.log)와 [`c11-c12-feature-coactivation.log`](./c11-c12-feature-coactivation.log), 정적 검사는 [`clippy.log`](./clippy.log)와 [`rustfmt.log`](./rustfmt.log)에 있다. Android·iOS 원본 log, 앱 hash, 비교 JSON, 화면과 표면 표본은 각각 [Android 실기기 run](../c11-2-1-grid-placement-android-runtime-matrix-2026-10-13/c1121-android-20261012T203140900Z-69e4ca98/README.md), [iOS Simulator run](../c11-2-1-grid-placement-ios-simulator-runtime-matrix-2026-10-13/c1121-iossim-20261012T203042604Z-f7983af0/README.md)에 보존했다.

## 검증 범위

이 행렬은 제한된 C11.2.1 numeric line/span fixture 10개에 해당한다. iOS 표본은 compositor 제출 전 Metal drawable 값이다. 이 결과는 최종 화면 compositor parity, 성능, 전체 Grid/CSS 지원 또는 Spinon WPT 적합성을 뜻하지 않는다. C11.2.2 runtime 구현은 C11.2.1 변경의 PR 통합 뒤 시작한다.
