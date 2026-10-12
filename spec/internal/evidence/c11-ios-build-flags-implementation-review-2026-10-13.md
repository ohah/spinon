# C11 iOS Grid 표면 픽셀 빌드 연결 구현 검토 · 2026-10-13

검토 범위는 내부 Metal drawable 표본 검증 flag가 iOS simulator 빌드 설정부터 Xcode, C++/Objective-C++, Rust FFI, runtime 비교기까지 전달되는 경로다. 제품 CSS 지원이나 C11 전체 완료를 검토하는 문서는 아니다.

| # | 실패 관점 | 결과 | 확인 근거 |
| --- | --- | --- | --- |
| 1 | simulator에서 flag를 생략했을 때 검증 코드가 우발적으로 켜짐 | 통과. 기본값은 `0`이고 Xcode 인자에도 `0`을 전달한다 | 기본값 경로 검사 |
| 2 | native Rust/C++ 빌드에서 flag 생략 시 검증 코드가 우발적으로 켜짐 | 통과. Cargo feature는 추가하지 않고 C++ define은 `0`을 사용한다 | `build-ios-native.sh` 검토 |
| 3 | simulator 환경 변수에 `true` 등 비정규 boolean이 들어옴 | 통과. 빌드 전에 종료 코드 2로 거부한다 | 빌드 flag 테스트 |
| 4 | native 환경 변수에 `2`가 들어와 feature 조건과 define 값이 달라짐 | 통과. Cargo 빌드 전에 종료 코드 2로 거부한다 | 빌드 flag 테스트 |
| 5 | simulator의 Grid 표본 기능만 켜고 C04 GPU runtime은 끔 | 통과. Xcode 호출 전에 거부하고 가짜 Xcode 인자 파일이 생성되지 않는다 | 빌드 flag 테스트 |
| 6 | native build에서 C04 없이 C11 표본 기능을 켬 | 통과. V8 산출물·Rust 빌드 단계로 넘어가기 전에 거부한다 | 빌드 flag 테스트 |
| 7 | `mise` 경유 Xcode 호출에서 C11 환경 변수가 누락됨 | 통과. 명시적인 `mise exec -- xcodebuild` 인자를 확인한다 | 빌드 flag 테스트 · 실제 simulator build log |
| 8 | `mise`가 없는 직접 Xcode 호출에서 인자 누락 | 통과. 직접 `xcodebuild` 경로를 별도로 검사했다 | 빌드 flag 테스트 |
| 9 | 출력상 flag는 켰지만 native Cargo feature 또는 C++ define은 꺼짐 | 통과. native 빌드 marker가 둘 다 `1`임을 실제 iOS 빌드 log에서 확인한다 | [현재 simulator 빌드 log](./c11-2-1-grid-placement-ios-simulator-runtime-matrix-2026-10-13/c1121-iossim-20261012T191045812Z-79554a35/build.log) |
| 10 | C11 Rust feature가 C04 runtime GPU dependency 없이 링크됨 | 통과. feature 선언이 `c04-runtime-gpu`를 의존한다 | `crates/spinon-ffi/Cargo.toml` 검토 · feature 공존 검사 |
| 11 | Rust feature는 켰으나 C++ 컴파일 define이 빠짐 | 통과. define을 clang++ 호출에 전달하고 build marker로 값이 드러난다 | `build-ios-native.sh` · 실제 simulator build log |
| 12 | Xcode Debug 또는 Release 중 한 구성에서 C11 define이 빠짐 | 통과. 두 build configuration의 preprocessor definitions에 같은 변수가 있다 | `SpinonBootstrap.xcodeproj/project.pbxproj` 검토 |
| 13 | Objective-C++는 define을 받지만 Swift 조건부 컴파일은 받지 못함 | 통과. 두 configuration의 Swift active compilation conditions에 변수가 있다 | Xcode project 검토 · 빌드 flag 테스트 |
| 14 | 헤더 선언과 C++ runtime host 생성 메서드의 feature 조건이 다름 | 통과. C11 전용 생성 API는 동일한 C04/C11 매크로 조건으로 게이트된다 | FFI header와 `SpinonRunner.mm` 검토 |
| 15 | C11 전용 Rust FFI가 기본 빌드에 노출됨 | 통과. pixel host/API는 Cargo feature 뒤에 있고 기본 빌드 API와 분리된다 | `runtime_gpu/ffi.rs`, `runtime_gpu/ffi/c11.rs` 검토 |
| 16 | Pixel readback을 요청하지 않은 일반 host가 readback 전용 renderer로 초기화됨 | 통과. 일반 host는 false로 시작하며 C11 전용 생성자만 명시적으로 활성화한다 | `runtime_gpu.rs` host 생성 경로 검토 |
| 17 | 표본 좌표 count 곱셈 overflow, 빈 배열, 64 초과, null pointer | 통과. overflow·범위·null을 표본 slice 접근 전에 검사한다 | FFI negative tests 포함 feature 검사 |
| 18 | C11과 C12.4 feature를 동시에 켜 공유 readback 함수가 빌드되지 않음 | 통과. feature co-activation에서 39 passed, 2 ignored, 0 failed | [FFI feature 공존 log](./c11-ios-build-flags-2026-10-13/ffi-feature-coactivation.log) |
| 19 | 기대 거부 장면이나 허용 목록 밖 장면에서 Metal readback이 시작됨 | 통과. 10 scene 중 성공 6개에서만 20 sample을 비교했고, 거부 4개와 허용 목록 음성 대조에서 readback 시작 marker가 없다 | [iOS runtime matrix](./c11-2-1-grid-placement-ios-simulator-runtime-matrix-2026-10-13/c1121-iossim-20261012T191045812Z-79554a35/README.md) |
| 20 | drawable 표본 일치를 화면 scanout·compositor pixel parity 또는 iOS 실기기 결과로 과장 | 통과. 근거는 Metal drawable texture의 compositor 제출 전 표본이며 screenshot parity와 iOS 실기기 검증은 명시적으로 범위 밖이다 | runtime matrix 요약 및 기기 metadata |

