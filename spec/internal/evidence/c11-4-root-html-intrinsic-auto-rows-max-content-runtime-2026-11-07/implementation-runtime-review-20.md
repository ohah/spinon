# `grid-auto-rows:max-content` 구현·runtime 적대 검토 20개 · 2026-11-07

계획 검토 20개와 다른 관점으로, 최종 Rust diff와 기본 Root HTML runtime 경로를 점검했다. 항목마다 경계·실패 경로를 따로 확인했다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | Style projection에서 `max-content`가 `min-content`로 낮춰짐 | typed `ComputedCssGridTrackSize::MaxContent`가 `GridTrackSize::MaxContent`로 투영된다. `root_html_profile_projects_singleton_max_content_implicit_rows_into_intrinsic_mode`와 RuntimeSession cascade assertion 통과. |
| 2 | `max-content`가 singleton 이외 list의 원소로 허용됨 | implicit auto-row 후보는 정확히 `[MaxContent]`만 허용한다. Runtime mixed-list negative control은 atomic failure. |
| 3 | Max-content가 `auto` 또는 min-content와 같은 계산 분기로 합쳐짐 | Taffy intrinsic `MaxContent` track을 그대로 사용한다. Fixture typed identity assertion과 used row trace `[10,20,34]` 통과. |
| 4 | 실제 parsed text 대신 높이가 고정된 child만으로 결과를 맞춤 | 기본 RuntimeSession fixture는 기존 direct-text item과 normal wrapping을 사용하며 6 glyph와 두 baseline을 확인한다. |
| 5 | 자동 최소 크기·line wrapping이 track contribution을 바꿈 | 40px inline Grid의 실제 text item이 두 line으로 배치되고 text-row frame height 20px를 독립 기대값과 대조했다. 플랫폼 glyph 분포는 별도 한계로 기록. |
| 6 | Explicit row가 빠지거나 여러 fixed row가 들어옴 | bounded profile은 단일 positive fixed-px explicit row로 제한된다. 공통 Root HTML track validator와 capacity admission이 shape를 재검사한다. |
| 7 | Explicit column이 intrinsic 또는 indefinite가 됨 | fixed-track candidate는 단일 positive fixed-px column을 요구한다. `max-content` + `min-content` explicit column RuntimeSession negative control은 실패한다. |
| 8 | 기존 min-content 전용 intrinsic-column 조합까지 max-content로 넓어짐 | 결합 경로는 `root_html_has_intrinsic_implicit_min_content_rows`를 별도로 유지한다. Projection와 Layout의 기존 min-content tests 및 max-content 거부가 통과. |
| 9 | Nonzero row/column gap이 계산되어 지원 범위가 넓어짐 | max-content RuntimeSession negative control에서 `row-gap:1px`은 `grid-auto-rows` failure로 거부된다. |
| 10 | Mixed row sizing이 max-content의 이름으로 통과함 | `[max-content,min-content]`와 `[max-content,12px]` negative controls가 거부된다. |
| 11 | Row-flow 대신 column/dense placement가 intrinsic mode로 들어옴 | Grid projection과 intrinsic placement validator의 기존 flow 제한을 유지한다. 해당 후보가 아닌 다른 flow path는 열지 않았다. |
| 12 | Named areas, repeat 또는 implicit column 예약이 함께 admission됨 | 공통 intrinsic profile guard의 영역·repeat·axis shape 제한을 보존했다. 이 candidate는 새 area/repeat/implicit-column path를 추가하지 않는다. |
| 13 | Item capacity가 explicit+implicit 행 수보다 작게 계산됨 | `validate_root_html_grid_item_capacity`가 singleton min/max rows를 fixed-track candidate에서 수용한다. RuntimeSession 3-child fixture가 complete layout까지 도달한다. |
| 14 | Layout admission과 sparse placement가 서로 다른 허용 값을 씀 | `grid.rs`, `grid_auto_placement.rs`, one-axis container validation 모두 intrinsic placement mode에서 singleton min/max만 허용한다. Layout focused suite 17/17 통과. |
| 15 | Gap/track 합산 또는 큰 track 한계가 우회됨 | max-content 신규 코드는 전역 track/gutter ceiling을 늘리지 않았다. 공통 invalid/ceiling regressions를 포함한 layout filter가 통과. |
| 16 | 거부 시 다른 CSS property 오류로 귀속됨 | mixed tracks, row gap, intrinsic-column combination은 RuntimeSession에서 `grid-auto-rows` 오류로 보고된다. |
| 17 | 실패한 cascade/layout이 이전 완료 snapshot을 부분 결과로 남김 | mixed-track media mutation은 `Failed`, `completed=None`을 확인했다. |
| 18 | 같은 session에서 환경 복구 뒤 stale snapshot을 재사용함 | light→dark reject→light recovery cycle 후 새 완료 snapshot과 마지막 item frame `[0,30,40,34]`를 확인했다. |
| 19 | Paint source와 computed/layout revision이 서로 다른 세대에서 결합됨 | RuntimeSession 성공 경로가 cascade/layout key와 text source generation·document/tree/style revisions를 비교한다. |
| 20 | Simulator screenshot을 pixel readback 또는 일반 CSS 지원으로 과장함 | Android AVD/iOS Simulator 기본 route 및 화면만 기록했다. PixelCopy·Metal drawable readback·compositor-after pixels·physical device·WPT·C11.4 완료 주장은 하지 않는다. |

검토 중 확인된 admission 누락은 fixed-track max-content item capacity predicate였다. 이를 min-content-only에서 bounded min/max predicate로 고치고 위 RuntimeSession test를 다시 실행해 통과했다. 남은 범위는 README의 검증 경계를 따른다.
