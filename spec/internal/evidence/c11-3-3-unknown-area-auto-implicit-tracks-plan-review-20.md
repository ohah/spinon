# C11.3.3 unknown-area auto implicit track 계획 실패 관점 검토 20개

**검토 대상:** [`plan/c11-3-3-unknown-area-auto-implicit-tracks.md`](../../../plan/c11-3-3-unknown-area-auto-implicit-tracks.md)

**검토 기준:** CSS Grid Layout Module Level 1 2025-03-26 CRD §7.3.2, §7.5–7.6, §8.3–8.4, §11.4–11.8; 고정 Chrome 154 C11.3.3 reference; 기존 C11.3.3/C11.4 runtime contract.

**결과:** PASS 20 · PARTIAL 0 · FAIL 0. 이 표는 계획 실패 관점이며 구현·runtime 검토나 PR diff 검토로 재사용하지 않는다.

| # | 독립 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | 알 수 없는 bare ident가 named area와 line placement 중 어디로 해석되는지 뒤바뀐다 | PASS — 계획은 area와 area-derived alias를 먼저 조회하고, 둘이 없을 때만 §8.3 implicit named-line placement를 택한다. |
| 2 | `grid-area` shorthand의 네 longhand 확장을 두 개 이하로 계산해 한 축의 implicit extent를 빠뜨린다 | PASS — fixed input은 네 computed longhand가 `not-authored`인 Chrome reference이며 row/column start/end를 각각 검사한다. |
| 3 | 같은 이름의 start/end 선이 explicit grid에서 발견되지 않았을 때 start와 end를 같은 line으로 합쳐 track 수가 줄어든다 | PASS — expected resolved grid는 각 축의 2 explicit + 2 implicit tracks와 N+1 line groups다. |
| 4 | CSS Grid에서 area row/column 수와 authored track 수 중 큰 쪽이 grid를 정한다는 규칙을 무시한다 | PASS — 입력은 양 축 template-area 2, explicit track 2로 unknown-name line placement만 분리한다. 별도 area-dimension auto track 경로는 PR #807의 성공 회귀 대조군으로 유지한다. |
| 5 | root의 definite size와 explicit track 합을 누락해 auto track stretch 공간을 잘못 배분한다 | PASS — explicit columns 합은 160px, rows 합은 120px로 root `160×120`을 채운다. Chrome에서 implicit auto track은 각 0px다. |
| 6 | 초기 `grid-auto-*` 값인 `auto`를 고정 0px 입력으로 바꿔 content contribution을 잃는다 | PASS — 계획은 Auto sizing 경로를 사용하고 C11.4 contribution regression을 요구하며 0px 특례를 금지한다. |
| 7 | Chrome의 0px implicit track을 규범 기대값으로 검산하지 않고 오직 oracle 결과로 복사한다 | PASS — 독립값은 §8.3 placement, §7.5–7.6 track 생성/기본값, §11 sizing 단계에서 산출하고 Chrome은 검산자로 한정한다. |
| 8 | 기본 auto size 대신 explicit authored `grid-auto-rows/columns` 값을 덮어쓴다 | PASS — 기본 auto와 기존 fixed-px list 입력을 분리한 positive/negative controls와 fixed-list 회귀를 요구한다. |
| 9 | 기존 fixed-px `grid-auto-*` 순환 pattern phase가 auto path 추가로 바뀐다 | PASS — area track-list cycling을 명시적 회귀 관문으로 두고 별도 mode 경계를 검사한다. |
| 10 | explicit authored line-name 또는 implicit area alias가 미존재 이름 fallback에 가려진다 | PASS — area/alias precedence와 명시 named-line control을 우선 계약으로 둔다. |
| 11 | 숫자 line placement가 새 implicit named-line mode를 빌려 profile 바깥 track을 생성한다 | PASS — numeric placement는 별도 control이며 새 mode는 missing area name의 실제 implicit usage에만 연결한다. |
| 12 | C11.3.1–3.2 wrapper의 기존 missing-name 결과가 auto track 추가로 성공으로 바뀐다 | PASS — 기존 profile 의미를 고정하고 fixture test profile만 opt-in 확장한다. |
| 13 | row와 column mapping이 서로 뒤바뀌거나 한쪽 축만 implicit sizing에 포함된다 | PASS — reference는 columns `[80,80,0,0]`, rows `[60,60,0,0]`을 축별로 비교하며 placement shorthand 양 축을 검사한다. |
| 14 | explicit known-area sibling의 위치, NodeId 또는 paint order가 unknown-area item 해석 때문에 변한다 | PASS — same-template sibling의 frame, source order, paint ordinal을 regression 불변식으로 둔다. |
| 15 | empty target item에만 0px 결과를 반환해 C11.4에서 이미 지원하는 text/image contribution을 억제한다 | PASS — C11.4 기존 contribution case를 재실행하며 미지원 기여는 구체 오류, 지원 기여는 같은 sizing path로 보존한다. |
| 16 | zero-size target frame에서 viewport pixel sample 부재를 렌더 실패로 오판한다 | PASS — target geometry/track을 주 판정으로 삼고 pixel sample은 보이는 control에서만 사용한다. |
| 17 | generated implicit track의 line-name metadata 길이가 track 수와 어긋나거나 한쪽 경계가 누락된다 | PASS — resolved track count/order와 N+1 line groups, checked arithmetic 및 reservation boundary를 검사한다. |
| 18 | track quota·index overflow·allocation 실패 뒤 일부 frame 또는 draw 결과가 남는다 | PASS — quota, checked math, fallible reserve, no-partial-output를 명시 완료 관문으로 둔다. |
| 19 | document/style revision 변경 뒤 이전 implicit placement snapshot을 재사용한다 | PASS — runtime은 동일 source/style revision으로 바인딩하고 stale revision은 frame/draw 전에 차단한다. |
| 20 | AVD/Metal sample을 physical GPU, iOS compositor 이후 결과 또는 Spinon WPT 적합성으로 과장한다 | PASS — AVD software renderer, pre-compositor Metal drawable, Chrome oracle-only 한계를 계획과 완료 기준에 명시한다. |

## 근거 확인

- Chrome 154 DPR 1·2 case: `tests/fixtures/css/references/c11-3-3-grid-template-areas-precomparison-v1.json` → `unknown-area-implicit-tracks`.
- Source input: `tests/fixtures/css/c11/grid-template-areas-c1133.html`; current expected rejection: `tests/fixtures/css/c11/grid-template-areas-c1133-runtime-matrix-v1.json`.
- Existing fixed-track overflow control: `tests/fixtures/css/c11/unknown-named-area-implicit-lines-c1133-inventory.json` 및 `unknown-area-fixed-implicit-tracks-mobile-pixels-v1.json`.
- W3C CSS Grid 2025 CRD의 §7.5–7.6은 implicit track 및 기본 auto sizing, §8.3은 이름 검색 부족 시 implicit line 처리, §11.4–11.8은 track sizing 단계와 auto stretch를 규정한다.
