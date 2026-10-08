# R06 · 실제 V8 큐 포화·복구 실행 근거

**일자:** 2026-10-08
**판정:** Android와 iOS Simulator 각각 5/5 통과
**계획:** [R06 큐 포화·복구 계획](../../../plan/r06-queue-saturation.md)
**사전 기준:** [구현 전 비교 모델](./r06-queue-saturation-precomparison-2026-10-08.md)
**내부 계약:** [JavaScript 작업 스케줄러](../0006-js-task-scheduler.md)

## 실행 환경과 방법

- Android Emulator: `sdk_gphone64_arm64`, Android 16 / API 36, `arm64-v8a`, serial `emulator-5554`.
- iOS Simulator: iPhone 17 Pro, iOS 26.2, device ID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`.
- 두 플랫폼 모두 저장소가 pin한 V8 revision `7b50b62cb18f28617959e8452e2cd18195b38bcf`, `v8_jitless=false`.
- 실행 명령: `SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 mise exec -- bun run verify:r06-priority:fairness:simulators`.
- 5회마다 Android ARM64와 iOS Simulator 바이너리를 빌드하고, 앱을 재시작한 뒤 실제 V8 진단을 수행했다.

## 결과

모든 Android·iOS 실행에서 결과가 다음 조건과 일치했다.

```text
queue_saturation_probe=PASS capacity=64 accepted=64 overflow_status=-5 overflow_returned_before_cancel=true overflow_queue_len=64 overflow_marker=not-run completed=64 recovered=PASS
```

실행 중인 동기 blocker를 취소하기 전에 대기 queue 64개를 수락했고, 별도 제출 thread의 65번째 요청은 `-5`로 반환됐다. 거부 직후 queue 크기는 64였다. blocker는 `-8`로 끝났고 수락된 64개는 FIFO 순서로 모두 완료됐다. 거부된 command의 표식은 남지 않았다. queue가 빈 뒤 새 eval이 성공했고 recovery 응답의 owner/callback thread도 일치했다. 검증은 각 회차마다 실제 marker·실행 순서·sequence·thread ID를 내부에서 판정했다.

| 회차 | Android | iOS Simulator |
| --- | --- | --- |
| 1 | 통과 | 통과 |
| 2 | 통과 | 통과 |
| 3 | 통과 | 통과 |
| 4 | 통과 | 통과 |
| 5 | 통과 | 통과 |

## 원본과 재현 자료

각 플랫폼의 5개 원본 로그, 환경 정보, 마지막 회차 화면 캡처와 SHA-256 manifest를 이 폴더에 둔다. 대표 화면은 마지막 회차 캡처다. 빌드·실행 명령은 위에 기록했으며 빌드 로그는 실행 당시 `build/spinon/priority-fairness-simulators/` 아래에 생성됐다.

- [Android 화면](./r06-queue-saturation-2026-10-08/android.png)
- [iOS 화면](./r06-queue-saturation-2026-10-08/ios.png)
- [실행 환경](./r06-queue-saturation-2026-10-08/environment.txt)
- [SHA-256 목록](./r06-queue-saturation-2026-10-08/SHA256SUMS)
- 원본 파일: `android-run-1.log` … `android-run-5.log`, `ios-run-1.log` … `ios-run-5.log`.

SHA-256 검증은 evidence 디렉터리에서 `shasum -a 256 -c SHA256SUMS`로 재현할 수 있다.

## 범위와 한계

이 결과는 한 Android emulator와 한 iPhone simulator에서 현재 내부 런타임 queue admission 경로를 반복 확인한 것이다. `64`와 `-5`는 공개 JS API나 제품 역압력 계약이 아니다. platform adapter의 선행 queue, 등급별 예약, 병합·폐기·재시도, 장기 공정성, 실제 기기, release 성능은 확인하지 않았다. screenshot은 진단 UI의 결과 표식이고 queue 동작 판정은 실제 V8 응답·marker·sequence에 기반한다.
