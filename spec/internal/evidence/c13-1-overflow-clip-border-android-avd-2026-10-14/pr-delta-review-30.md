# Android border route PR 변경 적대 검토 · 30개 관점 · 2026-10-14

이 검토는 PR #158의 Android AVD follow-up delta를 서로 다른 계약·실패 경계로 나눠 확인한다. iOS의 이전 review와 별도이며 병합 승인 기록이 아니다.

1. **실패 원인 분리** — ADB 연결 실패와 renderer pixel mismatch를 혼동하지 않았다. emulator online 뒤 3/6 paint 실패를 재현했다.
2. **실기기 경계** — `adb devices -l`은 `emulator-5554` 하나만 반환했고 `ro.kernel.qemu=1`이다. 물리 기기 결과로 확대하지 않았다.
3. **AVD 부팅 조건** — API 37, `arm64-v8a`, `sys.boot_completed=1`을 기록했다.
4. **빌드 opt-in** — C04 GPU와 C13.1 fixture-test define을 함께 켜 `BUILD SUCCESSFUL`을 확인했다.
5. **기본 제품 경계** — 새 host/eval JNI 함수는 기존 opt-in fixture-test compile define 안에서만 활성화한다.
6. **JNI 생성 경로** — Java native host 생성 declaration과 C++ symbol 이름이 일치한다. 최종 APK가 실제 host를 만들었다.
7. **JNI 평가 경로** — Java eval declaration과 C++ symbol이 일치하고 실제 report의 `status=0`, 5개 frame이 나왔다.
8. **앱 intent routing** — border extra가 고정 CSS 400×160 route, title, descriptor, PixelCopy branch에 연결된다.
9. **독립 fixture host** — border route는 기존 13-node host를 재사용하지 않고 별도 5-node C13.1 host를 만든다.
10. **서로 다른 route 충돌** — 기존 overflow와 border extra 동시 입력이 MainActivity에서 FFI host 생성 전에 거부됐다.
11. **두 번째 충돌 guard** — C0410 runtime에도 동일 guard를 남겨 직접 route 진입에서도 중복 host 초기화를 막는다.
12. **frame oracle** — Android report의 5 frame 좌표가 고정 CSS fixture와 정확히 일치했고 geometry summary가 `ok`다.
13. **hit sample 개수** — C++ logger와 Java parser가 각각 여섯 샘플을 요구한다. 초과/누락은 성공으로 보고되지 않는다.
14. **hit sample 순서/ID** — Java 검증은 ID, target, 좌표, node index, matched status를 표본별로 확인해 6/6을 통과했다.
15. **paint oracle** — 고정 Chrome DPR1 screenshot pixel과 비교해 내부 blue·gold strip·외부 root 색을 각각 대조한다.
16. **density scaling** — 실제 PixelCopy frame 1050×420과 density 2.625를 로그에 남기고 CSS viewport를 고정했다.
17. **SurfaceView 원본** — UI screenshot이 아니라 앱 내부 `PixelCopy` capture를 검사했다. WGPU presentation 이후 색상 표본이 나왔다.
18. **픽셀 해시 재현성** — 최종 PNG SHA-256 `d5a970cf...499578`가 이전 green run과 동일하다.
19. **surface generation stale guard** — callback은 surface/renderer generation과 dimensions가 바뀌면 discard한다.
20. **기존 route 회귀** — 같은 final APK에서 이전 C13.1 route의 13 frame, 15/15 hit, 11/11 PixelCopy를 다시 통과했다.
21. **실행 종료 범위** — 검증 뒤 앱 process만 force-stop했고 AVD는 booted로 유지했다. 다른 emulator나 tmux session을 전역 종료하지 않았다.
22. **Chrome oracle 재확인** — agent-browser에서 저장된 local fixture를 직접 열고 `elementFromPoint()`/rect JSON을 재수집해 저장된 oracle과 일치시켰다.
23. **Chrome image oracle** — README의 좌표는 고정 screenshot pixel 목록과 맞고 Android 샘플 색은 모두 exact RGBA다.
24. **route 기능 격리 test** — Node fixture boundary test가 JNI/Java symbol routing을 포함해 5/5 통과했다.
25. **Rust rendering unit tests** — border band geometry·ancestor clipping·유효 폭 경계의 WGPU crate test 18/18을 확인했다.
26. **스타일 실패 경로** — 비-solid와 부분 alpha를 cascade에서 거부하는 tests를 추가했고 둘 다 통과했다.
27. **workspace regression** — final `cargo test --locked --workspace`는 736 passed, 0 failed, 4 ignored다.
28. **정적 품질 검사** — final workspace Clippy `-D warnings`, rustfmt check, `git diff --check`가 통과했다.
29. **상태 문서 정직성** — 이 PixelCopy gate를 C13.1 또는 상위 C13 완료로 표시하지 않았다. 전체 matrix가 남아 있다.
30. **플랫폼/paint 한계** — AVD renderer는 ANGLE/SwiftShader의 GL이며 hardware GPU나 scanout 증거가 아니다. complex z-index descendants, 비-solid/부분-alpha, radius, event dispatch는 남은 범위다.

검토 중 실제 구현 결함은 parent background 뒤에 child paint가 가려지는 것과 solid border data 부재였다. 이를 수정하고 위의 Android route와 기존 route regression으로 다시 확인했다. PR #158은 draft/open 상태로 유지한다.
