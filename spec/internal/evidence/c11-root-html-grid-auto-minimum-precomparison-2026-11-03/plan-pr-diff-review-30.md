# C11.1 Root HTML Grid auto minimum 계획 PR diff 적대 검토 30개

검토 대상은 해당 계획 PR의 실변경 diff다: [C11 plan index](../../../../plan/c11-css-grid.md), [bounded minmax plan follow-up](../../../../plan/c11-root-html-grid-minmax.md), [feature plan](../../../../plan/c11-root-html-grid-auto-minimum.md), [plan review](../../../../plan/c11-root-html-grid-auto-minimum-plan-review-20.md), [fixture](../../../../tests/fixtures/css/c11/root-html-grid-auto-minimum-precomparison.html), 이 폴더의 README/JSON/PNG/review/hash 자료. 계획 검토 20개와 별도이며 구현/runtime 검토에 재사용하지 않는다.

| # | PR diff 실패 관점 | 판정 |
|---:|---|---|
| 1 | C11 색인에 아직 구현되지 않은 기능을 완료로 표기한다 | **PASS** — “계획 후보”, 구현 전, 상위 미완료를 명시한다. |
| 2 | C11.1 bounded `minmax()` 문서의 이번 scope를 묵시적으로 확장한다 | **PASS** — 이전 계획에 별도 follow-up를 연결하고 해당 범위 밖이라고 쓴다. |
| 3 | 내부 계약 버전이나 public status를 무단 갱신한다 | **PASS** — 계약/STATUS 파일을 변경하지 않고 72/213을 보존한다. |
| 4 | plan 제목·링크 ID가 기존 C11 문서 구조와 충돌한다 | **PASS** — 파일명과 C11.1 section naming을 고유하게 사용한다. |
| 5 | 부모 문서의 상대 링크가 실제 경로를 가리키지 않는다 | **PASS** — 관련 plan, 0059, fixture와 evidence 대상을 모두 확인했다. |
| 6 | CSS 규범 링크가 최신 초안이 아니라 계획에서 고정한 baseline과 다르다 | **PASS** — versioned 2025-03-26 CRD URL을 명시한다. |
| 7 | flex sizing의 zero-factor 경계가 규범 인용에서 빠진다 | **PASS** — §11.7과 expected JSON의 zero-factor 항목을 추가했다. |
| 8 | expected value를 Chrome computed output에서 역산한다 | **PASS** — expected와 browser observation JSON을 분리하고 산출 근거를 설명한다. |
| 9 | Chrome 결과를 테스트 기준 구현 또는 전체 conformance로 과장한다 | **PASS** — 독립 cross-check 및 고정 입력 한계라고 명시한다. |
| 10 | Chrome 버전·viewport·DPR을 재현할 수 없다 | **PASS** — observation에 HeadlessChrome 156, 1280×633, DPR 1이 있다. |
| 11 | browser observation이 fixture hash와 연결되지 않는다 | **PASS** — observation에 fixture 경로가 있고 별도 hash manifest가 fixture와 observation을 고정한다. |
| 12 | screenshot이 빠져 overflow 시각 근거가 링크에서 끊긴다 | **PASS** — README와 plan에 PNG 경로를 연결하고 PNG가 존재한다. |
| 13 | screenshot을 pixel equality 증거로 오해한다 | **PASS** — screenshot을 시각 확인으로 분류하고 native pixel evidence 요구를 별도 둔다. |
| 14 | fixture 외부 style이나 JS가 네 장면의 결과를 만든다 | **PASS** — 정적 self-contained HTML/CSS이며 script/외부 asset 의존이 없다. |
| 15 | 100px content axis와 104px outer border box를 혼동한다 | **PASS** — box sizing, 2px border와 content-axis 값을 별도 기록한다. |
| 16 | 40px neighbor 또는 0 gap이 결과에 반영되지 않는다 | **PASS** — 두 track, zero gap, fixed neighbor가 expected 및 fixture 양쪽에 있다. |
| 17 | rows scene이 fixed height 대신 width basis를 사용한다 | **PASS** — width/height 및 track direction별 geometry를 독립 기록한다. |
| 18 | `0px` control을 `auto` minimum control로 오해한다 | **PASS** — track list와 `[60,40]` expected를 명시적으로 구별한다. |
| 19 | item authored width/height와 track size를 동일시한다 | **PASS** — item frame과 used track size를 각각 보관한다. |
| 20 | root border 밖 overflow의 부호/측정 origin이 불명확하다 | **PASS** — border-origin frame, right/bottom overflow 길이를 구체적으로 적는다. |
| 21 | DOM ID/class가 fixture profile selector로 제품 경로에 새어든다 | **PASS** — 계획에서 ID/class 변경 positive control과 selector 금지를 명시한다. |
| 22 | source tree의 `minmax()` converter는 맞지만 default route가 그대로 닫혀 있다 | **PASS** — validator, projection, implicit rows와 default Root HTML route를 구현 조건으로 분리한다. |
| 23 | C11.4 fixture-only 자동 minimum을 다시 product route evidence로 주장한다 | **PASS** — 이번 경로를 Root HTML default route로 지정하고 C11.4를 reference로만 둔다. |
| 24 | 지원 범위 문구가 calc, intrinsic breadth, auto-repeat까지 열린다 | **PASS** — 각 입력을 별도 제외하고 exact shape만 admission한다. |
| 25 | invalid CSS와 valid-but-unsupported typed value 오류가 혼합된다 | **PASS** — CSS invalid fallback과 product unsupported diagnostic을 나눠 검사한다. |
| 26 | negative case에서 partial frame의 부재가 acceptance에서 빠진다 | **PASS** — no-partial-output 조건을 explicit criterion으로 둔다. |
| 27 | simulator draw/present를 post-compositor native screenshot으로 오해한다 | **PASS** — PixelCopy/Metal sampling과 그 한계를 명확히 나눈다. |
| 28 | 아직 확보하지 않은 Android/iOS evidence를 기존 증거처럼 표기한다 | **PASS** — 항목은 future implementation acceptance로 적고 현재 evidence로 주장하지 않는다. |
| 29 | 계획 20개 검토와 PR diff 30개 검토를 중복 카운트한다 | **PASS** — 별도 파일·대상·용도임을 양쪽 문서에서 구별한다. |
| 30 | 계획 PR merge 후 로드맵 문서와 Tailnet preview 갱신이 누락된다 | **PASS** — 구현/merge 후 문서 sync와 served verification을 수락 기준에 둔다. |

수정: 계획 검토 중 발견한 `0fr` 기대값 누락을 expected manifest에 채웠다. 수정 후 final diff에서 30개 관점을 다시 확인했고 계획 차단 사항은 남아 있지 않다. 이 기록은 검토 내역이며 별도 에이전트의 승인을 뜻하지 않는다.
