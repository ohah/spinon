# R05 직접 touch join 분석기 구현 검토

## 검토 범위

- 대상: `tools/benchmark/analyze-android-physical-touch-join.mjs`, 자동 검사, 2026-10-09 Android 16 실기기 phase-2 block 01의 redacted 사본.
- 비교 기준: 수집 계획의 첫 30 raw contact 고정 규칙, `sec_touchscreen` direct capability, 앱 `ACTION_UP`·WGPU submit·현재 surface generation callback·usable fence 연결, 기존 R05 로그 필드.
- 결론 범위: 현재 한 block의 수집·연결 무결성이다. 제품 입력 지연, VSync·scanout 시각, p95나 성능 비교를 검증하지 않는다.

## 서로 다른 실패 관점

| # | 공격 관점 | 확인·수정한 내용 |
|---:|---|---|
| 1 | 누락된 수집 단계나 거꾸로 된 UTC 시각을 정상 capture로 인정 | 시작·preflight·GO·첫 접촉·limit 도달·종료 요청·종료 순서를 검사하고 역전 및 누락을 거부한다. deadline 경계 자동 검사를 추가했다. |
| 2 | manifest의 timeout을 0, NaN, 문자열 또는 비현실적 값으로 바꿔 제한 없는 수집을 정상화 | first-contact/after-first-contact/drain 정책의 정수 형식과 범위를 검사한다. |
| 3 | 첫 접촉 또는 목표 접촉이 고정 시간 제한을 넘었는데 manifest의 성공 flag만 이용 | GO→첫 접촉, 첫 접촉→목표, drain→종료 요청의 간격을 정책과 대조한다. 제한 초과 fixture는 거부된다. |
| 4 | APK hash는 맞지만 앱 PID가 0·음수·비정수라 다른 프로세스 로그를 수집 | 양의 안전 정수 PID, 앱 ID, 설치/예상 APK hash, foreground activity를 함께 확인한다. |
| 5 | 동일 event path를 가진 중복 장치 설명 중 편한 한 항목만 선택 | `input-devices.txt`에서 지정 경로가 정확히 한 번 나타나고 이름·`INPUT_PROP_DIRECT`·`ABS_MT_TRACKING_ID`가 전부 맞아야 한다. |
| 6 | `getevent` 또는 `logcat` 이름을 명령 문자열 일부에 숨겨 잘못된 collector provenance 통과 | command는 argument 배열이어야 하고 각 collector의 명령·event path 또는 앱 PID를 확인한다. PID·parent PID·시작 시각도 검증한다. |
| 7 | 서로 다른 host collector가 같은 PID를 사용하거나 종료 뒤 device `getevent`가 잔존 | collector PID는 서로 달라야 하고 양쪽 bounded-stop 성공과 device 잔존 0을 요구한다. 기존 supervisor 검증과 연결한다. |
| 8 | 앱 로그에 다른 프로세스의 R05 marker를 섞어 sequence를 연결 | marker가 있는 각 행의 PID가 manifest app PID와 같아야 한다. 다른 PID fixture는 거부된다. |
| 9 | raw event timestamp가 역행했는데 정렬로 감춰 접촉 순서를 바꿈 | 입력 원본 순서를 순회하며 timestamp 회귀를 protocol error로 거부한다. |
| 10 | 서로 다른 slot의 동시 손가락을 하나의 탭으로 축약 | slot별 tracking ID 수명을 보존하고 overlap을 실패 분모에 남긴다. 멀티포인터 fixture가 두 접촉을 합치지 않는지 확인한다. |
| 11 | active contact 없는 release 또는 한 slot에서 새 tracking ID로 이전 접촉을 덮어씀 | 비정상 release·release 전 ID 교체를 protocol error로 거부한다. |
| 12 | `SYN_REPORT` 없이 끝난 release를 완전한 raw contact로 집계 | release 이후 frame 경계가 있어야 완결 contact이며 pending frame과 active contact가 남으면 성공 gate가 닫힌다. |
| 13 | 동시 timestamp의 원본 순서를 버리고 정렬 안정성에 우연히 의존 | down timestamp 동률은 원본 getevent 행 순서로 first-N을 고정한다. fixture에서 다른 slot의 첫 행을 선택한다. |
| 14 | 초과 접촉이나 drain 입력을 유효한 표본 수에 합산 | 고정된 첫 N개만 scoring set으로 만들고 뒤 contact는 `extraContactsOutsideScoringSet`으로 분리한다. |
| 15 | manifest raw count를 과장하거나 축소해 로그 절단을 감춤 | parser 완료 수·manifest count·overlap·protocol error 수를 대조하며 원시 로그 수 불일치를 거부한다. |
| 16 | 한 raw release에 여러 `ACTION_UP`이 같은 시각으로 기록돼 임의 sequence를 고름 | timestamp 일치가 유일하지 않으면 `ambiguous_action_up`으로 남기고 join하지 않는다. |
| 17 | 중복 `input_seq`, submit 누락·중복 또는 잘못된 revision/generation을 재사용 | 각 scoring input sequence의 유일성, 단일 submit, event time·revision·generation·draw acceptance를 검사한다. |
| 18 | `request_id`를 stage 사이 또는 다른 입력에 재사용해 성공 callback을 오귀속 | listener/apply/callback/wait의 request identity와 stage별 고유성, listener 등록, callback surface/generation을 검사한다. 중복 request 및 VSync sentinel 불일치는 fixture에서 거부된다. |
| 19 | callback만 도착했지만 fence가 timeout·pending·무효이거나 종료 뒤 대기 작업이 남음 | fence wait의 signal·valid-before/after·usable·무오류 상태와 pending/active/queue-depth 0을 모두 요구한다. |
| 20 | clock bracket 폭을 무시하거나 다른 offset 값으로 latency 후보를 축소 | 앱 input의 monotonic before/after bracket과 callback offset을 exact 비교하고 signal/latch/wait 순서를 검사한다. bracket 불일치는 touch/fence join은 보존하되 후보 시각은 withheld 처리한다. |

