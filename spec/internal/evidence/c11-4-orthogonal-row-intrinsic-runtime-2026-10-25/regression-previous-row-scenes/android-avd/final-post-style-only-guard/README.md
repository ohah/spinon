# C11.4 최종 style-only guard 이후 Android 회귀

2026-10-25에 fail-closed style-only guard가 포함된 staged 최종 소스로 APK를 다시 빌드하고, orthogonal 4개와 기존 #611/#620 scene 8개를 API 37 ARM64 16 KB AVD에서 실행했다. 12개 모두 Android native matcher, frame/track/contribution 검사, draw 제출, trace 재조립, PNG 캡처를 통과했다.

## 소스와 APK

- HEAD: `d37ee6ea8c985ab27eb32770491c8998dcbf756c`
- staged index tree: `45bb3313f5c126fd724eb54e877f86746107a715`
- staged diff SHA-256: `e5c870f07de8345841aca43fee29bba53b50b81c4f4bff27bfe863ff0739224e`
- staged path list SHA-256: `d90b2f201a6273f574e72c06b6ab02ac91f5cf47d92b04d98c5743549ba39fba`
- source snapshot SHA-256 (88 staged source/config files, working tree == index): `2cae262c3b1cc8bfcb420b84313f53540909c711d834f001d69058dde249ce11`
- 빌드 명령: `mise exec -- env SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 bun run build:android`
- 결과 APK: `platforms/android/app/build/outputs/apk/debug/app-debug.apk`
- local/installed APK SHA-256: `cf978a383a3abceeaa99fe37d2f5c202f859193f20a352d4eea67dd0626371af` (일치)
- 빌드 성공. 로그의 유일한 경고는 AGP 8.13.2가 compile SDK 37.2를 아직 검증하지 않았다는 호환 권고다.

`source-before-build.json`과 `source-after-build.json`은 HEAD, staged tree/diff, 경로 목록, staged source file별 index/worktree 해시를 보존한다. 실행 전후 source snapshot과 APK 해시가 동일했다.

## AVD 및 종료

- `spinon_api37_1_compat16k`, serial `emulator-5570`, API 37, `arm64-v8a`, qemu=1, `getconf PAGESIZE=16384`.
- 기기 식별: `sdk_gphone16k_arm64` / `emu64a16k`; 화면 1080×2400, density 420; SwiftShader.
- ADB 명령은 모두 `adb -s emulator-5570`로 실행했다. 물리 기기나 다른 emulator를 조회하거나 사용하지 않았다.
- 검증 뒤 `adb -s emulator-5570 emu kill`이 성공했다. 최종 serial check는 `device 'emulator-5570' not found`, 시작한 emulator PID 36810도 사라졌다. 자세한 원본은 `shutdown-verification.json`과 로그에 있다.

## 결과

각 행은 native matcher PASS, `SPINON_C0410_DRAW status=0 presented boxes=3`, 예상 frame 및 최종 track 크기 일치를 뜻한다. `android-results.json`에는 개별 frame, contribution, trace/log/PNG 해시가 들어 있다.

| Scene | Root / text / image frame | Columns / rows | Contribution 수 |
| --- | --- | --- | ---: |
| `direct-orthogonal-row-min-content-forward` | 74×50 / 50×50 / 24×16 at (50,0) | [50,24] / [50] | 4 |
| `direct-orthogonal-row-min-content-reversed` | 74×50 / 50×50 / 24×16 at (50,0) | [50,24] / [50] | 4 |
| `direct-orthogonal-row-max-content-forward` | 74×80 / 50×80 / 24×16 at (50,0) | [50,24] / [80] | 6 |
| `direct-orthogonal-row-max-content-reversed` | 74×80 / 50×80 / 24×16 at (50,0) | [50,24] / [80] | 6 |
| `direct-min-content-forward` | 120×48 / 50×20 / 24×16 at (0,24) | [50] / [24,24] | 4 |
| `direct-min-content-reversed` | 120×48 / 50×20 / 24×16 at (0,24) | [50] / [24,24] | 4 |
| `direct-max-content-forward` | 120×48 / 80×10 / 24×16 at (0,24) | [80] / [24,24] | 6 |
| `direct-max-content-reversed` | 120×48 / 80×10 / 24×16 at (0,24) | [80] / [24,24] | 6 |
| `direct-row-min-content-forward` | 74×20 / 50×20 / 24×16 at (50,0) | [50,24] / [20] | 4 |
| `direct-row-min-content-reversed` | 74×20 / 50×20 / 24×16 at (50,0) | [50,24] / [20] | 4 |
| `direct-row-max-content-forward` | 74×20 / 50×20 / 24×16 at (50,0) | [50,24] / [20] | 6 |
| `direct-row-max-content-reversed` | 74×20 / 50×20 / 24×16 at (50,0) | [50,24] / [20] | 6 |

Every captured contribution was checked against its item, min/max kind, value, span=1, and `crossesFlexibleTrack=false`. The direct orthogonal max-content cases retained both min- and max-content query records. Draw telemetry proves frame presentation; it does not prove painted text/image pixels.

## 원본 근거

- `android-results.json`, `final-verification.json`: scene별 판정과 파일 해시.
- `android-runtime-<scene>.log`, `android-launch-<scene>.log`: 원본 runtime 및 launch log.
- `android-trace-<scene>.json`: 원본 trace chunk를 순서대로 이어 붙여 파싱한 JSON.
- `android-<scene>.png`: AVD에서 직접 캡처한 1080×2400 PNG.
- `android-build.log`, `android-install.log`, `avd-metadata.txt`: 빌드, 설치, 실행 환경.
- `capture-scenes.py`: scene별 재현 및 검증 harness.
- `harness-diagnostics/`: trace parser 수정 전 두 번의 capture 시도 원본. 두 시도 모두 native matcher와 draw는 통과했으며, parser가 part 2–N의 JSON continuation을 놓쳐 trace를 조립하지 못했다. 수정된 parser로 같은 APK/index에서 12개 scene을 모두 다시 캡처했다.
- `emulator-start.log`, `emulator-start-retry.log`, `emulator-shutdown.log`, `shutdown-verification.json`: AVD 시작 및 종료 기록.
