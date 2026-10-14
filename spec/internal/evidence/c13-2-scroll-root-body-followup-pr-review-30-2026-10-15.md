# C13.2 root/body 후속 PR 변경 적대 검토 30개

- **대상:** 이번 후속 PR의 Rust FFI/host adapter, Android·iOS app gate, JS/Chrome fixtures, 상태 문서와 실행 artifact 간 통합.
- **독립성:** PR #172의 기존 30개 변경 검토를 복사하거나 재서술하지 않았다. 이번 변경이 레이어를 통과할 때 생기는 별도 결함 경로를 검토했다.
- **근거:** `cargo test --locked --workspace` 797 passed / 0 failed / 4 ignored, FFI fixture 43 passed / 0 failed / 2 ignored, Clippy, rustfmt, reference 5/5, fixture-boundary 5/5, API 37 ARM64 AVD·iOS Simulator build/run.
- **판정:** PR 변경 통합에서 미해결 P0/P1은 없다. 한 항목은 시뮬레이터 pixel readback과 app-level stale/fault 실행이 없어 범위 제한으로 기록한다.

| # | PR 통합 실패 시나리오 | 확인과 판정 |
|---|---|---|
| 1 | app용 C13.2 FFI symbol이 기본 binary에 포함되어 제품 runtime 경계를 넓힌다. | 기본 static archive에서 두 C13.2 symbol이 없고 opt-in positive control에는 있다. [기본 검사](c13-2-scroll-container-followup-2026-10-15/default-archive-scan.log)와 [opt-in 검사](c13-2-scroll-container-followup-2026-10-15/opt-in-archive-scan.log)로 확인했다. **PASS** |
| 2 | 새 JS fixture 문자열이 default archive에 들어가 fixture feature 격리가 무력화된다. | 기본 archive의 `c132-root` marker absent, opt-in archive present를 `strings`로 대조했다. **PASS** |
| 3 | root/body include 경로만 opt-in으로 보이고 실제 `include_str!` 모듈 선언은 기본 경로에서 살아 있다. | `runtime_gpu/fixtures` feature boundary 검사 5/5와 static archive marker 검사를 함께 확인했다. **PASS** |
| 4 | Cargo feature가 Android C define과 달라 앱이 null/unavailable FFI를 호출한다. | Android build log에서 `c04_runtime_gpu=1`, `c13_2_scroll_fixture_test=1`을 확인하고 새 APK에서 route가 `status=0`이었다. **PASS** |
| 5 | iOS Xcode define은 켜졌지만 Rust target feature가 빠져 link 또는 runtime 경로가 실패한다. | `ios-simulator-build.log`의 Xcode define과 성공 build, simulator runtime assertion를 함께 확인했다. **PASS** |
| 6 | FFI route가 root/body를 기존 scroll matrix 뒤에 실행해 matrix cleanup과 root/body owner 결과가 섞인다. | FFI sequence는 visible root/body → hidden root/body → state reset → 기존 matrix/lifecycle 순서다. 양 플랫폼 log에서 두 root report 다음 final 9-owner draw를 확인했다. **PASS** |
| 7 | 첫 root scene error를 무시하고 hidden scene 또는 matrix로 넘어가 거짓 성공을 보고한다. | 모든 eval stage의 `layout=ready`와 helper `Result`를 gate로 사용하고 error 시 조기 반환한다. FFI null/invalid tests 및 앱 정상 경로가 통과했다. **PASS** |
| 8 | 앱은 FFI status 0만 확인해 root/body 세부 assertion이 빠져도 pass를 표시한다. | Android Java와 iOS Swift가 각각 visible propagation, hidden body, border escape, hit, lifecycle 표지를 모두 검사한다. 두 플랫폼 log가 전부 기록했다. **PASS** |
| 9 | Android와 iOS route 중 한쪽에서 assertion marker 문자열이 달라 동일 fixture가 불균형하게 검증된다. | 두 app callsites의 marker 목록을 직접 대조했고 두 Simulator에서 같은 report token 집합이 통과했다. **PASS** |
| 10 | fixture 전환 때 이전 scroll state가 다음 matrix owner에 적용된다. | hidden root/body 뒤 `reset_c13_2_scroll_state_for_fixture_transition`를 호출하고 matrix의 9-owner initial/lifecycle 검사를 재실행했다. **PASS** |
| 11 | Chrome oracle는 400×240인데 simulator 앱은 다른 CSS viewport여서 range 수치가 우연히 맞거나 틀린다. | Android·iOS runtime logs 모두 `viewport_css_px=400x240`; Chrome JSON도 400×240, DPR1이다. **PASS** |
| 12 | README는 PR #172의 오래된 emulator-5554 screenshot/hash를 새 변경 증거로 재사용한다. | 새 API 37.0 AVD serial 5556, APK hash, screenshot hash, runtime log를 다시 기록했다. PR #172의 이전 artifact hash를 쓰지 않는다. **PASS** |
| 13 | 다른 작업의 emulator를 조용히 덮어써 앱 설치가 충돌하거나 다른 worktree 상태를 바꾼다. | 5554의 QEMU CWD가 `/private/tmp/spinon-c14-intrinsic-sizing`임을 확인하고 우리 설치 요청을 중단했다. 별도 read-only 5556에서 실행한 뒤 그 인스턴스만 종료했다. **PASS** |
| 14 | ADB가 실제 Android 기기를 선택해 사용자가 금지한 실기기 검증이 섞인다. | 모든 설치/실행 명령이 `-s emulator-5556`이고 serial은 API 37 emulator다. 해당 시점 `adb devices -l`은 emulator들만 보였다. **PASS** |
| 15 | app 화면 screenshot은 새 apk와 무관한 이전 install에서 캡처된다. | 새 APK build hash `324803…3424a` 후 read-only AVD에 `adb install` success를 받고 즉시 route 실행·screenshot 캡처했다. **PASS** |
| 16 | 화면을 픽셀별 oracle parity 또는 real hardware GPU 증거라고 과장한다. | screenshot은 육안 확인으로만 사용한다. AVD는 SwiftShader, iOS는 Simulator Metal이며 PixelCopy/Metal drawable readback은 이 후속에서 실행하지 않았다. **PASS — 범위 제한 명시** |
| 17 | iOS와 Android를 모두 빌드했지만 한쪽 runtime에서만 root/body adapter가 실행된다. | iPhone 17 Pro/iOS 26.2 log 및 API 37.0 ARM64 log가 각각 두 root/body marker와 final draw를 포함한다. **PASS** |
| 18 | app status label의 `boxes=5`를 final scroll matrix frame count로 오인한다. | README는 status label이 첫 root/body 평가 값을 보여줄 수 있음을 밝히고 final scene 판정은 draw log의 27 boxes/scroll offsets 및 FFI assertions로 분리했다. **PASS** |
| 19 | 구현 상태 대장이나 계획이 root/body app 검증을 계속 미실행으로 표시하거나 C13.2 완료로 과장한다. | STATUS/plan은 root/body 두 app route 완료, pixel readback·stale/fault·ink-only 미완료, 상위 C13 미완료로 맞췄다. **PASS** |
| 20 | PR reviewer가 실행 가능한 상세 근거 없이 simulator success만 본다. | 이 후속 PR에만 적용한 root/body 구현 review 20개와 PR delta review를 독립 문서로 기록하고 screenshots/logs/hashes에 연결한다. **PASS** |
| 21 | PR 본문에서 Android와 iOS screenshot 링크가 서로 뒤바뀐다. | PR #173의 두 `img` alt와 raw 경로가 각각 Android API 37 AVD·iPhone 17 Pro Simulator 파일과 일치하는지 에이전트 브라우저로 확인했다. **PASS** |
| 22 | screenshot의 raw URL이 현재 branch 이름을 가리켜 rebase/merge 뒤 사라진다. | PR #173 본문은 head branch가 아닌 commit `107718749d3733645e84624196ef9fa4a1dc7f59`에 URL을 고정했고 이미지가 로드됐다. rebase-and-merge 뒤 merge SHA로 다시 고정하는 후속 확인은 남아 있다. **PARTIAL — post-merge 확인** |
| 23 | URL은 열리지만 파일 경로나 artifact가 틀려 다른 이미지 또는 404가 표시된다. | PR DOM의 두 `img.currentSrc`가 지정된 Android/iOS PNG를 가리키고 `complete=true`; intrinsic size가 각각 1080×2400, 1206×2622였다. **PASS** |
| 24 | 본문 링크가 markdown anchor로만 보여 이미지를 inline으로 렌더링하지 못한다. | PR #173 conversation을 에이전트 브라우저와 rendered screenshot으로 확인했다. 두 inline 이미지는 body 안에서 표시되고 PR #171 결과를 대신 쓰지 않는다. **PASS** |
| 25 | evidence README의 상대 링크가 PR diff 위치에서 잘못된 상위 디렉터리를 가리킨다. | 고정 SHA의 evidence README를 에이전트 브라우저로 열고 Chrome JSON, 두 simulator screenshot/runtime log 및 회귀 로그 상대 링크가 같은 evidence 경로로 해석됨을 확인했다. **PASS** |
| 26 | 이미지 설명이 visual inspection을 pixel-exact readback 또는 hardware GPU proof로 오해시킨다. | PR #173 alt와 본문은 SwiftShader AVD·Simulator Metal 및 pixel readback 미실행을 명시한다. README도 동일한 한계를 기록한다. **PASS** |
| 27 | workflow가 앱/FFI 경로를 선택하지 않아 green CI가 새 변경을 빌드하지 않는다. | `.github/workflows`에 PR 실행 workflow가 없고 `docs-pages.yml`은 수동 `workflow_dispatch`만 제공한다. `gh pr checks 173`에는 체크가 없으며 CI 통과로 주장하지 않았다. **PASS — CI 미구성 기록** |
| 28 | 체크가 green이어도 branch protection의 required check 또는 mergeability가 충족되지 않는다. | main 보호 설정에서 필수 approval 0, required status checks 미설정, linear history 필수다. PR #173은 `MERGEABLE/CLEAN`이다. 병합 직전 재조회한다. **PARTIAL — merge 직전 재확인** |
| 29 | PR에 작업 유형 또는 핵심 영역 label이 빠져 저장소 분류 규칙을 위반한다. | PR #173에 기존 `enhancement`, `area: runtime`, `area: ffi`, `area: android`, `area: ios`, `area: testing` label을 붙였고 GitHub 화면에서도 확인했다. **PASS** |
| 30 | rebase-and-merge 뒤 PR 본문의 screenshot 및 evidence URL이 재작성된 SHA를 가리켜 깨진다. | 병합된 tree의 새 merge SHA로 URL을 갱신하고 에이전트 브라우저에서 이미지와 README 링크를 다시 열어야 한다. 병합 뒤에만 판정 가능하다. **PENDING** |

## PR gate

이 표는 로컬 변경 검토와 게시/병합 뒤에 확인할 PR 통합 관문을 함께 기록한다. PR #173에서 21, 23–27, 29번은 확인했다. 22번 raw image permalink 갱신과 28번 보호 설정 재조회, 30번 merge SHA permalink 검증은 병합 때 마무리한다. `Rebase and merge`만 사용하고 C13.2 전체 완료를 주장하지 않는다.
