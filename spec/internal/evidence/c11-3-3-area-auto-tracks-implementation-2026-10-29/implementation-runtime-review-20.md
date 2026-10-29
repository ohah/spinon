# C11.3.3 area-dimension implicit tracks 구현·런타임 실패 관점 20개

대상은 `feat/c11-3-3-area-auto-tracks`의 구현 변경과 2026-10-29 최종 Android/iOS 시뮬레이터 실행이다. 기존 계획 검토를 재사용하지 않았다. 각 관점은 서로 다른 입력·계약 경계를 확인한다. 결과는 구현 후보 검토이며, C11.3.3 공식 완료·WPT 적합성·실기기 지원 판정이 아니다.

| # | 실패 관점 | 확인과 근거 | 판정 |
|---:|---|---|---|
| 1 | `grid-template-areas` 차원 불일치가 모든 CSS profile에 무차별 허용됨 | `project_grid_layout_with_math`의 우회는 `RuntimeBlockFlexGridTemplateV1`에만 한정되고, 다른 profile의 불일치 반환은 유지된다. `grid_values::tests`에서 TrackSizing profile은 같은 입력을 거부한다. | PASS |
| 2 | TrackSizing auto-repeat 계약이 함께 넓어짐 | auto-repeat은 기존처럼 TrackSizing profile에서만 수용된다. 검증 함수의 별도 auto-repeat 분기와 기존 auto-repeat 테스트가 그대로 있다. | PASS |
| 3 | 0 또는 지원 한도 밖 area dimension으로 무한·과대 track 생성 | `project_template_areas`가 0 및 `GRID_MAX_TRACKS` 초과를 거부하고 layout validator도 같은 범위를 검사한다. 확장 크기는 u8 area dimension에서 온다. | PASS |
| 4 | 중복·NUL 이름 또는 잘못된 area 사각형이 새 mismatch 예외를 악용 | `validate_template_areas`의 이름 중복, 빈/NUL 이름, 경계 및 양수 크기 검증은 mismatch 허용보다 계속 적용된다. layout 전체 테스트 306개가 통과했다. | PASS |
| 5 | `grid-template: none`가 area 없는 입력까지 빈 explicit track을 허용 | cascade의 빈 track 경로는 areas 존재, integer-repeat 허용, intrinsic 미허용 조건을 모두 요구한다. 회귀 테스트에서 areas 없는 `none`은 오류다. | PASS |
| 6 | 두 축 모두 authored track이 없을 때 area 크기만큼 행·열을 만들지 못함 | `area-auto-tracks-none`와 projection assertion이 각 2행·2열 track 및 area metadata를 확인한다. Chrome/Android/iOS 기하 대조가 통과했다. | PASS |
| 7 | 행만 짧은 입력에서 열 또는 authored row 값이 손상됨 | `area-auto-tracks-short-row`는 첫 authored row 12px 뒤에 `grid-auto-rows` 7px를 적용한다. 보충 Chrome CSSOM·frame oracle와 플랫폼 행렬이 일치한다. | PASS |
| 8 | 열만 짧은 입력에서 행 또는 authored column 값이 손상됨 | `area-auto-tracks-short-col`은 첫 authored column 13px 뒤에 `grid-auto-columns` 9px를 적용한다. 보충 Chrome CSSOM·frame oracle와 플랫폼 행렬이 일치한다. | PASS |
| 9 | authored explicit track이 area dimension보다 길 때 잘림 또는 재배열 | `area-auto-tracks-long`와 Rust 회귀 테스트가 authored 3×3 track을 보존하면서 area의 첫 두 행·열 배치를 대조한다. | PASS |
| 10 | 새 implicit track이 `grid-auto-*`의 실제 값이 아닌 상수로 생성됨 | projection은 단일 computed `auto`를 `GridTrackSize::Auto`, px를 `LengthPx`로 투영한다. `grid-auto-rows`/columns list 입력은 지원하지 않는 값으로 fail-closed 한다. | PASS |
| 11 | auto track item 기여가 양 축에서 계산되지 않음 | `area-auto-tracks-auto`는 50×30 root에서 Chrome used rows 10/20px·columns 20/30px와 네 item geometry를 비교한다. Android/iOS 모두 0 CSS px다. | PASS |
| 12 | area item의 마지막 행·열 명명 배치가 생성 track 경계와 어긋남 | `area_dimensions_generate_implicit_tracks_and_preserve_authored_tracks`와 `shorter_explicit_column_list_places_area_item_on_created_track`가 마지막 area cell 좌표를 검사한다. | PASS |
| 13 | area alias lookup이 새 빈 line-name group과 함께 누락되거나 잘못된 경계에 매핑 | 생성 track 수 뒤에 경계 group을 `tracks.len()+1`까지 확장한다. area edge lookup은 group index를 통해 기존 line-name 경로와 합쳐지고 named-area projection 테스트가 통과했다. | PASS |
| 14 | implicit named-placement mode 전환 뒤 area metadata가 dimension mismatch로 다시 거부 | layout validator가 TemplateGrid implicit/signed implicit 두 모드의 area metadata를 보존한다. `template_grid_areas_remain_valid_after_implicit_placement_mode_selection`에서 양쪽을 검사한다. | PASS |
| 15 | unknown named area가 지원되지 않는 implicit sizing인데 성공으로 잘못 보고됨 | 최종 runtime contract는 `unknown-area-implicit-tracks`를 `unsupported_grid_auto_rows`/`grid-auto-rows`로 거부하고 frame 생성을 막는다. Android/iOS 런타임 행렬에서 기대 거부 1건이 확인됐다. | PASS |
| 16 | implicit named-line mode가 원래의 fixed-px 자동 트랙 제한을 우회 | `GridPlacementMode::TemplateGridImplicitTracks` 및 signed mode에서 `grid-auto-rows:auto` 거부 분기가 유지된다. 최종 unknown-area rejection log의 typed code와 property가 이를 확인한다. | PASS |
| 17 | 넓은 line-name 문자열에 area 확장용 metadata group header 예산이 빠짐 | combined metadata 계산이 새 group header 수를 사전 포함하며 1 MiB 초과 입력은 `grid-template-areas` 문맥 오류가 된다. 새 경계 테스트의 limit/limit+1 입력이 통과했다. | PASS |
| 18 | track 또는 line metadata reserve 실패가 panic/잘못된 성공으로 전파 | track reserve는 `GridAllocationFailure`로 바꾸고 line group은 `try_reserve_exact` 실패를 같은 구체 오류로 변환한다. 기존 reservation failure 회귀군과 style-to-layout 전체 255개 테스트가 통과했다. | PASS |
| 19 | fixture FFI 심볼·payload가 기본 artifact에 유출되거나 opt-in artifact에서 빠짐 | Rust 1.96/LLVM 22.1 `llvm-nm`으로 기본 archive는 두 C11.3.3 심볼 0개, `strings`로 보충 scene marker 부재를 확인했다. `c04-runtime-gpu-fixture-test` positive control은 심볼 2개와 payload marker를 포함한다. [archive hash와 feature 기록](./ffi-artifact-boundary.json). | PASS |
| 20 | 보충 scene route가 빠지거나 CSSOM/frame/표면 결과가 달라도 전체 PASS 처리 | FFI matrix와 두 앱 allowlist/capture에 5개 ID가 반영됐다. pinned Chrome 154 26 scene과 Chrome 156 supplemental 5 scene을 분리했고, 두 플랫폼의 31/31 행렬이 matched다. Android visible 88 frame 및 iOS frame은 최대 0 CSS px, PixelCopy/Metal 표본 2/2 exact, overflow 3/3 및 leading-line 7/7 표본도 일치한다. | PASS |

## 실행 근거

- Android: [최종 API 37 AVD/SwiftShader 행렬](../c11-3-3-grid-template-areas-runtime-2026-10-29/android-avd-c1133-matrix-20261029T132906693Z-b13baf02/README.md), 결과 `matched`.
- iOS: [최종 iPhone 17 Pro/iOS 26.2 Simulator 행렬](../c11-3-3-grid-template-areas-runtime-2026-10-29/ios-simulator-c1133-ios-matrix-20261029T131950859Z-f6d1bd47/README.md), 결과 `matched`; Metal readback은 compositor 제출 전이다.
- 코드 검사: `cargo test --locked -p spinon-layout` 306 passed; `cargo test --locked -p spinon-style-to-layout` 255 passed; CSS reference Node tests 24/24; `cargo fmt --all -- --check` 통과.
