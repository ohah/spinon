# R05.3 Android callback 실패 fixture API 호환성 행렬

**상태:** API 35·37.0·37.1 emulator fixture 실행 완료 · emulator 전용 · R05.3 미완료

## 목적

앞선 callback failure fixture는 Android API 37.2 / 16KB ARM64 AVD에서 11개 scenario를 통과했다. 같은 failure 경로가 `SurfaceControl.TransactionStats` 및 `SyncFence` 최소 API인 API 35와 API 37.0·37.1에서도 성립하는지 확인한다. 이 작업은 API 35 이상 transaction completion failure fixture에 한정하며 API 29–34에서는 해당 `TransactionStats` 경로가 없으므로 정상 GLES fallback을 별도 판정한다.

| 대상 AVD | `SDK_INT` / full version | page size | 역할 |
|---|---:|---:|---|
| `spinon_api35_compat` | 35 / 가능한 경우 기록 | 4,096 bytes | TransactionStats 최초 API gate |
| `spinon_api37_0_compat` | 37 / 37.0 | 4,096 bytes | 37 minor version gate |
| `spinon_api37_1_compat16k` | 37 / 37.1 | 16,384 bytes | 16KB 이미지와 transaction callback 호환 |

계획 구현 시 Android 공식 API reference에서 `TransactionStats.getPresentFence()`가 API 35에 추가됐으며 `SyncFence`의 invalid/pending 값을 구분한다는 내용을 기준으로 둔다. ([TransactionStats](https://developer.android.com/reference/android/view/SurfaceControl.TransactionStats), [SurfaceControl.Transaction](https://developer.android.com/reference/android/view/SurfaceControl.Transaction), [SyncFence](https://developer.android.com/reference/android/hardware/SyncFence))

## 비교 모델과 합격 기준

| 경로 | oracle | 예상 결과 |
|---|---|---|
| 실제 transaction timeout | 실제 `SurfaceControl.Transaction`에 listener를 붙이고 적용하지 않은 뒤 main Handler의 2초 timeout을 기다림 | API 35·37.0·37.1 각각 timeout 1, callback 0, pending/timer 제거, transaction close |
| 실제 cancel | 미적용 transaction에 listener와 timeout을 예약한 뒤 owner 일치 cancel | cancelled terminal state, timeout Runnable 제거, pending 제거 및 transaction close |
| 합성 상태 전이 | 기존 API 37.2 fixture와 동일한 late/cancel/duplicate/stale/renderer mismatch transition | 예상 terminal state, `fence_signal_usable=false`; 합성 입력을 실제 표시 callback으로 세지 않음 |
| terminal 경합 | callback과 timeout을 barrier로 동시 시작 100회 | 회차별 terminal winner 하나, pending map 잔류 0 |
| callback executor overflow | 실제 production executor worker 1개 차단, queue 64개 수락, 초과 callback 1개 제출 | caller-runs 1회 표시, fence unusable, bounded drain 후 queue/active 0 |
| 종료 후 유휴 | actual request 정리 뒤 60초, 초당 1회 관찰 | pending/queue/active 0, 새 callback/timeout 없음 |
| 안전 runner negative control | physical serial, 허용되지 않은 API/page tuple, 결과 폴더 재사용을 각각 입력 | build/install/launch 전에 거부, 기존 자료와 AVD 상태 보존 |

각 AVD는 별도 새 앱 프로세스와 별도 출력 폴더로 실행한다. 실행 전 `SDK_INT`, full version(플랫폼이 제공하면), page size, ABI, model을 기록한다. runner는 `emulator-숫자` serial만 허용한다. API/page size tuple은 matrix의 API 35·4KB, 37.0·4KB, 37.1·16KB와 기존 기본 회귀 대상 37.2·16KB만 허용한다. 알려지지 않은 조합, 연결 손실, 기대하지 않은 fixture summary는 실패다.

세 API 모두 11 scenario를 통과해야 한다. 특정 API에서 실제 listener를 등록할 수 없거나 상태 결과가 다르면 API 지원으로 우회 처리하지 않고 원본 오류와 해당 API 결과를 남긴다. 결과로 latency·성능 순위·하드웨어 GPU·광학 scanout을 주장하지 않는다. API 37.2의 기존 자료는 별도 보존하고 이번 세 API 결과와 합산하지 않는다.

## 실행 및 산출물

1. `spec/STATUS.md`와 이 계획을 실행 전에 연결한다. fixture 코드는 앞선 PR #73 병합본을 기준으로 하며 API 결과 파일은 새로운 날짜 폴더에 둔다.
2. 재현 runner를 세 API/page tuple에 한해 일반화한다. default API37.2 실행은 기존 동작과 출력 보호를 유지한다.
3. 세 AVD를 cold process로 각각 실행하고 각자의 build·install·launch·Logcat·screenshot·PID·environment·APK hash를 보존한다.
4. run별 11-scenario summary, 세 API의 결과 분리, SDK metadata, API35/37.0/37.1 비교 한계와 runner negative controls를 evidence 문서에 기록한다.
5. `spec/STATUS.md`, [R05.3 상위 계획](r05-input-to-presentation.md), 내부 근거 인덱스를 갱신한다. `R05.3` 완료 체크를 바꾸지 않는다.
6. allthatnba `spinon/` Tailscale preview를 proposal/in-progress 상태로 갱신하고 URL을 확인한다. GitHub Pages는 배포하지 않는다.

## 계획 적대 검토 · 독립 실패 관점 20개

| # | 실패 관점 | 차단·판정 기준 |
|---:|---|---|
| 1 | API35 이미지를 API36/37로 오인 | AVD 이름만 믿지 않고 device `SDK_INT`·ABI·model을 run별 보존한다. |
| 2 | API37.0과 API37.1을 하나의 runtime으로 합산 | full SDK version을 기록하고 각 출력 폴더·로그·summary를 분리한다. |
| 3 | 16KB AVD 결과를 4KB AVD 결과로 잘못 라벨링 | `getconf PAGE_SIZE`를 직접 읽고 API37.1 run에서 16384가 아니면 중단한다. |
| 4 | API35/37.0 callback gate가 API37.2-only JankData 경로를 요구 | 이 matrix는 TransactionStats failure fixture만 실행하며 JankData support를 pass condition으로 삼지 않는다. |
| 5 | `getPresentFence` 또는 SyncFence API level을 혼동해 API34 이하로 확대 | 공식 API 35 minimum을 근거로 35+만 실제 failure fixture에 넣고 29–34는 별도 fallback으로 남긴다. |
| 6 | `sdk_int_full`이 없는 API35에서 property 부재를 임의 값으로 치환 | API35에서는 base level 35와 page size로 판정하고 full property 부재 여부를 그대로 기록한다. |
| 7 | API matrix를 위해 실행기 serial 검사를 약화해 실기기를 포함 | serial allowlist는 `emulator-숫자`로 유지하고 invalid serial negative control을 실행한다. |
| 8 | API35의 API class eager load가 fixture 시작 전에 앱을 crash | 동일 APK를 fresh process에서 실제 실행하며 시작·summary timeout을 모두 판정한다. |
| 9 | 최신 debug APK와 오래된 installed APK를 섞음 | 매 run APK digest·install log·app PID를 보존하고 run별 동일 digest를 확인한다. |
| 10 | 서로 다른 source revision 결과를 한 matrix로 합산 | 모든 run의 source head/dirty count를 같은 commit 기준으로 기록한다. |
| 11 | 이전 API run의 pending request나 executor 작업이 다음 API로 새어듦 | API별 별도 OS app process 및 AVD를 사용하고 run 시작 시 fixture idle을 요구한다. |
| 12 | actual un-applied transaction timeout을 synthetic function call로 대체 | 실제 listener 등록·미적용 transaction·main Handler timeout을 로그에서 따로 확인한다. |
| 13 | timeout 전에 OS callback이 도착했는데 timeout을 정상으로 계수 | callback count 0, timeout count 1, map/timer 제거를 각각 확인하며 상이하면 실패다. |
| 14 | synthetic late/duplicate/stale callback을 OS callback 증거로 설명 | 합성 callback은 null stats·unusable fence이며 actual request id/source와 분리한다. |
| 15 | 경합 100회에서 terminal state 두 개를 허용하거나 표본 탈락 | 각 회차 barrier, winner 합계 100, pending 0을 요구한다. |
| 16 | saturation 시험 task 수를 callback record 수로 오해 | queued tasks, overflow callback, callback count를 서로 구분해 로그와 보고서에 적는다. |
| 17 | assertion 실패 뒤 executor worker/blocker를 남겨 다음 run 오염 | `finally` worker release, 제한 시간 drain, process 종료 상태를 합격 조건으로 둔다. |
| 18 | 60초 idle snapshot을 앱 장기 안정성·전력 결과로 확대 | 상태 전이 부재만 결론 내며 CPU·전력·제품 지속 실행은 미측정이라고 쓴다. |
| 19 | 기존 출력 폴더를 재사용해 성공 산출물을 덮어씀 | result path 존재 시 runner가 build 전에 거부하는 negative control을 둔다. |
| 20 | 3개 API 결과를 평균내 API 지원·R05 완료로 선언 | API별 결과를 별도 표로 유지하고 전체 R05.3은 미완료로 둔다. |

계획 검토에서 API35 full-version property 부재, 37.0/37.1 혼합, transaction fence와 JankData 구분, 실제 listener 경로와 synthetic transition 혼합을 별도 공격으로 추가했다. API35의 기본 SDK·page size와 전체 API37 full version은 각각 분리 기록하며, 기존 API37.2 결과를 이번 matrix의 분모나 성공 판정에 재사용하지 않는다. 이 계획 검토는 코드 구현 후 별도의 새 20개 관점 검토로 대체하지 않는다.

## 실행 결과

API 35·4KB, API 37.0·4KB, API 37.1·16KB ARM64 emulator에서 같은 APK와 fixture 소스 hash로 각각 11개 scenario, timeout/callback 경합 100회, 종료 후 idle 60초를 통과했다. 원본 log·screenshot·APK hash·실행 환경과 구현 후 별도 검토는 [실행 근거](../spec/internal/evidence/r05-android-callback-api-matrix-2026-10-08/README.md)에 있다. API 37.0은 AVD 생성·렌더링 조건이 다른 세 시도 중 하나만 부팅에 성공했으며, 원인은 분리 확정하지 않았다. 이 검증은 에뮬레이터 내부 callback failure fixture이며 renderer의 실제 적용 transaction, iOS device callback, API 29–34 fallback, 제품 표시 지연은 검증하지 않았다. R05.3과 R05는 미완료다.
