# C15 zero-width long-word 계획 보완 검토 20개

**검토 대상:** [C15 계획](https://github.com/ohah/spinon/blob/main/plan/c15-inline-text-layout.md)의 기존 `long-word` case에 추가한 0px definite content-width 경계<br>
**근거:** [Chrome DOM 관찰](./c15-inline-text-precomparison-2026-10-17/zero-width-long-word-observation.md)<br>
**검토 시점:** 구현 중 0px 경계를 발견하고 Chrome oracle를 확인한 뒤, 그 조건을 기존 long-word overflow case와 계획에 반영할 때<br>
**판정:** 기존 10개 fixture ID는 유지한다. 0px는 새 CSS profile이 아니라 기존 unbreakable-word overflow 검증의 추가 폭 조건이다.

| # | 독립 실패 관점 | 확인과 판정 |
| ---: | --- | --- |
| 1 | 0px를 indefinite width와 혼동해 거부한다 | Chrome DOM 관찰은 definite parent width 0px를 반환했다. 0은 유효한 입력으로 둔다. 통과. |
| 2 | 원본 `long-word` fixture의 고정 15px 폭을 바꾼다 | HTML fixture hash는 기존 `efba6981…daf64` 그대로다. 브라우저에서 live DOM만 수정했다. 통과. |
| 3 | 화면 label 문자를 text oracle에 포함한다 | Range는 element 첫 text node `XX`에만 만들었다. 결과 `20×10px`다. 통과. |
| 4 | font fallback 결과를 Ahem 수치로 오인한다 | 캡처 전 `document.fonts.check("10px Ahem")`가 true였다. 통과. |
| 5 | parent rect와 text rect를 같은 값으로 복사한다 | Chrome 결과는 parent `0×10px`, text Range `20×10px`로 각각 기록됐다. 통과. |
| 6 | viewport 좌표를 CSS 폭 판정에 섞는다 | 위치는 기록하되 판정은 각 rect의 폭·높이만 사용한다. 통과. |
| 7 | device pixel과 CSS pixel을 혼동한다 | viewport DPR 1과 CSS px 단위를 JSON에 기록했다. 통과. |
| 8 | 0px 지원이 음수 폭까지 받아들인다 | `layout_paragraph`는 음수를 `InvalidWidth`로 거부하고 0만 허용한다. 경계 분리. 통과. |
| 9 | NaN이 비교 연산으로 통과한다 | `is_finite` 선검증이 NaN을 거부하며 focused test가 이를 확인한다. 통과. |
| 10 | 무한대가 Taffy나 Parley로 전달된다 | finite 검사에서 양·음 무한대를 거부한다. 양의 무한대 focused negative control 통과. |
| 11 | 음의 0 비트가 width cache key를 갈라놓는다 | text engine과 Taffy adapter에서 signed zero를 양의 0으로 정규화한다. 테스트에서 snapshot width bit를 비교한다. 통과. |
| 12 | unbreakable word를 parent 폭에 맞게 자른다 | Chrome은 0px parent에서도 Ahem `XX`를 한 줄 `20×10px`로 유지한다. 구현은 snapshot advance/fragment를 parent 폭으로 clamp하지 않는다. 통과. |
| 13 | 0px에서 line box 높이를 0으로 만든다 | Chrome의 parent height와 text line height는 모두 10px다. Ahem line-height 조건을 유지한다. 통과. |
| 14 | text Range의 위치 offset을 overflow 폭으로 오인한다 | Range x는 parent x와 같고, 판정은 `width=20px`로 한다. 통과. |
| 15 | `word-break:break-all` 0px 동작을 같은 기준으로 일반화한다 | 이 추가 조건은 normal word-break `X` 한 글자다. break-all 0px 행은 새 요구로 주장하지 않는다. 통과. |
| 16 | `white-space:nowrap` 0px 동작까지 자동으로 확장한다 | `nowrap`의 기존 30px case는 유지하고, 0px nowrap은 현재 판정 밖이라고 표시한다. 통과. |
| 17 | 공백 collapse 또는 hanging-space 결과를 측정한다 | 입력은 공백 없는 `X`/`XX`이므로 whitespace 판정을 섞지 않는다. 통과. |
| 18 | DOM geometry를 ink pixel 또는 clip 결과라고 주장한다 | screenshot을 만들지 않았고, evidence는 명시적으로 DOM geometry 전용이다. 통과. |
| 19 | 새 폭 조건 때문에 fixture ID 개수와 전체 C15 상태를 올린다 | `long-word`의 추가 subcase로 기록해 10개 ID를 유지하며 C15 parent는 미완료다. 통과. |
| 20 | simulator/runtime 화면이 0px 경계를 확인했다고 말한다 | Chrome oracle만 기록한다. Spinon runtime, Android/iOS Simulator, paint·clip은 미검증으로 남긴다. 통과. |

이 검토는 0px oracle와 계획 범위만 다룬다. 구현 실패 관점 20개 및 PR diff 관점 검토를 대신하지 않는다.
