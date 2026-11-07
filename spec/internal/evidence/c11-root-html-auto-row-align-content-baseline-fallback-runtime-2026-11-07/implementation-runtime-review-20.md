# C11.4 `align-content:baseline` 구현·runtime 적대 검토 20개

계획 검토와 겹치지 않도록 실제 코드 경로, RuntimeSession 결과와 두 Simulator 캡처를 대상으로 서로 다른 실패 조건을 확인했다. 발견한 기존 rejection 기대값은 수정하고 해당 회귀를 다시 실행했다.

| # | 실패 관점 | 확인 및 결과 | 판정 |
|---:|---|---|---|
| 1 | Style projection이 authored 값이 아닌 computed `baseline`을 허용하지 않는다. | 고정 Root HTML fixture의 computed `align-content`를 보존하고 style projection 11/11에서 확인했다. | PASS |
| 2 | `first baseline`이 computed canonical value `baseline`으로 들어오는 경로가 빠진다. | Agent Browser 157 observation과 candidate projection을 대조했다. | PASS |
| 3 | allowlist가 `last baseline` 또는 안전 modifier 조합까지 함께 열어 준다. | exact `baseline`만 추가하고 `last baseline`/modifier control은 통과시키지 않았다. pinned parser의 fatal `CascadeDiagnostic`도 고정했다. | PASS |
| 4 | 새 값이 legacy `RootHtmlIntrinsicTrackSizingV1`에도 통과한다. | legacy profile gate negative test를 추가해 baseline rejection을 확인했다. | PASS |
| 5 | Taffy의 `FirstBaseline` enum이 새 profile에서도 거부된다. | `RootHtmlAutoRowStretchV1` only gate와 positive layout test를 확인했다. | PASS |
| 6 | baseline mapping이 CSS first-baseline 계산으로 과장되어 shared group에 적용된다. | 직접 Flex 및 Grid 부모는 `UnsupportedBaseline`으로 거부하도록 validator와 두 부모 테스트를 확인했다. | PASS |
| 7 | 다른 부모 display에서 parent lookup이 임의의 flex/grid 가정으로 오판한다. | block-root positive scene과 Flex/Grid parent negative scene을 각각 layout test로 확인했다. | PASS |
| 8 | baseline이 positive free space에서 start가 아닌 center/end로 이동한다. | positive frame offsets `[0,10,20]`을 Chrome/독립 기대값 및 Layout test와 대조했다. | PASS |
| 9 | overflow에서 baseline fallback이 unsafe 위치를 만든다. | `calc(120% - 2px)` overflow offsets `[0,56,112]`을 Rust 및 Android/iOS screenshot bounds에서 확인했다. | PASS |
| 10 | row-gap percentage basis가 viewport 또는 inline size로 잘못 resolve된다. | definite 40px block basis에서 46px gap을 계산하고 두 gutter trace와 일치시켰다. | PASS |
| 11 | align-content fallback이 implicit row breadth를 늘린다. | rows `[10,10,10]px`를 positive·overflow 모두 확인했다. | PASS |
| 12 | `StretchAutoTracks`가 no-op이 아니라 실제로 적용된다. | trace의 `StretchAutoTracks { applied: false }`를 단언하는 Layout test를 추가하고 20/20 filter를 재실행했다. | PASS |
| 13 | baseline이 gutter breadth를 바꿔 item 위치가 우연히 맞는다. | positive gutter `[0,0]`, overflow gutter `[46,46]`을 track trace에서 확인했다. | PASS |
| 14 | Style과 Layout revision이 다른 snapshot으로 섞인다. | 새 RuntimeSession positive·overflow 검증에서 computed serialization과 published geometry를 함께 대조했다. | PASS |
| 15 | 첫 fixed item이 바뀌거나 누락된 screenshot 기준점이 좌표 분석을 오염시킨다. | analyzer를 Grid 배경색 대신 첫 fixed item의 top edge를 origin으로 쓰도록 고쳤고 두 플랫폼에서 확인했다. | PASS |
| 16 | Simulator bundle이 확인 fixture와 다른 HTML을 실행한다. | Android APK 및 iOS app의 embedded `index.html` SHA가 각 scene fixture SHA와 일치했다. | PASS |
| 17 | Simulator가 앱 route에 도달하지 못했는데 screenshot만 성공으로 간주한다. | Android/iOS 모두 Root HTML boot `layout=ready`, 6 boxes, 2 text draw 및 `present` marker를 요구한다. | PASS |
| 18 | 지원 밖 item shape가 partial frame을 내보낸다. | dark media에서 text item을 숨긴 입력이 `grid-auto-rows`로 실패하고 `completed=None`인지 RuntimeSession 41/41에서 확인했다. | PASS |
| 19 | 실패 후 같은 session이 복구되지 않거나 revision이 오염된다. | media를 light로 돌려 complete positive layout이 재발행되는 same-session recovery test를 통과했다. | PASS |
| 20 | 새 slice 때문에 기존 alignment 값의 rejection 테스트가 거짓 실패한다. | safe center/end와 unsafe end 및 baseline을 함께 “지원 밖”으로 검사하던 stale assertion을 고쳤고 RuntimeSession 41/41을 재실행했다. | PASS |

## 실제 실행 결과

- `cargo test --locked -p spinon-layout root_html_auto_row_`: 20 passed.
- `cargo test --locked -p spinon-style-to-layout root_html_auto_row_`: 11 passed.
- `cargo test --locked -p spinon-runtime session::tests::root_html_grid_intrinsic_auto_rows`: 41 passed.
- Scoped Clippy over `spinon-layout`, `spinon-style-to-layout` and `spinon-runtime` with `--all-targets -- -D warnings`: passed.
- `cargo fmt --all -- --check`: passed.
- Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator의 positive·overflow 실행은 각각 layout-ready와 두 text draw에 도달했다. Screenshots는 OS visual bounds이며 PixelCopy/Metal drawable readback 또는 compositor 이후 pixels로 분류하지 않는다.
- Simulator run 전후 설치 앱 및 bootstrap/build 자산 hash가 일치했다. 전체 원본 로그, 캡처, hash, pixel-bound analysis는 [README](README.md)에 연결한다.
