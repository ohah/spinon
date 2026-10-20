# C11.4 음수 implicit track 계획 적대 검토 20개

**검토 대상:** `plan/c11-4-negative-implicit-track-sizing.md`, 2026-10-20 draft.

**근거:** 현재 main의 `grid_auto_placement/axis.rs`, `dense_column.rs`, `GRID_MAX_TRACKS`; CSS Grid Layout §7.1, §7.5–§7.6, §8.5; Chrome 155의 33-case observation과 분리 작성한 W3C 기대값. 기능 구현 및 그 PR diff 검토에서는 이 계획 검토표를 재사용하지 않는다.

| # | 독립 실패 관점 | 적대 질문과 확인한 근거 | 결과 |
|---:|---|---|---|
| 1 | 음수 line 기준점 | `-1`이 이미 확장된 grid 끝을 가리켜 기존 line offset이 바뀌지 않는가? §7.1은 explicit grid 끝을 기준으로 한다. 계획은 `N + 1 + n`으로 고정한다. | 통과: expanded origin과 negative line resolution 분리 |
| 2 | positive/negative line 좌표 | 첫 explicit boundary가 0이고 track interval이 `[q,q+1)`인가? `N=2`에서 `-1 → 2`, `-2 → 1`, `-3 → 0`, `-4 → -1`이 된다. | 통과: 경계와 track 좌표를 정의 |
| 3 | explicit track 없음 | `grid-template-columns:none`은 line도 없는 것으로 처리하지 않는가? §7.1의 line 하나와 fixture의 2-leading/2-trailing case를 대조했다. | 통과: 경계 0에서 양쪽 implicit extents |
| 4 | 음수 방향 list cycling | 한·두·세·네 leading tracks가 `[31,47]`의 reverse cycle을 따르는가? W3C의 §7.6 backward rule과 4개 Chrome case의 computed list를 확인했다. | 통과: explicit 경계부터 역순 반복 |
| 5 | 양수 방향 cycle 회귀 | origin 이동으로 trailing pattern이 기존 index가 아닌 새 시작점부터 반복되지 않는가? 양·음 cycle fixture와 positive baseline formula를 비교했다. | 통과: trailing index는 explicit count 기준 |
| 6 | 빈 목록과 0px | modulo 0이나 zero-sized track 제거가 발생하는가? list는 nonempty로 guard하고 zero-size member를 유효 track으로 보존하며 Chrome oracle에 0px control을 둔다. | 통과: 빈 목록 fail-closed, 0px 보존 |
| 7 | typed sizing | min-content/max-content/minmax가 leading side에서 adjacent-track 역순을 따르는가? Ahem을 로드한 3개 intrinsic case와 computed values를 확인했다. | 통과: 기존 typed converter 재사용 |
| 8 | CSS line zero | signed 변환이 `0`을 허용하는가? §8.3에서 numeric line zero는 invalid이고 계획은 이를 명시하며 별도 negative control을 요구한다. | 통과: zero는 기존 오류 유지 |
| 9 | reversed/equal endpoints | 양수·음수 endpoint를 expanded shift 전/후 잘못 비교하지 않는가? 계획은 CSS boundary로 정규화하고 reversed/equal tests를 요구한다. | 통과: 정규화 순서 고정 |
| 10 | span과 signed overflow | negative definite endpoint에서 `span`을 빼며 signed overflow나 잘못된 `usize` 변환이 생기는가? checked arithmetic, `unsigned_abs`, extreme line/span controls를 지정했다. | 통과: overflow 전에 구체적 오류 |
| 11 | explicit 경계 span | 한 item의 양 끝이 leading track과 explicit track을 걸칠 때 track count가 한 칸 어긋나는가? Chrome 경계-span case와 동일한 interval model을 요구한다. | 통과: 축 extent는 half-open interval로 통일 |
| 12 | 두 축 혼합 | columns와 rows에서 서로 다른 leading count/list가 하나의 shift로 섞이는가? 2축 독립 패턴 fixture와 축별 signed min/max를 대조했다. | 통과: 각 축의 origin/순환 독립 |
| 13 | hidden item extent | `display:none` 아이템의 매우 작은 line이 grid를 확장하는가? hidden case는 visible track list와 hidden display를 별도 assertion한다. | 통과: hidden item은 placement 입력에서 제외 |
| 14 | sparse row cursor | leading negative column이 생겼는데 row-flow cursor가 0부터 시작해 첫 칸을 놓치는가? W3C §8.5 start-most cursor와 paired sparse observation을 사용한다. | 통과: 실제 implicit-grid 시작에서 시작 |
| 15 | dense cursor rewind | dense row가 앞선 locked items 뒤에 남은 hole을 건너뛰는가? 동일 placement의 sparse/dense Chrome pair에서 definite item과 마지막 auto item의 y가 달라지는 것을 확인했다. | 통과: dense는 start-most부터 재탐색 |
| 16 | column sparse/dense 대칭 | 구현이 row만 signed로 만들고 column-flow 또는 dense column을 양수로 간주하지 않는가? columns/rows 양축에서 column sparse·dense case를 요구한다. | 통과: 4개 flow mode 모두 계획에 포함 |
| 17 | definite placement 단계 | auto-placement 전에 negative definite extent를 모두 만들지 않아 cursor가 나중에 알게 된 leading track을 놓치는가? §8.5 definite extent 단계와 signed occupancy 조건을 계획에 고정했다. | 통과: definite extent/occupancy 선반영 |
| 18 | 10,000 track 제한 | leading + explicit + trailing을 각각 따로 제한해 총 10,000 초과가 허용되는가? 상한을 allocation 전에 total extent에 적용하고 9,999/10,000/10,001을 요구한다. | 통과: total cap 및 early reject |
| 19 | 할당 실패 원자성 | 새 expanded vector 중간 실패 뒤 partial Taffy layout이 나오거나 일반 allocator 복구를 과장하는가? fallible reservation/no-partial-output을 요구하고 Taffy internal OOM 한계를 그대로 명시한다. | 통과: scope와 제한 모두 표시 |
| 20 | profile·auto-repeat·플랫폼 경계 | 변경이 C11.2.2/2.3이나 auto-repeat 경로에 새 지원을 우회 도입하거나 surface 검증을 geometry로 대체하는가? profile/error controls, explicit rejection, 106-scene 양 simulator gate, border 안·stroke·밖 PixelCopy/Metal을 요구한다. | 통과: fail-closed와 OS surface gate 유지 |

## 검토 결과와 수정

검토에서 세 가지 계획 누락을 구현 전에 닫았다.

1. line 0이 signed transform 중 허용될 위험을 확인해 명시적으로 invalid로 고정하고 검증 항목에 추가했다.
2. `-q` 계산의 signed minimum overflow 위험을 확인해 list index 계산을 unsigned magnitude로 정의했다.
3. sparse/dense가 동일 위치를 내는 일반 case만 있으면 dense cursor 경계를 판정할 수 없음을 확인했다. negative column + definite-column locked items + hole을 사용하는 paired sparse/dense case를 추가해 Chrome 관찰과 독립 기대값에 고정했다.

또한 Ahem URL이 한 단계 잘못돼 fallback font를 읽던 사전 fixture 결함은 수정했다. 재캡처에서 `document.fonts.check('12px Ahem') == true`와 33/33 결과를 확인했다. 20개 관점에서 미해결 설계 차이는 없다. 계획 완료는 아직 기능 완료나 C11.4 완료를 의미하지 않는다.
