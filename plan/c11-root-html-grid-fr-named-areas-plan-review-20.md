# named-area + fr 계획 실패 관점 검토 20개

구현 전 검토. 항목은 계획을 실제 코드 변경으로 옮길 때 scope creep, 잘못된 oracle, 거짓 통과가 생기는 별도 경계다.

| # | 적대적 질문 | 판정과 계획 반영 |
|---:|---|---|
| 1 | 기존 named-area px 조합의 수치 조건이 사라지는가? | PASS. 기존 40/80 × 20/50 고정-px 입력을 별도 경로로 보존한다. |
| 2 | area matrix가 tracks보다 크거나 작아도 성공하는가? | PASS. 오직 2×2 matrix와 두 track씩만 받으며 dimension mismatch는 거부한다. |
| 3 | fractional 값이 이름이나 숫자만으로 추정되는가? | PASS. same computed snapshot typed `Fraction` 입력과 layout 결과를 검사한다. |
| 4 | `fr` 단위가 percentage로 해석될 수 있는가? | PASS. percentage tracks/axis와의 조합은 범위 밖 negative control이다. |
| 5 | implicit grid size가 definite라고 오판되는가? | PASS. root `width:120px`와 `height:70px`를 요구하고 기존 axis validator를 재사용한다. |
| 6 | min/max/aspect-ratio가 track denominator를 변경하는가? | PASS. 기존 fr axis gate를 유지하고 세 override 모두 rejection에 포함한다. |
| 7 | Chrome의 양자화된 값이 표준 기대값으로 오인되는가? | PASS. `70/3`, `140/3` 독립 계산과 Chrome `DOMRect` observation을 분리하고 0.02px 허용치를 고정한다. |
| 8 | 값 차이가 Taffy 사후 보정으로 숨겨지는가? | PASS. projection만 좁게 허용하며 geometry divergence 때 구현 중단한다. |
| 9 | named area가 CSS text에서 다시 파싱되는가? | PASS. 기존 Stylo typed area/placement projection만 사용한다. |
| 10 | NodeId나 CSS class가 semantic owner로 고정되는가? | PASS. renamed selector와 source-order reverse control을 수락 조건에 넣는다. |
| 11 | area 이름과 item이 중복되거나 빠져도 통과하는가? | PASS. missing/duplicate/extra item을 rejection matrix로 고정한다. |
| 12 | template의 고유 area 모양 외 임의 영역이 허용되는가? | PASS. 동일 2×2 map만 허용하며 임의 template은 계속 거부한다. |
| 13 | auto placement 또는 `grid-auto-*`까지 넓어지는가? | PASS. 기본 sparse row flow만 유지하고 unsupported placement/flow를 negative로 둔다. |
| 14 | `minmax()`, `auto`, repeat가 indirect track sizing으로 유입되는가? | PASS. 직접 `[1fr,2fr]` 외 syntax를 거부한다. |
| 15 | gap이나 box edges가 free-space 식을 바꾸는가? | PASS. 신규 oracle은 zero gap/edges를 사용하고 기존 nonzero fixed gap 경계는 그대로 둔다. |
| 16 | browser screenshot만으로 geometry를 주장하는가? | PASS. typed track, root/item frames, revisions, layout state가 primary evidence다. |
| 17 | simulator draw/present를 pixel equality로 과장하는가? | PASS. readback이 없으면 screenshot/draw evidence로만 설명한다. |
| 18 | rejection 후 이전 scene이나 partial scene이 재사용되는가? | PASS. 실패 layout에서 completed scene 부재를 검사한다. |
| 19 | 새 profile/ABI가 fixture를 제품 path에 고정하는가? | PASS. default parsed route의 existing typed flow만 쓰고 fixture-only symbol/ID를 추가하지 않는다. |
| 20 | 제한 slice가 공식 C11 완료나 WPT 적합성으로 잘못 승격되는가? | PASS. plan과 acceptance criteria에서 공식 checkbox/72 count 유지와 제외 범위를 명시한다. |

검토 결과: 구현 가능. 표준 계산과 browser observation이 불일치하거나 existing px named-area regression이 깨지면 PR 범위를 넓히지 않고 중단한다.
