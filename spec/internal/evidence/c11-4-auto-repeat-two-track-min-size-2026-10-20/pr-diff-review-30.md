# C11.4 definite minimum-size two-track auto-repeat · PR diff 적대 검토 30개

**검토 범위:** 현재 PR diff 전체. 계획 검토와 구현 후 코드·런타임 검토는 별도 기록이며 이 표에 합산하지 않았다. 각 행은 서로 다른 실패 경계다.

| # | 실패 관점 | 판정과 근거 |
| ---: | --- | --- |
| 1 | Rust scene source에 넣은 8개 ID와 capture matrix ID가 서로 다르면 일부 장면이 실행되지 않는가? | **통과.** Rust의 18 scene 중 기존 10개와 신규 8개가 있고, simulator matrix 및 두 앱 gate에 동일한 신규 ID가 있다. 양 플랫폼 결과에 21개 관측이 기록됐다. |
| 2 | 신규 fixture가 일반 빌드나 제품 artifact에 새 scene 문자열을 끌어들이는가? | **통과.** 기본 `spinon-ffi` rlib에는 신규 ID가 없고, `c04-runtime-gpu-fixture-test` positive-control rlib에는 있다. [artifact SHA와 scan 결과](./fixture-artifact-boundary.md) |
| 3 | 이 변경이 테스트 전용 adapter에서 제품 Grid sizing 경로로 새 조건을 우회시키는가? | **통과.** diff는 scene source, 앱 fixture validator, capture assertion과 결과 문서만 바꾼다. product track-sizing algorithm은 수정하지 않았다. |
| 4 | 두-track pattern이 실제로 한 repeat fragment 안의 `minmax(40px,1fr) 20px`로 작성됐는가? | **통과.** columns와 rows fixture CSS가 같은 2-track fragment를 사용하며 Chrome 사전 비교 fixture와 일치한다. |
| 5 | both-neighbor 반복 횟수 산식에 prefix/suffix/gap이 빠져 경계가 잘못됐는가? | **통과.** 사전 비교는 `38 + 76n`으로 고정했고, 구현 결과 track·root 값이 산식과 맞는다. [Chrome 사전 비교](../c11-4-auto-repeat-two-track-min-size-precomparison-2026-10-20/README.md), [simulator 결과](./README.md) |
| 6 | 190px 경계가 두 번 반복되어야 할 때 반복 수가 잘못 선택되는가? | **통과.** columns/rows가 각각 root `190×60`, `50×190`과 fragment 2회를 기록했다. |
| 7 | 200px 경계가 세 번 반복되어야 할 때 이전 반복 수에 머무는가? | **통과.** columns/rows가 각각 `266×60`, `50×266`과 fragment 3회를 기록했다. |
| 8 | 342px 경계에서 네 번째 반복이 누락되거나 다섯 번째로 넘어가는가? | **통과.** columns/rows가 각각 `342×60`, `50×342`과 fragment 4회를 기록했다. |
| 9 | prefix-only 구성에서 앞 fixed track 및 그 gutter가 minimum-size 계산에서 누락되는가? | **통과.** prefix-only scene은 3회 반복과 root width 248px, 기대 used tracks를 양 플랫폼에서 기록했다. |
| 10 | suffix-only 구성에서 뒤 fixed track이 반복 pattern에 포함되거나 위치가 바뀌는가? | **통과.** suffix는 마지막에 한 번만 나오며 root width 238px 및 `40,20` pattern 3회 뒤 10px track을 확인했다. |
| 11 | 같은 column 가정을 rows 축에 복사해 min-height나 Rows trace를 놓치는가? | **통과.** rows 축에 190/200/342 경계를 따로 구성했고 simulator validator가 `Rows` final pass를 검사했다. |
| 12 | 두 개의 repeat axis를 한 scene에서 켜 지원 범위를 넓혀 보이는가? | **통과.** 각 scene은 auto-repeat 한 축만 둔다. 문서에는 두 축 동시 auto-repeat을 남은 범위로 명시했다. |
| 13 | `auto-fill`이 빈 grid에서 `auto-fit`처럼 반복 track을 접는가? | **통과.** 신규 scene의 collapsed 상태는 모두 false로 검사했고 결과 track 수가 유지됐다. |
| 14 | 빈 scene에서 track sizing을 가짜 item frame 또는 두 번째 node가 가려 주는가? | **통과.** Swift/Java 기대값은 item frame 공백, box count 1, root-only node frame이며 런타임 결과도 `boxes=1`이다. |
| 15 | 앱의 scene allowlist에 신규 ID가 빠져 native launch에서 거부되는가? | **초기 실패를 발견·수정.** 첫 iOS run에서 `SPINON_C114_GRID_ARGUMENT_ERROR`가 발생했다. Swift/Java allowlist와 frame table을 함께 고친 뒤 양쪽 21/21이 통과했다. [초기 거부 로그](./ios-simulator-c114-neighbors-ios-simulator-20261019T205026403Z-b00ac83e/runtime-auto-fill-two-track-columns-both-190.log) |
| 16 | root expected frame 문자열을 재사용하면서 dimensions가 이전 scene에 고정되는가? | **통과.** 8개 scene별 기대 frame을 명시했고 summary와 전체 node frame을 독립적으로 대조한다. |
| 17 | root-only frame serializer가 구분자·node ID를 누락해 partial string이 우연히 맞는가? | **통과.** Android/iOS가 `node=1,x=0,y=0` 및 해당 scene root dimensions 전체를 비교한다. |
| 18 | Java와 Swift가 한 플랫폼에만 scene을 등록하거나 기대 frame이 서로 달라지는가? | **통과.** 두 플랫폼의 동일 8 ID/frame contract를 검토했고 각 Simulator 결과가 전 scene 통과했다. |
| 19 | capture scene 목록에서 item frame 누락을 빈 문자열로 오해하는가? | **통과.** capture는 `null`을 명시해 item frame 검사를 건너뛰고, root-only 전체 node frame과 box count는 계속 요구한다. |
| 20 | final track list에서 gutter를 track으로 세거나 track을 gutter로 세는가? | **통과.** final non-gutter track 수·순서·크기를 각각 검사하고, 신규 scene은 active gutter 수를 `tracks - 1`로 별도 검사한다. |
| 21 | layout root frame만 맞고 실제 used track 크기는 틀려도 성공하는가? | **통과.** 각 used track을 사전 고정값과 대조하고 CSS px 허용 오차는 1/64로 제한한다. |
| 22 | active gutter가 8px가 아니어도 total root width가 우연히 맞아 통과하는가? | **통과.** 신규 8 scene마다 활성 gap 개수와 각 gutter base size 8px를 별도 assert한다. |
| 23 | trace chunk 하나가 누락되거나 서로 다른 run의 chunk가 합쳐져도 JSON parse가 성공하는가? | **통과.** run ID별 chunk의 part/total 완전성과 총수 일치를 검사한 뒤 순서대로 JSON을 parse한다. |
| 24 | Columns trace를 Rows scene에서 사용하거나 오래된 stage를 마지막 결과로 취급하는가? | **통과.** scene axis와 일치하는 pass만 고르고 해당 pass의 마지막 stage를 검사한다. |
| 25 | layout만 준비되고 draw/present가 되지 않은 scene을 앱 확인 완료로 기록하는가? | **통과.** 모든 장면에서 draw marker와 예상 presented box 수가 필요하며 run evidence에 스크린샷이 있다. |
| 26 | Android API 37라는 이름만 확인해 USB 기기·비 ARM64 emulator에 실행하는가? | **통과.** emulator serial, `ro.kernel.qemu=1`, API 37, `arm64-v8a`를 확인했다. 실행 metadata는 `emulator-5554` AVD이고 USB 기기는 사용하지 않았다. |
| 27 | iOS 결과가 다른 simulator/device인데 고정 기종으로 적히는가? | **통과.** 지정 UDID의 Booted 상태, `iPhone 17 Pro`, iOS 26.2 runtime을 assert하고 metadata를 저장했다. |
| 28 | 캡처 이미지가 다른 바이너리에서 생성됐는데 빌드 artifact hash를 현재 앱 hash로 표시하는가? | **통과.** README의 iOS executable과 Android APK SHA-256은 각 최종 simulator metadata 값과 일치한다. PNG 해상도도 실행 target에 맞는 1206×2622 및 1080×2400이다. |
| 29 | 실행 산출물 경로 변경이 앞서 병합된 explicit-neighbor 결과를 덮어쓰거나 삭제하는가? | **통과.** 이번 캡처는 별도 `c11-4-auto-repeat-two-track-min-size-2026-10-20/` 아래에 저장하며 이전 neighbors evidence root는 변경하지 않았다. |
| 30 | 제한 fixture 결과가 전체 C11.4 완료나 공개 CSS 지원으로 표현되는가? | **통과.** STATUS와 두 계획 문서는 C11.4 checkbox·완료 수를 유지하고, WPT·실기기·hardware GPU·compositor-after·auto-fit 차이를 남은 범위로 기록한다. |

두 simulator matrix는 각 21/21이고 관련 Rust focused test, Clippy, rustfmt, Node syntax check, iOS/Android app build가 통과했다. `main`의 required check와 PR CI rollup은 현재 별도로 확인해야 하며, 이 검토 결과가 GitHub CI 통과를 뜻하지 않는다.
