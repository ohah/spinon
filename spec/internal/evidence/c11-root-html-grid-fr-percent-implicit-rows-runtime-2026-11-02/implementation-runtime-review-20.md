# 구현 코드·런타임 실패 관점 검토 20개

계획 검토 표를 재사용하지 않고 실제 구현 코드와 현재 runtime 산출물을 서로 다른 실패 경계에서 확인했다. 범위는 기본 parsed Root HTML 경로와 `RootHtmlImplicitRowsV1` 조합에 한정한다.

| # | 실패 관점 | 확인 결과와 근거 |
|---:|---|---|
| 1 | 변경이 다른 Grid profile까지 넓어지는가 | `RuntimeRootHtmlTextPaintV1`에서 쓰는 전용 validator와 `RootHtmlImplicitRowsV1` mode의 검사만 변경했다. 이름 있는 area profile의 fr/percentage 거부 테스트는 그대로 있다. `grid_values.rs:120-165`, `root_html_grid_track_tests.rs:250-320`. 통과. |
| 2 | 지원 목록이 direct px/%/fr 밖으로 넓어지는가 | 양 축 모두 direct `LengthPx`, `Percentage`, `Fraction`만 열고 나머지는 projection에서 거부한다. `grid_values.rs:125-155`. `Auto`/`MinMax` 거부 projection 테스트 통과. |
| 3 | NaN, infinity, 음수 breadth가 통과하는가 | 각 admitted track은 finite 및 `>= 0`을 통과해야 한다. `grid_values.rs:141-146`, `grid.rs:1076-1084`; 기존 Root HTML invalid track matrices 통과. |
| 4 | unnamed-track 제한을 named line이 우회하는가 | 양 축의 비어 있지 않은 line-name group과 auto-repeat metadata를 projection/layout 양쪽에서 차단한다. `grid_values.rs:138-146`, `grid.rs:1073-1086`. named/auto-repeat negative runtime은 scene 없이 실패한다. |
| 5 | named area가 implicit row 경로에 섞이는가 | projection과 layout이 `template_areas`를 거부한다. `grid_values.rs:158-162`, `grid.rs:1053-1058`; runtime negative case 통과. |
| 6 | 이 확장이 Root HTML 이외 profile에 적용되는가 | profile dispatch는 Root HTML projection만 이 제한 validator로 보내고 layout mode도 명시적으로 `RootHtmlImplicitRowsV1`일 때만 추가 검사한다. `grid.rs:358-361`; fixed named-area tests는 확장되지 않는다. 통과. |
| 7 | column `fr`에서 width basis가 indefinite일 때 계산되는가 | 기존 fraction-axis validator는 column `fr`에 definite finite width를 요구하며 aspect-ratio 결합을 거부한다. `root_html_grid_tracks.rs:17-58`; 기존 indefinite-basis tests 통과. |
| 8 | row `fr`에서 height basis가 indefinite일 때 계산되는가 | row `fr`도 definite finite height를 요구한다. `root_html_grid_tracks.rs:35-47,232-260`; 기존 indefinite-basis tests 통과. |
| 9 | column percentage가 root width가 아닌 다른 크기를 basis로 삼는가 | percentage validator는 `grid-template-columns`에 computed definite width를 전달한다. `root_html_grid_tracks.rs:61-77,127-160`; Chrome 및 Rust `30px/90px` 결과 일치. |
| 10 | row percentage가 definite height를 요구하지 않는가 | `grid-template-rows`에 computed definite height를 전달하며 불확정값은 실패한다. `root_html_grid_tracks.rs:79-91,127-160`; Chrome 및 Rust `20px/20px` 결과 일치. |
| 11 | percentage 합계·gap 변환에서 f32 overflow가 생기는가 | fixed px, resolved percentage, gap 합계를 f64로 계산한 뒤 finite f32 경계를 검사한다. `root_html_grid_tracks.rs:162-217`; range negative tests 통과. |
| 12 | implicit row 자체가 percentage/fr/intrinsic/list로 넓어지는가 | mode는 scalar `LengthPx`만 받는다. `grid_values.rs:171-197`, `grid.rs:1092-1102`; implicit %/fr/minmax/list negative runtime matrix 통과. |
| 13 | explicit row `fr` sizing에서 fixed implicit row 공간을 빼지 않는가 | 120×60 root의 `1fr 2fr` rows와 15px implicit row가 placement y=`0,15,45`를 만든다. `root_html_grid_implicit_rows_tests.rs:72-87`, runtime test `composition.rs:120-139`; 통과. |
| 14 | percentage row scene에서 leftover block space가 track을 임의 stretch하는가 | 120×80에서 25%+25% rows는 각각 20px이고 item y=`0,20,40`; 남은 25px는 보존된다. runtime `composition.rs:141-160`, Chrome precomparison; 통과. |
| 15 | sparse row auto-placement가 gap과 explicit row 경계를 잘못 계산하는가 | percentage column case는 5px row-gap을 포함해 implicit item y=25px, x=0에 놓인다. `root_html_grid_implicit_rows_tests.rs:52-70`, runtime `composition.rs:95-114`; 통과. |
| 16 | capacity preflight가 newly admitted `%`/`fr`을 차단하거나 상한 검사를 우회하는가 | 통합 실행에서 별도 preflight가 px만 인정하는 문제를 발견해 direct finite non-negative px/%/fr와 같은 10,000-track product 계산을 인식하도록 수정했다. `style_values.rs:1097-1129`; 수정 뒤 Root HTML runtime 20/20과 Clippy 통과. |
| 17 | row expansion 전에 자원 상한을 적용하는가 | explicit-capacity 곱과 implicit row cap은 checked arithmetic을 쓰고 layout은 track expansion 전에 10,000 ceiling을 검사한다. `style_values.rs:1083-1129`, `grid_auto_placement_tests.rs:223-235`; resource-limit test 통과. |
| 18 | hidden child가 배치 occupancy를 소비하거나 source reorder가 무시되는가 | runtime은 hidden item 미출력, ID/class rename 안정성, 문서 순서 변경에 따른 item frame 소유권 이동을 검사한다. `root_html_grid_implicit_rows.rs:104-167`; 통과. |
| 19 | out-of-scope 입력 실패 뒤 partial/stale scene이 게시되는가 | negative matrix는 named/intrinsic/flow/placement/item/overflow 후보마다 failed layout, 오류 존재, `completed=None`을 확인한다. `root_html_grid_implicit_rows.rs:225-358`; runtime suite 통과. Positive scene은 cascade/layout key identity도 확인한다(`root_html_grid_implicit_rows.rs:56-101`). |
| 20 | 앱 확인이 fixture-only 경로를 잘못 통과하는가 | Android/iOS app build에서 C04 runtime만 켜고 fixture flags는 끈 뒤 기본 Root HTML boot, `layout=ready boxes=6`, draw/present를 기록했다. HTML package hash가 Android APK와 iOS bundle에서 candidate와 일치한다. Simulator 캡처는 화면 근거이며 PixelCopy/Metal readback으로 확대 해석하지 않는다. |

**판정:** 현재 검토에서 열린 결함은 없다. capacity preflight에서 발견한 누락은 코드와 integration suite에서 수정·재확인했다. surface pixel equality, physical-device GPU, WPT, arbitrary CSS Grid는 검증 범위 밖이다.
