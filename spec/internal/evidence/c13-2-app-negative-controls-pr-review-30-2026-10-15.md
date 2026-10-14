# C13.2 앱 오류 원자성 PR 변경 적대 검토 30개

- **대상:** 이번 PR의 Rust FFI control, Android/iOS app assertions, 재평가 fixture, 상태 문서, simulator evidence와 build 경계.
- **독립성:** C13.2 계획, core runtime, root/body 후속 PR의 review 표를 복사하지 않았다. PR diff가 각 레이어 사이를 통과할 때 생기는 별도 실패 경로를 확인했다.
- **근거:** `cargo test --locked --workspace` 797/0/4, FFI feature test 43/0/2, workspace·feature Clippy, rustfmt/diff, reference 및 boundary Node test, default/opt-in archives, API 37 ARM64 AVD와 iOS 26.2 Simulator의 final build/run.
- **판정:** 잘못된 오류 원인 오인 가능성을 찾아 exact marker 검사로 고쳤다. 남은 픽셀 readback, 실제 OOM과 concurrent race는 범위 밖으로 문서에 남겼다.

| # | PR 변경이 깨뜨릴 수 있는 경로 | 확인 및 판정 |
|---|---|---|
| 1 | opt-in failpoint field 또는 error string이 기본 runtime에 유입된다. | 관련 Rust field/use와 fixture module은 feature-gated이며 default static archive에서 C13.2 marker가 없다. **PASS** |
| 2 | app 전용 FFI symbol이 기본 C ABI로 노출된다. | 기본 archive에서 C13.2 new/eval symbol이 없고 opt-in archive에서 positive control로 보인다. public header에는 새 public declaration을 추가하지 않았다. **PASS** |
| 3 | Cargo feature 이름이 앱 빌드에서 쓰는 fixture 목적과 달라 잘못 연결된다. | `c13-2-scroll-fixture-test`가 runtime fixture adapter 목적을 명시하고 fixture source를 opt-in으로 compile한다. **PASS** |
| 4 | Android Gradle flag만 켜지고 Cargo feature는 꺼져 null FFI route를 통과한다. | Android build log는 C04와 C13.2 fixture define을 확인하고 새 APK route가 status 0으로 종료했다. **PASS** |
| 5 | iOS Xcode define은 켜지지만 Rust native feature가 빠진다. | iOS build log의 native feature list, 성공 link, Simulator runtime assertion을 각각 대조했다. **PASS** |
| 6 | FFI negative control이 root/body 또는 lifecycle baseline보다 먼저 돌아 baseline state를 검증하지 않는다. | FFI order는 visible root/body, hidden root/body, matrix/offset, lifecycle, negative controls다. 두 앱 route가 이 순서로 전체 success report를 만들었다. **PASS** |
| 7 | internal control 실패가 FFI status 0으로 감싸져 앱이 성공 처리한다. | negative control `Err`는 `write_host_status` success path보다 먼저 반환된다. 양 app gate는 status와 assertion token을 함께 확인한다. **PASS** |
| 8 | reserve failpoint report에 error marker가 빠져도 `ERR_LAYOUT`만으로 pass한다. | diff 검토에서 이 false-positive를 발견했다. exact `c13_2_injected_reserve_failure` marker 검사를 더하고 Android/iOS gate에 `reserve_failure_marker=matched`를 추가했다. **FIXED · PASS** |
| 9 | Android UI가 reservation failure token을 검사하지 않는다. | Java route는 `c13_2_failure_controls`, `reserve_failure=atomic`, `reserve_failure_marker=matched`, stale rejection 및 same-host recovery를 모두 요구한다. **PASS** |
| 10 | iOS UI가 Android와 다른 token 집합으로 Swift route에서 성공 표시한다. | Swift gate가 동일한 실패·stale·recovery token을 요구하고 final simulator log가 같은 set을 기록한다. **PASS** |
| 11 | success log가 세부 assertion 검증 전에 발생해 로그만 보고 pass로 오인된다. | app success log는 FFI report token 검사 분기 안에서만 출력된다. simulator log에는 assertion 뒤 `status=passed`가 온다. **PASS** |
| 12 | response buffer가 확장된 report를 자르고 새 assertion token을 숨긴다. | Android와 iOS 실제 FFI caller가 최종 token을 모두 받았고 app gates가 이를 검사했다. **PASS** |
| 13 | 동일 fixture source를 다시 평가할 때 global lexical declaration `SyntaxError`가 난다. | Android 초기 trace에서 재현한 뒤 JS 전체를 IIFE로 감쌌다. 마지막 두 Simulator run은 같은 source 재평가를 통과했다. **FIXED · PASS** |
| 14 | matrix fixture가 재실행 시 이전 root를 남겨 owner count나 document root가 늘어난다. | 이전 matrix root, root/body scene 및 lifecycle/map globals를 cleanup한다. 다중 root failure trace 이후 최종 9-owner matrix가 양 플랫폼에서 통과했다. **FIXED · PASS** |
| 15 | scroll-state revision만 달라진 valid render/layout pair를 stale로 거부한다. | layout frame matching은 generation/document/render-tree/style/environment tuple과 render snapshot 존재를 확인해 independent scroll revision을 제외한다. Android/iOS matrix가 통과했다. **PASS** |
| 16 | render snapshot이 없는 layout 완료 값을 valid frame oracle로 사용한다. | `completed.render_snapshot.is_some()` gate를 유지한다. 없는 snapshot은 stale/error 처리한다. **PASS** |
| 17 | deterministic failpoint 결과를 OS memory exhaustion 증거라고 과장한다. | code와 evidence는 `try_reserve_exact` 직전 deterministic return만 주장하며 OS OOM을 명시적으로 제외한다. **PASS — 한계 기록** |
| 18 | sequential stale sequence 거부를 concurrent publish race 부재로 넓혀 해석한다. | review/evidence는 순차 negative control이라고 적고 scheduler race를 검증하지 않았다고 구분한다. **PASS — 한계 기록** |
| 19 | reserve error 뒤 화면이 계속 draw된다고 오해한다. | normal `eval`이 sequence를 먼저 invalidate하는 점과 control이 stored scroll state만 exact-equal 검사한다는 범위를 문서에 명시했다. old display frame 보존은 주장하지 않는다. **PASS — 한계 기록** |
| 20 | status label의 `boxes=5`를 final matrix가 다섯 box만 그렸다고 오인한다. | Android UI label은 첫 root/body scene 값을 보일 수 있다. final 26-box matrix는 runtime draw log로 별도 확인하며 README에 차이를 기록했다. **PASS** |
| 21 | workspace test summary가 feature-gated path를 검증한 것처럼 과장된다. | feature-specific FFI test 43/0/2를 별도 기록하고 797/0/4 workspace suite와 구분했다. **PASS** |
| 22 | feature 전용 module이 workspace 기본 Clippy에서는 컴파일되지 않아 lint failure가 숨는다. | targeted feature Clippy에서 경고 3개를 발견해 `is_none_or`, elided lifetime, identity map 제거로 고쳤다. feature Clippy 재실행이 통과했다. **FIXED · PASS** |
| 23 | default workspace Clippy와 formatting 회귀가 남는다. | `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`, `git diff --cached --check`를 staged PR diff에서 통과했다. **PASS** |
| 24 | Chrome fixture oracle의 source/scope가 negative-control 작업에서 바뀐다. | Chrome 155 reference test 5/5 통과했고 기존 `visible`, hidden/clip, nested/range observation을 그대로 유지했다. **PASS** |
| 25 | JS fixture relocation 뒤 module boundary 또는 app feature isolation이 깨진다. | fixture boundary Node tests 5/5 통과했고 source/payload는 opt-in Cargo adapter에 남는다. **PASS** |
| 26 | Android artifact가 marker 수정 전 코드에서 만들어져 test log와 source가 불일치한다. | 마지막 marker 및 lint 수정을 반영한 APK를 새로 build/install/launch했다. APK SHA와 current PID runtime log가 evidence에 고정됐다. **PASS** |
| 27 | ADB가 실기기 또는 다른 worktree 소유 AVD에 명령을 보내 검증 오염이 생긴다. | 최종 route는 API 37 `spinon_api37_0_applied_txn` / `emulator-5556`만 사용했다. `emulator-5554` 및 실기기는 건드리지 않았다. **PASS** |
| 28 | Android runtime 성공 log가 report token과 actual final draw를 분리하지 못한다. | exact assertion row에는 marker들이 있고 후속 draw에는 26 boxes, scroll revision 8, 9 offsets가 기록됐다. **PASS** |
| 29 | Android screenshot이 이전 build거나 잘못된 화면이어도 PR에서 성공 캡처로 보인다. | 새 run 뒤 1080×2400 app screen을 직접 캡처·육안 확인하고 SHA-256을 기록했다. 시각용이며 pixel readback은 아니다. **PASS** |
| 30 | iOS result가 Android와 다른 binary·scene·fixture에서 나왔거나 screenshot/link가 어긋난다. | iPhone 17 Pro/iOS 26.2 Simulator를 최종 binary로 재설치·실행했다. exact marker runtime, 26-box screen, 1206×2622 screenshot과 executable/screenshot hashes가 evidence에 묶였다. **PASS** |

## 통합 gate와 남은 범위

최종 PR을 게시한 뒤 GitHub checks/mergeability, labels, inline screenshot URLs와 merge SHA permalink를 별도 확인한다. 이 review는 OS OOM, concurrent race, ink-only overflow, PixelCopy/Metal readback, public CSSOM/gesture/event, scrollbar/snap/RTL 및 상위 C13 완료를 주장하지 않는다.
