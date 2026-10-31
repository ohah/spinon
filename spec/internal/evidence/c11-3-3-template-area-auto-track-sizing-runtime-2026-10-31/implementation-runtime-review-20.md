# C11.3.3 area-backed typed `grid-auto-*` 구현·runtime 실패 관점 검토 20개

계획 검토표와 별도로 실제 코드, fixed Chrome oracle, Rust/FFI 실행, Android AVD와 iOS Simulator 산출물을 검토했다. 각 항목은 독립 실패 경계를 다룬다.

| # | 실패 관점 | 확인 근거와 판정 |
| --- | --- | --- |
| 1 | area가 없는데 template profile이 C11.4 typed auto-track parser를 연다 | Stylo cascade가 computed `template_areas` 존재 여부와 template profile gate를 함께 검사한다. `template_profile_preserves_fixed_lists_and_area_backed_typed_auto_tracks` 통과. **PASS** |
| 2 | 다른 computed profile에서 area만으로 typed parsing이 열린다 | Style projection gate는 `RuntimeBlockFlexGridTemplateV1`과 area 모두를 요구한다. typed area 없는 profile 회귀와 fixture boundary 검토. **PASS** |
| 3 | area가 있어도 빈 `grid-auto-*` 목록이 유효한 pattern처럼 사용된다 | cascade는 빈 computed list를 Unsupported로 보존하고 projection은 빈 목록을 거부한다. `template_area_track_lists_reject_empty_but_project_typed_values` 통과. **PASS** |
| 4 | 잘못된 값/지원하지 않는 track 문법이 초기값이나 `auto`로 대체된다 | cascade·projection의 Unsupported 경로와 기존 C11.4 typed converter를 확인했다. 실패는 node/property 오류로 닫힌다. **PASS** |
| 5 | computed auto-track 목록의 상한 초과 또는 Vec 예약 실패가 잘못된 입력으로 숨겨진다 | `MAX_COMPUTED_GRID_TRACKS`, checked reservation, axis/property가 포함된 allocation error 경로를 확인했다. **PASS** |
| 6 | typed flag 때문에 area 없는 fixed-px list의 기존 허용 범위가 넓어진다 | typed permission은 area 조건에서만 열리고 fixed-list permission은 이전 분기에서 px만 받는다. area 없는 negative projection test 통과. **PASS** |
| 7 | signed/leading implicit 경로까지 percentage·intrinsic sizing이 열린다 | `TemplateGridSignedImplicitTracks`는 layout validator와 `FixedPxList` expansion으로 계속 px-only다. `implicit_auto_tracks_still_require_template_areas_and_signed_mode_rejects_them` 통과. **PASS** |
| 8 | 단일 `px` auto track이 typed list 전환으로 의미가 바뀐다 | cascade는 기존 단일 `LengthPx` representation을 유지하며 fixed-px C11.3.3 회귀를 통과한다. **PASS** |
| 9 | authored explicit track을 area extension이 덮어쓴다 | projection은 `missing_tracks = area_count - tracks.len()`만 append한다. `template_area_lists_do_not_change_authored_tracks_when_no_extension_is_needed` 통과. **PASS** |
| 10 | area 생성 track이 양축에서 pattern 첫 항목이 아닌 다른 항목을 사용한다 | typed projection test가 row minmax와 column percentage의 첫 missing track을 직접 확인한다. **PASS** |
| 11 | rows와 columns가 동일 pattern state를 공유해 한 축 소비가 다른 축 phase를 바꾼다 | 축별 복사본과 `template_area_row_and_column_patterns_keep_independent_phase` 검증. **PASS** |
| 12 | area track 소비 후 phase를 잘못 회전하거나 목록 길이 밖을 읽는다 | consumed count를 pattern length로 나눠 회전한다. typed projection 기대 목록과 row/column cycling test 통과. **PASS** |
| 13 | positive trailing named-line extension이 area가 소비한 다음 값 대신 첫 값을 다시 쓴다 | 새 layout test가 양축 `[10px, 25%, 8px]`, area 두 항목 소비 뒤 `8px,10px`과 `(70,60,18,18)` frame을 검증한다. **PASS** |
| 14 | `TemplateGridImplicitTracks`의 typed list가 area 없는 상태에서도 통과한다 | layout validator는 nonempty typed list의 Template profile을 `template_areas.is_some()`에 묶는다. 기존 no-area negative case 통과. **PASS** |
| 15 | percentage track이 잘못된 축의 definite basis를 사용한다 | 기존 typed track engine과 percentage row/column layout controls를 재사용한다. area row `minmax/fr` 및 column `25%`가 독립 Chrome frame과 일치. **PASS** |
| 16 | minmax/fr sizing 값을 px 값으로 평탄화하거나 intrinsic 형태를 잃는다 | `track_size(..., allow_typed_track_sizes)`와 `project_tracks`가 기존 typed enum/Math projector를 재사용한다. cascade·projection tests 및 runtime row fixture 통과. **PASS** |
| 17 | line-name metadata 확장 실패 후 partial area track output을 publish한다 | track reservation과 line metadata reservation은 `Result` 전파하며 projection 실패는 반환값으로 닫힌다. reservation-failure regression 통과. **PASS** |
| 18 | runtime fixture catalog에 누락/중복 scene, stale oracle hash가 생긴다 | builder와 Node tests가 scene ID 순서, 39/126 count, fixture/oracle/screenshot hash 및 runtime catalog를 고정한다. 관련 50개 Node test 통과. **PASS** |
| 19 | Android surface sample이 다른 node/scene을 읽고도 통과한다 | typed scene·sample allowlist를 exact ID, 좌표, RGBA로 제한했다. 최신 Android AVD에서 typed scenes 2개·7/7 PixelCopy exact. **PASS** |
| 20 | iOS 표본 또는 Android 결과를 compositor/실기기/WPT 증거로 과장한다 | iOS는 Metal drawable pre-compositor, Android는 API 37 AVD SwiftShader임을 README/result에 한정했다. WPT·실기기·전체 CSS 지원을 주장하지 않는다. **PASS** |

초기 workspace Clippy에서 auto-track helper 인자 수 lint가 발생했다. typed 허용 정책을 한 값으로 전달하도록 정리한 뒤 `cargo clippy --locked --workspace --all-targets -- -D warnings`를 다시 통과시켰다. 최종 source에서 blocker는 남지 않았다. 이 검토는 PR 변경 diff의 별도 30관점 검토를 대체하지 않는다.
