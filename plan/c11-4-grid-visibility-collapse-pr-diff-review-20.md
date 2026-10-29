# C11.4 Grid visibility collapse 계획 PR 변경분 · 독립 실패 관점 20개

검토 대상은 plan/c11-4-grid-visibility-collapse branch의 계획 PR 변경분이다. 기준 HEAD는 155cebc03이고, diff에는 계획·계획 리뷰·고정 CSS fixture·Agent Browser 증거 및 C11.4 상위 계획 갱신만 포함한다. Runtime 구현이나 공식 상태 승격은 없다.

| # | PR 변경 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | 계획 PR에 runtime 구현을 섞어 계획과 제품 동작을 혼동시킬 수 있음 | staged path 10개를 확인했다. Rust/C/Swift/Kotlin/runtime 소스는 없고 계획·HTML oracle·evidence만 변경됐다. 통과. |
| 2 | 거대한 checkout의 다른 untracked 산출물이 PR에 섞일 수 있음 | 명시한 10개 path만 stage했고 git add -A를 사용하지 않았다. tracked diff와 staged name list가 일치한다. 통과. |
| 3 | 변경이 최신 main 아닌 별도 worktree에서 나올 수 있음 | 같은 checkout의 plan/c11-4-grid-visibility-collapse branch가 155cebc03에서 시작했고 새 worktree를 만들지 않았다. 통과. |
| 4 | C11 상위 또는 C11.4가 계획 단계에서 완료로 바뀔 수 있음 | 공식 spec/STATUS.md는 변경하지 않았고 새 계획은 계획 단계로 표시한다. 통과. |
| 5 | 공식 전체 완료 수가 계획 변경 때문에 오를 수 있음 | C11 checkbox와 공식 72/213을 유지한다. 계획, 부모 계획 및 evidence README에서 같은 경계를 기록한다. 통과. |
| 6 | PR 범위가 generic CSS visibility 지원처럼 읽힐 수 있음 | opt-in C11.4 fixture path와 direct Grid item만 명시하며 public author CSS, 다른 Grid profile 및 arbitrary DOM은 제외한다. 통과. |
| 7 | standards 링크가 source 또는 section이 아닌 search result를 가리킬 수 있음 | CSS Display, CSS Grid 2025 CRD, Flexbox §4.4의 직접 W3C 문서 링크를 사용했다. 통과. |
| 8 | Grid hidden-equivalence가 표준 원문 인용처럼 과장될 수 있음 | Display fallback과 Grid 전용 규칙 부재를 결합한 inference로 구분했고 Flexbox 알고리즘은 제외한다. 통과. |
| 9 | fixture source가 Chrome 관측과 달라질 수 있음 | expected JSON의 SHA-256을 실제 fixture bytes와 비교하며 Agent Browser capture는 그 hash와 함께 기록했다. 통과. |
| 10 | positive control이 없어 overflow 표본의 부재가 geometry 오류일 수 있음 | 양축 visible control이 같은 root 경계 1px 밖에서 red paint와 hit target을 만든다. collapse/hidden 및 visible descendant scene과 짝지었다. 통과. |
| 11 | matrix가 한 축만 또는 하나의 visibility state만 확인할 수 있음 | 10개 고정 scene은 Columns/Rows 각각 collapse, hidden, display-none, visible descendant, visible control을 포함한다. 통과. |
| 12 | screenshot sample 개수와 계획/evidence 문서의 수가 다를 수 있음 | 처음 발견한 13/14 개수 불일치를 README와 계획 문서에서 14개로 고쳤고 validator는 14개 모두 검사한다. 통과. |
| 13 | overflow sample이 다음 stacked scene에 덮여 의미를 잃을 수 있음 | boundary 바로 1px 밖을 고르고 8px scene gap 안에서 sample한다. root-local 위치와 실제 overflow hit target을 검사한다. 통과. |
| 14 | elementFromPoint target을 item의 실제 hit 결과로 오해할 수 있음 | hidden/collapse 내부 point는 underlying scene, visible descendant/positive control은 실제 child ID를 기록하며 root 밖을 별도로 확인한다. 통과. |
| 15 | screenshot의 수동 색 해석이 문서에만 남을 수 있음 | 14개 RGB sample을 expected.json에 적고 standard-library PNG reader로 exact 비교한다. 통과. |
| 16 | validator가 다른 source 또는 screenshot으로 조용히 성공할 수 있음 | fixture와 screenshot SHA-256, Chrome version, viewport/DPR, 고정 ordered IDs, geometry, hit target 및 pixel values를 모두 검사한다. 통과. |
| 17 | review evidence에 workspace 개인 절대 경로가 공개될 수 있음 | 관측 JSON의 file URL에서 workspace prefix를 정규화했고 README에 이 정규화를 밝혔다. staged text scan에서 /Users와 /tmp 경로가 없다. 통과. |
| 18 | 계획 PR 링크가 target files가 없어 깨질 수 있음 | 새 계획, 20항목 plan review, PR diff review, fixture 및 evidence의 local Markdown links를 확인했다. 통과. |
| 19 | image attachment가 잘못된 source/commit URL로 PR에 들어갈 수 있음 | binary screenshot은 PR 변경분에 포함된다. PR body에는 commit-pinned raw URL을 넣고, 생성 후 Agent Browser에서 load/display를 확인한다. 통과 조건을 고정했다. |
| 20 | PR template, labels 및 구현 전 gate가 빠질 수 있음 | repository template을 따르고 기존 planning, area:layout, area:testing label을 쓴다. status 변경이 없는 이유를 적고, plan PR 병합 전 구현하지 않는다. 통과. |

**결과:** 계획 PR 변경 검토 20/20 완료. 발견한 scene-count 불일치와 workspace 절대 경로를 고쳤고, 현재 validator는 10/10 scene과 14/14 screenshot sample을 통과한다. PR body image URL의 최종 browser load 검증은 PR 생성 뒤 완료한다.
