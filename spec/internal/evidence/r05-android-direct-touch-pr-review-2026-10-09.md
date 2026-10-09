# R05 Android 직접 입력 PR 통합 검토

## 검토 경계

- 대상: PR [#86](https://github.com/ohah/spinon/pull/86)의 실제 본문·첨부·라벨·diff와 R05.3 계획, 수집기, 분석기, Android 실기기 block 01 근거.
- 기준: PR의 제품 주장은 한 block의 exact transaction-fence 후보 수집에 한정한다. 제품 입력 지연, p95, VSync, scanout 또는 광자 시각을 확정하지 않는다.
- 검토 중 direct-touch 요약의 남은 block 수가 계획과 맞지 않는 것을 발견해 수정했다. 총 12개 중 block 01 하나가 끝났으므로 남은 수는 11개다.

## 서로 다른 PR 실패 경로 검토

| # | 실패 관점 | 확인·수정 |
|---:|---|---|
| 1 | 본문·계획·근거의 남은 block 수가 달라 진행 상태를 과장 | 12개 중 하나를 완료해 11개가 남는다. direct-touch 요약의 잘못된 `10개`를 `11개`로 수정했다. |
| 2 | 12×30을 유효 후보 360개로 잘못 표시 | 360은 raw 시도 목표이고, 분석 gate는 전체 exact candidate 300개다. 계획과 PR 본문을 구분했다. |
| 3 | drain 입력을 점수 집합에 옮겨 후보 수를 부풀림 | raw 32개 가운데 사전 규칙의 첫 30개만 점수화하고 뒤 2개는 제외했으며, 분석 결과도 별도 집계한다. |
| 4 | 실패 raw 접촉을 성공 분모에서 제거 | 첫 30개 raw 시도 중 2개 미연결을 실패 시도로 유지해 28/30으로 보고한다. |
| 5 | timestamp 없는 `cancelled`·`outside_target`을 실패 접촉에 임의 배정 | 앱 제외 로그와 raw 미연결 접촉의 일대일 원인 연결을 하지 않는다고 근거와 PR 본문에 명시했다. |
| 6 | 단순 event 수 일치로 raw touch와 앱 입력을 결합 | tracking contact의 release 시각에서 `ACTION_UP`, submit, callback, fence까지 식별자·revision·generation을 검사한다. |
| 7 | stale surface callback을 current surface 완료로 간주 | callback과 fence wait에서 request의 surface generation 일치를 요구하고 테스트 fixture가 stale generation을 거부한다. |
| 8 | callback 도착만으로 fence 완료를 주장 | 동일 request의 signaled·usable·valid fence wait와 오류 부재를 요구한다. |
| 9 | 후보 interval envelope를 p95 또는 제품 지연으로 오표기 | 28.230–61.515 ms는 block 후보 구간으로만 기록하고 p95·제품 latency 주장을 금지했다. |
| 10 | Android transaction fence를 실제 화면 표시 시각으로 설명 | `target_vsync_id=-1`, frame ID·scanout 미측정 및 광자 시각 한계를 STATUS·계획·PR에 남겼다. |
| 11 | 다른 입력 장치나 synthetic ADB touch를 direct finger input에 혼합 | 지정된 단일 `sec_touchscreen` direct device를 검증하며 synthetic control process는 별도 실행·분리한다. |
| 12 | 잘못된 앱 프로세스·APK를 기존 block과 합산 | fresh app PID, package, foreground, APK hash와 collector process provenance를 확인하고 analyzer가 불일치를 거부한다. |
| 13 | 서로 다른 clock domain이나 시간 오차를 정확한 단일 시각으로 오해 | clock anchor bracket의 구간으로 candidate를 계산하고 bracket이 불일치하면 join과 latency candidate를 분리한다. |
| 14 | raw timestamp 회귀·동률을 정렬로 숨겨 first-30을 바꿈 | 원본 event 순서의 회귀를 거부하며 동일 timestamp는 원본 행 순서를 보존한다. |
| 15 | 동시 포인터·잘못된 release를 정상 탭으로 축약 | slot/tracking ID 수명, overlap, release frame을 보존하고 불완전·중첩 입력을 성공 join으로 승격하지 않는다. |
| 16 | capture 중 기기 연결·collector가 끊기거나 잔존해도 성공 처리 | 수집 종료 뒤 host client와 기기 getevent 상태를 각각 검사하며 잔존·불완전 capture를 유효 표본에서 제외한다. |
| 17 | SIGINT에 응답하지 않는 client를 무한 대기하거나 무관 PID에 signal | PID·명령·parent·시작 시각 identity를 확인한 자체 client만 bounded SIGINT→SIGTERM→SIGKILL로 종료하고 단계별 제한을 적용한다. |
| 18 | 공개 증거 checksum이 빠졌거나 오래된 상태 | 직접 입력 evidence의 nested/root manifest와 shutdown 최소 evidence manifest를 재생성하고 저장소 사본에서 검증했다. |
| 19 | public evidence에 serial·원시 좌표·불필요한 전체 시스템 입력 덤프가 노출 | 추가된 evidence에서 serial·좌표·credential 모양을 검색해 비워 두었고, 최근 입력 상태가 들어갈 수 있는 전체 system dump는 저장소에 포함하지 않았다. |
| 20 | PR 검토자가 실기기 화면·작업 유형·테스트 상태를 확인할 수 없음 | `benchmark`와 `area: android` 라벨, 한국어 양식, before/after 첨부를 GitHub PR에서 확인했다. GitHub checks는 보고되지 않아 로컬 전체 테스트와 별도로 표시한다. |

## 현재 판정

- 수정 후 계획·상태 대장·근거의 표본 수와 남은 block 수를 일치시켰다.
- PR 첨부는 GitHub asset URL로 본문에 반영됐고 Tailscale 링크는 PR 본문에 없다.
- 로컬 `mise exec -- bun run test`는 통과했다. GitHub는 이 branch에 status check를 보고하지 않았다.
- phase-2 collection은 28/300이며 11개 block이 남았다. 이 PR만으로 R05.3 완료나 제품 성능을 판정하지 않는다.