## 검토에서 수정한 보고 오류와 공개 자료

- 실기기 결과는 첫 30 raw contact 중 28건 exact join이다. 나머지 raw 두 건은 `ACTION_UP`과 연결되지 않았다. 앱의 `cancelled`·`outside_target` 두 로그는 event timestamp가 없어 그 raw 두 건에 일대일 대응한다고 말할 수 없다. 상태 대장·계획·evidence 문구를 이 사실에 맞췄다.
- drain의 raw 두 접촉과 timestamp가 맞는 accepted `ACTION_UP` 두 건은 scoring 집합 밖임을 별도 집계한다.
- 기존 exploratory raw log에서 X/Y 좌표값을 제거했다. 신규 block 사본에도 device serial·raw 좌표는 없다. 분석 결과에는 좌표를 출력하지 않는다.
- 단일 block의 28개 bracket 후보 envelope만 기록한다. p95, 제품 input-to-photon, frame scanout, 광자 시각은 계산하거나 주장하지 않는다.

## 실행 근거

- `mise exec -- bun run test:benchmark:r05-physical-touch`: 29개 테스트 통과.
- `mise exec -- bun run analyze:r05:physical-touch-android -- spec/internal/evidence/r05-android-direct-touch-exploration-2026-10-09/phase2-block01-direct-touch`: 저장소 redacted 원본에서 28/30 exact join, 28 callback, 28 usable fence, 28 clock-bracket 후보를 재현.
- 실기기 phase-2 block: Android 16·새 process·고정 APK hash·WGPU/Vulkan. 화면 전후 캡처와 전체 join summary는 [block evidence](r05-android-direct-touch-exploration-2026-10-09/phase2-block01-direct-touch/README.md)에 있다.

이 검토는 분석기를 공격하는 독립적인 failure path를 확인한 기록이다. 현재 결과를 R05.3 완료 또는 성능 승인으로 승격하지 않는다.
