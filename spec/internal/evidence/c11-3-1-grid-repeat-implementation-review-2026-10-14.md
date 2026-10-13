# C11.3.1 정수 `repeat()` 구현 후속 검토 · 2026-10-14

## 검토 경계

현재 C11.3.1 변경과 `origin/main` 경로 정리를 반영한 후보를 대상으로 소스·fixture 경로·빌드 경계·실행 증거를 다시 확인했다. 이 구현은 기존 Stylo typed computed 값을 제한된 Taffy profile로 전달하고, 앱 검증 scene은 feature-gated fixture 모듈에서만 제공한다.

## 서로 다른 실패 경로 20개

| # | 실패 경로 | 확인 |
| --- | --- | --- |
| 1 | CSS 선언 문자열을 별도 parser로 다시 읽어 Stylo cascade 결과와 어긋난다 | computed `TrackList` 타입에서 직접 projection한다. |
| 2 | 정수 이외의 repeat count가 정수로 잘려 성공한다 | `RepeatCount::Number`의 양의 정수 경로만 받는다. |
| 3 | `auto-fill` 또는 `auto-fit`이 고정 count처럼 계산된다 | auto repeat은 지원 profile 밖으로 거부한다. |
| 4 | repeat 안에 중첩 repeat이 허용된다 | nested repeat fixture가 실패 경로를 확인한다. |
| 5 | 0·음수·소수 count가 빈 track 또는 잘린 값으로 처리된다 | Chrome 기준 입력과 typed projection 거부 테스트를 확인했다. |
| 6 | `minmax()`·지원하지 않는 sizing·template area가 조용히 기본값 처리된다 | 허용 track-size subset 밖 입력은 전체 projection을 거부하고 부분 결과를 내지 않는다. |
| 7 | count 곱셈·누적 track 수가 overflow 또는 한도 초과 후 일부 축만 남긴다 | checked arithmetic과 명시 track 10,000개 경계, 10,001 거부를 확인했다. |
| 8 | line 그룹 수가 expanded track 수와 같아져 마지막 경계 line이 사라진다 | expanded track N개마다 line group N+1개를 만든다. |
| 9 | repeat seam 병합에서 이름 중복·순서가 손실된다 | seam은 한 physical line group으로 합치되 name 순서와 중복을 보존한다. |
| 10 | 축별 metadata bytes 계산에서 이름 길이·header 크기가 빠진다 | header와 UTF-8 name bytes를 checked arithmetic으로 계산하고 축별 1 MiB 예상 한도를 적용한다. |
| 11 | vector reserve 실패 뒤 부분 track metadata가 다음 layout에 남는다 | track·line·name reserve의 오류 처리를 확인하고 다음 정상 계산 복구 테스트를 통과했다. |
| 12 | 이전 Grid profile이 새 line-name 의미를 우연히 받는다 | `RuntimeBlockFlexGridTemplateV1` 경로에서만 metadata를 전달하고 기존 profile 테스트를 통과했다. |
| 13 | Taffy adapter가 raw CSS 문자열을 재해석하거나 생성 track 수를 다르게 받는다 | typed projection 결과와 Taffy track vector·line metadata fixture가 각각 비교된다. |
| 14 | C11.3.1 runtime scene source가 제품 runtime 모듈에 들어간다 | scene matrix adapter는 `runtime_gpu/fixtures/c11_3_1.rs`에 있고 부모 `fixtures` 모듈은 fixture-test Cargo feature로 gated 된다. |
| 15 | 작업 ID FFI 진입점 또는 sample readback이 제품 ABI에 노출된다 | C11.3.1 FFI는 `runtime_gpu/ffi/fixtures/c11_3_1.rs` 아래 있고 부모 모듈·C 선언은 fixture-test 조건을 따른다. 경계 회귀 테스트에 legacy/canonical 경로도 추가했다. |
| 16 | 기본 static library에 fixture 심벌 또는 embedded runtime matrix가 남는다 | 기본 `spinon-ffi` archive의 C11.3.1 심벌·schema marker 부재를 확인했다. |
| 17 | fixture positive control이 잘못 설정되어 기본 artifact 검사를 거짓 통과시킨다 | fixture feature archive에서 같은 C11.3.1 심벌과 schema marker의 존재를 확인했다. |
| 18 | C11.3.1 FFI 선언·앱 호출부가 fixture feature off/on에서 서로 다르게 구성된다 | C header 전처리에서 fixture off일 때 C11.3.1 심벌이 없고 opt-in일 때 선언되는지 확인했다. Android JNI·iOS Objective-C++ 경로와 빌드 flag 테스트도 대조했다. JNI/Objective-C++ wrapper 자체는 fixture off에서 명시적 disabled 결과를 반환하며 Rust fixture payload는 기본 feature에서 컴파일되지 않는다. |
| 19 | invalid sample 좌표가 PixelCopy/Metal readback을 먼저 시작한다 | 실기기 비교기와 iOS Metal 비교기에서 바뀐 x 좌표 negative control이 readback 전 거부됐다. |
| 20 | 한 scene·다섯 표본의 결과를 전체 Grid/전체 화면 지원으로 확대한다 | Android 16 실기기 PixelCopy와 iPhone 17 Pro iOS 26.2 Simulator Metal drawable 표본은 각각 5/5 RGBA exact다. 한 scene·compositor 전 표본이라는 경계를 명시했다. |

## 후보 소스 검증

- CSS reference 전체: 387 passed, 0 failed, 0 skipped.
- Rust workspace: 656 passed, 0 failed, 4 ignored.
- 기본 FFI: 18 passed. 앱 fixture feature 조합: 53 passed, 2 ignored.
- `cargo fmt --all -- --check`, workspace Clippy와 fixture feature Clippy가 통과했다.
- 2026-10-14 앱 실행은 Android 16 SM-S731N / Xclipse 940 PixelCopy와 iPhone 17 Pro / iOS 26.2 Simulator Metal drawable에서 각각 5/5 exact, 최대 채널 오차 0이었다. Chrome 154 DPR 1 고정 scene과 표본만 비교했다.
- fixture 포함 경계의 static library positive/negative 결과는 `c11-3-1-grid-repeat-pixels-2026-10-14/fixture-binary-contract/`에 있다. `crates/`와 `platforms/`의 빌드 입력은 같은 실행 증거를 만든 후보와 바이트 단위로 동일함을 확인했다. 이는 최종 Android/iOS release app artifact 검증이 아니다.

## 남은 범위

C11.3.1 runtime은 `multi-track-repeat` 한 장면이다. named line 배치와 template areas는 C11.3.2/.3 미구현이며, 전체 C11·전체 CSS 지원 및 Spinon WPT 적합성도 미완료다. Android/Metal 결과는 지정 기기·시뮬레이터 경계에 한정되고 iOS compositor 이후 pixel은 측정하지 않았다.
