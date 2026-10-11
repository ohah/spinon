# C12.4 CSS worker stack 상주 페이지 귀속 계획 · 실패 관점 검토

**대상:** [계획 초안](../../../plan/c12-4-worker-stack-residency.md)  
**판정:** 코드 구현 전에 검토 완료 · 20개 관점의 중복 없음 · 확인된 경계를 계획에 반영

| # | 독립 실패 관점 | 계획의 통제 또는 남은 한계 |
| ---: | --- | --- |
| 1 | 요청한 8 MiB를 실제 OS stack 크기로 기록 | 설정값과 pthread가 보고한 usable range를 서로 다른 필드로 저장하고 mismatch를 실패로 처리한다. |
| 2 | guard page까지 usable stack으로 셈 | 플랫폼 wrapper가 usable `[start,end)`를 반환하고 guard/예약 영역 포함 여부는 원본 기록으로 구분한다. |
| 3 | Linux와 Apple의 stack 주소 방향 차이를 혼합 | native wrapper에서 exclusive range로 정규화하고 현재 SP가 범위에 속하는지 검사한다. |
| 4 | thread 이름의 15자 절단이나 PID 추정으로 worker를 식별 | worker가 직접 게시한 OS TID·session ID·role을 결합하고 process PID도 run metadata에 고정한다. |
| 5 | UI/V8/render thread resident page를 CSS worker에 귀속 | stack bounds는 CSS worker 자신이 취득하고 live worker마다 고유 TID를 요구한다. |
| 6 | `mincore` residency를 committed memory 또는 PSS로 과장 | metric 이름을 `mincore_resident_pages`로 고정하고 commitment/PSS/RSS와 등치하지 않는다. |
| 7 | nonresident page를 미사용 stack bytes로 해석 | 결과는 순간 page residency만 나타내며 최대 stack 사용량·high-water mark를 주장하지 않는다. |
| 8 | Linux와 Darwin의 resident bit field 전체를 boolean 취급 | 각 platform의 residency bit만 세고 raw errno/status와 page vector hash를 보존한다. |
| 9 | 4 KiB page size를 16 KiB AVD나 다른 OS에 고정 | 각 실행에서 `sysconf(_SC_PAGESIZE)`를 읽고 alignment·count 산술을 checked 수행한다. |
| 10 | stack 끝 주소나 길이 overflow로 엉뚱한 VMA 측정 | checked add/subtract, page rounding, `start < end`, bounds containment 검증을 적용한다. |
| 11 | measurement가 대상 worker stack을 더 써서 값을 부풀림 | worker가 descriptor만 게시하고 coordinator가 `mincore`를 실행하며 vector는 제한된 heap에 둔다. |
| 12 | `/proc/<pid>/smaps` 권한 거부를 0 RSS로 숨김 | 같은 app UID `run-as`로 시도하고 실패를 별도 교차검증 실패로 남긴다. |
| 13 | `smaps`의 `[stack]` 이름만 보고 worker를 잘못 매칭 | 주소 범위와 TID를 대조하고 VMA label은 식별 근거로 사용하지 않는다. |
| 14 | Android 외부 수집기를 실제 앱과 다른 프로세스나 package에 연결 | 고정 AVD serial·model·API, package UID, PID, run ID를 상호 확인한다. |
| 15 | iOS `vmmap` 제한을 재현하지 못해 다른 도구를 stack metric으로 혼동 | iOS는 앱 내부 mincore와 pthread bounds만 사용하고 footprint는 process aggregate로 분리한다. |
| 16 | session stage와 worker count가 일치하지 않음 | 0·1·2·4·8의 생성 성공 수, 고유 TID, 해제/join marker를 비교기에서 정확히 확인한다. |
| 17 | 여러 worker를 동시에 측정하며 scheduling noise를 숨김 | sample 횟수와 timestamp를 worker별로 기록하고 단계별 분포를 보여 주며 평균 단일값으로 축약하지 않는다. |
| 18 | 계측 API 실패가 정상 CSS 동작까지 변경 | 오류는 진단 record에 한정하고 layout/session 수명 결과에 영향이 없음을 negative control로 확인한다. |
| 19 | raw stack 주소가 공개 페이지에 노출 | 재검토 때 Funnel이 인터넷 공개 상태임을 확인했다. 주소 원본은 로컬에만 남기고 GitHub와 Funnel 미리보기에는 요약값만 둔다. |
| 20 | AVD/Simulator 결과를 실기기 안정성·누수·GPU 결과로 확대 | 계획과 상태에 두 시뮬레이터의 순간 residency 관측임을 제한하고 다른 C12.4 gates를 그대로 미완료로 둔다. |

## 사전 가능성 확인

- AVD `emulator-5554`에서 `run-as dev.spinon.bootstrap cat /proc/<pid>/maps`와 `smaps_rollup`을 실행해 app UID로 프로세스 mapping과 aggregate metric을 읽을 수 있음을 확인했다. shell UID 접근 거부를 같은 app UID 접근 불가로 일반화하지 않는다.
- 현재 AVD 기본 화면에서는 C12.4 CSS worker가 시작되지 않았다. 따라서 이 조사는 Android의 실제 worker TID·stack VMA 귀속을 아직 증명하지 않는다. 후속 구현 검증에서 전용 C12.4 session probe를 실행해야 한다.
- 공식 Bionic source는 `pthread_getattr_np` 및 `pthread_attr_getstack`을 제공한다. Apple libpthread header는 `pthread_get_stackaddr_np`/`pthread_get_stacksize_np`를 선언한다. Apple 및 Linux `mincore` 문서는 이를 page residency 조회로 정의한다. 계획은 이 결과를 committed memory로 부르지 않도록 제한했다.
