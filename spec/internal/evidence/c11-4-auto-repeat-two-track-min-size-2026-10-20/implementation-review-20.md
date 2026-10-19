# C11.4 definite minimum-size two-track auto-repeat 구현 실패 관점 검토 20개

**검토 대상:** Rust C11.4 scene source, Android/iOS app fixture gate와 기대 frame 판정, simulator capture/assertion, final app/runtime evidence. PR #364의 plan-review 표는 재사용하지 않았다.

| # | 독립 실패 관점 | 결과와 근거 |
| ---: | --- | --- |
| 1 | 새 Rust scene ID가 추가됐어도 platform allowlist가 낡아 앱이 거부하는가? | **초기 실패 발견·수정.** 첫 iOS 실행에서 `SPINON_C114_GRID_ARGUMENT_ERROR`가 발생했다. iOS/Android scene allowlist 및 기대값 테이블을 함께 갱신한 뒤 양 플랫폼 21/21 통과. [초기 거부 로그](./ios-simulator-c114-neighbors-ios-simulator-20261019T205026403Z-b00ac83e/runtime-auto-fill-two-track-columns-both-190.log), [iOS gate](../../../../platforms/ios/Sources/C0410RuntimeGpuDemo.swift#L15), [Android gate](../../../../platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java#L125). |
| 2 | scene resolver가 추가 입력을 잘못된 `nodeIds` 또는 HTML source로 직렬화하는가? | **통과.** 18개 auto-repeat scene을 bounded `auto_repeat_neighbor_scene_source`로 생성한다. 새 8개 source는 root-only이며 scene ID와 `data-node`가 유지된다. [Rust scenes/source](../../../../crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs#L343). |
| 3 | min-size가 preferred width/height를 덮어쓰거나 잘못된 축에 들어가는가? | **통과.** columns는 `width:max-content; min-width`, rows는 `height:max-content; min-height`를 사용하며 반대 축은 고정한다. [scene inputs](../../../../crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs#L430). |
| 4 | 양쪽 neighbor, min-size 190px이 경계에서 2회가 아닌 1회/3회로 반올림되는가? | **통과.** CSS Grid에서 도출한 `38 + 76n`과 root `190×60`, 두 반복 fragment sequence를 양 simulator가 반환했다. [iOS result](./ios-simulator-c114-neighbors-ios-simulator-20261019T210104021Z-887d74fe/result.json), [Android result](./android-avd-c114-neighbors-android-avd-20261019T205756565Z-3355bf45/result.json). |
| 5 | min-size 200px 경계에서 3회 반복과 root 266px이 유지되는가? | **통과.** columns·rows가 각각 `266×60`, `50×266`과 `40,20` pattern 3회를 기록했다. 같은 result JSON의 `both-200` scene. |
| 6 | min-size 342px 경계에서 4회 반복과 root 342px이 유지되는가? | **통과.** columns·rows가 각각 `342×60`, `50×342` 및 정확한 4회 track list를 기록했다. 같은 result JSON의 `both-342` scene. |
| 7 | prefix-only scene에서 prefix 고정 track과 그 gap이 반복 수 계산에서 빠지는가? | **통과.** min-size 200px은 3회 fragment, root width 248px과 track `20,40,20,40,20,40,20`을 만들었다. |
| 8 | suffix-only scene에서 repeat 뒤 fixed track 순서가 바뀌거나 suffix가 반복마다 포함되는가? | **통과.** suffix는 반복 fragment 뒤 한 번만 오고 root width 238px, `40,20` 세 fragment 뒤 `10`을 확인했다. |
| 9 | row-axis repeat이 column formula에 의존하거나 min-height를 잃는가? | **통과.** 190/200/342 경계를 height 축에서 다시 실행해 높이와 Rows final stage의 모든 track을 대조했다. |
| 10 | `minmax(40px, 1fr) 20px`가 반복마다 `40,20` 대신 fragment 경계를 바꿔 직렬화되는가? | **통과.** 양축 trace의 최종 used tracks는 반복마다 `40,20`; explicit 20px track은 각 fragment에 포함됐다. |
| 11 | `1fr` max만으로 repeat-count basis를 계산해 definite minimum 40px을 무시하는가? | **통과.** 계획에서 고정한 spec fallback에 맞춰 각 반복의 flexible track 최소 40px을 사용하고 190/200/342 임계값을 맞췄다. Chrome 155 precomparison도 같은 세 경계다. |
| 12 | gap 수가 track 수와 불일치하거나 8px이 아닌 gutter가 생겨 root dimension만 우연히 맞는가? | **검증 추가·통과.** final trace에서 active gutter 수가 `tracks - 1`인지, 각 baseSize가 8px인지 1/64px 허용차로 검사했다. 최종 21/21 양 플랫폼 실행에 적용했다. [capture assertion](../../../../tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs#L352). |
| 13 | `auto-fill` 반복 track이 item이 없다는 이유로 `auto-fit`처럼 접히는가? | **통과.** 8개 새 scene은 root만 있고 모든 `collapsed`가 false인지 비교했다. |
| 14 | 빈 scene을 2 box 또는 가짜 item frame이 있는 것으로 보고하는가? | **초기 검증에서 검출될 경로를 닫음.** Swift/Java의 expected box count는 1, item frame은 빈 문자열, expected node frame은 root 하나다. 실제 summary도 `boxes=1`을 기록했다. [iOS frame contract](../../../../platforms/ios/Sources/C0410RuntimeGpuDemo.swift#L91), [Android frame contract](../../../../platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java#L290). |
| 15 | root expected frame과 실제 전체 node frame이 서로 다른 box-sizing 결과를 숨기는가? | **통과.** simulator capture가 exact root summary와 root-only `node_frames_css_px`를 별도로 대조하고 Swift/Java도 해당 frame을 독립 assertion한다. |
| 16 | app log의 trace chunk 하나가 잘려도 파서가 부분 JSON을 성공으로 처리하는가? | **통과.** chunk sequence가 1..total 모두 있는지 확인한 뒤 합쳐 JSON parse하고, 누락·총수 불일치 시 실패한다. [trace reconstruction](../../../../tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs#L329). |
| 17 | Columns trace를 rows scene에, 또는 Rows trace를 columns scene에 사용해 거짓 양성이 나는가? | **통과.** scene별 axis를 고르고 해당 축의 마지막 pass/final stage만 검사한다. [axis selection](../../../../tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs#L341). |
| 18 | frame 결과가 맞아도 renderer draw/present가 발생하지 않은 scene을 통과 처리하는가? | **통과.** 각 scene은 draw marker와 정확한 `presented boxes` 수를 모두 요구하며 PNG 캡처 로그도 저장했다. |
| 19 | Android target이 API 37 이름만 비슷한 실기기나 32-bit emulator로 바뀌어도 실행되는가? | **통과.** serial, `ro.kernel.qemu=1`, API 37, `arm64-v8a`를 모두 assert한다. 실행 metadata는 `sdk_gphone16k_arm64`; 연결 USB 기기는 대상에서 제외했다. [target guard](../../../../tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs#L256). |
| 20 | 다른 iOS runtime/기기 유형에서 나온 결과를 iPhone 17 Pro / iOS 26.2로 표시하는가? | **통과.** capture는 지정 UDID가 Booted이고 runtime `iOS-26-2`, name `iPhone 17 Pro`인지 assert한다. 새 장면 결과는 iOS Simulator 21/21이며 실기기 증거로 확대하지 않았다. [iOS guard](../../../../tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs#L372). |

이 검토는 고정 scene subset과 app simulator 실행을 다룬다. WPT, 실기기, hardware GPU, iOS compositor-after sampling과 전체 C11.4를 통과로 간주하지 않는다.
