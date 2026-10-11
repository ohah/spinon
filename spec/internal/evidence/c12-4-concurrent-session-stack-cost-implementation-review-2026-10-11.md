# C12.4 동시 session stack 비용 구현 실패 관점 검토

**대상:** [계획](../../../plan/c12-4-concurrent-session-stack-cost.md), iOS/Android test-only session probe 및 [stage별 원본 결과](./c12-4-concurrent-session-stack-cost-2026-10-11.md)  
**판정:** 측정 경로의 실패를 고쳐 두 플랫폼에서 새 프로세스로 재실행했다. 관측은 0–8 session 시뮬레이터 범위에 한정한다.

| # | 독립 검토 관점 | 확인 결과와 조치 |
| --- | --- | --- |
| 1 | 기본 앱 시작 시 이미 session 1개가 들어옴 | probe 인수는 일반 C12.4 screen host를 만들지 않고 0개 baseline에서 시작한다. |
| 2 | C12.4와 다른 runtime profile 측정 | 각 추가 handle은 `new_c12_4_stacking_fixture`를 재사용한다. |
| 3 | GPU renderer/surface 사용량이 stack 비용에 섞임 | probe는 surface와 renderer를 생성하지 않는다. |
| 4 | requested stack과 physical memory를 같은 값으로 표시 | 8 MiB × session 수는 요청합이고 PSS/RSS/footprint는 process metric으로 따로 쓴다. |
| 5 | staged samples가 한 순간의 noise | 0·1·2·4·8단계를 8초 유지하고 세 번 샘플했다. |
| 6 | 앱 startup 지연을 runtime session 증가로 오인 | baseline stage를 별도 저장했고 세 샘플의 분산을 기록했다. 첫 run은 초기 흔들림을 노출해 안정화·재실행했다. |
| 7 | release 뒤 측정을 즉시 해 누적 해제 누락 | 모든 handle free 뒤 10초 관찰 구간을 두고 세 번 수집했다. |
| 8 | 실제 C12.4 worker가 뜨지 않아 가짜 수치가 생성 | 로그에 session ready와 `css_worker_per_session=1`가 나왔고 8개 handle을 해제했다. |
| 9 | session create 실패 뒤 부분 handle 누수 | iOS/Android 경로는 create 실패 시 루프를 멈추고 현재 handle을 역순 해제하도록 확인했다. 실패 강제 주입은 하지 않았다. |
| 10 | 8개 이상 무제한 생성으로 emulator OOM | 단계 상한을 8로 고정했다. |
| 11 | Android 실기기를 API 37 AVD로 오인 | 수집기에서 serial `emulator-5554`, API 37, `sdk_gphone` model을 검사한다. |
| 12 | 다른 Android app PID를 조회 | 최초 실행은 `am start` 직후 PID 조회 race로 측정 전 실패했다. PID가 나타날 때까지 기다리게 고쳐 재실행했다. |
| 13 | 실패한 수집 프로세스가 logcat을 남김 | 시작 후 모든 경로를 `finally`에 두어 log stream과 app을 종료한다. |
| 14 | stage baseline과 release file 경로가 덮어씀 | 재검토에서 stage-0/release 표본 이름 충돌을 발견해 분리하고 최종 run을 새로 수집했다. |
| 15 | PSS/RSS/VmSize를 서로 환산해 같은 메트릭 취급 | Android 원본 metric 이름과 KiB를 그대로 적고 iOS footprint MB와 직접 동치 비교하지 않는다. |
| 16 | Android `VmSize`를 실제 RAM으로 설명 | 증가량은 virtual size 관측으로만 기록했다. 큰 mapping의 소유자는 `/proc/maps` 권한 거부로 미확정이다. |
| 17 | Android 접근 거부를 누락하거나 값으로 대체 | `maps`·`smaps_rollup` permission denied 원문을 evidence로 남겼다. |
| 18 | iOS `vmmap` 실패를 footprint 수집 성공과 혼동 | `vmmap` DYLD query 실패를 남기고 동작한 `footprint`만 표에 사용했다. |
| 19 | process-wide 증분을 CSS worker stack에 귀속 | V8/OS/allocator를 포함하며 stack worker 단독값이 아니라는 한계를 반복해서 명시했다. |
| 20 | 결과로 실기기 안전성·최대 session 수를 선언 | 실제기기 RSS, thread별 stack commit, 반복 장기 회수, 성능, 최대 session 수를 미확정으로 유지한다. |

결과에서 추가 조사 가치가 있는 것은 Android `VmSize`가 첫 runtime host 뒤 약 16.6 GiB 증가하고 전체 해제 후에도 일부 유지된 관측이다. mapping 소유자 자료를 읽을 권한이 없어 원인을 특정하지 않는다. 이 수치를 resident memory나 stack 실사용량으로 읽지 않는다.

