# C11.3.3 named-line auto implicit tracks · 구현 실패 관점 검토 20개

검토 기준은 `feat/c11-3-3-named-line-auto-implicit-tracks`의 현재 수정과 fixed-fixture 계약이다. 검토 중 mode 과허용, fixed-only mode의 no-area `auto` 잔여 허용, auto-size item 전파 위험을 찾아 수정하고, 관련 회귀 검사를 다시 실행했다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | 기존 template profile에 새 auto sizing 허용이 새어 들어가는가 | 새 enum mode는 projection의 C11.3.3 opt-in에서만 선택한다. 기존 `TemplateGrid` 및 default mode는 그대로다. |
| 2 | area-backed typed auto track이 새 named-line policy로 바뀌는가 | area-backed container는 `TemplateGridImplicitTracks` 경로를 유지하고 `TemplateAreaImplicit`으로 확장한다. 기존 area sizing 회귀가 통과한다. |
| 3 | signed leading/trailing named line이 auto sizing을 얻는가 | `requires_signed_extent`가 signed mode를 먼저 선택하며 auto는 거부한다. signed named-line 회귀가 통과한다. |
| 4 | 양 끝 named line이 모두 implicit인 item이 새 mode를 얻는가 | 새 허용은 named start + auto end만 선택한다. `missing / missing` auto-row 사례는 계속 거부한다. |
| 5 | named end + auto start가 지원 범위를 넓히는가 | candidate 판정은 named start + auto end에만 적용한다. 반대 배치는 새 auto mode를 받지 않는다. |
| 6 | 다른 축의 start가 auto라서 auto placement가 섞이는가 | 다른 축은 definite `Line` start를 요구한다. 새 mode에서도 implicit/auto placement는 `GridImplicitTrackRequired`로 닫힌다. |
| 7 | 두 축이 동시에 implicit named line인 item이 auto mode로 들어오는가 | candidate는 정확히 한 implicit axis만 요구한다. 둘 이상의 implicit axis는 새 mode를 선택하지 않는다. |
| 8 | 양 축별 성공 배치가 혼동되는가 | columns와 rows를 별도 Rust/style fixture로 각각 확인하고 Android/iOS의 별도 scene 결과를 기록한다. |
| 9 | auto-size named item이 fixed-size candidate로 분류되는가 | projection은 width와 height 모두 `Fixed`일 때만 허용한다. auto-size named item은 `UnsupportedGridAutoColumns`로 거부된다. |
| 10 | layout 입력자가 새 mode를 직접 지정해 auto-size item을 넣는가 | layout validator가 새 mode의 각 item width/height를 `Fixed`로 제한한다. direct layout negative test가 통과한다. |
| 11 | 적격 item 뒤에 부적격 implicit 배치가 와도 auto가 유지되는가 | mode를 fixed-only implicit mode로 내리고 auto-row sizing을 거부한다. positive trailing named-end-only item도 뒤에 오면 같은 경계를 지킨다. |
| 12 | 부적격 item 뒤에 적격 item이 오면 auto가 다시 열리는가 | signed implicit mode와 positive named-end-only implicit mode 모두 fixed-only 경로를 유지한다. source-order 회귀에서 auto sizing 거부를 확인한다. |
| 13 | fixed-only mode가 no-area `auto`를 직접 수용하는가 | validator의 기존 template-area 조건을 복원했다. direct layout negative test와 style projection regressions가 모두 `UnsupportedGridAuto*`를 확인한다. |
| 14 | 기존 fixed-px named-line auto track control이 바뀌는가 | fixed-px implicit list는 기존 fixed-only mode에서 유지된다. columns fixed control과 grid suite가 통과한다. |
| 15 | invalid numeric implicit line이 named fallback으로 승격되는가 | unmarked numeric implicit line·span negative fixture가 계속 거부된다. |
| 16 | hidden item이나 auto placement가 새 mode의 implicit track을 늘리는가 | dense placement와 hidden-item 회귀를 포함한 143 Grid layout 테스트가 통과한다. |
| 17 | row/column expansion의 track ceiling이 우회되는가 | 신규 mode도 기존 count validation과 `GRID_MAX_TRACKS` 확장 경로를 사용한다. grid track ceiling tests가 통과한다. |
| 18 | new mode와 `grid-template-areas`가 혼용돼 area semantics가 빠지는가 | template-area validator는 새 named-line mode를 area profile로 인정하지 않는다. area tests는 별도 mode에서 통과한다. |
| 19 | 앱 surface 또는 sample negative control이 실패하는가 | 최종 소스 변경 뒤 두 simulator의 세 scene을 다시 실행해 frame/pixel과 invalid-coordinate rejection을 확인한다. |
| 20 | fixture payload가 기본 제품 archive로 유출되는가 | fixed-only guard 수정 후 기본 Release archive는 C11.3.3 symbol 0개/payload marker 0개이고 fixture-test opt-in은 symbol 2개/세 marker를 가진다. APK/IPA 전체 제외로 확대 해석하지 않는다. |

검증: `cargo test --locked -p spinon-style-to-layout --lib c11_3_3_grid_template_areas_tests`, `cargo test --locked -p spinon-layout --lib grid::`, formatter, workspace/fixture Clippy, CSS reference fixture 경계 Node tests, Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator. 별도 PR diff 30관점 검토는 [기록](./pr-diff-review-30.md)에 완료했다. GitHub CI/merge 검증은 PR 이후 판정한다.
