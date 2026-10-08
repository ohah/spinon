# Android 실기기 callback fixture · 실행 후 독립 실패 관점 검토

이 검토는 실행 전 계획의 20개와 별도로, API 36.1 실기기 두 실행과 수집기 실패·복구 근거를 검사한다.

| # | 독립 실패 관점 | 실제 근거와 판정 |
|---:|---|---|
| 1 | 에뮬레이터 결과를 실기기 결과로 표기 | USB transport `SM-S731N`에서 실행했고 기록에는 개인 ADB serial을 남기지 않았다. 통과. |
| 2 | SDK 36.1을 SDK 36.0/37.0과 혼합 | `SDK_INT=36`, `SDK_INT_FULL=36.1`, Android 16 fingerprint를 사전·사후 확인했다. 통과. |
| 3 | 16KB image/host page size를 기기 조건으로 대입 | 실기기 `getconf PAGE_SIZE=4096`, ABI `arm64-v8a`를 직접 확인했다. 통과. |
| 4 | pinned V8 checkout이 다른 빌드로 섞임 | 첫 기본 경로 실패 로그와 pinned clean checkout 지정 뒤 성공한 빌드 로그를 분리 보존했다. 통과. |
| 5 | 서로 다른 APK 결과를 같은 반복으로 합침 | run별 APK SHA-256이 동일하고 source 파일 checksum을 기록했다. 통과. |
| 6 | APK 재설치 과정이 앱 data를 초기화 | `install -r`만 사용했고 `pm clear`/uninstall은 하지 않았다. 통과. |
| 7 | 과거 logcat marker를 새 앱 실행에 귀속 | 전역 buffer는 지우지 않고 새 app PID 6473/8010으로 각 로그를 필터링했다. 통과. |
| 8 | 임시 driver의 첫 실행 실패를 제품 실패 또는 성공으로 뭉개기 | 첫 driver는 앱 시작 직후 빈 `pidof`로 조기 종료했다. 보존된 Android logcat을 사후 읽어 앱 fixture의 PASS를 확인했고, 수집기는 수정 후 재실행했다. 도구 실패와 앱 결과를 분리했다. |
| 9 | 조기 종료 원인을 확인하지 않고 임시 재시도 | 존재하지 않는 package의 `pidof`가 exit 1인 것을 확인하고 pipefail 하 시작 경합을 수정했다. 두 번째 실행은 전체 driver exit 0이다. 통과. |
| 10 | summary가 stale process 또는 부분 실행에서 옴 | run별 새 PID의 `START`, scenarios, 12 checks summary가 각각 존재한다. 통과. |
| 11 | 요약 PASS만 보고 필수 lifecycle marker 누락 | baseline, recreation stale, generation recovery, queue 3건, GLES revision draw, timeout, overflow, idle marker를 따로 확인했다. 통과. |
| 12 | race winner 비율을 일정한 결과/성능으로 주장 | 87/13과 93/7 차이를 raw count로 보존하고 합격 판정이나 성능 순위로 쓰지 않았다. 통과. |
| 13 | idle 종료 뒤 pending 작업이 남음 | 각 run의 60 samples에서 first_bad none, pending·queue·active 0이다. 통과. |
| 14 | actual callback 전달을 usable present fence로 확대 | baseline/recovery는 stats available이지만 fence pending·unusable이었다. 보고서에서 표시시각 미검증으로 제한했다. 통과. |
| 15 | recreation 뒤 stale callback을 현재 화면 표시로 합침 | generation 1→2, late_after_cancel, current surface false를 보존했다. 통과. |
| 16 | overflow 시 caller-runs 경로를 거부/손실로 오해 | 코드의 `InlineOnRejectHandler`가 caller thread에서 task를 실행하고 inline counter를 올린다. 두 기기 실행 marker와 queue drain을 확인했다. 통과. |
| 17 | 실제 손가락 입력으로 시험했다고 오인 | 입력 source는 `synthetic_fixture`다. touch 조작·터치 반응성 근거로 사용하지 않았다. 통과. |
| 18 | black/다른 앱 화면을 PASS로 오인 | 두 기기 PNG를 직접 확인했고 둘 다 PASS bitmap이다. run 02 hierarchy와 Spinon focus도 일치한다. 통과. |
| 19 | logcat 또는 Chrome 사용자 상태를 훼손 | 전역 logcat 삭제 없이 새 PID만 필터링했고 Chrome을 force-stop하지 않았다. 테스트 후 Chrome foreground 복귀를 확인했다. 통과. |
| 20 | 시험 후 emulator/device 설정·프로세스를 방치하거나 성능 주장 | Spinon API 37.0 AVD를 종료하고 data를 유지했다. `zl_poc`는 보존, Spinon 앱은 force-stop, 기기 설정은 사전값 유지다. 실기기 지연/scanout 주장은 없다. 통과. |

## 결론

Samsung SM-S731N / Android 16 SDK 36.1에서 debug callback failure fixture 두 번이 통과했다. Android transaction callback의 usable fence는 baseline/recovery에서 얻지 못했다. 한 기기의 synthetic fixture 결과이므로 사용자 입력·표시 지연·R05.3 완성으로 일반화하지 않는다.
