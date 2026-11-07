# C11.4 percentage fit-content 계획 PR diff 적대 검토 30개

대상: [계획 PR #1267](https://github.com/ohah/spinon/pull/1267)의 실제 GitHub diff 및 PR 본문. 계획 검토 20개와 분리된 변경분 검토다.

| # | 독립 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | PR base가 최신 공식 구현 tree가 아님 | Base는 `main` SHA `77f2216e3b81b9597bc7ed7c1c0ec716e41a70d2`이며 작성 시 최신이다. PASS |
| 2 | 계획 PR에 제품 동작 코드가 섞임 | 변경 8개는 계획·fixture·사전 비교 자료만 포함한다. PASS |
| 3 | 기존 px slice 선행 관계가 틀림 | PR #1265를 정확히 선행 slice로 연결했다. PASS |
| 4 | C11.4 또는 C11을 완료 처리 | 계획·PR 본문 모두 미완료를 명시한다. PASS |
| 5 | 공식 완료 수가 잘못 바뀜 | `78/214`를 유지한다고 명시했고 STATUS를 수정하지 않았다. PASS |
| 6 | 공개 CSS 지원 또는 WPT 성과를 과장 | 문서·fixture PR이며 public admission/WPT 범위 밖임을 적었다. PASS |
| 7 | Grid의 track-size 문법 근거가 2차 자료 | W3C CSS Grid Level 2 §7.2.1 직접 링크를 쓴다. PASS |
| 8 | 음수 percentage를 지원 범위에 넣음 | 계획은 finite non-negative 값만 허용한다. PASS |
| 9 | 행 percentage의 basis를 inline size로 오해 | inner block size를 명시하고 관측은 100px 높이를 사용한다. PASS |
| 10 | indefinite/cyclic percentage를 definite처럼 처리 | indefinite Grid height를 제외하며 intrinsic sizing 경계를 적었다. PASS |
| 11 | CSS viewport 높이를 basis로 사용 | 기대값은 Grid content-box 100px에서 계산한다. PASS |
| 12 | border-box와 content-box를 혼동 | border/padding 0과 content-box shape를 고정한다. PASS |
| 13 | gap이 percentage reference에 영향을 줌 | row/column gap 0을 고정하고 nonzero gap을 제외한다. PASS |
| 14 | direct percentage가 ratio 대신 정수로 전달 | 계획은 typed ratio 보존을 acceptance gate에 둔다. PASS |
| 15 | `0%` cap의 zero-height frame 누락 | zero 장면과 frame publication 검사를 명시한다. PASS |
| 16 | 20/24% cap이 max-content보다 작을 때 잘못 확장 | 독립 expected와 Chrome used rows 20/24px이 일치한다. PASS |
| 17 | 34% cap의 동률 경계가 빠짐 | max-content 34px와 같은 cap을 별도 장면으로 둔다. PASS |
| 18 | 40% cap이 max-content 초과로 과성장 | 40% 장면은 34px contribution에 머문다. PASS |
| 19 | `120%`가 100%로 임의 축소 | 원본 cap 120px을 관측하고 used size를 contribution으로 구분한다. PASS |
| 20 | `min-height` automatic minimum이 상한 아래로 clamp | 28px item minimum vs 20px cap scene을 고정한다. PASS |
| 21 | 반복 auto-row의 다음 item을 무시 | 6px probe 행의 위치와 track을 oracle/expected JSON에 넣었다. PASS |
| 22 | `fit-content(calc())`가 direct percentage로 오인 | computed `Calc` AST를 거부하는 acceptance/negative control이 있다. PASS |
| 23 | 다중 또는 mixed track list가 singleton route에 유입 | singleton admission과 mixed-list 거부를 명시한다. PASS |
| 24 | 다른 explicit track, area, repeat, dense/column flow와 교차 | 해당 profile 조합을 지원 밖으로 분리했다. PASS |
| 25 | failure가 partial render snapshot을 남김 | no-partial publication 및 같은 session recovery를 구현 acceptance에 둔다. PASS |
| 26 | 기존 px implicit row 또는 explicit percentage regression | px 경로 유지와 explicit `RootHtmlFitContentV1` regression을 적었다. PASS |
| 27 | Chrome fixture에 여러 Grid가 있어 runtime 입력으로 오해 | precomparison fixture와 runtime fixture를 별도 산출물로 구분한다. PASS |
| 28 | Chrome 수치가 독립 기대값으로 둔갑 | browser observations와 formula-derived expected JSON을 분리했다. PASS |
| 29 | PR screenshot URL이 commit 또는 경로를 잘못 가리킴 | PR 본문 URL은 plan commit `3953b7550`의 고정 raw asset이며 1280×845 원본이 로드된다. PASS |
| 30 | Agent Browser가 남아 사용자 환경에 영향을 줌 | 이미지 자연 크기·표시 크기를 확인하고 전용 session 종료 뒤 inactive, PID null, session list empty를 확인했다. PASS |

결과: 30/30 관점 통과, 수정이 필요한 결함은 찾지 못했다. PR diff는 제한 계획/사전 비교 범위를 유지한다. 문서 변경 전용이므로 기능 테스트를 실행하지 않았다. 구현 PR에서는 새 구현/runtime 20관점과 별도 PR diff 30관점 검토를 수행한다.
