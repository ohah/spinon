# C11.1 Root HTML Grid auto-repeat 계획 실패 관점 검토 20개

이 검토는 일반 CSS Grid 전체가 아니라 현재 Root HTML default-route 후보의 경계와 완료 판정에만 적용한다. 2026-11-05 구현을 닫기 전 계획·사전 비교·candidate diff를 다시 읽으며 서로 다른 실패 경로 20개를 확인했다. `normative-expected.json`은 CSS Grid 규칙에서 독립 산술로 만들었고 Chrome 156 비교가 13/13 통과한다. 검토 중 발견한 행 반복 native smoke 공백은 기본 앱 fixture에 row scene을 추가해 보완했다.

| # | 공격 질문 | 확인 결과 |
|---:|---|---|
| 1 | 기존 C11.4 `TrackSizing`이 Root HTML로 새어 들어오는가? | 별도 `RootHtmlAutoRepeatV1` mode와 Root HTML profile gate를 요구한다. 일반 profile 허용은 변경하지 않는다. |
| 2 | 한 grid에서 두 축 repeat를 동시에 허용하는가? | cascade grammar와 layout contract가 거부한다. |
| 3 | auto-repeat 앞이나 뒤에 fixed track 또는 fixed-repeat을 붙일 수 있는가? | 전체 track list 단독 repeat로 제한한다. |
| 4 | authored 또는 반복 내부 line name이 보존되지 않고 잘못 위치될 수 있는가? | 이 profile에서는 모든 line-name group을 거부한다. |
| 5 | 빈 pattern, 3개 이상 track pattern 또는 잘못된 range를 통과시키는가? | pattern 길이 1–2와 `start=0,end=len`를 cascade·projection·layout에서 확인한다. |
| 6 | 0/음수/non-finite minimum이 분모나 sizing을 깨뜨리는가? | positive finite px minimum만 허용한다. |
| 7 | `2fr`, 다른 flex factor 또는 비-flex 최대 breadth가 통과하는가? | `1fr` exact pattern만 허용하며 다른 maximum은 거부한다. |
| 8 | 반복 축의 auto/percentage/계산식/0 크기로 repeat 수가 불명확한가? | positive finite fixed px definite axis만 허용한다. |
| 9 | 반복 축 min/max 제한이 repeat-count 기준과 달라지는가? | repeated axis의 `min:auto`, `max:none`만 허용한다. |
| 10 | gutter가 repeat 개수 산식에서 빠지거나 169/170px 경계가 어긋나는가? | 열·행 gap을 산식에 포함하고 169/170px 및 2-track boundary test를 둔다. |
| 11 | UA 1px floor가 track sizing까지 적용되어 subpixel track을 왜곡하는가? | floor는 count 산식에만 쓰고 원래 minimum을 sizing에 전달한다. |
| 12 | 큰 값, NaN/Inf 또는 합산 overflow가 allocation/placement로 번지는가? | finite checks, checked capacity와 10,000-track cap을 적용하고 해당 경계 test를 둔다. |
| 13 | content-box와 border-box 입력으로 available size가 달라지는가? | padding/border 0을 고정하여 repeat basis가 달라지지 않게 한다. |
| 14 | percentage/calc/음수 gap 또는 margin·padding·border가 cell 좌표를 밀어내는가? | fixed non-negative px gap과 zero edge만 허용한다. |
| 15 | row/column dense 또는 implicit auto track으로 visible items가 재배치되는가? | row sparse auto-placement와 auto implicit tracks만 허용한다. |
| 16 | explicit line placement, span 또는 item 수 초과가 implicit track을 만드는가? | direct item placement를 모두 auto로 제한하고 expanded cell capacity를 검사한다. |
| 17 | hidden child가 capacity를 소비하거나 auto-fit이 비어 있는 repeat를 잘못 collapse하는가? | hidden child를 occupancy에서 제외하고 13-scene matrix에서 auto-fit/empty를 비교한다. |
| 18 | `order`, positioned item, aspect ratio 또는 self-alignment가 frame을 바꾸는가? | 별도 item validator가 이를 거부한다. |
| 19 | alignment, direction, overflow, area/name 조합이 unsupported Taffy behavior를 노출하는가? | row flow, default/start alignment, LTR, visible overflow 및 no-area/no-name 계약을 검사한다. |
| 20 | Rust scene만 통과하고 기본 native Root HTML 경로 또는 문서가 다른 동작을 주장하는가? | Android/iOS default app route에 열·행 repeat fixture를 실행하고 screenshot을 visual smoke로 제한한다. STATUS와 계약은 C11 완료로 승격하지 않는다. |

## 수정 내역

- Review #20에서 native default-app fixture가 columns만 확인하는 것을 발견했다. rows auto-repeat scene과 기대 frame을 추가했다.
- 테스트를 독립 파일로 나누는 동안 Rust `#[path]` 경로가 기존 `grid_values/tests.rs`를 가리키는 컴파일 오류가 났다. 기존 sibling-test 경로 방식으로 고치고 focused suite에서 다시 검증한다.
- Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 기본 Root HTML route가 `layout=ready boxes=14` 및 draw/present를 기록했다. 두 OS screenshot은 visual smoke만 증명하며 PixelCopy·Metal drawable readback은 아니다. [실행 evidence](../spec/internal/evidence/c11-root-html-grid-auto-repeat-runtime-2026-11-05/README.md).
