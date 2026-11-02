# C11.1 Root HTML fixed implicit rows 구현/runtime 실패 관점 검토 20개

검토 대상은 `RootHtmlImplicitRowsV1` projection/layout 경계, fixed-px row placement, error atomicity, 기본 parsed Root HTML runtime과 Android/iOS Simulator 실행이다. 각 항목은 별도 실패 경로를 골랐다. 발견 사항은 수정 후 해당 검사를 다시 실행했다.

| # | 실패 관점 | 확인 근거와 판정 |
|---:|---|---|
| 1 | named-area template에서 Root HTML implicit-row 지원이 새어 나오는가 | projection은 `template_areas`가 있으면 `grid-auto-rows`를 거부하고 core mode도 areas를 거부한다. named-area projection regression에서 계속 실패한다. 통과. |
| 2 | 기본 auto-row 경로가 새 mode 때문에 달라지는가 | `RootHtmlImplicitRowsV1` 진입은 computed `LengthPx`와 no-area 조건으로 한정된다. `root_html_` runtime 34/34 및 gap/px-fr-percentage 회귀 통과. |
| 3 | 음수·NaN·무한 implicit px가 allocation 또는 Taffy에 도달하는가 | projection 및 core에서 finite/non-negative를 확인한다. 기존 invalid-track 검증과 zero boundary runtime test 통과. |
| 4 | 여러 auto-row 값 또는 intrinsic/percentage/fraction sizing이 조용히 첫 값만 쓰는가 | projection이 단일 `LengthPx` 이외 값을 거부한다. runtime negative controls의 list, `%`, `fr`, `minmax()`가 모두 예상 stage에서 거부된다. |
| 5 | auto-repeat 또는 named line explicit track이 이름 없이 평탄화되는가 | Root profile과 core mode 모두 repeat metadata 및 named-line metadata를 거부하고 direct px track만 받는다. projection tests 통과. |
| 6 | implicit column이 row 전용 profile에서 만들어지는가 | `grid-auto-columns:auto` 및 fixed explicit column 수를 두 경계에서 확인한다. `grid-auto-columns:10px` negative control 통과. |
| 7 | column/dense flow가 sparse row 구현을 잘못 재사용하는가 | `grid-auto-flow`를 `row`로 고정한다. column과 dense negative controls가 cascade에서 거부된다. |
| 8 | explicit placement가 auto placement와 섞여 순서가 바뀌는가 | visible direct item은 모두 Auto placement여야 한다. numeric/name placement negative controls가 partial scene 없이 거부된다. |
| 9 | nonzero `order`가 DOM source order와 계산 순서를 분리하는가 | core prevalidation이 item `order == 0`을 확인한다. runtime `order:1` 거부와 positive DOM reorder 비교 통과. |
| 10 | hidden direct child가 occupancy 또는 row cap을 소비하는가 | `display:none`은 computed node collection 및 layout hidden map에서 생략된다. hidden child positive control의 frame과 paint가 없다. |
| 11 | text/nested child가 빈 fixed-size block으로 취급되는가 | `TextParagraphInput::has_content`, child list와 item shape를 함께 검사한다. visible text 및 nested child는 다른 실패 stage에서 거부된다. |
| 12 | auto placement가 class/id를 성공 분기로 사용하는가 | 같은 source의 class/id rename positive control에서 동일 item frame을 확인했다. |
| 13 | source order 변경이 배치 결과에 반영되는가 | 두 explicit-row item의 DOM 순서를 바꿔 x 위치가 서로 바뀌고 세 번째 item은 implicit row에 유지된다. exact frame test 통과. |
| 14 | zero-height implicit row가 누락되거나 양의 크기로 변환되는가 | 0px row의 item은 paint snapshot에서 빠질 수 있어 layout frame을 별도로 검사한다. `(0,25,40,0)` exact assertion 통과. |
| 15 | auto row 크기가 반복될 때 gap이 중복되거나 빠지는가 | 두 implicit rows의 independent rows `[20,15,15]`, y=`25,45`와 row-gap 5px frame을 exact 비교했다. 통과. |
| 16 | explicit+implicit 전체 track 한도를 넘겨 과할당하는가 | 10,001행 입력이 `GridImplicitTrackRequired`로 거부되는 전용 core test 1/1. track expansion 전에 반환된다. |
| 17 | item/track 용량 곱셈이 정수 overflow 또는 hidden count에 취약한가 | projection은 `checked_mul`/`checked_add`를 사용하고 hidden/nonparticipating direct child를 item-owner map에 넣지 않는다. 경계 검토 통과. |
| 18 | invalid style 후 이전 scene이 최신 성공 scene처럼 남는가 | 17개 unsupported runtime controls 모두 `layout=Failed`, error 보유, `completed=None`을 확인한다. partial scene 미발행. |
| 19 | Root HTML document/style revision identity가 새 mode에서 건너뛰어지는가 | 새 경로에서 cascade/revision 코드를 바꾸지 않았다. `stale_document_revision_is_rejected_before_cascade_or_layout` 1/1 및 positive test의 cascade/layout key equality 통과. |
| 20 | fixture 전용 FFI나 stale packaged source로만 성공하는가 | native app은 기본 `index.html` route와 `new_runtime_gpu_with_root_html()`를 사용했고 fixture-test flags는 off다. Android/iOS installed executable SHA가 빌드 후보와 일치하며 각 앱은 캡처 뒤 종료했다. |

## 수정 및 재검토

- 첫 scoped Clippy에서 새 test의 `needless_borrow`를 발견해 고쳤다. 이후 세 crate all-target Clippy `-D warnings`가 통과했다.
- 검토 도중 10,001-row resource-cap 조건의 직접 test가 빠진 것을 확인해 전용 layout test를 추가했다. 단독 실행 1/1 및 최종 `grid_` suite에서 재확인했다.
- Android/iOS 화면은 시각 확인이다. GPU surface PixelCopy/Metal readback, physical device, hardware GPU와 WPT 적합성은 검토 범위에 포함하지 않는다.
