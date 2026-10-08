# API 37.0 캡처 재현 · 실행 후 독립 실패 관점 검토

이 검토는 실행 전 계획 검토와 분리해 두 cold-boot 실행·캡처·종료 증거를 확인한다. 각 항목은 실제 저장된 결과로 판정했다. 실행 수를 늘리거나 원인을 추정하는 대신 증거가 허용하는 범위를 제한한다.

| # | 실패 관점 | 확인한 근거와 판정 |
|---:|---|---|
| 1 | API 37.0을 Android release 숫자만으로 잘못 식별 | 두 실행의 `sdk_full=37.0`, SDK API 37을 각각 기록했다. 통과. |
| 2 | snapshot resume을 cold boot로 오인 | 두 실행 모두 snapshot load/save를 끄고 각기 다른 QEMU PID로 부팅했다. 통과. |
| 3 | 4KB AVD를 다른 guest 조건으로 실행 | 양 실행에서 guest page size 4096, ABI `arm64-v8a`, 같은 AVD name을 기록했다. 통과. |
| 4 | AVD 재생성·wipe가 이전 검은 캡처 변수를 바꿈 | `.avd`를 재사용하고 `-wipe-data`를 쓰지 않았다. 시험 전·종료 후 데이터 디렉터리가 존재했다. 통과. |
| 5 | APK 재빌드 차이가 캡처 차이를 설명할 수 있음 | 두 run의 APK digest가 같고 입력 source checksum 목록도 같다. 통과. |
| 6 | V8 checkout revision/dirty 상태가 run 간 달라짐 | 고정 revision `7b50b62c…` 및 clean checkout을 각각 기록했다. 통과. |
| 7 | 이전 프로세스 Logcat을 새 실행의 PASS로 읽음 | run별 앱 PID·별도 logcat 파일을 확인했다. 각 summary는 현재 run PID에서 기록됐다. 통과. |
| 8 | 한 run이 fixture gate를 실패했는데 시각 결과에 포함 | 두 run 모두 12 checks, failures 0으로 종료했다. 통과. |
| 9 | 경합 100회 결과를 성공률·성능 차이로 해석 | winner 수는 run 01 91/9, run 02 66/34로 달랐다. 기준은 각 종결 상태와 pending 0이며 winner 비율을 비교 지표로 쓰지 않았다. 통과. |
| 10 | idle 동안 callback/queue 작업이 남아 있음 | 각 실행에서 idle 60 samples와 최종 pending·queue·active 0을 확인했다. 통과. |
| 11 | 실제 applied-transaction 검증을 synthetic-only 경로로 오인 | baseline·surface recreation stale·generation recovery 행이 `actual_surface_callback_*`로 기록되고 `stats_available=true`였다. 통과. |
| 12 | surface 파괴 뒤 늦은 callback을 새 surface 성공으로 셈 | 두 run의 `after_recreation`은 old generation 1→new generation 2, `late_after_cancel`, current surface false로 분류됐다. 통과. |
| 13 | 새 generation 회복 경로에서 callback이 누락 | 두 run에서 generation 2의 recovery callback, `fence_signal_usable=true`, executor idle을 확인했다. 통과. |
| 14 | 결과 PNG가 빈 파일·잘못된 이미지·hierarchy 전용 PASS | PNG가 실제로 디코드되며 육안 검사에서 runner 화면 모두 PASS 문구를 표시했다. hierarchy도 별도 대조했다. 통과. |
| 15 | 시간 시계열 중 화면이 잠시 PASS였다가 사라짐 | run 01 0·1·5·15초 화면이 byte-identical이고, run 02 네 화면 모두 육안으로 PASS였다. 통과. |
| 16 | run 02에서 checksum 차이를 앱 화면 변동으로 잘못 판단 | 15초 및 복귀 캡처의 raw hash 차이는 status bar 시각 변화와 일치하며 본문 PASS는 동일했다. 통과; raw PNG hash 동일성을 화면 안정 조건으로 과장하지 않는다. |
| 17 | Settings 양성 대조가 실행되지 않았는데 성공으로 기록 | 두 Settings intent 뒤 실제 Settings UI bitmap을 확인했다. 통과. |
| 18 | Back 이후 다른 Activity/process가 PASS 화면처럼 보이거나 즉시 캡처 누락이 가려짐 | 두 run에서 resumed/current focus는 Spinon MainActivity이며 PID가 해당 run의 runner PID와 같았다. 다만 계획한 Back 직후 bitmap은 수집하지 않고 +1초 bitmap만 남겼으므로 Activity 복귀는 확인했지만 즉시 픽셀 전환 시점은 미검증이다. 부분 통과; 계획 편차를 보고서에 유지한다. |
| 19 | screenshot timing을 Android presentation timestamp로 표현 | manifest는 host monotonic의 adb 요청/수신 interval로 한정하고 frame-present/scanout 주장에 사용하지 않았다. 통과. |
| 20 | 정리 과정에서 AVD 데이터 또는 다른 프로젝트 emulator를 손상 | Spinon API 37.0 emulator-5580만 정상 종료했다. 데이터 1.7GB가 남았고 5580은 사라졌으며 `zl_poc` emulator-5554는 살아 있다. 통과. |

## 결론과 남은 불확실성

두 동일 조건 cold boot에서 이전 black runner capture가 재현되지 않았다. 현 증거는 일시적 또는 캡처/상태 타이밍 관련 가능성을 배제하지 못하며 그 원인을 확정하지 않는다. AVD 재설치 없이 PASS가 나왔으므로 재설치 효과도 주장하지 않는다. API 지원, 실제 기기 표시, 물리 지연 또는 R05.3 전체 완료 근거가 아니다.
