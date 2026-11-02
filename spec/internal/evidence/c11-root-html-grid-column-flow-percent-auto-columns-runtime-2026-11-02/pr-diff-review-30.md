# C11.1 percentage implicit columns PR diff review (30)

검토 대상은 PR #1063 head `4c29498fb17634ecb4ceecaea782f3b3cd3cd5b9`와 base `9d8cafb06e93271ee813f71422bbea0772c62a04` 사이의 실제 diff다. 결과는 30개 실패 관점 통과, blocker 0이다. 이 검토는 제한 fixture slice만 다루며 C11.1/C11 완료 판정이 아니다.

| # | 별도 실패 관점 | 결과 및 확인 근거 |
|---:|---|---|
| 1 | C11.1 또는 C11 공식 checkbox가 이 PR에서 완료로 바뀌는가 | 통과: status 대장 파일은 diff에 없고 본문도 공식 72/213을 유지한다고 밝힌다. |
| 2 | 기능 경계가 일반 CSS Grid 지원으로 넓어지는가 | 통과: gate는 parsed Root HTML sparse column-flow candidate로 한정된다. |
| 3 | Chrome 관측이 규범 기대값처럼 기록되는가 | 통과: CRD 근거와 Chrome 관측·독립 산술 기대값을 별도 evidence로 연결했다. |
| 4 | 고정 fixture의 입력이 hash 없이 바뀔 수 있는가 | 통과: fixture source SHA-256이 precomparison expected와 runtime README에 기록됐다. |
| 5 | percentage basis가 viewport 또는 남은 공간으로 바뀌는가 | 통과: definite 240px content-width 기준과 zero-width 별도 case를 명세했다. |
| 6 | column gap이 percentage basis에서 빠지거나 위치에 두 번 더해지는가 | 통과: expected starts와 layout/runtime tests가 track resolution과 gap 위치 계산을 분리한다. |
| 7 | explicit columns 뒤 첫 implicit track이 pattern 중간부터 반복되는가 | 통과: `[10%,15%]` 두 차례 순환의 여섯 시작점과 구현 verifier가 pattern 순서를 확인한다. |
| 8 | scalar, mixed, zero, over-100%, zero-basis 경계 중 하나가 누락되는가 | 통과: 다섯 경계가 expected, Rust test matrix 및 구현 전 Chrome evidence에 있다. |
| 9 | syntax-invalid CSS fallback과 typed-but-unsupported profile 입력을 혼합하는가 | 통과: cascade fallback tests와 unsupported-shape runtime tests가 별도 경로를 유지한다. |
| 10 | 명시적 track이나 `grid-auto-rows`까지 percentage admission이 넓어지는가 | 통과: fixed px explicit tracks와 auto rows 제한을 그대로 검사한다. |
| 11 | 직접 자식이 아닌 item, 비어 있지 않은 item, nested Grid가 자동 포함되는가 | 통과: adapter와 layout 검사가 비지원 구조를 fail-closed로 거부한다. |
| 12 | 숨김 item 또는 authored placement가 required column 계산을 왜곡하는가 | 통과: visible owned direct items만 세고 네 placement longhand와 nonzero `order`를 거부한다. |
| 13 | 10,000 track cap 전후에서 목록 확장이나 정수 계산이 넘치는가 | 통과: explicit-plus-implicit cap과 empty-row guard가 expansion 전에 실행되고 기존 quota tests가 유지된다. |
| 14 | NaN, infinity, 음수 percentage가 track expansion에 도달하는가 | 통과: cascade, projection 및 layout 검사 모두 finite/non-negative를 요구하고 synthetic invalid tests가 있다. |
| 15 | aggregate f32 overflow를 개별 track 검사만으로 놓치는가 | 통과: cumulative width/gap preflight와 aggregate-overflow test가 있다. |
| 16 | expansion output이 validated pattern과 다른 track을 반환하는가 | 통과: expanded verifier가 explicit prefix, 길이, 반복 값과 type을 비교한다. |
| 17 | track 예약 실패나 unsupported input 뒤 부분 scene이 공개되는가 | 통과: 기존 atomic failure/recovery test 및 새 unsupported-without-scene test가 있다. |
| 18 | auto columns가 cascade 이후 projection에서 px로 바뀌거나 탈락하는가 | 통과: typed percentage/mixed list cascade 및 projection tests가 값을 끝까지 비교한다. |
| 19 | 기존 non-column profile이 새 typed percentage list를 받는가 | 통과: cascade parser에 percentage option을 넘길 때 computed flow가 sparse column인지 확인한다. |
| 20 | layout validator는 허용하지만 실제 Taffy placement가 좌표를 다르게 내는가 | 통과: focused layout tests가 15개 start/frame과 0/120% 결과를 고정한다. |
| 21 | 앱 evidence가 bespoke scene adapter를 사용해 Root HTML parse 경로를 우회하는가 | 통과: Android/iOS FFI host는 같은 HTML asset으로 `new_with_root_html`을 만들고 host eval을 호출한다. |
| 22 | FFI 오류나 짧은 output buffer에서 host pointer가 잘못 반환되는가 | 통과: null/capacity 검사 및 `write_host_status` 실패 검사가 있고 오류 시 null을 반환한다. |
| 23 | fixture FFI 선언·구현이 제품 ABI에 기본 노출되는가 | 통과: Cargo feature, Rust module, C header declaration과 native bridge가 opt-in gate를 공유한다. |
| 24 | 기본 static archive에 fixture symbol/payload가 남는가 | 통과: default archive는 symbol/title absent, opt-in positive control은 둘 다 present다. |
| 25 | Android build flag가 잘못된 값 또는 C04 없이 fixture를 허용하는가 | 통과: Gradle과 build script는 0/1만 받고 C04 runtime GPU 동반을 요구한다. |
| 26 | Android JNI macro와 Rust/header feature가 서로 달라 route가 link-fail 또는 leak되는가 | 통과: native compile define, feature list, C header guard와 successful app build가 일치한다. |
| 27 | Android C11.1 route가 기존 generic pixel fixture를 함께 실행하는가 | 통과: 전용 route는 C0410에 따로 분기하고 runtime log에 generic C11 surface-pixel marker가 없다. |
| 28 | iOS build flag가 xcodebuild 경로 중 하나에서 빠지거나 기본 활성화되는가 | 통과: simulator/native 전달과 project default 0을 확인했고 opt-in build가 성공했다. |
| 29 | iOS route가 다른 C11 fixture를 선택하거나 실제 화면 캡처와 runtime geometry가 불일치하는가 | 통과: iPhone 17 Pro/iOS 26.2 Simulator에서 전용 host, 15 boxes, 240×63 grid 및 12 item frames를 기록했다. |
| 30 | PR 본문의 캡처 이미지 링크가 깨지거나 플랫폼 한계를 감추는가 | 통과: Agent Browser에서 Android 1080×2400 및 iOS 1206×2622 이미지가 모두 로드됨을 확인했고, pixel readback/device claim은 제외했다. 브라우저 세션은 확인 후 종료했다. |

PR 상태 확인 당시 GitHub는 status check를 보고하지 않았다. 이는 통과로 세지 않았으며, merge 전 required-check 설정과 현재 PR 상태를 다시 확인한다.
