# C11.1 `grid-auto-rows: calc(px + %)` 구현·런타임 적대 검토 20개

검토 대상은 `657a703993892d0b83a403e56b110d719c84d022` 기반 dirty candidate이며, 계획 검토와 구별되는 구현 코드·runtime 실패 경로를 확인했다. 경계 테스트는 [`spinon-style`](../../../../crates/spinon-style/src/stylo_dom/cascade/runtime_layout_tests/c11_1_root_html_auto_row_list.rs), [`spinon-style-to-layout`](../../../../crates/spinon-style-to-layout/src/projection/style_values/grid_values/root_html_grid_track_tests.rs), [`spinon-layout`](../../../../crates/spinon-layout/src/grid_auto_placement/root_html_implicit_rows_tests/calc.rs), [`spinon-runtime`](../../../../crates/spinon-runtime/src/session/tests/root_html_grid_implicit_rows/calc.rs), [Android/iOS Simulator logs](README.md)와 대조했다.

| # | 적대 관점 / 확인한 실패 경로 | 판정과 근거 |
|---:|---|---|
| 1 | 새 calc admission이 C11 전체 또는 다른 CSS profile에 새지 않는가 | 통과. `allow_root_html_calc_auto_row_tracks`는 `RuntimeRootHtmlTextPaintV1`에서만 켜지고 cascade test가 해당 profile 경계를 확인한다. |
| 2 | track list가 아닌 scalar 또는 빈 목록이 새 grammar로 잘못 들어오는가 | 통과. 입력은 비어 있지 않은 typed track list 경로를 유지하며, projection/list validation이 빈 목록을 거부한다. |
| 3 | 두 항이 아닌 합, 세 항, 중첩 식이 통과하는가 | 통과. projection helper는 `Sum`의 항 수가 2가 아니면 거부하고 unit test가 세 항과 product 형태를 포함한다. |
| 4 | 동일 차원 두 항 또는 px/% 누락이 통과하는가 | 통과. 정확히 길이 하나와 percentage 하나를 각각 요구하는 projection helper test가 거부를 확인한다. |
| 5 | 단항 부호가 빠지거나 다른 AST 위치에서 허용 범위를 넓히는가 | 통과. 직접 px/% 항의 `Negate`만 허용한다. nested negate/product와 다른 형태는 helper가 거부한다. |
| 6 | cascade와 projection 사이에 calc AST 또는 단위가 손실되는가 | 통과. Root HTML auto-row cascade test는 typed calc list를 보존하고 projection test는 `GridTemplateRows` math ID를 확인한다. |
| 7 | percentage basis가 width나 row-gap을 잘못 사용하거나 indefinite height를 수용하는가 | 통과. layout은 definite finite non-negative container height를 사용하고 auto height를 거부한다. `root_html_calc_auto_rows_require_unconstrained_definite_height`가 경계를 확인한다. |
| 8 | `min-height`, `max-height`, `aspect-ratio`가 percentage row basis를 불명확하게 만들어도 통과하는가 | 통과. 세 제한을 각각 거부하는 layout test가 있다. |
| 9 | definite height 0을 indefinite로 오인하는가 | 통과. 0px basis의 `calc(10% + 5px)` 결과가 5px이며, runtime frames에서 gap 0/10px의 별도 결과를 확인한다. |
| 10 | row-gap을 percentage basis에 포함하거나 track 크기와 중복 합산하는가 | 통과. 같은 calc를 두 gap 조건에서 실행해 row 시작점 `[0,0,5]`와 `[0,10,35]`를 확인한다. |
| 11 | 음수 calc 결과가 오류로 노출되거나 음수 track을 만드는가 | 통과. `GridTemplateRows` non-negative math resolution이 0px range clamp를 적용하며 layout/runtime regression은 뒤의 row를 3px 간격으로 배치한다. |
| 12 | 100% 초과 calc를 clamp해 실제 overflow를 감추는가 | 통과. `calc(120% + 1px)`는 121px이며 runtime item y=127px로 visible overflow를 유지한다. |
| 13 | 반복 목록 주기가 explicit prefix를 포함하거나 첫 implicit row에서 한 칸 밀리는가 | 통과. 8개 frame이 `[0,13,36,51,79,97,112,140]`px이고 첫 implicit row부터 `[12,25,15]` 순환과 일치한다. |
| 14 | 기존 direct px/%/fr track이나 calc와의 혼합 순서가 바뀌는가 | 통과. 기존 Root HTML implicit-row regression 32/32와 mixed-track tests가 direct cases 및 authored list order를 확인한다. |
| 15 | calc와 이웃한 `1fr`이 나머지 공간 분배를 깨거나 calc를 flexible track으로 오인하는가 | 통과. `root_html_calc_auto_rows_compose_with_fraction_tracks`와 runtime `...distributes_calc_neighbor_fr_remaining_space`가 fixed calc 뒤 fr 배분을 검증한다. |
| 16 | NaN, 무한값 또는 개별 used track의 f32 범위 초과가 Taffy에 도달하는가 | 통과. AST term finite 검사와 expanded-row 검증이 layout 전에 거부한다. projection/helper tests는 NaN, layout tests는 대형 값 경계를 포함한다. |
| 17 | 큰 row-gap과 여러 track의 누적 extent overflow가 partial geometry를 만드는가 | 통과. aggregate 검증은 모든 expanded tracks와 gap을 누적하면서 f32 범위를 검사하고, overflow test는 레이아웃 전 오류를 확인한다. |
| 18 | math ID가 누락·중복되거나 다른 node/property에 연결될 때 잘못된 값을 재사용하는가 | 통과. missing, duplicate ID, wrong owner와 `GridTemplateColumns` property binding mismatch를 각각 거부하는 layout helper tests가 있다. |
| 19 | syntax-invalid 선언과 syntax-valid 미지원 계산식이 같은 방식으로 처리되어 cascade fallback을 잃는가 | 통과. invalid `calc(10% +)`는 앞선 valid 선언으로 fallback하며, valid `min()` 포함 식은 부분 scene 없이 fail-closed한다. |
| 20 | 재계산 중 revision이 섞이거나 Android/iOS에서 다르게 배치되는가 | 통과. runtime test는 computed style과 layout revision identity를 확인한다. 두 Simulator 로그 모두 같은 8개 row 시작점, `layout=ready`, 11-box draw/present를 기록한다. PixelCopy/Metal readback은 수집하지 않았다. |

20개의 별도 실패 관점에서 새 결함을 찾지 못했다. 집중 테스트는 `spinon-style` root-html profile 14/14, `spinon-style-to-layout` root-html profile 16/16, layout calc 7/7, row helper 4/4, implicit-row regression 32/32, runtime calc 5/5 및 전체 Root HTML implicit-row runtime 16/16 통과로 기록됐다. Rust formatter와 두 모바일 Simulator 앱 빌드도 통과했다. 이는 focused evidence이며 전체 workspace, WPT, 실기기 GPU 또는 pixel parity 판정은 아니다.
