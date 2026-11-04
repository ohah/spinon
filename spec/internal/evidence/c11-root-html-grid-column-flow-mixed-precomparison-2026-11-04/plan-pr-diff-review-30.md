# C11.1 column-flow mixed placement 계획 PR 변경분 적대 검토 30개

**대상:** [계획](../../../../plan/c11-root-html-grid-column-flow-mixed-cross-axis.md), [계획 실패 관점 검토 20개](../../../../plan/c11-root-html-grid-column-flow-mixed-cross-axis-plan-review-20.md), 이 디렉터리의 구현 전 비교 fixture·독립 기대값·Agent Browser capture·화면. 기준 `origin/main` `089b76be997aa0114d752be494d318c93eea37df`.

각 관점은 이번 계획 PR diff와 연결된 입력·근거를 대조했다. 계획 자체의 20개 검토와 구분하며, 기능 구현/runtime 검토는 구현 PR에서 새로 수행한다.

| # | 독립 실패 관점 | 판정 |
|---:|---|---|
| 1 | 규범 알고리즘의 인용 버전 또는 §8.5 앵커가 다른 문서를 가리킨다 | **PASS** — 2025-03-26 CSS Grid CRD와 `#auto-placement-algo`가 계획·기대값에서 동일하다. |
| 2 | column 흐름 설명에서 row/column 교환을 누락해 row 흐름 규칙을 적용한다 | **PASS** — 규범 원문은 column이면 rows와 columns를 교환한다고 명시하며, 계획은 column 방향 cursor와 고정 column 단계를 분리한다. |
| 3 | 완전 definite 배치를 auto-placement 뒤에 두어 점유 우선순위가 달라진다 | **PASS** — fully definite 항목의 사전 예약을 계획과 independent model 모두에 둔다. |
| 4 | column-locked 항목의 고정 column을 잃거나 단계 순서를 바꾼다 | **PASS** — 고정 column 항목을 order-modified 순서로 해당 column의 빈 row에 먼저 배치한다. interleaved DOM scene으로 확인한다. |
| 5 | `order`를 허용하면서 DOM 순서로만 처리한다 | **PASS** — admission을 `order: 0`으로 제한하고, 이를 통해 DOM order와 order-modified order가 일치한다. |
| 6 | sparse cursor가 column에서 먼저 진행해 column-flow 축을 반대로 돈다 | **PASS** — cursor는 row를 먼저 진행하고 explicit row 끝에서 다음 column으로 이동한다. |
| 7 | 다음 fully-auto item에서 sparse cursor 상태를 초기화한다 | **PASS** — 계획은 이전 배치가 남긴 cursor를 유지하며 exact frame 비교와 기존 column-all-auto 회귀를 요구한다. |
| 8 | row-locked placement가 definite row를 바꾸거나 다른 row를 탐색한다 | **PASS** — definite row 보존과 충돌 시 column 진행이 규범 순서 및 `reverse-order`·span expected와 맞는다. |
| 9 | 유일한 row-locked/column-locked 대표가 `visibility:hidden`일 때 mode 선택에서 빠진다 | **PASS — 검토 중 보완** — 처음 diff의 “visible items” 표현을 발견해 참여 항목(`display:none` 제외, hidden 포함)으로 명확히 고쳤다. 두 hidden-only qualification scene을 Chrome 및 independent expected에 추가했다. |
| 10 | `visibility:hidden`을 분류에는 넣고 occupancy에서는 빼거나 반대로 처리한다 | **PASS** — hidden-only 두 장면과 기존 hidden occupancy scene에서 hidden item frame 및 후속 item의 빈 셀 선택을 각각 비교한다. |
| 11 | `display:none` child가 mode 분류 또는 점유에 영향을 준다 | **PASS** — property를 얻기 전에 제외하며, display-none control에서 해당 셀이 재사용된다. |
| 12 | 기존 all-auto column mode가 새 mixed mode로 선택된다 | **PASS** — 새 selector는 두 partial lock class가 모두 있어야 하고 all-auto column regression을 계획 gate에 둔다. |
| 13 | row-only/column-only partial input이 새 selector에 잘못 들어온다 | **PASS** — 두 class 동시 존재를 admission 조건으로 쓰며 기존 one-axis 경계 테스트를 요구한다. |
| 14 | sibling/nested Grid가 lock class를 제공하거나 occupancy를 공유한다 | **PASS** — selector와 validator 범위를 하나의 Grid owner와 direct children으로 제한한다. |
| 15 | authored CSS 문자열, tag/class 또는 scene ID가 placement mode를 결정한다 | **PASS** — typed computed snapshot과 owner-scoped item placement만 selector 입력으로 허용한다. |
| 16 | stale revision 또는 duplicate/missing NodeId가 다른 layout snapshot의 점유를 오염한다 | **PASS** — revision, owner, ID 유일성 검사를 focused runtime gate로 지정한다. |
| 17 | definite/partial/auto 분류 중 한 placement endpoint의 `auto`를 numeric으로 오해한다 | **PASS** — 네 longhand grammar와 지원 class를 따로 적고 mixed endpoint를 fail-closed로 둔다. |
| 18 | positive span이 점유 범위 전체를 차지하지 않아 후속 item이 겹친다 | **PASS** — 양의 span을 양 축 occupancy에 적용하며 2-row/2-column span fixture가 후속 placement를 막는다. |
| 19 | 음수 line을 반대 축 또는 1-based line 규칙으로 잘못 정규화한다 | **PASS** — `-2 / -1`을 각 explicit axis의 끝에서 해석하는 Chrome/expected control이 있다. |
| 20 | named line, named area, auto-repeat 또는 unsupported grammar가 새 numeric profile에 누수된다 | **PASS** — 새 mode에서 명시적으로 제외하고 property/node rejection 회귀를 요구한다. |
| 21 | implicit row/column 생성이 track sizing까지 지원되는 것으로 확대된다 | **PASS** — 해당 profile은 implicit extent를 거부하며 Chrome의 5×4 확장은 negative control로만 기록한다. |
| 22 | implicit boundary 거부가 일부 frame·trace·draw를 이미 게시한 다음 발생한다 | **PASS** — 계획은 owner 전체 atomic rejection 및 부분 output 부재를 요구한다. |
| 23 | 서로 다른 width/height와 track 합 때문에 Chrome 좌표가 비교 기준으로 무효가 된다 | **PASS** — definite content size와 fixed track 합, zero gap을 각 scene에 기록한다. |
| 24 | auto track, nonzero gap, padding/border 또는 alignment가 별도 sizing 결과를 섞는다 | **PASS** — auto rows/columns만 허용하고 gap·edges 0, start alignment 및 item box 제한을 명시한다. |
| 25 | RTL 또는 vertical writing mode에서 물리 row/column cursor를 잘못 해석한다 | **PASS** — LTR/horizontal-tb만 허용하고 나머지는 새 mode 밖으로 남긴다. |
| 26 | CSS spec의 sparse auto-placement와 fixture 결과가 서로 모순된다 | **PASS** — Agent Browser Chrome 156의 computed flow/tracks와 independent cell occupancy가 모든 recorded frame에서 일치한다. |
| 27 | 기대값 산출이 Chrome frame을 그대로 복사해 독립 검증이라고 잘못 주장한다 | **PASS** — expected JSON은 cell 점유 유도와 규칙을 별도로 기록하고 Chrome은 교차 확인으로만 취급한다. |
| 28 | 캡처 viewport, DPR, browser identity 또는 scene ownership가 재현되지 않는다 | **PASS** — capture JSON에 HeadlessChrome 156, 1280×633, DPR 1, scene-relative track/frame이 기록된다. |
| 29 | hidden item을 visible frame 수에 더하거나 반대로 누락한다 | **PASS — 검토 중 보완** — 원래 “25 visible” 문구가 hidden 2개를 포함함을 찾아 수정했다. 최종 positive matrix는 31 participating frames(27 CSS-visible, 4 hidden)이며 9 scene capture 모두 exact다. |
| 30 | screenshot/manifest/document 링크가 오래된 fixture를 가리키거나 runtime·C11 완료 근거처럼 보인다 | **PASS** — screenshot을 수정된 fixture에서 다시 캡처했고 manifest hash를 재생성한다. README는 browser precomparison 한계와 C11 미완료를 유지한다. |

## 판정

30개 관점의 검토에서 hidden item mode qualification과 실제 visible-frame 집계 문구를 보완했다. Agent Browser capture와 독립 expected를 갱신한 뒤 9 scene, 8 positive scene, positive 31 participating item frame(27 visible·4 hidden)을 자동 대조해 모두 exact 일치를 확인했다. 아직 Spinon runtime·시뮬레이터 검증이나 기능 완료를 주장하지 않는다.
