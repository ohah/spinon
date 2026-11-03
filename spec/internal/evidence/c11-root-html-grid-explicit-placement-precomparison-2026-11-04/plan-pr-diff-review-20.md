# C11.1 parsed Root HTML numeric Grid placement 계획 PR diff 검토 20개

계획 PR의 실제 변경분을 서로 다른 실패 관점으로 확인한다. 구현/runtime 검증 및 향후 구현 PR 검토를 대신하지 않는다.

| # | 적대적 검토 관점 | 결과 및 근거 |
|---:|---|---|
| 1 | 계획 변경에 런타임 코드가 섞여 지원 범위를 우회 | **통과.** 변경 경로는 계획, 상태 기록, 사전 비교 fixture/evidence뿐이다. Rust 및 앱 runtime 소스 변경은 없다. |
| 2 | C11 상위 또는 하위 체크박스를 계획 단계에서 완료 처리 | **통과.** C11 parent와 기존 세부 체크 상태를 수정하지 않았다. |
| 3 | 전체 공식 완료 수가 후보 계획 때문에 증가 | **통과.** `72/213`을 유지한다고 계획, STATUS, 공개 roadmap에 기록했다. |
| 4 | 현재 no-area numeric placement 동작을 자동 배치 성공 또는 무음 fallback으로 오기 | **통과.** 현재 Root HTML 경로는 non-auto placement를 fail-closed한다고 기술했다. |
| 5 | 이미 구현된 C11.2.1 normalizer를 재구현하도록 계획 | **통과.** 기존 typed numeric line/span 변환과 `ExplicitOnly` validator 재사용을 요구한다. |
| 6 | Root HTML 문서 전체에 explicit-only mode를 전파 | **통과.** admission과 placement mode는 Grid owner 단위로 선택한다. |
| 7 | explicit Grid 때문에 all-auto sibling Grid까지 거부 | **통과.** sibling owner isolation을 계획 및 검토 항목에 포함했다. |
| 8 | 일부 visible item만 검사해 나머지를 auto-placement로 암묵 처리 | **통과.** selected owner의 전체 layout-participating child를 대상으로 한다. |
| 9 | anonymous text item이 CSSOM element scan에서 빠져 mixed placement를 허용 | **통과.** HostDocument layout child 전체와 anonymous text 거부를 명시했다. |
| 10 | partial-axis 또는 mixed-auto placement가 성공 처리 | **통과.** 두 축 모두 definite인 all-item 입력만 성공시키고 나머지는 scene 전체 거부로 계획했다. |
| 11 | 양수 line의 CSS 1-based와 layout 0-based 변환 오류 | **통과.** 기존 C11.2.1 normalizer 재사용과 독립 expected frame 검증이 요구된다. |
| 12 | 음수 line, reversed endpoint 또는 span 처리가 누락 | **통과.** Chrome fixture에 각각의 positive scene과 computed longhand 관측이 있다. |
| 13 | `grid-area`/shorthand를 다시 문자열 parse해 cascade winner와 불일치 | **통과.** Stylo computed longhand만 사용하도록 계획했다. |
| 14 | line overflow가 implicit track 생성이나 산술 wrap으로 성공 처리 | **통과.** authored explicit track 범위만 허용하고 implicit boundary와 overflow 거부를 포함한다. |
| 15 | named-area, dense, column-flow route의 검증 소유권을 침범 | **통과.** 해당 owner는 후보에서 제외하고 기존 전용 검증을 보존한다. |
| 16 | hidden child의 placement가 visible item 집합에 잘못 포함 | **통과.** `display:none` control scene과 visible-item 경계를 명시했다. |
| 17 | overlap/source order 또는 C11.5 alignment 교차가 회귀 | **통과.** hit-test overlap scene 및 explicit placement와 self-alignment 결합 scene이 있다. |
| 18 | Chrome 관측을 Spinon 구현 결과나 전체 CSS 적합성으로 주장 | **통과.** 자료를 사전 비교로 한정하고 runtime 검증 미실행을 반복해서 표시한다. |
| 19 | 화면, fixture, observation, expected 값이 서로 다른 입력을 가리킴 | **통과.** verifier가 11 scene·23 item frame과 fixture/evidence SHA-256 manifest를 확인했다. 첫 대조에서 README fixture SHA가 manifest와 달라 실제 해시 `e6beb2…10d7`로 수정한 뒤 verifier와 README를 일치시켰다. PR #1136을 Agent Browser로 열어 본문 image가 로드됐고 `naturalWidth×naturalHeight=1280×640`임을 확인했다. |
| 20 | 문서 검증 실패를 감추거나 이번 변경의 성공으로 오인 | **제한 기록.** `mise exec -- bun run docs:build`는 현재도 저장소의 1,337개 dead-link error group 때문에 실패한다. 첫 로그에서 이번 STATUS plan 상대 경로 오류 하나를 찾아 GitHub 링크로 고쳤고, 재실행 로그에는 이번 후보 경로가 더는 나타나지 않는다. 전체 docs build는 통과로 표시하지 않는다. |

## 판정

계획 diff는 범위와 공식 상태를 보존하며 구현 PR로 넘어갈 수 있다. 단, 이 검토는 Rust 구현, Android/iOS Simulator 동작, PR CI 또는 병합을 증명하지 않는다. 전체 문서 빌드는 기존 저장소 오류로 실패한 상태다.
