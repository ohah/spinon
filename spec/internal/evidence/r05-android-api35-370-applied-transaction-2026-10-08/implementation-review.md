# API 35·37.0 applied-transaction 구현·실행 후 공격 검토

**대상:** 2026-10-08 API 35·37.0 fresh AVD debug fixture와 저장 증거 · **결과:** runtime fixture 두 행 PASS, 캡처 재개 차이는 공개 기록 · **R05.3:** 미완료

| # | 독립 실패 관점 | 확인 근거와 판정 |
|---:|---|---|
| 1 | API 35 실행을 API 37 또는 다른 image로 잘못 분류 | guest `api=35`, API 35 image path/revision과 fingerprint를 대조했다. 통과. |
| 2 | API 35의 `sdk_full` 누락으로 잘못된 정확 버전 주장을 함 | 속성이 비어 있음을 그대로 남기고 fingerprint와 image package를 추가 식별 근거로 썼다. `sdk_full`을 확인했다고 주장하지 않았다. |
| 3 | API 37.0을 37.2로 오인 | guest `api_full=37.0`과 API 37.0 package path/revision이 일치한다. 통과. |
| 4 | host page size를 guest page size로 대체 | 두 run에서 guest `getconf PAGE_SIZE=4096`을 기록했다. 통과. |
| 5 | ARM64 host만으로 guest ABI를 추정 | 두 run 모두 guest ABI `arm64-v8a`와 image ABI를 확인했다. 통과. |
| 6 | stale/삭제된 기존 AVD를 fresh boot로 셈 | 신규 AVD 명칭, 실제 `.avd` data directory, QEMU `-avd` 인자와 guest AVD name을 대조했다. 통과. |
| 7 | 두 API run 사이 다른 AVD를 실행 | 각 실행에서 `emulator-5580`와 AVD name/PID를 확인했고 API별 serial을 순차 재사용했다. 통과. |
| 8 | 다른 프로젝트 emulator를 종료·재사용 | `zl_poc` / `emulator-5554` PID 23588은 실행 전후 유지했다. Spinon serial만 종료했다. 통과. |
| 9 | 서로 다른 소스 revision으로 API 결과를 비교 | 두 environment의 source head가 모두 `300443b868d06a4fdc59a0fb72770df4edb57ca8`이고 tracked changes는 0이다. digest 파일도 동일하다. 통과. |
| 10 | V8 checkout이 API별로 다르거나 dirty | 두 run에서 pinned V8 revision `7b50b62cb18f28617959e8452e2cd18195b38bcf`, clean 상태를 확인했다. 통과. |
| 11 | 재빌드 APK가 달라 binary comparison을 과장 | 양쪽 APK SHA-256이 `d9a26b62d326d4f9310394b9eb06ffcecbb7c31e27758548c27eb1b454e40bb1`로 같다. 통과. |
| 12 | 시스템 JankData 지원을 applied transaction callback과 혼동 | API 35·37.0 Logcat은 JankData unavailable을 명시한다. 통과 판정은 실제 `TransactionStats` callback에만 한정했다. |
| 13 | baseline에서 실제 OS callback 대신 성공 marker만 검사 | actual baseline request에 non-null `TransactionStats`, callback count와 current surface를 확인하고 세 draw revision을 기록했다. 통과. |
| 14 | surface detach 전에 callback을 이미 처리해 stale 경로가 아님 | fixture의 `callback_queued` marker는 callback count 0, request pending, queue/active 1인 상태에서 detach가 이뤄졌음을 보여준다. 통과. |
| 15 | 이전 generation late callback이 새 surface callback으로 수용 | 이전 요청의 취소 시점·late 분류·`current_surface=false`·unusable fence를 실제 lifecycle 로그에서 대조했다. 통과. |
| 16 | 새 surface generation이 회복되지 않고 다음 작업을 영구 대기 | generation 증가 뒤 새 실제 callback 성공 및 queue drain marker를 확인했다. 통과. |
| 17 | 경합 100회 집계가 누락·중복 또는 결과 사후 조작 | API 35는 callback/timeout 36/64, API 37.0은 81/19이며 각 합은 100이다. 동일 비율을 기대하지 않고 원 로그를 보존했다. 통과. |
| 18 | idle 표본과 terminal cleanup을 실행 요약과 다르게 보고 | 양쪽 로그에서 idle 60 samples, pending/queue/active 0과 12 checks·0 failures를 확인했다. 통과. |
| 19 | API 37.0 최초 검정 캡처를 숨기거나 emulator 전체 고장으로 과장 | 초기/재확인 검정 화면, 정상 Settings 대조 화면, 앱 복귀 후 PASS 화면을 모두 보존했다. 원인은 surface 준비·resume·capture timing 사이에서 미확정으로 기록했다. |
| 20 | 검증 뒤 emulator 정리가 다른 AVD나 사용자 데이터를 삭제 | 두 Spinon process만 `adb emu kill`로 종료했고 두 fresh AVD data는 보존했다. data directory가 없는 오래된 등록은 자동 삭제하지 않았고 `zl_poc`는 그대로 두었다. 통과. |

이 검토는 계획의 20개 사전 관점과 별개로 실행 결과에 적용했다. 이 범위에서 발견한 미해결점은 API 37.0 screenshot 첫 캡처의 검정 화면 원인이다. 앱 복귀 화면의 PASS는 fixture 통과를 보여주지만 최초 캡처 현상을 설명하지 않으며, 제품 표시 안정성의 증거로 확대하지 않는다.