## 검토 중 보완

처음 테스트는 정상 flag 전달과 simulator C04 선행 조건만 확인했다. 기본 비활성화, simulator/native의 잘못된 값, native C04 선행 조건을 직접 실행하는 검사가 없어 이를 추가했다. 새 빌드 flag 테스트 6/6, feature 공존 검사 39/39(ignored 2), 저장소 전체 검증과 현재 iOS Simulator 행렬은 통과했다.

## 결과와 남은 범위

- 현재 iPhone 17 Pro / iOS 26.2 Simulator 실행은 10/10 scene, 성공 6/6·기대 거부 4/4, 21 node, geometry 최대 오차 0 CSS px, Metal 표본 20/20 exact다.
- 저장소 전체 검증은 JS example 2/2, CSS reference 348/348, benchmark/tool tests 29/29, Rust 625 passed·4 ignored·0 failed, Release depth 1/1이다. Clippy와 rustfmt도 통과했다.
- 전체 테스트 첫 실행에서 기존 fake ADB benchmark 통합 테스트가 한 차례 실패 표시 후 runner가 종료되지 않았다. 격리 실행 29/29와 후속 전체 실행은 통과했다. 첫 실패 원인은 확정되지 않았으며 [첫 실행 log](./c11-ios-build-flags-2026-10-13/first-full-test-attempt.log)로 보존했다.
- 이 turn에는 Android 또는 iOS 앱 소스 동작을 바꾸지 않았다. iOS simulator runtime 근거는 플래그 전달을 포함한 현재 소스를 다시 빌드해 생성했다. iOS 실기기, compositor 이후 pixel readback, 전체 WPT 적합성, 전체 CSS Grid 지원은 검증하지 않았다.
- C11.1·C11.2.1 변경은 아직 통합되지 않았다. C11.2.2 sparse auto-placement runtime 구현은 선행 통합 게이트를 기다리고 있다.
