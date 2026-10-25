# C11.4 cyclic row auto-repeat 일반화 구현·실행 검토 20개

## 검토 기준과 범위

- 기준 checkout: `HEAD=6150d4f289c0e2aabfd2b696748dc3c3be546827`, Git tree `55b79d5616dfff4381766c83e1625053fd37c57a`.
- 최종 Android/iOS 캡처 당시 tracked-diff SHA-256은 `f0f43809c99079c416fcefa6d26a93098e6d59581d33a0b91b78390fbc312570`이며, 캡처 당시 전체 diff를 뜻하고 code-only hash는 아니다. 현재 구현 코드 staged diff SHA-256 `6e16c33893a15ad5e083cde731f0883b6f2e2931d92191c1a6de6c388032ac06` 및 build/capture 시각 대조는 [source revision 검증](./source-revision-verification.json)에 기록했다.
- 실행 근거는 [Android 최종 run](./android-avd-c114-cyclic-percent-tracks-android-avd-20261025T182325255Z-dd43faa6/result.json)과 [iOS 최종 run](./ios-simulator-c114-cyclic-percent-tracks-ios-simulator-20261025T182411249Z-0b566316/result.json)이다. 앞선 캡처들은 서로 다른 diff hash를 기록하므로 현재 코드의 실행 근거에 섞지 않았다.
- 현재 소스, adapter, 기존 Rust test 코드와 이미 저장된 simulator 결과·로그·화면을 읽었다. 이 검토를 위해 빌드, 테스트, 브라우저, 시뮬레이터를 실행하지 않았다.

## 서로 다른 실패 관점

1. **기존 row fixture로의 분기 오염 — 정상 확인.** `grid.rs:861–868`은 새 4-node profile을 먼저 확인하고 일치하지 않으면 기존 scene validator로 계속 간다. 새 gate는 node 수가 4가 아니면 거부하므로 이전 root+item 2-node 장면을 새 규칙으로 잘못 받지 않는다. 이번 simulator run은 새 네 scene만 포함하므로 기존 장면의 재실행 근거는 별도 회귀 실행에 의존한다.

2. **fixture 경계가 기본 경로로 새어 나가는 문제 — 소스상 제한, 앱 전체 artifact는 미검증.** layout validator는 `c11-4-grid-track-sizing-fixture-test` 조건부 경로에 있다. 실행 근거 README는 static FFI archive의 기본/fixture positive control을 기록하지만 APK·IPA 전체 payload 부재까지 증명하지 않는다고 구분한다. 이번 검토에서는 빌드나 archive를 다시 만들지 않았다.

3. **scene ID 누락·오타로 adapter가 다른 검사를 적용하는 문제 — 정상 확인.** FFI `SCENES` 네 항목, Android scene list 및 `isC114GeneralizedCyclicRowScene`, iOS scene set이 네 식별자를 같은 철자로 포함한다. 두 `result.json`은 네 ID 각각을 한 번씩 기록한다.

4. **root 치수·box model·정렬·gap mutation을 조용히 허용하는 문제 — 정상 확인.** `cyclic_percentage_auto_repeat_rows.rs:44–69,108–110`은 content-box 너비 24px, max-content 또는 허용된 control 높이, start 정렬, row gap 2px·column gap 0px 및 그 밖의 기본 style 값을 구조체 전체 비교로 고정한다. 이 allowlist는 root style 변화가 새 profile에 들어오는 것을 막는다.

5. **repeat 범위·주변 track·percentage grammar의 오해 — 정상 확인.** 같은 validator `:110–132`는 `[8px, minmax(0,25%), 8px]`, repeat index `[1,2)`, 단일 반복 track, 빈 line names, fixed 24px column만 받는다. 저장된 FFI 입력은 이 문법과 맞고 arbitrary track-list 일반 지원을 주장하지 않는다.

6. **column 또는 양축 cycle이 row 예외로 통과하는 문제 — 정상 확인.** profile은 `rows_auto_repeat`가 있고 `columns_auto_repeat`가 없어야 한다(`:15–17`). `grid.rs:861`의 새 gate는 기존 column/양축 검증을 바꾸지 않으며 계획의 fail-closed scope와 맞는다. 두 simulator trace에서 유일한 cyclic sizing pass는 Rows다.

