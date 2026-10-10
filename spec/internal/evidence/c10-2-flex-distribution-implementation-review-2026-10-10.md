# C10.2 구현 실패 경로 검토와 Simulator 근거

이 검토는 계획 리뷰와 분리해 구현 diff, Chromium fixture, Rust/Taffy 비교, Stylo cascade, FFI·플랫폼 실행 경로를 대상으로 했다. 해당 변경은 [PR #116](https://github.com/ohah/spinon/pull/116)으로 리베이스 병합했다. 내부 계약 숫자 버전은 `0.1.0`으로 유지한다.

## 구현 변경을 겨냥한 20개 실패 관점

| # | 공격 관점 | 확인 결과와 근거 |
| ---: | --- | --- |
| 1 | Chrome 실행 파일이나 revision이 바뀌었는데 기존 frame을 정답으로 쓰는가 | Chrome `154.0.8037.98`, revision, executable SHA-256과 fixture·capture 입력 digest를 테스트에서 고정한다. 값이 달라지면 검사가 중단된다. |
| 2 | capture를 다시 실행할 때 기존 reference를 조용히 덮어쓰는가 | capture는 `wx`로 새 파일만 만들며 고정 기준 덮어쓰기를 거부한다. |
| 3 | case나 node가 inventory에서 빠져 전체 결과가 좋아 보이는가 | 27개 case·92개 node 수와 각 case의 DOM preorder·ID 목록을 inventory와 대조한다. |
| 4 | 비활성 fixture가 layout 흐름에 들어가 다른 case의 free space를 바꾸는가 | harness 밖으로 node를 옮기지 않고 선택 case만 정상 흐름에 보이게 한다. 고정 reference 테스트에서 case 순서와 node 목록을 검증한다. |
| 5 | 계산에 필요한 computed property가 일부 빠져 CSS 오차가 Taffy 오류로 가려지는가 | 모든 관찰 node의 fixture 지정 computed property 문자열과 네 frame field의 유한성을 reference 검사에서 요구한다. Rust 변환기는 필요한 값이 없거나 지원 밖이면 명시 실패한다. |
| 6 | Stylo 값이 특정 runtime Flex profile에서만 빠지는가 | 여섯 runtime profile에서 basis·grow·shrink·min/max와 spacing typed projection을 검사했다. 기본 앱 경로인 custom-property paint profile도 포함한다. |
| 7 | inline style 변경 뒤 incremental snapshot이 full cascade와 달라지는가 | grow/basis/min/max 변경 전후 inline full·incremental snapshot의 typed 값과 revision을 비교했다. 사용자 지정 author stylesheet는 unsupported incremental reuse 대신 full cascade 경로를 확인한다. |
| 8 | author stylesheet의 custom property 또는 등록 shorthand가 runtime 입력을 바꾸지 못하는가 | author custom property와 `@property` 등록 값을 통한 `flex` shorthand를 full cascade에서 검사했다. |
| 9 | DPR에 따라 Stylo app-unit이나 capture 좌표가 다른 값으로 고정되는가 | DPR 1·2의 모든 case·node frame이 같아야 하며 각 DPR을 별도로 검사한다. |
| 10 | 여러 독립 root, 자식 순서, root 원점 차이가 frame 비교에서 섞이는가 | 각 case의 자식 관계로 subtree를 만들고 DOM 순서·NodeId를 검증한다. frame 위치는 case root 기준으로 정규화하고 width·height는 그대로 비교한다. |
| 11 | `flex-basis`와 width/height가 충돌할 때 잘못된 값을 flex base로 쓰는가 | `basis-overrides-size`에서 basis가 명시 주축 크기를 덮는 결과를 Chrome과 비교한다. |
| 12 | `auto`를 `content`와 혼동하거나 indefinite percentage를 임의 px로 바꾸는가 | `auto`는 definite 주축 크기를 가진 item에서만 확인하고, row width·column height percentage를 각각 paired case로 고정한다. content basis와 indefinite container는 범위 밖이다. |
| 13 | `flex` shorthand omitted component나 뒤따르는 longhand가 잘못 전개되는가 | `1`, `auto`, `none`, 3-part shorthand와 이후 longhand override의 computed 값 및 geometry를 검사한다. |
| 14 | 음수 `flex-grow` CSS 선언이 앞선 유효 선언을 지우는가 | CSS-invalid 선언은 cascade에서 무효가 되어 앞선 유효값을 보존하는 Chrome computed 값과 비교한다. |
| 15 | grow 비율, 0 factor 또는 서로 다른 basis의 결과가 단순 두 항목 공식에만 맞는가 | equal·weighted·zero grow와 basis 100×3의 1/2/1 분배를 각각 고정했다. |
| 16 | grow factor 합이 1보다 작아도 남은 공간을 전부 채우는가 | 합 0.5인 case에서 두 폭 125px과 남은 50px을 독립적으로 확인한다. |
| 17 | shrink에서 factor만 나누고 basis를 곱한 scaled factor를 빼먹는가 | basis 200/100의 동일 shrink에서 160/80, unequal shrink와 zero shrink 결과를 비교한다. |
| 18 | min/max clamp를 한 번 적용한 뒤 freeze·재분배를 멈추거나 위반 방향을 혼동하는가 | grow max/min, shrink max/min, min overflow와 동시 min/max 위반 case를 각각 확인한다. 여러 항목 동시 위반 뒤 남은 폭의 재분배도 고정했다. |
| 19 | min이 max보다 큰 값, hypothetical main size 분기, gap·margin·wrap line·box sizing이 배분에 새는가 | min 우선 case와 hypothetical-size 분기, fixed gap/margin, 줄별 독립 배분, content-/border-box paired case를 모두 reference로 비교했다. 소수 좌표·큰 factor case도 포함한다. |
| 20 | 어댑터 직접 입력의 음수·NaN·Infinity가 성공 frame으로 바뀌거나 Android/iOS 실행이 다른 fixture를 쓰는가 | grow·shrink 각 값의 negative/NaN/Infinity를 node와 property가 있는 오류로 거부한다. C ABI·JNI·Objective-C++·앱 launch route를 정적으로 확인했고, Android와 iOS에서 동일 V8 fixture의 다섯 node frame이 Chrome과 일치하며 WGPU 다섯 box 제출을 로그와 화면으로 확인했다. |

## 실행 결과와 판정 경계

- `node --test tools/css-reference/*.test.mjs`: 105개 통과.
- `mise exec -- cargo test --locked --workspace --all-features --quiet`: 454개 통과, 2개 무시.
- `cargo test --locked -p spinon-layout flex_distribution -- --nocapture`: 3개 통과.
- `cargo test --locked -p spinon-style c10_flex_distribution_tests -- --nocapture`: 4개 통과.
- `mise exec -- bun run test:js`: 2개 통과.
- `cargo fmt --all -- --check`, `mise exec -- cargo clippy --locked --workspace --all-targets -- -D warnings`, `git diff --check`: 통과. Clippy가 새 cascade test helper의 불필요한 lifetime을 찾아 제거하고 다시 확인했다.
- 고정 Chrome 기준은 27개 case·92개 node, DPR 1·2다. Taffy 0.14.0 전체 node frame 최대 절대 오차는 각 field `0.5 CSS px` 이내다.
- 앱 runtime은 별도의 다섯 node fixture다. Android API 37 ARM64 emulator와 iPhone 17 Pro / iOS 26.2 Simulator의 root·flex·세 item frame이 각각 Chrome 기준과 일치했고 각 WGPU surface가 다섯 box를 제출했다. 화면은 [Android](./c10-2-flex-distribution/android-api37-emulator.png), [iOS](./c10-2-flex-distribution/ios-26.2-iphone-17-pro-simulator.png), 원본은 [Android Logcat](./c10-2-flex-distribution/android-api37-emulator-logcat.txt)·[iOS 로그](./c10-2-flex-distribution/ios-26.2-simulator-log.txt)다.
- 앱 빌드는 `SPINON_ENABLE_C04_RUNTIME_GPU=1`과 기존 pinned V8 source checkout을 지정해 `mise exec -- env SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 SPINON_ENABLE_C04_RUNTIME_GPU=1 bun run build:android` 및 `mise exec -- env SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 SPINON_ENABLE_C04_RUNTIME_GPU=1 bun run build:ios-sim`으로 실행했다. 두 빌드는 성공했다.
- Android emulator는 ANGLE/SwiftShader software backend다. 두 시뮬레이터 결과는 실기기·hardware GPU 성능을 입증하지 않는다. C10.2는 제한 runtime profile이며 C10 전체, 자동 최소 크기, intrinsic/text sizing, `flex-basis:content`, indefinite percentage는 미완료다.

## 남은 상태

이 검토에서 해결되지 않은 코드 결함은 없었다. PR #116 병합 뒤 공식 상태 대장, 계획, 내부 계약과 인덱스의 상태를 동기화했다. 동기화 점검은 [별도 기록](./c10-2-postmerge-doc-sync-review-2026-10-10.md)에 남겼다.
