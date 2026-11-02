# C11.1 named-area + `fr` 구현 PR diff 적대 검토 30개

대상은 named-area validator, projection/runtime 테스트, 고정 HTML fixture, 실행 근거와 해당 계획 문서의 구현 결과 갱신이다. 계획 PR의 사전 검토표를 재사용하지 않고 실제 코드·fixture·실행 자료를 각각 다른 실패 경계로 확인했다.

| # | PR diff에서 확인한 실패 경로 | 판정 및 근거 |
|---:|---|---|
| 1 | validator 변경이 Root HTML Grid 전체를 포괄 허용하도록 조건을 제거했는가? | PASS. 기존 고정 track branch와 bounded fraction branch 둘 중 하나만 통과하도록 좁게 추가했다. |
| 2 | 기존 named-area 고정 px baseline의 수치 조건이 바뀌었는가? | PASS. `40px 80px`/`20px 50px` 조건을 보존했다. |
| 3 | fraction이 임의 가중치나 임의 개수로 확대됐는가? | PASS. 각 축의 direct `[1,2]`만 비교하며 projection test도 이 두 value만 positive로 둔다. |
| 4 | 양 축 중 하나만 fraction이어도 area allowlist가 통과하는가? | PASS. fraction branch는 columns와 rows match의 conjunction이다. 기존 rejection matrix도 named areas의 단일 축 fraction을 계속 거부한다. |
| 5 | 새 branch가 existing definite-axis validator 호출 순서를 건너뛰는가? | PASS. 변경은 named-area shape check에 한정되며 공통 root input 검증을 대체하지 않는다. |
| 6 | width min/max 검사가 삭제되거나 fraction branch 바깥으로 빠졌는가? | PASS. shared axis constraints와 runtime min/max negative controls가 남아 있다. |
| 7 | height min/max 검사가 열 축으로만 검사되는가? | PASS. 독립 row-axis gate와 min/max height negative controls가 포함됐다. |
| 8 | aspect-ratio gate를 fraction + areas 조합에서 생략하는가? | PASS. 공통 fraction-axis gate를 재사용하고 runtime override를 거부한다. |
| 9 | area template 이름 또는 alias의 기존 정확성이 달라졌는가? | PASS. `header`, `side`, `content`의 기존 fixed rectangle 검증은 변경되지 않았다. |
| 10 | area matrix의 row/column count가 tracks와 다를 때 일부만 무시되는가? | PASS. 기존 exact 2×2 map/track dimension 검증을 유지한다. |
| 11 | anonymous implicit track 또는 `grid-auto-rows`가 fraction+areas를 통해 유입되는가? | PASS. 기존 named-area implicit-row rejection을 건드리지 않았고 65개 Root HTML 회귀에서 확인했다. |
| 12 | `grid-auto-flow: dense` 또는 다른 flow mode가 새 허용 branch에 딸려 들어가는가? | PASS. 새 코드는 `auto_flow` admission을 바꾸지 않는다. |
| 13 | item placement, named lines, repeat 규칙이 named-area track 허용과 함께 넓어졌는가? | PASS. placement 구현 경계는 수정하지 않았고 C11.3.3 회귀 26/26이 통과했다. |
| 14 | validator가 CSS source text를 다시 읽거나 문자열 형태로 값을 추측하는가? | PASS. existing `ComputedCssGridLayout` typed `Fraction`과 typed areas만 검사한다. |
| 15 | 기대 geometry 차이를 Taffy output 보정으로 숨겼는가? | PASS. 레이아웃 구현은 바꾸지 않았고 independent frame assertions가 결과를 검증한다. |
| 16 | 새 FFI entrypoint, ABI, scene id, public profile이 추가됐는가? | PASS. 수정 파일은 projection allowlist와 Rust tests/HTML fixture/docs뿐이다. |
| 17 | 새 admission이 다른 computed-style profile에도 전역으로 적용되는가? | PASS. 기존 `validate_root_html_named_area_template` 범위에서만 분기하고 다른 profile behavior는 건드리지 않는다. |
| 18 | Rust test include가 실제 이름 있는 fixture 대신 다른 입력을 가리키는가? | PASS. `FRACTION_SOURCE`는 별도 C11 fixture를 `include_str!`로 읽는다. |
| 19 | fixture의 percentage viewport, root sizing 또는 dimensions가 Chrome oracle와 달라졌는가? | PASS. fixture root는 고정 `120×70px`, 두 축 direct `1fr 2fr`, zero gap이다. 기준 파일/해시는 precomparison 자료에 고정했다. |
| 20 | fixture item의 HTML source 순서가 정답 배치와 우연히 일치하는가? | PASS. source order는 content/header/side이고 test가 semantic header/side/content frame을 검사한다. |
| 21 | selector/id rename control이 CSS와 HTML 양쪽을 일관되게 바꾸는가? | PASS. `#layout`/`id=layout` 및 `data-area` selector/attribute를 함께 바꾸고 같은 frames를 확인한다. |
| 22 | row/column 기대값이 서로 바뀌거나 root/child frame extent가 틀렸는가? | PASS. root, header, side, content를 독립 CSS-pixel vector와 비교한다. |
| 23 | tolerance가 넓어 Chrome/타입 차이를 무시하는가? | PASS. 독립 계산 대비 고정 0.02 CSS px이며 Chrome 1/64 CSS px observation을 설명했다. |
| 24 | frame comparison이 NaN/Infinity를 허용하는가? | PASS. 각 component는 tolerance 계산 전에 `is_finite()`로 확인한다. |
| 25 | layout frame과 cascade typed style이 다른 document generation에서 나온 값을 합치는가? | PASS. completed key 일치 및 typed row/column/area metadata를 확인한다. |
| 26 | negative test의 wrong factor/count를 fixture 문자열 교체 실패로 false-pass하는가? | PASS. 기대 error property/code, `Failed` state, `completed=None`을 각각 확인한다. |
| 27 | mixed unit 및 percentage rejection이 실제 Grid track 선언을 바꾸지 못해 false-pass하는가? | PASS. CSS declaration literal을 fixture의 고유 track 문자열에서 교체하며 no-scene을 요구한다. |
| 28 | indefinite axis, min/max, aspect ratio가 error 없이 빈 결과로 끝나는가? | PASS. 결과 state `Failed`, error 존재와 partial completion 부재를 확인한다. |
| 29 | gap controls 또는 `minmax()`의 진단 stage를 잘못 고정해 parser/cascade 변화에 취약한가? | PASS. row/column gap은 현재 computed property를 확인한다. intrinsic `minmax()`는 cascade-stage rejection이므로 error stage 대신 fail-closed/no-scene만 계약한다. |
| 30 | 근거 문서가 simulator 캡처를 pixel equality로 승격하거나 unrelated WIP를 PR에 포함하는가? | PASS. README가 draw/present와 screenshot의 한계를 분리하고 readback을 주장하지 않는다. PR stage 목록은 explicit allowlist로 제한하며 기존 untracked 자료는 추가하지 않는다. |

결과: 30개 관점 모두 PASS. 검토 중 도출된 lint/test assertion 문제는 implementation/runtime review와 같이 수정했다. 최종 코드로 Root HTML 65/65, named-area projection 5/5, C11.3.3 26/26, focused runtime 22/22, scoped Clippy, rustfmt, `git diff --cached --check`, Android AVD와 iOS Simulator rebuild/run을 확인했다. GitHub PR 이미지 URL 표시, Actions/check 및 merge 결과는 PR 생성 뒤 확인한다.
