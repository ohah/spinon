# C11.4 `fit-content()` columns 계획 PR 변경분 적대적 검토 20개

**검토 대상:** 열 축 계획 PR의 fixture, Chrome precomparison, 계획, plan review, parent plan 및 공식 STATUS 변경. 이 검토는 runtime 구현 PR 검토를 대신하지 않는다.

| # | 독립 실패 관점 | 판정 |
| ---: | --- | --- |
| 1 | 계획 PR에 production algorithm 변경이 섞여 아직 검토되지 않은 동작이 들어간다 | **통과.** 변경 파일은 계획·oracle·STATUS와 precomparison HTML/evidence뿐이다. |
| 2 | 새 plan-only slice를 C11.4 완료로 올린다 | **통과.** 계획과 STATUS는 C11.4 unchecked 및 `72/213`을 유지하고 runtime not started로 표시한다. |
| 3 | 기존 row WPT 결과를 열 축 app evidence처럼 재사용한다 | **통과.** row implementation #787과 신규 Chrome columns observation을 분리한다. |
| 4 | Chrome observation을 규범 기대값의 근거로 둔다 | **통과.** 독립 expected formula는 CSS Grid 2025 CRD §11.2에서 도출하고 browser values는 observation으로만 보존한다. |
| 5 | 초기 60px root의 200px limit case를 잘못된 expected 100px의 근거로 쓴다 | **통과.** 초기 관찰은 README에 입력 결함으로 기록하고 final JSON은 300px definite root를 사용한다. |
| 6 | `min-width:0` main scene이 automatic minimum을 시험한다고 오해된다 | **통과.** main contribution과 `min-width:auto` floor control을 따로 분류한다. |
| 7 | WPT floor control의 100px child가 min/max contribution을 구분하지 못한다 | **통과.** 계획은 raw growth-limit trace와 used base를 분리해 검사한다. |
| 8 | `grid-template-columns:none`이 implicit column 생성을 막는다 | **통과.** `grid-auto-flow:column`이 single item으로 필요한 implicit column을 생성하는 CSS를 Chrome에서 관측했다. |
| 9 | explicit row와 child height가 열 계산에서 추가 row sizing을 만든다 | **통과.** 60px explicit row와 10px child height를 고정하며 별도 축 trace를 요구한다. |
| 10 | screenshot의 section order/root offsets가 정규 geometry로 착각된다 | **통과.** expected JSON은 root offsets와 used track/frame을 separate fields로 저장한다. |
| 11 | `--limit` custom property가 `fit-content()`의 computed argument가 아니라 임의 runtime value가 된다 | **통과.** CSS custom property는 authored fixture의 값 전달에만 쓰고 computed `gridTemplateColumns`와 DOMRect를 별도 확인한다. |
| 12 | screenshot 또는 viewport가 clipping된 scene을 감춘다 | **통과.** 200px scene의 root는 x=168, width=300으로 viewport 안에 있고 overflow samples는 이후 simulator gate에 명시됐다. |
| 13 | fixture HTML 변경 뒤 source hash가 낡는다 | **통과.** expected JSON의 fixture hash를 현재 파일과 대조했다. |
| 14 | screenshot bytes가 expected JSON과 달라 image link가 잘못된다 | **통과.** screenshot hash와 1280×633 PNG artifact를 기록했다. |
| 15 | browser 실행 정보가 재현에 부족하거나 Browser가 열린 채 남는다 | **통과.** Agent Browser 0.31.1, HeadlessChrome 156, DPR 1, viewport, capture와 종료 확인을 README에 기록했다. |
| 16 | plan review를 implementation/runtime 검토로 합산한다 | **통과.** 계획 검토 파일은 pre-implementation gate이며 계획 PR diff 검토와 구현 review를 분리한다. |
| 17 | 일반 CSS allowlist나 public API가 계획 PR에서 바뀐 것으로 표현된다 | **통과.** 기존 fixture-test route만 대상으로 하며 신규 product CSS/API는 계획 범위 밖이다. |
| 18 | 경로 또는 링크가 Tailnet/RSPress publication에서 사용할 수 없는 외부 absolute path에 묶인다 | **통과.** 새 계획/evidence 링크는 repo-relative이고 fixture 경로는 저장소의 기존 CSS fixture tree다. |
| 19 | 산출물 범위를 Android hardware GPU나 iOS compositor-after pixels로 확대한다 | **통과.** 계획은 API 37 AVD/Simulator와 pre-compositor readback의 한계를 명시한다. |
| 20 | 계획 PR diff 적대 리뷰가 사용자 요청과 프로젝트 경계를 섞는다 | **통과.** 계획 PR은 별도 20관점이며 구현 PR의 신규 runtime 20관점과 사용자 요청 20관점 diff 검토는 구현 단계에서 수행한다. |

**판정:** 계획 PR 변경분 20/20 통과. 본 문서는 계획·oracle PR diff 전용이며, 미실행 Android/iOS runtime 또는 제품 구현을 증명하지 않는다.
