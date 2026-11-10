# C11.4 three-fit auto-placement 구현·RuntimeSession 적대 검토 20개

검토 대상은 현재 변경의 style projection, layout admission, RuntimeSession matrix/trace/rejection tests, app fixture와 simulator evidence다. 계획 PR의 별도 20/30관점 표를 재사용하지 않았다.

| # | 실패 관점 | 코드·실행 근거 | 판정 |
|---:|---|---|---|
| 1 | 3-fit 외 track 수가 신규 predicate에 섞이는가 | Style projection과 layout helper가 길이 6을 먼저 검사하며 0/1/2/4-fit mutation을 거부한다. | 통과 |
| 2 | tail multiset이 다른 list가 통과하는가 | 정확히 min/max/auto 각 1개를 요구하고 duplicate min-content runtime case가 거부된다. | 통과 |
| 3 | 일부 fit-position triple이 누락되는가 | 두 독립 helper test가 20 triple × 6 tail order = 120 조합을 전부 센다. | 통과 |
| 4 | supported tail order가 위치에 따라 달라지는가 | 모든 tail permutation을 positive control로 검사하며 runtime Chrome corpus의 720 list도 확인한다. | 통과 |
| 5 | fit cap 위치나 형식이 다른데 허용되는가 | typed/computed list 순서를 matrix에서 비교하고 direct px 및 percentage의 unlisted 값도 실행한다. | 통과 |
| 6 | negative, NaN, infinity, `calc()` cap이 새 gate에 들어오는가 | projection/layout helper negatives와 runtime negative px/calc cases가 닫힌 상태를 확인한다. | 통과 |
| 7 | exact 6-member 조건이 5 또는 7개에서 풀리는가 | projection/layout unit tests와 RuntimeSession rejection이 양쪽 길이를 거부한다. | 통과 |
| 8 | 기존 one-fit/two-fit auto-placement가 새 helper로 오염되는가 | 각각 zero-minimum/candidate 1,440·2,880 scene matrix가 통과했고 별도 helper가 유지된다. | 통과 |
| 9 | three-fit row-span이 auto-placement gate의 경계와 합쳐지는가 | span predicate와 auto-placement predicate는 분리되어 있고 11,520 scene span matrix가 통과한다. | 통과 |
| 10 | 빈 explicit-row control이 automatic minimum을 받는가 | 모든 Chrome scene 및 rejection control에서 item-0 `min-height:0`을 검사하며 `auto` mutation은 거부된다. | 통과 |
| 11 | 일부 repeated item만 automatic minimum인 혼합 상태가 통과하는가 | mixed zero/auto mutation은 RuntimeSession에서 실패한다. | 통과 |
| 12 | text floor가 fit-content cap에 의해 잘리는가 | 2,880 paired keys에서 eligible fit/auto rows의 payload floor를 검사하고 feature trace의 automatic-minimum source/value/base size도 확인한다. | 통과 |
| 13 | min/max-content tail이 automatic-minimum eligible로 잘못 분류되는가 | runtime trace는 `fit-content()`/`auto` track row만 확인하고 eligible node 집합을 정확히 비교한다. | 통과 |
| 14 | media environment 변경이 한 session에 남은 style을 재사용하는가 | 같은 session에서 light→dark→light revision·computed list·alignment를 각각 다시 검사한다. | 통과 |
| 15 | definite percentage basis가 무효화되거나 block size가 indefinite인가 | definite 220px fixture에서 matrix를 돌리고 `height:auto` mutation은 거부한다. 중복된 같은 mutation은 rejection table에서 제거했다. | 통과 |
| 16 | forward/reverse DOM order에서 item/payload mapping이 뒤바뀌는가 | 5,760-scene corpus의 양 order에서 item ID별 frame, payload frame, line/glyph count를 대조한다. | 통과 |
| 17 | 텍스트 child shape/style/direction 검증이 빠지는가 | nested markup, 비지원 `A1`, 11px, RTL mutation이 fail-closed한다. | 통과 |
| 18 | column/dense flow, span, scrollable overflow, unsupported alignment가 새 profile로 들어오는가 | 각각의 독립 RuntimeSession mutation case가 layout failure로 끝난다. | 통과 |
| 19 | 추가 Grid owner 또는 누락 item이 부분 scene을 게시하는가 | 두 번째 inline `display:grid`와 missing item을 거부하며, failure는 completed layout/render를 공개하지 않는다. | 통과 |
| 20 | stale/duplicate trace, 잘못된 recovery 또는 platform smoke가 제품 동작 근거로 과장되는가 | 성공 trace는 한 Grid·같은 completion/cascade key·eligible contributions를 확인한다. 실패 snapshot에는 completed output이 없고 같은 session의 새 revision은 복구된다. iOS/Android 근거는 ready/draw-present simulator smoke와 screenshot으로 한정한다. | 통과 |

## 검토 중 발견 및 수정

- 첫 extra-Grid negative test는 이 fixture에 없는 `</main>`을 찾아 테스트 자체가 실패했다. 이어 붙인 `class="grid"`도 이 fixture에서 grid style을 갖지 않아 Ready로 남았다. 기존 DOM 구조에 실제 inline `display:grid` owner를 추가해 두 번째 visible Grid가 fail-closed함을 실행으로 확인했다.
- 같은 `height:220px`→`height:auto` mutation이 percentage basis와 Grid block-size 이름으로 두 번 들어가 있었다. 중복 case를 제거하고 한 mutation의 의미를 명확히 했다.
- Workspace Clippy가 두 test-only range loop의 `needless_range_loop`를 보고했다. 경고 suppress 대신 `iter_mut().enumerate().take(...)`로 바꾸고 전체 workspace 재실행을 통과시켰다.

미해결 차단 결함은 없다. 이 표는 구현/runtime 20개 관점이며 구현 PR의 실제 diff에 대한 별도 30개 표와 계획 단계 검토를 대신하지 않는다.
