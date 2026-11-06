# C11.4 Root HTML auto implicit-row stretch 계획 PR diff 적대 검토 30개

대상 변경: 새 계획, 독립 계획 실패 관점 20개, 고정 Chrome precomparison fixture·expected JSON·화면·README, C11.4 상위 계획의 후보 연결. 이 검토는 계획 PR 변경분을 대상으로 하며 구현 코드/runtime 동작을 검토하지 않는다.

| # | 독립 실패 경로·계약 경계 | 판정 |
|---:|---|---|
| 1 | §11.8에 정의되지 않은 behavior를 계획의 근거로 사용하는가? | PASS — W3C 2025 CRD §7.6·§11.4–§11.8을 연결했다. |
| 2 | `normal` content distribution을 author가 쓴 `stretch`와 같은 computed value로 기록하는가? | PASS — computed `normal`과 그 Grid sizing 효과를 분리했다. |
| 3 | Grid block-size가 definite하지 않은 fixture를 기준으로 하는가? | PASS — 40px definite height와 CSS viewport 조건을 고정했다. |
| 4 | explicit row가 free-space 산식에서 빠지는가? | PASS — 독립 계산에 10px explicit row를 뺀다. |
| 5 | implicit row의 base와 post-stretch size를 하나의 값으로 뭉개는가? | PASS — 20px base와 30px final을 별도 기록한다. |
| 6 | line box 수가 browser font fallback 때문에 달라지는가? | PASS — 10px monospace/10px line-height와 fixed literal source를 고정했다. |
| 7 | spacing/gutter가 free-space에 섞이는가? | PASS — zero gap과 zero edge를 고정했다. |
| 8 | fixed explicit row의 contribution이 text item에 귀속되는가? | PASS — 첫 fixed child와 둘째 text child의 source/frame을 분리했다. |
| 9 | item stretch를 track stretch의 증거로 잘못 해석하는가? | PASS — `usedRows`, track trace, item frame을 별도로 검사한다. |
| 10 | `align-content:start` control이 지원 범위로 유입되는가? | PASS — browser control로 명시하고 제품 gate에서는 제외한다. |
| 11 | min/max-content row control과 auto row의 결과 차이가 누락되는가? | PASS — independent expectation에 두 control을 보존했다. |
| 12 | fixed 10px row에서 text overflow 10px이 row size로 오인되는가? | PASS — fixed track과 child frame을 분리해 예상값을 명시했다. |
| 13 | browser observation이 규범 기대값의 유일한 원본이 되는가? | PASS — normative 산술을 독립 기준으로 두고 Chrome은 교차 확인만 한다. |
| 14 | HeadlessChrome 157 결과를 binary-pinned 기준으로 과장하는가? | PASS — CLI UA와 version-bounded 한계를 기록했다. |
| 15 | device scale factor 또는 viewport가 재현되지 않는가? | PASS — 1280×633 CSS px와 DPR 1을 JSON/README에 고정했다. |
| 16 | source file 변경 뒤 캡처가 다른 fixture에 묶이는가? | PASS — fixture SHA-256을 evidence에 넣었다. |
| 17 | expected JSON과 실제 관측 행렬의 scene 집합이 다른가? | PASS — positive 1개와 controls 4개를 동일한 순서/ID로 기록했다. |
| 18 | Text Range bounds를 CSS line count와 혼동하는가? | PASS — range rect 개수만 줄 수로 쓰고 rect 높이를 track 결과로 쓰지 않는다. |
| 19 | fixture screenshot이 Spinon 결과처럼 보이는가? | PASS — README에서 browser fixture only라고 한정했다. |
| 20 | screenshot hash가 화면 파일과 맞지 않는가? | PASS — `fixture.png` SHA-256을 함께 기록했다. |
| 21 | 계획이 default parsed Root HTML route 대신 fixture-only success route를 요구하는가? | PASS — 기본 `RuntimeRootHtmlTextPaintV1` route를 acceptance에 명시했다. |
| 22 | 제한 profile이 임의 text grammar를 새로 열도록 유도하는가? | PASS — 기존 ASCII alphanumeric/hyphen/whitespace gate 재사용으로 제한했다. |
| 23 | 여러 implicit row·extra item이 자유롭게 허용되는가? | PASS — 단일 implicit row와 고정 cardinality를 명시했다. |
| 24 | authored placement, named area, repeat, intrinsic columns이 섞이는가? | PASS — 지원 밖 fail-closed 입력으로 나열했다. |
| 25 | 10,000 track limit 또는 checked arithmetic이 계획에서 사라지는가? | PASS — 기존 bound와 atomic failure를 유지한다. |
| 26 | stale revision 또는 잘못된 owner가 trace/frame을 남기는가? | PASS — identity/revision과 no-partial-output 검사를 포함했다. |
| 27 | PR 작성·병합 순서가 계획 승인보다 implementation을 먼저 허용하는가? | PASS — plan PR merge 이후 implementation branch로 순차 진행한다. |
| 28 | simulator draw/present를 surface pixel equality로 확대하는가? | PASS — screenshot, runtime, readback, compositor의 증명 범위를 분리했다. |
| 29 | plan-only merge가 official completion count를 올리는가? | PASS — C11.4/C11 checkboxes와 `78/214`를 그대로 둔다. |
| 30 | user가 요청한 roadmap continuity가 없어 다음 구현을 찾기 어려운가? | PASS — 상위 C11.4 plan에 다음 후보와 독립 evidence 링크를 추가했다. |

계획 PR 변경 검토는 30개 독립 관점에서 PASS다. 구현 이후의 새 20개 구현/runtime 검토와 별도 구현 PR diff-30 검토를 이 결과와 합산하지 않는다.
