# C11.4 §11.7 flexible track expansion 계획 PR · 변경분 검토 30개

**대상 변경:** 전용 계획, C11.4 계획 요약과 상태 기록, 20관점 계획 검토, CSS fixture, 독립 Fraction 산출물, Agent Browser 관측 및 화면. 이 문서는 계획 자체의 20관점 검토와 별도로 현재 PR diff의 연결·범위·재현성·표현을 확인한다.

| # | 변경 실패 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | 상위 C11.4 계획에 새 작업이 보이지 않음 | **통과.** plan/c11-4-grid-track-sizing.md 첫 항목에서 집중 계획과 사전 비교로 연결한다. |
| 2 | 공식 상태에 계획이 기록되지 않음 | **통과.** spec/STATUS.md는 새 C11.4 작업을 계획으로만 기록한다. |
| 3 | test-only 계획이 C11.4 완료로 오인됨 | **통과.** C11.4 checkbox는 그대로 미완료이고 공식 집계 72/213도 바꾸지 않았다. |
| 4 | 상태 기록이 실제 app/runtime 성공을 암시 | **통과.** STATUS는 CSS trace 계획과 Agent Browser 사전 비교라고 한정하고 Android/iOS 실행을 주장하지 않는다. |
| 5 | 새 계획 파일의 상대 링크가 잘못됨 | **통과.** 상위 계획, 내부 0059 계약, STATUS anchor, evidence 경로가 파일 위치 기준으로 연결된다. |
| 6 | 표준 revision이 움직이거나 넓은 최신 표준을 암묵 참조 | **통과.** 2025 CRD 날짜 경로와 §§11.7–11.7.1을 계획·JSON에 고정한다. |
| 7 | 독립 오라클이 production Taffy 결과에서 기대값을 역산 | **통과.** derive_expected.py는 표준 규칙을 Python Fraction으로 계산하고 Chrome/Taffy 입력은 사용하지 않는다. |
| 8 | free-space 산식에서 internal gap을 빠뜨리거나 두 번 차감 | **통과.** 각 2-track case는 10px gap 한 개를 한 번 빼며 root는 zero border/padding이다. |
| 9 | 대상 축과 반대 축의 크기 설정이 혼동 | **통과.** 모든 root/viewport가 대상축 200px, 반대축 120px이고 JSON에 두 값을 기록한다. |
| 10 | 반대 축 gap이 target-axis 계산에 섞임 | **통과.** 계획은 gap을 지정 축에만 두고 반대 축 gap을 0으로 고정한다. HTML fixture도 axis별 gap property만 설정한다. |
| 11 | weighted case가 factor 비율이 아닌 총합만 검사 | **통과.** 두 CSS track 기대 배열을 순서대로 190/3, 380/3으로 보존한다. |
| 12 | freeze case의 첫 fr 계산과 retry 결과를 혼동 | **통과.** JSON은 fr fraction [190/3,55], track base [80,110], 첫 track의 freeze index를 따로 적는다. |
| 13 | sub-unit case의 fixed track과 남는 공간이 누락 | **통과.** 0.5fr/20px 입력, [85,20] bases와 85px remaining space를 구분한다. |
| 14 | 0fr 사례가 positive factor 사례와 중복 | **통과.** nonzero 24px base의 0fr track이 바닥을 유지하고 1fr track이 166px을 받는다. |
| 15 | 원본 expected JSON을 재생성했을 때 수치나 순서가 변함 | **통과.** 재생성 후 SHA-256이 문서 값 07bedf1b…9529000과 일치한다. |
| 16 | Fraction oracle이 float 입력으로 이미 오차를 가짐 | **통과.** 입력 factor는 1/2 같은 rational 문자열이고 pixel values는 정수다. |
| 17 | freeze loop가 고정 track base를 차감하지 않음 | **통과.** initial frozen 집합에 non-flexible tracks가 들어가고 retry마다 그 base 합을 available에서 뺀다. |
| 18 | oracle의 freeze 기록이 track 배열 인덱스와 어긋남 | **통과.** inflexible_track_indices는 sorted case-local index이고 각 case track 배열의 순서와 함께 JSON에 있다. |
| 19 | HTML fixture의 visual children이 기대값을 오염 | **통과.** fixture child는 빈 명시 placement item이며 min-width/height:0; 계획은 이를 Rust runner input이라고 주장하지 않는다. |
| 20 | case selector 또는 item 순서가 화면/관측과 어긋남 | **통과.** HTML ID, JSON ID, 관측 id 네 개가 같은 weighted/frozen/subunit/zero-fr 사례를 식별한다. |
| 21 | browser viewport/DPR 누락으로 관측 재현 불가 | **통과.** 관측 JSON은 1280×633 CSS px와 DPR 1, user agent Chrome 156을 기록한다. |
| 22 | Chrome decimal track list와 DOMRect를 같은 정밀도의 증거로 표현 | **통과.** README는 CSSOM track list와 DOMRect를 분리하고 exact DOMRect를 독립 rational 값과 비교한다. |
| 23 | Chrome quantization을 Spinon 기대값으로 사용 | **통과.** Chrome은 browser control이며 규범 rational 기대값을 대체하지 않는다고 README와 계획이 명시한다. |
| 24 | screenshot이 Spinon output 또는 pixel equality로 읽힘 | **통과.** 설명은 화면을 Chrome reference fixture라고 한정하고 screenshot hash를 보존한다. |
| 25 | 사전 evidence가 구현/runtime pass로 읽힘 | **통과.** README 제목과 첫 문단이 계획 단계 자료이며 Spinon 통과 결과가 아니라고 밝힌다. |
| 26 | 재현 명령의 로컬 경로 placeholder | **수정 반영.** /absolute/path 표기를 git rev-parse --show-toplevel에서 얻는 repo root URI로 바꿨다. |
| 27 | 재현 fixture/script/hash가 서로 다른 revision을 가리킴 | **통과.** 현재 fixture, oracle script, JSON, observation, screenshot의 SHA-256이 README manifest와 각각 일치한다. |
| 28 | 이전 Agent Browser/Chrome background process가 남음 | **통과.** 사전비교 기록과 세션 종료 후 agent-browser session list 및 process inventory가 빈 상태였다. |
| 29 | 계획 단계 문서가 미실행 test/runtime을 결과로 기술 | **통과.** test 명령은 구현 후 계획 gate에만 있고 사전 비교 README는 Spinon 실행 결과가 아니라고 구분한다. |
| 30 | 변경 문서에 상태 ID, 근거, 남은 범위가 빠짐 | **통과.** 집중 계획은 C11.4와 CSS Grid §11.7을 식별하고 독립 산출물·Chrome 관측·미검증 범위를 연결한다. |

**판정:** 검토에서 발견한 재현 경로 placeholder를 저장소 기준 URI로 수정했다. 수정 후 hash·상대 링크·현재 공식 상태를 다시 대조했다. 변경은 계획 문서와 test fixture/evidence에 한정되며, 계획 PR을 Rebase and merge한 뒤에만 Rust 구현으로 진행한다.
