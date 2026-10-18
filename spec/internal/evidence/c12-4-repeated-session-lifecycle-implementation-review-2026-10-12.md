# C12.4 반복 session 수명 검증 구현 검토

| 실패 관점 | 확인·수정 사항 |
| --- | --- |
| 실행 전 evidence 경로 부재 | 첫 시도에서 부모 evidence 폴더가 없어 수집기가 중단됐다. 부모를 recursive 생성한 뒤 run 폴더를 새로 만들어 재실행했다. |
| Android `proc` 줄바꿈 | 첫 memory summary parser가 기기 CRLF/탭 형식과 맞지 않았다. 원본을 보존하고 수평 공백·CRLF를 받도록 고쳐 검증했다. |
| cycle 번호 부분 일치 | `cycle=1`이 `cycle=10..16`과 겹칠 수 있어 marker를 payload 전체 equality로 바꿨다. |
| 잘못된 cycle 순서 | 종료 전 다음 cycle이 시작되는 로그를 놓치지 않도록 각 `complete`와 다음 `begin`의 순서를 검증한다. |
| 시작 로그 재사용 | 실행 전 logcat을 비우고 새 run ID를 생성해 과거 기록이 현재 결과로 섞이지 않게 했다. |
| emulator 오인 | `ro.kernel.qemu=0`, 모델 SM-S731N, API 36을 모두 검사해 대상 기기를 고정한다. |
| APK/source 불일치 | 빌드 성공을 확인하고 실행 APK의 SHA-256을 device metadata와 summary에 저장한다. |
| release에서 probe 실행 | 진입점에서 `BuildConfig.DEBUG`와 worker 진단 feature를 둘 다 요구한다. |
| 임의 반복 수 지정 | cycles 16, cycle당 session 8을 코드 상수로 제한하고 intent에서 수를 받지 않는다. |
| 동시 세션 과다 생성 | cycle 내 8개에 도달하면 더 생성하지 않고 최대 동시량을 64 MiB stack 요청으로 제한한다. |
| 부분 생성 실패 | 생성된 handle만 역순으로 해제하고 실패 marker를 남긴다. 전체 run parser는 이를 통과시키지 않는다. |
| owner TID 보고 누락 | cycle begin과 ready 사이의 native host creation report에서 8개 owner TID를 요구한다. |
| CSS worker 진단 오류 | JSON schema/status/thread ID, session index 0–7, unique TID를 검증한다. |
| owner/worker 혼동 | owner TID 8개와 CSS worker TID 8개를 별도 목록으로 기록하고 cycle 안에서 서로 disjoint인지 확인한다. |
| ready 전 thread 생존 누락 | 각 ready에서 앱 UID `/proc/<pid>/task`를 읽고 보고된 16 TID가 모두 있는지 확인한다. |
| free 반환의 과장 | `nativeFreeHost` 반환만 통과 조건으로 삼지 않고, freed 뒤 동일 16 TID가 task 목록에서 사라졌는지 확인한다. |
| cycle 간 TID 재사용 | OS TID의 전역 고유성을 요구하지 않는다. 각 cycle에서 생존과 종료를 따로 확인한다. |
| peak snapshot 시점 경합 | 외부 수집기가 peak PSS를 읽는 동안 세 고정 cycle의 live 시간을 늘리고, snapshot 뒤 freed marker 부재와 TID 생존을 재확인한다. |
| 실패를 success로 축약 | 누락·중복 cycle, host/worker 누락·중복 TID, 실패 marker, run ID·완료 집계 오류를 parser 단위시험으로 거부한다. |
| 결과 범위 과장 | process PSS/RSS/Vm 값은 보조 관찰로 기록한다. stack-only high-water, 전역 OOM, GPU, layout, WPT 또는 타 플랫폼 근거로 표시하지 않는다. |

## 실행 결과

Android 16/API 36 실기기 SM-S731N에서 APK SHA-256 `d8822ba473723e3f2d55ce35d650401c5fff2474fde0369a1c0d717197f8d781`로 16 cycle·128 session 생성을 완료했다. cycle마다 owner 8개와 CSS worker 8개가 ready 때 존재했고 해제 뒤 없어졌다. 기준 parser negative control과 전체 CSS reference suite도 통과했다. process PSS는 baseline 71,030 KiB, cycle 1/8/16 peak 91,069/94,560/94,600 KiB, 최종 92,026 KiB였다. process-wide 수치만으로 누수 여부를 판정하지 않는다.

[실기기 원본과 요약](c12-4-repeated-session-lifecycle-2026-10-12/c124-lifecycle-20261011T171814366Z-d07c0982/README.md) · [parser negative-control test](https://github.com/ohah/spinon/blob/feat/c12-4-session-lifecycle-20261019/tools/css-reference/c12-4-repeated-session-lifecycle-validation.test.mjs)
