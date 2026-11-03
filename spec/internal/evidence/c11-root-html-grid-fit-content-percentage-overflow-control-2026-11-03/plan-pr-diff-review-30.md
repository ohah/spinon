# C11.1 `fit-content(%)` 계획 PR diff 프로젝트 적대 검토 30개

검토 대상은 PR의 전체 staged plan, evidence, STATUS 및 review 문서 diff다. 각 항목은 구현 동작이 아닌 변경 계약·근거·scope 경계를 따로 확인한다.

| # | 독립 검토 관점 | 판정 및 근거 |
|---:|---|---|
| 1 | 계획이 기존 #1093 scope를 정확히 잇는가 | **PASS** — 기존 plan의 percentage/calc 제외 조건을 명시한다. |
| 2 | 계획이 공식 implementation state를 앞서가는가 | **PASS** — 이 문서는 candidate이며 구현 시작 전이라고 표시한다. |
| 3 | C11 checkbox가 변경됐는가 | **PASS** — 변경 없다. |
| 4 | C11.3/C11.4 checkbox가 변경됐는가 | **PASS** — 변경 없다. |
| 5 | 전체 counter가 후보를 완료로 세는가 | **PASS** — official `72/213`은 unchanged로 기록한다. |
| 6 | 표준 링크가 특정 규범 버전에 고정됐는가 | **PASS** — 2025-03-26 CSS Grid CRD §7.2.1을 사용한다. |
| 7 | 기대값과 oracle 관찰이 같은 artifact에 뒤섞였는가 | **PASS** — `expected.json`과 `chrome-observation.json`이 분리됐다. |
| 8 | Ahem 입력 자원이 재현 가능한가 | **PASS** — font path와 SHA-256이 기록됐다. |
| 9 | Chrome version과 viewport가 남는가 | **PASS** — Chrome 156, Agent Browser CLI, DPR/viewport가 저장됐다. |
| 10 | nine-scene expectation의 arithmetic가 서로 모순되는가 | **PASS** — cap, min/max contribution 및 used track을 README/JSON에서 대조했다. |
| 11 | width-axis percentage basis가 문서화됐는가 | **PASS** — definite content width를 명시한다. |
| 12 | height-axis percentage basis가 독립 검증되는가 | **PASS** — 100px definite row-height case를 포함한다. |
| 13 | 0%가 별도 입력으로 존재하는가 | **PASS** — zero percentage와 zero basis를 matrix에서 분리한다. |
| 14 | >100% 기대가 clamp되는가 | **PASS** — 120% limit 240px은 그대로 두고 used max-content 110px을 기록한다. |
| 15 | repeat semantics가 fixed integer subset인가 | **PASS** — 2-track integer repeat positive; auto-repeat excluded. |
| 16 | repeat list expansion/quota bypass가 남는가 | **PASS** — 기존 10,000-track quota acceptance에 포함한다. |
| 17 | named-area semantics가 새로 admit되는가 | **PASS** — percentage fit-content area combination은 제외한다. |
| 18 | implicit track sizing이 새로 admit되는가 | **PASS** — `grid-auto-*` paths는 negative scope다. |
| 19 | percentage calc와 direct percentage가 구별되는가 | **PASS** — calc/math arguments are explicitly excluded. |
| 20 | invalid CSS와 unsupported computed value가 혼동되는가 | **PASS** — parser/cascade invalid input과 typed/layout rejection을 분리한다. |
| 21 | indefinite axis fallback이 제품 규칙처럼 포장되는가 | **PASS** — 해당 상황을 지원 profile의 fail-closed 경계로 제한한다. |
| 22 | aggregate/frame overflow가 unchecked될 수 있는가 | **PASS** — finite axis, aggregate size, track quota와 atomic publication criteria가 있다. |
| 23 | visible border-overflow claim이 재현 가능한가 | **PASS** — 30px content, 200% cap, 60px item 및 28px overflow를 fixture로 고정했다. |
| 24 | border marker가 layout input에 영향을 주는가 | **PASS** — marker는 absolute sibling overlay이고 fixed expectation에는 border/root/child boxes가 기록됐다. |
| 25 | visual sample은 screenshot bytes에 연결되는가 | **PASS** — JSON은 screenshot hash와 RGBA sample을 보존한다. |
| 26 | untracked evidence가 manifest 없이 staging되는가 | **PASS** — 두 evidence directory에 개별 file SHA-256 manifest가 있다. |
| 27 | 새 문서 링크가 실제 repo target에 연결되고 RSPress가 후보 경로를 dead link로 세는가 | **PASS WITH BASELINE LIMIT** — 6개 새 Markdown 파일의 local-link check는 0 broken이다. final RSPress error 목록에는 후보 경로가 없지만 전체 빌드는 기존 dead-link 1,300개 때문에 실패했다. |
| 28 | API/internal contract numeric version이 부당하게 증가하는가 | **PASS** — 계획은 0059 `0.1.0`을 유지한다. |
| 29 | 플랫폼 evidence gate가 실기기/hardware proof로 확대되는가 | **PASS** — AVD software renderer와 pre-compositor iOS drawable 한계를 적는다. |
| 30 | 계획 검토 횟수와 PR review가 서로 대체되는가 | **PASS** — plan failure review 20, PR user-review 20, project diff review 30 및 future implementation review를 별도로 둔다. |

프로젝트의 계획 PR 30개 diff 검토를 마쳤다. 구현 근거로 확대할 수 있는 browser precomparison 한계도 계획에서 명시했다.

최종 문서 빌드 결과는 green이 아니다. `mise exec -- bun run docs:build`가 기존 dead-link 1,300개로 종료 code 1을 반환했고, 필터링한 출력은 이 후보 경로를 언급하지 않았다. 이 PR은 전체 문서 빌드 통과를 주장하지 않는다.