7. **browser 전용 suffix probe가 runtime child로 섞이는 문제 — 정상 확인.** FFI scene source `:24–53`은 root와 prefix·primary·secondary 네 div만 만들고 `nodeIds`도 네 개로 고정한다. `:57–64` registry는 네 scene만 resolve하며, `:66–108` source test는 추가 span/suffix node를 포함하지 않도록 작성돼 있다. 이번 실행의 box count도 네 개다.

8. **DOM 순서와 grid placement 불일치 — 정상 확인.** FFI source의 append 순서는 prefix, primary, secondary이고 root `nodeIds`도 같은 순서다. Layout gate `:20–42,91–106`은 direct-child 배열을 역할에 연결하고 prefix를 row 1, 두 겹침 item을 row 2 / column 1로 정확히 검사한다. Android/iOS 결과의 node 순서와 frame 로그가 이를 따른다.

9. **child의 intrinsic/style 입력이 장면 의미를 바꾸는 문제 — 정상 확인.** gate `:70–80,133–144`는 24×8, 24×24, 16×16의 leaf만 받고 min-size 0, static 위치, auto inset, 기본 margin/padding/border를 고정한다. 저장된 scene node frame과 pixel target ID는 각각 prefix·primary·secondary 역할과 맞는다.

10. **겹친 두 item 기여를 합산하는 문제 — 정상 확인.** Android `:1094+`와 iOS `:2833+`은 span 1의 MaxContent contribution 두 개를 item 3=24px, item 4=16px로 각각 검사한다. 최종 root 44px은 8+max(24,16)+8+두 gutter 4로 설명되며 40px 합산 결과와 다르다.

11. **intrinsic 단계의 percentage basis 또는 repeat count 오판 — 정상 확인.** 양 adapter의 generalized trace matcher는 max-content scene에서 순서가 증가하는 두 Rows pass를 요구한다. 첫 pass는 `IntrinsicMaxContent`, count 1, basis null, rows `[8,24,8]`과 ordered gutter 상태다. 최신 Android/iOS runtime trace에는 각각 이 단계의 contribution과 상태가 들어 있다.

12. **final cyclic resolution에서 count를 다시 늘리거나 basis를 틀리는 문제 — 정상 확인.** 같은 max-content trace matcher는 두 번째 pass가 `FinalDefinite`, count 1, basis 44px, rows `[8,11,8]`이어야 통과시킨다. 네 scene 중 두 cyclic target의 최신 Android/iOS 결과가 이 구조를 기록하고 반복 행은 11px로 확정된다.

13. **definite auto-fill control을 cyclic 예외로 계산하는 문제 — 정상 확인.** `grid.rs:835–852`는 definite trace 진단과 indefinite cyclic 허용을 별도로 호출한다. 새 gate는 `allow_definite_control`이 true일 때만 44px root를 수용한다. fill control의 최종 Rows count 2, basis 44px, `[8,11,11,8]`가 양 플랫폼 trace와 결과 JSON에 기록됐다.

14. **definite auto-fit에서 repeat count와 collapse/gutter 경계를 혼동하는 문제 — 정상 확인.** adapter matcher는 initial count 2를 보존하면서 두 번째 repeated CSS track(zero-based index 2, gutter 포함 trace index 5)을 0px/collapsed로, 뒤 gutter(trace index 6)를 0px/collapsed로 각각 검사한다. 결과 JSON은 `[8,11,0,8]`과 collapsed flag를 기록한다. 전체 trace에는 prefix/repeat 사이 2px gap과 occupied/collapsed repeat 경계의 shared 2px gap이 남는다. suffix의 `y=23px`은 track/gutter 누적값이며 suffix probe 자체는 runtime node가 아니다.

15. **trace 비교가 순서·gutter·collapse를 생략하는 문제 — 정상 확인.** Android `:1001–1040`과 iOS `:2793–2809`는 Rows 마지막 stage의 전체 ordered track 배열, `internalIndex`, gutter, collapsed 및 base size를 비교한다. Columns matcher는 Android `:1055–1085`, iOS `:2746–2766`에서 별도로 유일한 pass·`[24px]`·`FixedTrackEarlyReturn`을 검사한다. 허용오차는 Android 0.0001px, iOS 1/64px이고 기대값은 정수 px다.

