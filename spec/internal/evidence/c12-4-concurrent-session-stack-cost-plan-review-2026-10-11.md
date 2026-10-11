# C12.4 동시 session stack 비용 검증 계획 실패 관점 검토

**대상:** [검증 계획](../../../plan/c12-4-concurrent-session-stack-cost.md)  
**판정:** 발견사항을 계획에 반영한 뒤 시뮬레이터에서 실행한다.

| # | 독립적인 실패 관점 | 반영한 통제 또는 남는 경계 |
| --- | --- | --- |
| 1 | 비교 단계 간 앱 startup noise가 섞임 | 앱은 새 프로세스 1개로 시작하고 session 0 baseline 후 누적 측정한다. |
| 2 | 각 stage에서 기존 host가 포함되어 count가 어긋남 | 기존 화면용 session 생성 분기를 우회하고 probe session count를 직접 기록한다. |
| 3 | 실제 C12.4와 다른 기본 CSS profile 측정 | C12.4 stacking fixture 생성자를 재사용한다. |
| 4 | renderer/GPU 표면 메모리가 session 비용으로 섞임 | probe는 surface와 renderer를 생성하지 않는다. |
| 5 | 8 MiB 예약을 실제 resident 비용으로 오해 | reservation 합과 process metric을 별도 필드 및 문장으로 유지한다. |
| 6 | 프로세스 전체 RSS 증가분을 CSS worker stack에 귀속 | V8 heap·기타 thread·allocator도 포함한다는 한계를 기록한다. |
| 7 | iOS physical footprint와 Android PSS가 같은 metric처럼 취급 | 플랫폼 도구의 metric 이름 그대로 보존하고 직접 동치 비교하지 않는다. |
| 8 | 단계 수를 생성 요청 수로만 기록하고 성공 수를 누락 | 성공 handle 수와 create 오류를 각 stage marker에 기록한다. |
| 9 | 중간 생성 실패로 이미 만들어진 host가 누수 | 실패 즉시 중단하고 부분 생성 handle을 역순으로 해제한다. |
| 10 | 앱 OOM 또는 응답 정지로 probe가 무한 대기 | 상한은 8이며 수집기 timeout/프로세스 종료를 실패 evidence로 남긴다. |
| 11 | 단계별 샘플이 한 번뿐이라 순간 변동에 좌우 | 각 hold 구간에서 두 번 이상 외부 메모리 샘플을 취한다. |
| 12 | 샘플 전에 session이 해제되어 stage가 무효 | hold 중 log marker를 기준으로 외부 수집기가 측정한다. |
| 13 | session 해제 후 OS allocator가 memory를 안 돌려줘 누수로 오판 | free 후 감소 여부를 관찰하되 즉시 감소하지 않음을 누수 증거로 쓰지 않는다. |
| 14 | thread 수 증가를 stack commit 증가와 혼동 | 구현이 만드는 worker 종류와 session count를 기록하지만 thread별 commit은 별도 미확정이다. |
| 15 | V8 initialize laziness가 session당 다르게 나타남 | 같은 C12.4 host 생성 경로를 반복하며 startup 직후 안정 구간을 샘플한다. |
| 16 | 디버그 빌드의 메모리를 제품 release에 일반화 | build configuration을 기록하고 결과를 simulator diagnostic으로 제한한다. |
| 17 | Android 실기기 요청/연결을 잘못 사용 | 고정된 API 37 AVD만 선택해 기록한다. |
| 18 | iOS 실기기 결과로 오해 | iPhone 17 Pro/iOS 26.2 Simulator 이름과 UDID를 증거에 기록한다. |
| 19 | 단일 실행의 로그가 다른 실행의 값과 섞임 | 플랫폼별 fresh process, run ID 및 stage marker로 분리한다. |
| 20 | 이 진단만으로 최대 session 수나 allocator OOM 내성을 선언 | 결과는 측정한 0–8개 구간의 관측만 말하고 전역 OOM·제품 한도는 미확정이다. |

