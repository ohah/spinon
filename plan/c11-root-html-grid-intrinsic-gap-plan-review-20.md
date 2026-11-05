# C11.1 intrinsic Grid + fixed gap 계획 실패 관점 검토 20개

대상은 [계획](./c11-root-html-grid-intrinsic-gap.md)과 구현 전 Chrome/독립 비교 자료다. 코드는 아직 바꾸지 않았으며, 다음 항목은 구현 후 검토에서 재사용하지 않는다.

| # | 독립 실패 경로 | 계획의 판정 기준 |
|---:|---|---|
| 1 | Grid gutter를 track base에 더해 min-content contribution을 키움 | fixed 40px/10px contribution은 유지하고 gutter를 별도 track 사이 공간으로 계산한다. |
| 2 | `1fr` free space를 gap 차감 전에 계산 | definite 축에서 gutter 총합을 먼저 빼고 나머지를 flexible track에 배분한다. |
| 3 | row-gap과 column-gap mapping이 서로 바뀜 | 축별 단독 positive scene을 각각 둔다. |
| 4 | `gap` shorthand만 다르게 project됨 | 양축 10px shorthand를 두 longhand positive scene과 같은 결과로 검증한다. |
| 5 | 기존 no-gap behavior가 새 mode에서 바뀜 | 0-gap control과 #1167 회귀를 유지한다. |
| 6 | computed `normal` gap이 0과 다르게 처리됨 | gap 미선언 control의 Chrome `normal`을 used 0px와 구분한다. |
| 7 | gap을 content box 대신 border box 기준으로 사용 | root edges 0을 고정하고 definite 100×100 content box로 independently derive한다. |
| 8 | `%` gap을 fixed px로 조용히 축약 | Chrome valid `%` observation은 따로 보존하며 candidate는 typed gap 종류를 검사해 거부한다. |
| 9 | `calc()` gap이 단순 `LengthPx` 허용을 우회 | `LayoutLengthPercentage` 변형 단위에서 `LengthPx`만 허용하며 calc negative를 둔다. |
| 10 | negative/NaN/infinite typed gap이 input mutation으로 들어옴 | low-level layout tests에서 값 경계를 각각 직접 검사한다. |
| 11 | CSS-invalid negative author declaration을 runtime error로 오분류 | cascade fallback 검증과 typed runtime rejection을 분리한다. |
| 12 | implicit row가 만들어져 gap 합이 불완전하게 계산 | `grid-auto-rows` non-auto control을 property 귀속 진단으로 atomic reject한다. |
| 13 | track span에서 span 내부 gutter 수가 빠짐 | 이 fixed two-track fixture는 unspanned만 허용하며 existing span path는 regression으로 둔다. |
| 14 | `align-content:normal` auto stretch가 gap 효과와 섞임 | start alignment를 고정하고 auto max track은 여기서 열지 않는다. |
| 15 | item fixed size가 intrinsic track base를 덮음 | fixed input과 min-content base를 separately verify하고 no intrinsic text claims를 한다. |
| 16 | 한 Grid error가 다른 Grid의 frame/scene을 partial publish | 여러 owner atomic failure control을 fixture matrix에 추가한다. |
| 17 | stale document/style/layout revision에서 gap이 섞임 | 한 source revision을 묶고 revision mutation을 reject control로 둔다. |
| 18 | gap change가 unrelated row/column placement order를 바꿈 | explicit numeric placement로 이 slice의 auto-placement 영향을 제거하고 기존 profiles를 회귀한다. |
| 19 | screenshot을 surface pixel proof로 과장 | screenshot과 PixelCopy/Metal surface readback을 별도로 기록한다. |
| 20 | 제한 profile을 public CSS/C11 완료로 올림 | 상태·공개 roadmap·내부 계약에 fixed-px subset과 남은 범위를 명시하고 상위 check/집계를 유지한다. |

**판정:** 20개의 서로 다른 plan failure path를 정의했다. Positive oracle은 네 fixed-gap scene이고 `10%`는 관찰은 하되 의도적으로 지원하지 않는 CSS-valid control이다. 계획 단계에서는 구현을 시작하지 않는다.
