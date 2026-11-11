# C11.4 mixed automatic minimum 계획 실패 관점 검토 20개

**대상:** [six-fit repeated-text row-span 항목별 `min-height:auto` 계획](./c11-4-root-html-repeated-text-row-span-mixed-auto-minimum.md)과 [Agent Browser Chrome 157 사전 비교](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-mixed-auto-minimum-precomparison-2026-11-12/README.md). 제품 코드는 변경하지 않았다.

| # | 서로 다른 실패 관점 | 계획의 대응과 판정 |
|---:|---|---|
| 1 | CSS Grid §6.6 eligibility와 §11.5 span contribution을 하나의 판정으로 합쳐 자동 최소 크기가 잘못 적용될 수 있다. | item별 computed minimum과 span track 조건을 별개로 비교하고, Chrome 157 결과와 trace를 연결한다. 통과. |
| 2 | `auto`/`0px`의 bit 의미가 `auto`/`zero` 반대로 구현될 수 있다. | metadata에 mask bit 순서를 topology별 A–E/A–D로 고정하고, 장면마다 computed `minHeight`를 확인한다. 통과. |
| 3 | DOM reverse에서 mask bit가 DOM 위치를 따라 이동해 다른 semantic item에 적용될 수 있다. | bit는 source order와 무관한 semantic ID 순서로 정의하고 forward/reverse scene에서 동일 item mask를 검사한다. 통과. |
| 4 | 네 아이템 topology에 다섯 비트 mask를 잘못 적용하거나 E를 가짜로 추가할 수 있다. | A topology는 32 masks, B topology는 A–D 16 masks를 각각 세고 아이템 집합을 exact 검사한다. 통과. |
| 5 | all-zero 또는 all-auto control을 빼서 mixed 변경이 기존 동작을 깨뜨려도 발견하지 못할 수 있다. | 11,520 uniform control scene을 이전 독립 capture와 값 단위로 재비교한다. 통과. |
| 6 | 720 cap permutations 중 일부 누락·중복이나 authored order의 정렬 정규화가 생길 수 있다. | 여섯 cap의 720개 permutation set, computed track list와 raw order를 검사한다. 통과. |
| 7 | direct percentage cap `20%`가 viewport나 intrinsic content 기준으로 해석될 수 있다. | Grid block size를 definite 220px로 고정해 `20%=44px`를 기준에 기록한다. 통과. |
| 8 | 6 implicit row span이 실제 여섯 번째 track을 만들지 않는 배치가 포함될 수 있다. | span2-six 및 span2/span3-six의 positive implicit row 수와 7 used row(첫 explicit 포함)를 검증한다. 통과. |
| 9 | item frame이 covered-row 합과 다른데 payload floor만 보고 통과할 수 있다. | 모든 scene에서 item top/height를 used-row prefix/sum과 독립 비교한다. 통과. |
| 10 | `min-height:0` item의 visible payload overflow를 지원 오류로 오판하거나 자동 최소값 누락을 숨길 수 있다. | payload floor를 auto/zero 집단별로 나누고 Grid-end overflow를 별도 집계한다. 통과. |
| 11 | minimum이 auto인 item만 봐야 하는데 scene 전체를 auto로 간주해 zero item 결함을 놓칠 수 있다. | scene의 `autoItems`와 모든 item computed minimum을 대조하고 floor 위반을 해당 auto item에 한정한다. 통과. |
| 12 | explicit-row control item에 automatic minimum이 적용될 수 있다. | 모든 장면에서 control은 computed `0px`, frame 40×10px로 유지되는지 검증한다. 통과. |
| 13 | align-content stretch의 차이를 auto track stretch로 오해할 수 있다. | 모든 mask와 cap order에서 start/stretch의 used row 및 item geometry를 직접 비교한다. 통과. |
| 14 | DOM source order reversal에서 item frame은 유지되지만 line boxes가 달라질 수 있다. | source-order 비교에 rows, item/payload frames와 모든 line rect를 포함한다. 통과. |
| 15 | CSS text payload/font 변형이 mask 효과와 섞여 기존 admission을 사실상 확장할 수 있다. | 기존 A–E 제한 ASCII, monospace 10px, 10px line-height와 fixed style을 그대로 유지한다. 통과. |
| 16 | 일부 mask에서 모든 auto item은 통과해도 특정 cap/topology 결합에서 track sizing이 잘못될 수 있다. | mask를 720 authored order·두 topology·alignment·source order 전체와 교차해 캡처한다. 통과. |
| 17 | 같은 session에서 media/environment 변경 뒤 이전 mask의 trace나 layout이 섞일 수 있다. | implementation gate에서 owner/style/layout revision 일치, atomic rejection과 same-session recovery를 별도로 요구한다. 통과. |
| 18 | 기존 six-fit auto-placement admission이나 1–5-fit row-span까지 넓어질 수 있다. | 변경을 six-fit row-span의 per-item 0/auto mask predicate에만 적용하고 인접 profile regression을 요구한다. 통과. |
| 19 | 원본 oracle과 verifier가 같은 잘못된 expected geometry를 공유해 비교가 순환 논증이 될 수 있다. | CSS 관측과 verifier를 분리하고 frame/row 관계를 명세 기반 산술로 다시 계산하며 기존 uniform raw capture와 교차한다. 통과. |
| 20 | bounded mixed-mask slice가 전체 C11.4 또는 public CSS 지원 완료로 표시될 수 있다. | C11.4/C11 checkbox, official 78/214 및 contract 0.1.0을 그대로 두고 미지원 경계를 모든 산출물에 적는다. 통과. |

**결론:** 20개 계획 실패 관점을 확인했다. 남은 구현 위험은 Style projection, per-item Grid contribution/rounding, revision ownership, fail-closed recovery 및 RuntimeSession과 Chrome 간 geometry 비교다. 이 계획이나 사전 비교는 제품 구현 또는 적합성 결과가 아니다.
