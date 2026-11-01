# C11.1 Root HTML anonymous Grid text 계획 PR diff 검토 · 독립 30관점

검토 대상은 이번 계획 PR의 실제 staged diff 14개 파일이다. 기능 구현은 포함하지 않는다. Chrome screenshot은 사전 비교 관찰이며 Spinon runtime 결과가 아니다.

| # | 실패 관점 | diff 확인 및 판정 |
|---:|---|---|
| 1 | PR에 기존의 무관한 수천 개 untracked artifact가 섞이는가 | 명시한 PR 14개 파일만 stage했다. 그 밖의 untracked evidence는 index에 없다. PASS. |
| 2 | 별도 worktree/병렬 candidate가 결과를 뒤섞는가 | 단일 checkout의 planning 브랜치에서만 변경했고 staged diff는 하나의 plan slice다. PASS. |
| 3 | 새 slice가 C11.1 구현 완료처럼 표시되는가 | plan과 STATUS는 계획 단계라고 명시하고 C11.1 checkbox를 변경하지 않았다. PASS. |
| 4 | anonymous text가 현재 기본 경로에서 이미 성공하는 것으로 적히는가 | source와 Root HTML regression의 기존 거부 diagnostic을 증거에 연결했다. PASS. |
| 5 | 오래된 규격 링크가 기준 문서로 쓰이는가 | 계획과 evidence는 날짜 고정 W3C 2025-03-26 CRD §6·§8.5 URL을 사용한다. PASS. |
| 6 | anonymous text sequence 생성 규칙이 빠지는가 | 계획이 in-flow direct text run을 anonymous block-container Grid item으로 규정한다. PASS. |
| 7 | auto-placement order가 일반 source-order 지원으로 오인되는가 | 계획은 단일 item의 첫 row-major cell만 산출하며 mixed siblings를 거부한다. PASS. |
| 8 | Chrome version이 변동되었는데 캡처가 임의 버전으로 기록되는가 | observation과 verifier가 HeadlessChrome 156을 고정 검사한다. PASS. |
| 9 | browser observation이 규범 판정값을 대체하는가 | 독립 expected JSON과 normative W3C basis를 분리하고 Chrome을 관찰값으로 한정한다. PASS. |
| 10 | text item frame의 독립 계산이 tracks와 모순되는가 | 120×70 root, [40,80] columns, [70] row, 0 gap으로 첫 cell [0,0,40,70]을 산출한다. PASS. |
| 11 | 화면에서 anonymous box를 직접 DOM element로 조회했다고 오기하는가 | README와 observation limits가 item box는 DOMRect 대상이 아니라고 분명히 한다. PASS. |
| 12 | Chrome monospace Range 폭을 모든 platform의 제품 기대값으로 강제하는가 | 19.265625px는 관측값이고 expected에 origin·line count만 규범 기준으로 둔다. PASS. |
| 13 | whitespace-only 문자열이 item을 만든다는 잘못된 기준을 둔다 | normal whitespace control은 item 0·Range rect 0으로 고정한다. PASS. |
| 14 | preserved whitespace를 normal whitespace와 혼동하는가 | plan은 pre 계열과 preserved whitespace를 지원 밖으로 두고 positive control은 normal만 쓴다. PASS. |
| 15 | anonymous item의 inherited typography·color가 정의되지 않았는가 | plan은 normal, monospace 16px/20px, #e5484d와 기본 alignment tuple을 고정한다. PASS. |
| 16 | text source를 합치는 동안 원본 NodeId가 사라지는가 | 계획이 anchor와 모든 source-node provenance 유지 조건을 명시한다. PASS. |
| 17 | synthetic element를 만들어 DOM·accessibility tree를 바꾸는가 | HostDocument mutation과 child vector 재정렬을 금지한다. PASS. |
| 18 | 단순 C11.4 opt-in gate 활성화로 기본 runtime을 우회하는가 | 계획이 C11.4 collector gate를 직접 true로 바꾸지 않게 하고 별도 경계 검사를 요구한다. PASS. |
| 19 | C11.4 opt-in fixture payload가 default archive로 새는가 | default product와 opt-in positive-control archive 비교를 구현 단계 gate로 남겼다. PASS. |
| 20 | stale document/style/environment revision이 잘못 붙는가 | 동일 revision identity와 stale input failure 조건을 계획에 명시한다. PASS. |
| 21 | malformed UTF-16·duplicate source가 부분 성공이 되는가 | encoding·uniqueness 검사를 scene publication 전에 적용하도록 한다. PASS. |
| 22 | partial paragraph/draw failure가 정상 장면처럼 보이는가 | failure atomicity와 partial publication 부재를 runtime gate로 적었다. PASS. |
| 23 | 글자 wrapping 또는 track 밖 overflow를 숨기는가 | 단일 line·40px track 내 text를 기준으로 하고 wrapping/unsupported typography를 거부한다. PASS. |
| 24 | fixed track baseline이 root edges·gap 변동에 흔들리는가 | zero edge와 zero gap, definite px basis를 고정한다. PASS. |
| 25 | 기존 named-area/fixed-gap/overflow behavior를 놓치는가 | 구현 계획은 이 기본 Root HTML 회귀와 C11.4 suite를 분리 실행하도록 요구한다. PASS. |
| 26 | Android simulator screenshot이 실기기/hardware GPU 결과로 과장되는가 | 계획이 API 37 ARM64 AVD와 실기기/hardware GPU 한계를 구분한다. PASS. |
| 27 | iOS Simulator image가 compositor 이후 pixel equality로 과장되는가 | 계획은 draw 단계와 Metal drawable/compositor 시점을 구분하도록 한다. PASS. |
| 28 | 캡처 artifact가 바뀌어도 baseline 검증이 통과하는가 | SHA-256 manifest는 fixture, expected, observation, PNG, verifier와 README를 묶고 검증 JSON에 manifest hash를 저장한다. PASS. |
| 29 | verification record가 runtime/WPT 성공을 주장하는가 | JSON과 README는 사전 비교 verifier 범위로 한정하고 Spinon, mobile runtime, WPT를 제외한다. PASS. |
| 30 | 문서 업데이트가 공식 완료 수나 공개 지원 상태를 올리는가 | 기존 C11 plan index에서 끊어진 evidence link 4개를 존재하는 후속 기록으로 고쳤고 새 상대 Markdown link 검사 결과 0개 누락이다. STATUS checkbox와 roadmap 집계는 그대로 두고 plan-only progress와 72/213을 명시한다. PASS. |

## 검토 결론

실제 diff는 계획·사전 비교·상태 연결만 추가한다. 구현 PR은 이 계획 merge 후 같은 checkout에서 순차 진행하며, 새 runtime 실패 관점 20개와 별도 PR diff 30개가 남아 있다.
