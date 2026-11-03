# C11.1 parsed Root HTML dense auto-placement PR diff 실패 관점 검토 30개

이 검토는 실제 변경분을 대상으로 하며 [구현·runtime 검토 20개](./implementation-runtime-review-20.md)와 분리했다. 범위는 Rust projection/layout, opt-in fixture FFI, Android/iOS bootstrap과 capture 비교 도구, 고정 matrix·증거 문서다.

| # | 실제 diff에서 확인한 실패 경계 | 판정 |
| --- | --- | --- |
| 1 | Cargo feature가 기본 제품 빌드에서 새 fixture module을 켬 | 없음. `c11-1-root-html-dense-placement-fixture-test`는 별도 opt-in feature다. |
| 2 | FFI header의 새 symbol이 feature-off C consumer에 보임 | 없음. declaration은 runtime GPU와 dense fixture test compile define 둘 다 켜질 때만 전처리된다. |
| 3 | Rust FFI implementation이 header declaration과 이름·인자 순서가 다름 | 없음. scene ID, output pointer/capacity 및 return host 타입이 JNI call과 일치한다. |
| 4 | 새 source fixture가 `fixtures/` 밖에서 항상 컴파일됨 | 없음. Rust fixture와 FFI adapter module은 신규 Cargo feature로 각각 gated됐다. |
| 5 | matrix loader가 scene ID만 비교하고 잘못된 payload를 신뢰 | 없음. schema/profile, scene 수·순서, success/rejection/node 총계, source length와 허용 scene ID를 함께 검사한다. |
| 6 | matrix `source`에 script나 빈 payload가 들어와 임의 실행 범위를 넓힘 | 없음. source는 Root HTML, non-empty, size bound, script 부재 조건을 통과해야 한다. |
| 7 | 알 수 없는 scene 또는 구분자 포함 scene ID가 fixture 선택을 우회 | 없음. Android/iOS allowlist와 Rust loader가 등록된 14 ID만 허용한다. |
| 8 | C11 fixture extra가 dense scene과 다른 scene extra를 동시에 선택 | 없음. 두 platform bootstrap은 dense profile과 충돌하는 C11 profile/scene extra를 거부한다. |
| 9 | command-line scene ID와 PixelCopy/Metal scene ID가 달라짐 | 없음. surface marker 요청은 선택 scene ID와 샘플 matrix scene을 동일성 검사한다. |
| 10 | Android run ID에 공백·구분 제어값 또는 과도한 길이가 들어감 | 없음. run ID는 1–80자의 ASCII 영숫자·underscore·hyphen으로 제한한다. |
| 11 | 예상 rejection이 status만 맞고 다른 node/property 오류를 성공 처리 | 없음. scene별 expected code·property·node와 `status=-11`을 함께 확인한다. |
| 12 | rejection 로그가 일반 failure로 표시되거나 이전 layout을 새 성공으로 기록 | 없음. 예상 거부는 별도 marker와 status로 기록하고 정상 eval/draw 경로보다 먼저 반환한다. |
| 13 | positive fixture가 CSSOM에 다른 scene의 styles를 재사용 | 없음. 각 scene은 고정 Root HTML source를 새 app process로 실행하고 fixture host를 scene ID로 생성한다. |
| 14 | C11.1 dense log marker가 다른 C11 matrix에 혼입 | 없음. Android `C111RHD`, `C111_ROOT_HTML_DENSE`, iOS dense marker를 별도 prefix로 사용한다. |
| 15 | viewport 360×800이 simulator physical point 값으로 오인 | 없음. app CSS viewport와 device/page/physical scale을 별도 report하고 comparator가 360×800을 요구한다. |
| 16 | Android 초기 environment만 확대되고 refresh environment가 1×로 되돌림 | 없음. 초기·최신 environment 경로 모두 CSS dimensions를 environment scale에 맞춰 전달한다. |
| 17 | Android density와 PixelCopy bitmap 크기가 다른데 density만으로 pixel 좌표 계산 | 없음. 비교기는 캡처 bitmap 실측 크기를 사용하고 1×/density 불일치 control을 거부한다. |
| 18 | PixelCopy sample이 다른 PID·surface generation·scene에서 옴 | 없음. marker run ID, process ID, generation, dimensions, sample IDs와 PNG hash를 검사한다. |
| 19 | Android 10,001-track expected rejection이 성공 scene 표본으로 합산 | 없음. rejection은 surface pixel success에서 제외하고 status/code를 별도로 비교한다. |
| 20 | Android test runner가 끝난 뒤 fixture 앱 프로세스를 남기거나 다른 AVD를 종료 | 없음. runner는 선택한 target app을 종료한다. 검증 후 emulator-5556 앱 PID는 없고 emulator-5558 PID 4193은 유지됐다. |
| 21 | iOS Swift FFI wrapper selector가 Objective-C++ declaration과 불일치 | 없음. `SpinonRunner.h/.mm`, Swift call site, opt-in Rust ABI가 같은 scene 인자를 사용한다. |
| 22 | iOS dense probe argument가 기존 Root HTML/default scene argument와 충돌 | 없음. bootstrap은 지원 scene allowlist, run ID와 중복 profile을 확인하고 error marker를 남긴다. |
| 23 | iOS test flag가 default target compile에 fixture-only code를 강제 포함 | 없음. Xcode build setting과 native build script가 opt-in define을 지정한 검증 빌드에만 연결한다. |
| 24 | iOS environment width/height만 app points로 남아 drawable scale과 다름 | 없음. 초기·후속 environment 적용은 360×800 CSS viewport와 scale 2.625를 보존하며 final Metal comparison이 통과했다. |
| 25 | iOS superseded environment request가 stale frame으로 채점 | 없음. bounded retry 후 status·revision을 검사하며 final scene은 fresh process로 재실행한다. |
| 26 | iOS expected rejection이 frame draw나 pixel readback을 이어서 수행 | 없음. expected code/property/node가 맞으면 해당 scene callback에서 조기 반환한다. |
| 27 | iOS surface-generation 변경 또는 compositor 이후 결과를 Metal equality로 부름 | 없음. comparator는 run/revision/generation을 묶고 문서는 Metal drawable readback을 compositor 전으로 제한한다. |
| 28 | Android comparator의 새 scale 검사가 기존 C11 comparator marker를 혼동 | 없음. dedicated dense marker와 cache path를 분리했고 이전 C11.2.2/C11.2.3 marker 회귀가 포함된다. |
| 29 | evidence의 screenshot이 최종 점수 run이 아니거나 중간 ADB 실패를 숨김 | 수정했다. README screenshot links는 최종 run ID를 가리키며 `mw0o91n6` timeout artifact는 보존·제외 사유를 적었다. |
| 30 | 후보 문서가 부분 slice를 C11 완료·공개 CSS·WPT·실기기 GPU로 표현 | 없음. 계획·증거는 C11.1/C11 및 공식 `72/213`을 유지하고 AVD software/iOS compositor 전 한계를 분명히 한다. |

추가 검증에서 첫 scoped Clippy 실행이 test-only `field_reassign_with_default` 및 `items_after_test_module`를 발견했다. 두 위치를 수정한 뒤 `cargo clippy --locked -p spinon-layout -p spinon-style-to-layout --all-targets -- -D warnings`와 `cargo clippy --locked -p spinon-runtime --features c14-1-live-v8-fixture-test --lib -- -D warnings`가 통과했다. `fixture-module-boundary.test.mjs` 32/32, Android+iOS surface comparator 71/71, Rust focused suites와 `cargo fmt --all -- --check`도 통과했다. Fixture-free 기본 host archive와 feature-on positive control은 [artifact 경계 근거](./fixture-artifact-boundary.md)에 기록했다. 이 검토는 full APK/IPA release artifact 제외나 GitHub CI 통과를 주장하지 않는다.
