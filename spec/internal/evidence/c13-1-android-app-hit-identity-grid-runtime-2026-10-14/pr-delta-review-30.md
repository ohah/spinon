# C13.1 Android 앱 hit identity grid PR 변경 관점 검토 30개 · 2026-10-15

PR에 올릴 코드·상태·근거·이미지·플랫폼 설명을 계획 검토 및 구현 검토와 분리해 확인했다. 각 항목은 제출 delta의 독립적인 오해·누락·회귀 경계를 다룬다.

| # | PR delta 실패 관점 | 확인과 판정 |
|---:|---|---|
| 1 | 30,000점 앱 query를 일반 CSS 지원으로 표현한다 | route는 전용 C13.1 fixture-test profile이고 CSS 지원 계약은 아니다. PR 요약에서 범위를 고정한다. **통과.** |
| 2 | STATUS에서 C13.1을 완료 처리한다 | 이 delta는 앱 hit identity evidence만 추가하며 전체 C13.1 check는 미완료로 둔다. **통과.** |
| 3 | 내부 contract version을 구현 증분만으로 올린다 | 계약 0060은 `0.1.0-draft`를 유지하고 app route·evidence 범위만 덧붙인다. **통과.** |
| 4 | PR #163 계획이 implementation PR 전에 검토·병합되지 않았다 | #163은 merge SHA `9529b108476bb22e090d9ec3e6f1db5b70371f77`로 main에 병합됐다. **통과.** |
| 5 | Chrome oracle 파일이 다른 browser revision으로 바뀐다 | observation/grid file SHA와 Chrome 155 revision·UA가 고정돼 있다. validator 5/5가 provenance mutation을 거부한다. **통과.** |
| 6 | screenshots나 PR 그림이 oracle reference를 대신한다 | 앱 화면은 실행 표시, PixelCopy는 paint capture, Chrome JSON은 hit identity oracle로 각각 설명한다. **통과.** |
| 7 | 실제 앱 화면을 PR 본문에서 볼 수 없다 | PR #164 본문에 앱 screenshot을 inline embed했고 agent-browser에서 `complete=true`, natural size 1080×2400을 확인했다. **통과.** |
| 8 | PixelCopy surface crop을 앱 전체 화면으로 표시한다 | 1080×2400 앱 screenshot과 788×263 PixelCopy를 각각 저장하고 별도 alt text로 제공한다. **통과.** |
| 9 | raw image URL이 branch 이동이나 만료에 따라 달라진다 | 두 raw GitHub URL 모두 commit `3972c3d9e30a5106da725cde7f136076b5ecc5ad`와 저장소 경로를 고정했다. **통과.** |
| 10 | 52 fixed sample을 30,000 full integer point 비교로 잘못 부른다 | README/PR은 52 hit sample, 30,000 hit query, 52 PixelCopy sample을 분리해 쓴다. **통과.** |
| 11 | 30,000 CSS query를 full physical surface pixel parity라 설명한다 | 표본은 CSS 정수 좌표의 `elementFromPoint` 대 runtime hit identity 비교다. PixelCopy 전체 RGBA 대조가 아님을 명시한다. **통과.** |
| 12 | 화면의 `status=0`만으로 grid pass를 주장한다 | Logcat에 30,000/30,000, mismatch 0, identity/oracle SHA가 별도 남는다. **통과.** |
| 13 | 한 화면의 오래된 eval과 최신 PixelCopy를 같은 run으로 묶는다 | 실행의 eval/grid/PixelCopy가 동일 `run_id=22f72bd2a13f`로 연결된다. **통과.** |
| 14 | runtime generation과 SurfaceView generation이 같은 counter라 해석된다 | log의 runtime generation 1과 PixelCopy begin generation 2를 서로 다른 counter라고 설명한다. **통과.** |
| 15 | mismatch 상세가 과도하게 logcat/JNI 버퍼를 사용한다 | summary는 8,192 bytes 이하, mismatch preview는 16개 이하로 제한된다. 전부 missing인 focused control도 cap을 확인한다. **통과.** |
| 16 | malformed summary에서 앱이 성공 또는 matched 로그를 남긴다 | Java parser는 count, hash, preview shape·좌표를 확인하고 실패를 반환한다. 정상 AVD path는 재빌드 후 다시 실행했다. malformed JNI report fault injection은 미실행이다. **코드 검토 통과, 주입 미실행.** |
| 17 | 신규 fixture FFI ABI가 default header/API로 노출된다 | 기존 feature-gated FFI route를 사용하며 새 C header declaration/JNI symbol은 추가하지 않았다. **통과.** |
| 18 | oracle grid와 SHA 구현이 production/default 의존성을 추가한다 | `sha2`는 optional이며 fixture-test feature에서만 활성화된다. host archive default/positive control 확인을 별도 남겼다. **통과.** |
| 19 | fixture 코드가 경로만 보고 default에서 제외됐다고 단정한다 | 기본·opt-in host debug archive의 심볼·문자열 양방향을 검사해 결과를 기록했다. final release/APK size 판정은 하지 않는다. **통과.** |
| 20 | Android Gradle flag와 Rust feature명이 어긋난다 | API 37 app build에 `SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST=1`을 설정했고 APK가 route를 실행했다. **통과.** |
| 21 | Java report 보강이 앱 컴파일을 깨뜨린다 | 변경 뒤 Android debug APK build가 성공했고 같은 산출물을 AVD에 설치·실행했다. **통과.** |
| 22 | PR이 연결한 APK hash가 실행 APK와 다르다 | 설치 APK의 SHA-256 `3ba6b91753e687c0a81ca62a49b349897d5809ac09685a02add08553af41ce4f`를 evidence에 기록했다. **통과.** |
| 23 | 연결 장치가 physical phone인데 AVD 결과로 보고한다 | `adb devices -l`에는 `emulator-5554`만 있고 `ro.kernel.qemu=1`, ABI `arm64-v8a`다. **통과.** |
| 24 | GPU 결과가 hardware device 성능 또는 Metal parity로 과장된다 | log는 Android Emulator ANGLE/Vulkan/SwiftShader backend다. iOS는 build-only로 분리하고 hardware/Metal readback을 주장하지 않는다. **통과.** |
| 25 | app screenshot의 확대·상태 표시가 잘못된 route를 숨긴다 | 화면에 13개 case, 30,000 hit identity + 52 PixelCopy와 `boxes=41`, `layout=ready`가 보인다. **통과.** |
| 26 | 성공한 좁은 route를 WPT/event/C13.1 완료로 확대한다 | WPT, DOM event/CSSOM, frame 밖 negative controls, full physical-pixel parity와 C13.1 완료를 남은 범위로 명시한다. **통과.** |
| 27 | test log 수치가 현재 tree와 무관한 이전 run이다 | Rust tree 변경 뒤 workspace tests 779/0/4, workspace Clippy, feature FFI tests/Clippy와 Node validator 5/5를 기록했다. Java 변경 뒤 app build/AVD를 다시 실행했다. **통과.** |
| 28 | PR screenshot URL은 raw GitHub에서 표시되지 않는다 | PR #164 페이지를 agent-browser session `c13-1-apphit`으로 열어 두 raw PNG가 모두 `complete=true`인지 확인했다. natural sizes는 앱 화면 1080×2400, PixelCopy 788×263이다. **통과.** |
| 29 | PR template·labels·evidence links가 빠진다 | 본문은 상태 ID, 계약 0060, 실행 근거, 두 inline image, 남은 범위를 채웠다. GitHub에서 `enhancement`, `area: android`, `area: ffi` labels도 확인했다. **통과.** |
| 30 | 오래된 mergeability/check 상태를 보고 병합한다 | PR #164는 `MERGEABLE`, status check rollup/checks는 비어 있었다. main protection은 required checks 없음, required approvals 0, linear history 및 admin enforcement enabled다. 추가 push 뒤에는 현재 head를 다시 조회하고 Rebase and merge로 처리한다. **이 검토 시점 통과; 병합 직전 재조회 필요.** |

최종 evidence-link review commit 뒤에는 PR head·checks·mergeability를 다시 확인한다. PR #164는 repository 규칙대로 Rebase and merge 대상이다.
