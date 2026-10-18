# C12.4 반복 runtime session 수명 재검증 · 2026-10-19

## 판정

Android 17 / API 37 AVD `emulator-5554`에서 한 앱 process 안에 16 cycle × 8 session, 총 128개를 생성·해제했다. 매 cycle마다 V8 owner 8개와 CSS worker 8개 TID가 ready 시 존재하고 해제 뒤 사라졌으며, 다음 cycle 생성도 성공했다. 실행 요약은 [summary.json](./c124-lifecycle-20261018T194837821Z-5af75d12/summary.json), [PASS 화면](./c124-lifecycle-20261018T194837821Z-5af75d12/lifecycle-result.png), 전체 run 기록은 [README](./c124-lifecycle-20261018T194837821Z-5af75d12/README.md), [logcat](./c124-lifecycle-20261018T194837821Z-5af75d12/logcat.txt), [thread checks](./c124-lifecycle-20261018T194837821Z-5af75d12/thread-lifecycle-checks.json)와 cycle별 파일에 있다.

앱은 Android API 37 AVD (`sdk_gphone16k_arm64`, qemu=1) 하나에서만 실행했다. 최종 debug-only status screen은 PASS·16/16 cycle을 표시한다. 실기기나 iOS 기기 결과가 아니며 renderer, GPU surface, layout evaluation, WPT 적합성을 검증하지 않았다. PSS/RSS는 process 전체 관측값일 뿐 누수 판정이 아니다. stack-only commit/high-water 및 전역 allocator OOM 복구도 측정하지 않았다. 따라서 공식 C12.4 parent와 공개 CSS/API 지원은 계속 미완료다.

## 실패 원인과 수정

저장된 두 실패는 ADB 연결 실패가 아니었다. 지정한 AVD의 `get-state`와 emulator 속성 조회가 성공한 뒤 Android build 단계에서 멈췄다.

1. [`6475bae2` 실패 기록](./c124-lifecycle-20261018T190325038Z-6475bae2/failure.txt): V8 source checkout이 저장소의 고정 revision과 달라 native build가 거부했다. 고정 V8 source directory를 `SPINON_V8_DIR`로 지정했다.
2. [`01aab0ce` 실패 기록](./c124-lifecycle-20261018T190355067Z-01aab0ce/failure.txt): C12.4 worktree가 PR #328 이전 main에 있어 `spinon_jni.cc`가 선언되지 않은 C11.2.3 create/eval FFI symbol을 참조했다. C12.4 변경을 보존한 채 main에 rebase해 PR #328의 JNI gate 수정이 포함되도록 했다.
3. 최신 main에서 다시 빌드한 뒤 AVD run 전체가 통과했다. build log는 [성공 run](./c124-lifecycle-20261018T194837821Z-5af75d12/build.log)에 있다.

재발 방지를 위해 runner는 기본 target을 Android AVD로 두고 전체 기기 목록 `adb devices -l`을 열거하지 않는다. 인자로 받은 serial에만 `adb -s <serial> get-state`를 호출하며, 모든 이후 shell/install/force-stop 명령도 그 serial에 고정된다. 성공 실행은 [device.txt](./c124-lifecycle-20261018T194837821Z-5af75d12/device.txt)에 `emulator-5554`, `qemu=1`로 기록했다.

## 검사

[post-run process cleanup](./post-run-process-check.json) · [`focused-tests.log`](./focused-tests.log)는 `c12-4-android-device.test.mjs`와 `c12-4-repeated-session-lifecycle-validation.test.mjs`의 7/7 통과를 기록한다. [SHA-256 manifest](./SHA256SUMS)는 run 산출물 hash를 고정한다. AVD run은 [실패 관점 구현 검토 20개](../c12-4-repeated-session-lifecycle-implementation-review-2026-10-19.md)와 [PR diff 적대 검토 30개](../c12-4-repeated-session-lifecycle-pr-review-2026-10-19.md)에 연결했다.
