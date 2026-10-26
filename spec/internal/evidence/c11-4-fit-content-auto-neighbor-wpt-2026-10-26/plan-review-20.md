# C11.4 `fit-content()` 옆 `auto` implicit row 계획 검토 20관점

**검토 대상:** `plan/c11-4-fit-content-auto-neighbor-wpt.md`와 연결된 precomparison artifact.
**검토 기준:** base `d48d31698842866241dcb3317c6f8ff1ee068d1c`; WPT revision `c999c58338ee1d223df5ad62e48be1202ced0d37`; CSS Grid Level 2 2025 CRD.
**결과:** 20 PASS / 0 PARTIAL / 0 FAIL. 이것은 계획 자체 검토이며 implementation review나 PR diff review가 아니다.

| # | 독립 실패 관점 | 확인 결과 | 판정 |
|---:|---|---|---|
| 1 | WPT 파일 revision 변경 또는 floating URL | commit과 SHA-256을 고정하고 upstream URL도 commit permalink를 사용한다. | PASS |
| 2 | 인접 assertion을 하나의 slice로 잘못 묶음 | `autoRowAndFitContentRow` lines 130–132 한 assertion만 명시하고 preceding `autoRow`를 제외한다. | PASS |
| 3 | explicit rows로 implicit-row 동작을 대체 | 계획은 explicit `rows=[]`, `auto_rows=TrackList(...)`로 실제 implicit row를 생성한다. | PASS |
| 4 | auto-placement 순서가 두 tracks를 뒤집음 | 두 direct child를 row auto-flow 순서대로 놓고 첫 implicit track pattern order를 보존하도록 입력을 고정한다. | PASS |
| 5 | track-list pattern 반복 의미가 누락됨 | 두 implicit rows가 `[fit-content(40px), auto]` authored order를 사용해야 함을 구현 관문에 적는다. | PASS |
| 6 | 300px가 definite content height가 아님 | root height 300px, zero border/padding 조건으로 definite inner height를 만든다. | PASS |
| 7 | row gap이 free-space 산식에 누락됨 | row-gap은 0px로 고정한다. | PASS |
| 8 | content box와 border box 혼동 | border/padding 0으로 두 box size가 같게 정한다. | PASS |
| 9 | fit-content argument를 growth limit과 동일시 | max-content contribution 10px를 40px argument로 clamp해 10px을 얻는다. | PASS |
| 10 | fit-content automatic minimum floor 누락 | min track function과 10px nested content의 auto minimum 기여를 별도 설명한다. | PASS |
| 11 | fit-content max sizing function을 auto로 오인 | §12.2 및 §12.8 기준으로 stretch eligibility에서 제외한다. | PASS |
| 12 | auto row가 stretch 대상에서 빠짐 | 두 번째 track의 max function은 `auto`; normal alignment에서 대상 index `[1]`로 명시한다. | PASS |
| 13 | default align-content를 stretch와 혼동 | WPT의 computed `normal`을 유지하고 plan input도 `LayoutAlignContent::Normal`로 명시한다. | PASS |
| 14 | positive definite free space를 잘못 계산 | `300 - 10 - 10 - 0 = 280px`로 독립 산술을 기록한다. | PASS |
| 15 | 두 eligible track에 공간을 나눔 | eligible index 하나만 존재하므로 두 번째 track 전체에 280px을 더한다. | PASS |
| 16 | contribution child와 stretched grid item을 혼동 | direct wrapper는 auto height, nested content만 10px fixed로 계획한다. | PASS |
| 17 | stage trace에 최종 frame만 검증 | pre/post row bases, stage order, `applied:true`, frames를 각각 요구한다. | PASS |
| 18 | gutter/sentinel을 CSS track으로 세어 개수가 달라짐 | gutter 및 collapsed track 필터와 별도 flag 확인을 명시한다. | PASS |
| 19 | Chrome 비교를 구현의 oracle로 역산 | normative derivation과 Chrome 관측을 분리하고 candidate Taffy 결과를 source로 삼지 않는다. | PASS |
| 20 | file URL CSSOM 관측을 WPT PASS로 과장 | testharness resource request 실패와 WPT 미실행 경계를 보고서에 명시한다. | PASS |

검토 중 수정이 필요했던 차단 결함은 없었다. PR diff 검토는 이 표와 독립된 30관점 보고서에서 수행한다.

## §6.6 automatic minimum 근거 재확인

기존 view #10은 automatic minimum floor를 확인했지만, PR review 중 계획 문구가 조건을 정확히 적지 않았음을 발견해 이 관점을 보완했다. CSS Grid Level 2 2025 CRD의 [§6.6](https://www.w3.org/TR/2025/CRD-css-grid-2-20250326/#min-size-auto)은 computed overflow가 non-scrollable이고 item이 해당 축에서 `auto` min track sizing function을 가진 track 하나 이상을 span할 때 content-based automatic minimum을 적용한다. [§12.2](https://www.w3.org/TR/2025/CRD-css-grid-2-20250326/#algo-terms)는 `fit-content()`의 min track sizing function이 `auto`, max function이 `max-content`라고 정한다. 고정 WPT source의 두 direct wrapper는 기본 `overflow:visible`이고 각자 한 implicit row만 span한다. 각 nested child의 fixed block-size가 10px이며 wrapper에 지정·전달 크기 제안은 없으므로 content size suggestion 및 minimum contribution은 10px이다. 두 track은 §11.4–§11.5에 따라 base 10px에서 시작한다.

이 재확인으로 view #10은 PASS다. 대상은 독립적인 규범 기대값과 입력 조건이며 Chrome 관측이나 Spinon/Taffy 출력으로부터 유도하지 않았다. 나머지 계획 관점 판정은 바뀌지 않는다.
