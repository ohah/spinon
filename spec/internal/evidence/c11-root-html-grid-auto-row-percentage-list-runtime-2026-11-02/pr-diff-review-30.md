# C11.1 percentage `grid-auto-rows` 구현 PR 변경 검토 · 30개 관점

**검토 대상:** C11.1 percentage auto-row cascade·projection·layout, parsed Root HTML 통합 tests, implementation plan, 내부 계약 0059 변경, 실행 스크립트와 evidence. 구현/runtime의 20개 관점과는 별도다.

| # | PR diff 실패 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | profile options 새 필드가 생성되는 모든 callsite에서 기본 동작을 바꿈 | 유일한 computed layout option 생성부에서 Root HTML profile에만 켠다. 기존 profile의 flags는 false이며 scoped suites가 통과한다. |
| 2 | 추가 허용을 공용 cascade 함수의 기본 지원으로 오해 | 기본값은 구조체 필드 값으로 호출자가 정한다. public API나 parser 진입점을 추가하지 않았다. |
| 3 | helper refactor가 기존 `grid-auto-columns` semantics를 바꿈 | `allow_percentage_track_lists:false`, 기존 fixed-list 정책 유지. cascade/layout tests 통과. |
| 4 | fixed px scalar의 ABI/typed variant를 불필요하게 변경 | 단일 px는 기존 `LengthPx` 결과로 반환한다. px list도 order/px 단위를 보존한다. |
| 5 | one-element percentage가 scalar 최적화로 목록 순환 의미를 잃음 | percentage는 one-entry `TrackList`이며 projection/layout에서 동일 경로를 소비한다. |
| 6 | cascade 중간 `Vec` 생성에 infallible reserve를 새로 도입 | 신규 one-entry와 list path는 `reserve_computed_tracks`/`try_reserve_exact`를 쓰며 allocation failpoint 회복을 검사한다. |
| 7 | `TrackSize` percentage와 다른 breadth/calc variant를 direct percentage로 잘못 downcast | `track_size(size, false)`의 exact `LengthPx`/`Percentage` 결과만 받고 나머지는 Unsupported다. |
| 8 | profile guard보다 track-list projection이 먼저 적용되어 named-area route 통과 | RootHtml validator와 projection branch 모두 profile/shape를 검사한다. named-area mixed list는 명시적으로 거부된다. |
| 9 | fixed-list helper rename이 unrelated callers를 누락하거나 남은 px-only name과 동작을 혼합 | 검색된 RootHtml callsites가 새 이름을 사용한다. fixed-px semantics는 단일px와 list tests로 고정했다. |
| 10 | `GRID_MAX_TRACKS` 적용이 projection input list에만 있고 expansion에는 누락 | projection list cap 및 layout expanded row cap은 각각 유지된다. limit tests가 입력 및 placement 확장을 덮는다. |
| 11 | aspect-ratio 판단이 authored raw declaration을 읽어 낮은 우선순위 값을 오거부 | `ComputedElementStyle.properties`는 cascade computed values를 저장한다. computed `auto`만 통과, effective non-auto는 typed RootHtml profile에서 거부한다. |
| 12 | new RootHtml percentage basis gate가 `ComputedCssDimension::LengthPx` 이외를 silently zero로 변환 | definite finite height만 통과하고 math/auto/invalid size는 property-specific error다. |
| 13 | min/max constraint rejection이 explicit percent/fr behavior에서 다른 diagnostics를 냄 | 기존 percentage-axis validator는 공유되고, auto-row percentage는 같은 `validate_axis` 경계를 재사용한다. 신규 tests가 각각 높이 constraint를 검사한다. |
| 14 | explicit percentage/fr track를 implicit percentage list 때문에 지원 밖으로 보내거나 반대로 넓힘 | mixed explicit `%`/`fr`와 implicit `%` 조합은 exact layout frame test로 확인한다. 다른 unsupported auto track은 계속 거부한다. |
| 15 | aggregate range guard가 확장 row 앞 explicit row/gap을 빠뜨림 | validator에 explicit rows를 포함한 full `expanded_rows` slice를 전달하고 total track/gap sum을 검사한다. |
| 16 | aggregate sum에서 실제 gap 개수나 overflow를 틀림 | `expanded_rows.len()-1` gaps를 f64로 합산한 뒤 각 resolved row를 차례로 누적한다. negative/nonfinite gap은 기존 style guard에서 reject된다. |
| 17 | fraction row를 implicit list 내용처럼 받아 percentage guard를 회피 | `normalize_sparse_grid_container`가 implicit pattern에서 fraction을 먼저 거부한다. explicit fr는 별도 explicit track 경로다. |
| 18 | invalid percentage track에서 Taffy에 부분 expanded rows를 전달 | validation은 local expanded buffer와 `try_reserve` path 후 output에 한 번만 넣는다. 오류/allocator failpoint 이후 같은 worker recovery test가 성공한다. |
| 19 | test path `percentage.rs`가 제품 모듈이나 release artifact에 들어감 | 하위 모듈은 `#[cfg(test)]` 모듈 루트에서만 연결되고 product source path에 include되지 않는다. fixture FFI/ABI나 Cargo feature는 추가하지 않았다. |
| 20 | integration test가 실제 Stylo parsed CSS 대신 hand-built projection value를 재사용 | 새 `HostDocument` fixture가 inline CSS를 cascade하고 `compute_runtime_style_layout_with_root_html_grid_text_profiles`로 Taffy used frame까지 실행한다. |
| 21 | `grid-auto-rows` syntax invalid와 unsupported typed value를 같은 fallback으로 오해 | invalid negative declaration cascade fallback은 style test에서, valid unsupported/infinite/type 값은 projection/layout tests에서 각각 구분한다. |
| 22 | regression tests가 synthetic items 외 V8/document/runtime identity를 주장 | Rust integration은 HostDocument revision path만 증명한다고 명시했다. 플랫폼 V8 route는 별도 app probe/log evidence다. |
| 23 | Android run이 우연히 USB physical device에 설치 | 스크립트는 `-s emulator-5556`로 AVD만 지정한다. 기록된 AVD는 API37 ARM64 `sdk_gphone64_arm64`다. |
| 24 | iOS 실행이 임의 booted device를 골라 결과 불일치 | 스크립트는 UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`를 고정한다. 앱은 log capture 후 terminate한다. |
| 25 | build fixture가 원래 Bootstrap HTML/JS를 덮어쓴 채 남음 | build script의 EXIT trap으로 source/build bootstrap 백업을 restore하고, pass03 후 모든 파일 hash가 저장본과 일치했다. |
| 26 | 설치된 APK/executable이 기록된 candidate binary와 다른데 build hash만 보고함 | pass03에서 설치된 Android `base.apk`와 iOS app-container executable을 읽어 candidate 산출물과 각각 동일 hash를 확인했다. |
| 27 | 실행 script가 이전 통과 또는 실패 evidence를 덮어씀 | `PASS=passNN` file namespace와 `test ! -e` preflight를 쓴다. pass01/02 원본은 남기고 final pass03을 새로 기록했다. |
| 28 | simulator smoke screenshot을 PixelCopy/Metal pixel parity로 표현 | README와 plan은 screenshot을 visual capture로 한정하고 surface readback/pixel equality를 수행하지 않았다고 적는다. |
| 29 | 계획/계약 문서 수정이 C11/C11.1을 공식 완료 처리하거나 버전 상승 | 공식 `spec/STATUS.md`, checkbox, 숫자 version `0059 0.1.0`은 이 candidate PR에서 변경하지 않는다. dedicated plan와 contract는 limited candidate임을 표기한다. |
| 30 | simulator build warning·첫 harness count 오류·미실행 CI를 숨겨 성공으로 표시 | build/runtime 로그를 보존한다. pass01의 잘못된 기대값 11은 pass03의 실제 13으로 수정했다고 설명한다. iOS 기존 warnings 및 GitHub CI 미보고는 성공으로 과장하지 않는다. |

## 발견과 최종 판정

- Scoped Clippy에서 helper의 인자 수 warning이 확인되어 option struct로 묶었다. 수정된 코드의 cascade/layout/projection tests, scoped Clippy, rustfmt, diff check와 pass03 Android/iOS rebuild/runtime를 다시 실행했다.
- 별도 미해결 코드 blocker는 발견하지 않았다. GitHub CI와 required status checks는 PR 생성 후 live 상태를 확인하며, 이 표는 외부 reviewer의 승인이나 CI 결과를 대신하지 않는다.