16. **layout frame을 trace와 별도로 확인하지 않는 문제 — 정상 확인.** 두 결과 JSON의 네 scene 모두 root `(0,0,24,44)`, prefix `(0,0,24,8)`, primary `(0,10,24,24)`, secondary `(0,10,16,16)`, `boxes=4`, `drawPresented=true`, `supersededRetries=0`을 기록한다. 이 조합은 track trace만 우연히 맞고 배치·draw가 달라지는 경우를 잡는다.

17. **pixel matrix가 틀린 node/좌표를 읽거나 paint order를 놓치는 문제 — 정상 확인, 검증 범위 제한.** Android matrix allowlist `:6661+`와 iOS `:6030+`는 정확히 다섯 ID·target node·root-local 좌표를 확인한다. 표본은 prefix, 겹침 위치의 나중 paint된 secondary, secondary 바깥 primary, 11px track을 넘은 primary, 44px root 안의 배경이다. 이 표본은 paint order와 root 안 overflow를 증명하지만 root 밖 overflow나 border 획은 검사하지 않는다.

18. **Android DPR·PixelCopy 캡처 세대·색상 불일치 — 최신 AVD 근거 정상 확인.** adapter는 320×240 CSS viewport에 대해 840×630 surface와 density 2.625를 로그로 남긴다. `C124SurfacePixelCapture.java:218–219`의 pixel-center 좌표식과 `C0410RuntimeGpuDemo.java`의 origin `(0,0)`·surface generation guard가 연결돼 있다. 최신 API 37 ARM64 AVD 로그는 네 scene 모두 PixelCopy 5/5 exact이고 run metadata의 APK/설치본 SHA `9f045a13…ca77c81d`가 같다. 해당 AVD는 software renderer라 hardware GPU 검증은 아니다.

19. **iOS drawable scale·scene 색상·화면 상태 혼동 — 최신 Simulator 근거 정상 확인.** iOS adapter는 네 ID별 같은 다섯 root-local 표본과 RGBA를 기대한다. 최신 iPhone 17 Pro/iOS 26.2 Simulator 로그는 960×720, 3× scale, 각 scene Metal drawable 표본 5/5 exact를 기록하고 screenshot에도 `5/5` 상태가 보인다. 설치된 `SpinonBootstrap` executable SHA `99be8687…71425d05`는 build SHA와 일치한다. 이는 compositor 제출 전 drawable 결과이며 실제 iOS 기기나 compositor 이후 결과는 아니다.

20. **오래된 capture를 현재 source 근거로 오인하는 문제 — 최신 run만 현재 revision에 결합.** 최종 Android/iOS `simulator.json`의 commit·tree와 캡처 당시 tracked diff SHA는 해당 실행의 source revision을 고정한다. 이후 evidence/review 파일을 stage하면서 전체 diff SHA는 바뀌었지만 앱 구현 파일은 수정하지 않았다. 앞선 run들은 서로 다른 tracked diff SHA를 기록하므로 최신 네-scene 결론의 근거에서 제외했다. Android APK SHA `9f045a13…ca77c81d`, iOS app executable SHA `99be8687…71425d05` 및 설치본 일치는 각 플랫폼 evidence에 기록됐다.

## 결론과 남은 통합 범위

이 검토에서 blocking 구현 결함은 발견하지 못했다. 네 고정 scene의 adapter route, 저장된 양 플랫폼 실행 frame·trace·표면 결과는 현재 tracked revision에 연결된다. 이 결론은 일반 CSS Grid 지원이나 전체 C11.4 완료를 뜻하지 않는다. `spec/STATUS.md:273`의 C11.4 체크는 미완료이며 공식 완료 수 `72/213`은 그대로다.

실행 근거 README는 focused/workspace tests, fixture-module boundary, Android/iOS Simulator app build와 static FFI archive 경계의 이전 실행 결과를 기록한다. 이 리뷰는 작업 제한에 따라 그 명령을 재실행하거나 각 빌드 원본을 독립 재검증하지 않았다. Android hardware GPU, APK/IPA 전체 fixture payload 제외, iOS compositor 이후 표면, 일반 author CSS와 WPT는 이 근거로 판정하지 않는다. PR 변경 실패 관점 검토와 병합 검증도 이 문서의 20개 구현 검토와 별도다.
