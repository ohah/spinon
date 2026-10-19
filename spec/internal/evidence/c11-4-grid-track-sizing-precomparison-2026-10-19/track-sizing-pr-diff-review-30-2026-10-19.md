# C11.4 track sizing PR 변경 적대 검토 30개

**검토 날짜:** 2026-10-19
**대상:** `feat/c11-4-track-sizing-algorithm-20261019`의 현재 변경. Taffy 0.15.0 전환, typed Grid track 투영, 제한 auto-repeat, 계산 trace, fixture FFI 및 Android/iOS Simulator 경로를 검토했다. 계획·구현의 기존 20관점 표와 앞선 contribution PR 리뷰를 재사용하지 않았다.

| # | 실패 관점 | 확인 및 판정 |
|---:|---|---|
| 1 | vendored Taffy 버전이 workspace 의존성 그래프에서 빠짐 | `Cargo.toml`, `Cargo.lock`, `crates/spinon-layout/Cargo.toml`의 path dependency와 `vendor/taffy/Cargo.toml` 0.15.0을 대조했다. Grid profile의 전환 범위를 문서에 표시했다. |
| 2 | 새 intrinsic 값이 기존 Template profile에 노출됨 | `previous_template_profile_rejects_intrinsic_track_functions`는 `minmax()`를 거부한다. 새 allowlist는 TrackSizing profile 분기로 한정했다. |
| 3 | CSS computed track 종류가 projection 중 단일 숫자로 축약됨 | `track_sizing_profile_preserves_intrinsic_track_functions_and_calc_values`와 `track_sizing_profile_projects_typed_functions_and_axis_scoped_calc_ids`가 auto/min/max/minmax/fit-content/calc 변형 및 축별 calc handle을 보존한다. |
| 4 | `minmax()` minimum에 flex 값이 들어감 | 공개 Rust enum은 minimum과 maximum 타입을 분리하고 minimum projection에 fraction variant가 없다. 기존 parser/cascade의 잘못된 조합은 거부된다. |
| 5 | 유효한 `0fr` maximum이 0이 아닌 값으로 변하거나 거부됨 | `track_sizing_preserves_zero_fraction_in_minmax_maximum` 및 layout의 `track_sizing_profile_accepts_zero_fraction_tracks`가 0을 보존한다. |
| 6 | 직접 수치 track에 음수·NaN·무한대가 통과됨 | cascade와 layout validator에서 길이·백분율·fraction의 finite/non-negative 조건을 확인했다. 계산식의 모든 definite·cyclic 결과 조합은 별도 미완료 범위로 유지한다. |
| 7 | `fit-content()`가 고정 길이로 바뀌거나 percentage를 잃음 | cascade 테스트는 px와 percentage 입력을 typed 값으로 확인하고 projection은 Taffy fit-content 함수로 변환한다. 전체 intrinsic contribution 결과는 이 검사만으로 완료 처리하지 않는다. |
| 8 | row·column calc handle이 서로 바뀜 | `GridTemplateColumns`·`GridTemplateRows` property를 변환 시 분리 전달하며 projection 회귀가 축별 ID를 비교한다. |
| 9 | Grid item의 min/max constraint가 다른 profile에도 적용됨 | `grid.rs`의 `TrackSizing` 조건에만 auto/fixed/percentage min/max를 허용한다. cyclic percentage와 다른 constraint 종류는 미완료로 문서화했다. |
| 10 | aspect-ratio가 새 profile에서 사라져 transferred suggestion 입력이 끊김 | `track_sizing_carries_aspect_ratio_for_transferred_size_suggestions`와 computed style projection 경로를 확인했다. 모든 CSS suggestion 분기는 아직 범위 밖이다. |
| 11 | item span 밖의 track이 automatic minimum에 영향을 줌 | 고정 Chrome 155 span 3개와 Ahem span 회귀의 item별 contribution 및 frame을 확인했다. 전체 placement·span 조합의 표준 적합성으로 확대하지 않았다. |
| 12 | item contribution 종류가 minimum/min-content/max-content 사이에서 혼동됨 | Taffy trace 변환은 세 enum variant를 각각 보존하고, Chrome text fixture가 contribution 관찰값과 frame을 함께 비교한다. 독립 normative stage oracle은 없다. |
| 13 | fixed-track early return이 trace에서 일반 sizing 완료로 오인됨 | 고정 경로를 `FixedTrackEarlyReturn` stage로 분리한다. 이 hook은 trace이며 CSS 알고리즘 전체 증명은 아니다. |
| 14 | 행·열 또는 percentage 재계산 pass의 순서가 사라짐 | trace는 pass별 axis와 sequence를 유지한다. Chrome 26-case와 시뮬레이터 결과는 지원된 subset의 경로만 확인한다. |
| 15 | collapsed auto-fit track과 gutter state가 trace에서 누락됨 | track snapshot은 `is_collapsed`, `is_gutter`, base size, growth limit, infinite-growable 상태를 보존한다. auto-fit scene은 Android/iOS에서 검사했다. |
| 16 | Taffy 내부 node 번호를 Spinon `NodeId`로 잘못 공개함 | trace adapter는 node mapping 실패를 명시 오류로 반환한다. synthetic node identity가 없을 때 추정 매핑하지 않는다. |
| 17 | nested Grid trace가 바깥 container trace에 섞임 | Taffy의 `DetailedLayoutInfo`는 grid node에 붙고 변환은 해당 node의 trace만 순회한다. 중첩 Grid 전체 trace 조합은 아직 별도 앱 검증이 없다. |
| 18 | trace 예산 초과 또는 예약 실패 뒤 부분 LayoutOutput이 성공처럼 반환됨 | trace failure variant는 layout error로 변환되고 최종 output 생성을 중단한다. Taffy 내부 계산용 일부 Vec는 infallible allocation을 사용한다는 제한을 명시했다. |
| 19 | trace 기본 비활성 경로에서 진단용 buffer가 항상 추가됨 | `CompactTaffyStyle`의 trace 설정은 opt-in이며 style 기본값은 꺼져 있다. 기본 product artifact에도 task fixture가 빠진다. |
| 20 | auto-repeat 두 축 동시 사용 또는 비-TrackSizing profile 유입 | cascade, projection, layout validator 모두 단일 축·TrackSizing 조건을 둔다. 반대 축 조합을 포함해 거부 회귀가 있다. |
| 21 | auto-repeat과 일반 explicit track 혼합을 의도치 않게 지원으로 표시 | parser는 repeat component 하나만, 그 안의 제한 fragment만 받는다. 앞·뒤 일반 track 혼합은 계획된 subset 밖이며 fail-closed다. |
| 22 | repeat 안의 named line이 Taffy 변환에서 조용히 소실됨 | cascade와 projection은 repeat line metadata를 거부하고, `auto_repeat_rejects_named_lines_through_the_layout_api`는 low-level columns/rows 경계도 검사한다. |
| 23 | auto-repeat과 template areas를 동시에 받아 area 배치를 잘못 계산함 | cascade와 projection에서 함께 있을 때 거부한다. area placement 자체는 별도의 named-area 회귀로 유지한다. |
| 24 | 10,000 track 경계를 초과해 일부 track만 계산됨 | Spinon validator는 truncate 대신 오류를 반환한다. integer repeat 확장과 auto-repeat fragment 모두 계산·예약 한도를 갖는다. 반복 결과의 완전한 §7.2.3.2 matrix는 미완료다. |
| 25 | auto-fill과 auto-fit 모드가 서로 같은 배치가 됨 | layout API 및 Android/iOS scene에서 auto-fill 반복과 auto-fit 빈 반복 collapse를 별도 관찰했다. 이는 두 기본 장면의 결과다. |
| 26 | 120px item의 frame 초과가 화면 색상으로 이어지지 않음 | 양쪽 simulator에서 frame 검사와 별도로 item-color 표본 범위를 확인해 grid 우측 바깥 약 22 CSS px를 기록했다. border stroke RGB 자체는 assertion하지 않았다. |
| 27 | fixture FFI가 기본 product ABI에 섞임 | C11.4 생성·평가 symbol 및 payload가 기본 macOS ARM64 staticlib에는 없고 fixture-test 양성 대조에만 있는 것을 `llvm-nm`/`strings`와 hash로 기록했다. 이는 release app 크기 근거가 아니다. |
| 28 | FFI scene 문자열로 임의 JS source를 실행함 | FFI는 7개 고정 scene ID만 whitelist에서 찾고 unknown ID, UTF-8, 길이와 null argument를 검사한다. scene source는 내부 고정 상수다. |
| 29 | Android/iOS intent가 다른 C11 fixture 경로와 혼합됨 | 두 플랫폼은 scene allowlist·run id·fixture feature를 검사하고 scene별 root/item frame assertion을 출력한다. API 37 AVD와 iOS 26.2 Simulator의 7 scene에서 실행했다. |
| 30 | 새 Taffy 계산 변경이 선행 C11 profile을 회귀시킴 | Taffy 0.15.0 전환 뒤 C11.3.3 Android/iOS와 C11.2.3 iOS 고정 행렬을 별도로 실행했다. 해당 profile·기기·표본 밖의 회귀 전체가 닫힌 것은 아니다. |

**결론:** 검토 중 추가로 발견한 구현 결함은 없었다. auto-repeat line-name 거부 진단의 영어 문구는 프로젝트 언어 규칙에 맞게 한국어로 고쳤다. 남은 항목은 독립 normative stage oracle, 전체 CSS contribution/placement/calc 행렬, 앱 trace 표시, WPT, 실기기 및 border stroke 색 검증이다. 이 검토는 C11.4 완료를 뜻하지 않는다.
