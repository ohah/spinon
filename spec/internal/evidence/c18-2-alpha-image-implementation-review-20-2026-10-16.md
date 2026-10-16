# C18.2 구현 후 적대적 검토 — 20개 관점

## 검토 경계

2026-10-16, 구현 source와 현재 Android API 37 AVD/iPhone 17 Pro iOS 26.2 Simulator 실행 산출물을 대조했다. 계획 검토 20개와는 다른 코드·런타임 반례를 사용했다. 완료 판정은 fixed fixture 범위에만 적용한다. 발견한 route, color-format, rounding 문제를 보정한 뒤 두 플랫폼 drawable 및 full-screen comparator, C18.1 regression과 기본/opt-in artifact 검사를 다시 실행했다.

| # | 실패 관점 | 확인 결과와 근거 |
| ---: | --- | --- |
| 1 | 기본 앱 실행이 C18.2 fixture를 임의로 시작하는가 | Android extra와 iOS argument가 필요하고 두 app build의 기본값은 off다. 실제 성공 실행은 명시한 fixture route만 사용했다. 기본 artifact scan에서 C18.2 ABI/payload가 빠졌다. |
| 2 | route argument가 Activity/AppDelegate의 runtime-GPU 진입을 놓치는가 | 초기 연결 감사에서 C18.2 launch argument가 `MainActivity`/`AppDelegate` 경로에 빠진 점을 발견해 반영했다. Android `SPINON_C182_EVAL`와 iOS `SPINON_C182_EVAL` 로그 및 실제 fixture 화면으로 수정된 경로를 확인했다. |
| 3 | `<img>`가 없거나 하나보다 많으면 임의 node를 선택하는가 | attachment는 HTML namespace `img`가 정확히 하나여야 한다. 없음·예상하지 않은 ID·중복 ID·변경 source negative case가 `c18_2.rs` unit test에서 거부됐다. |
| 4 | 같은 이름을 다른 namespace/local element에 넣어 이미지로 오인하는가 | HTML namespace와 local name `img`, 고정 `id` 및 source를 모두 검사한다. 고정 CSS source와 실제 live HostDocument image report를 양쪽 앱 log에서 확인했다. |
| 5 | caller PNG가 예상과 다르거나 손상되면 고정 oracle을 오염시키는가 | bytes는 opt-in `include_bytes!` 하나이며 request decode·resource snapshot 오류를 scene attachment 실패로 전달한다. encoded/decoded fixture hash가 Android/iOS manifest와 generator 결과에서 고정됐다. 손상 bytes 주입은 이 scope에서 실행하지 않았다. |
| 6 | RGB가 alpha에 premultiply되어 alpha 0 source 색이 새는가 | PNG bytes는 straight RGBA8 그대로 전달한다. alpha 0의 magenta source가 배경 `(17,34,51,255)`로 나오는 것을 Android PixelCopy/full-screen과 iOS drawable/full-screen에서 각 exact 확인했다. |
| 7 | alpha 128을 binary alpha나 일반 opaque로 처리하는가 | fixed middle band에서 expected `(118,37,65,255)`를 양 플랫폼 drawable 및 전체 화면 표본에서 exact 비교했다. comparator negative control은 alpha 128을 0/255로 바꾼 로그를 거부한다. |
| 8 | alpha 255 혹은 C18.1 opaque 이미지가 바뀌는가 | C18.1 route를 현재 renderer source로 별도 실행했다. 두 플랫폼 모두 surface/drawable 9/9, screenshot 9/9, anchor 4/4 exact다. |
| 9 | source-over의 destination factor나 alpha channel factor가 잘못됐는가 | WGPU pipeline은 `ALPHA_BLENDING`으로 straight source의 RGB를 source-alpha 기준으로 합성하고 output alpha는 `src + dst×(1−src)`로 만든다. 불투명 backdrop에서 모든 표면 표본의 output alpha 255를 확인했다. |
| 10 | 투명 image가 draw 순서를 바꾸거나 배경까지 덮는가 | alpha 0 표본과 frame 앞/뒤 background 표본이 같은 backdrop 값으로 유지되고, report는 image draw 1회만 표시했다. transparent RGB bleed negative control도 comparator에서 거부한다. |
| 11 | iOS sRGB surface에서 실제 Chrome 값이 달라도 pass로 숨기는가 | 첫 iOS pass `Bgra8UnormSrgb`에서 red 162 vs Chrome 118을 관측해 실패 로그/screenshot을 보존했다. exact scope는 측정된 UNorm drawable로 제한하고 sRGB-only surface를 완료 주장 범위에서 제외했다. |
| 12 | Android UNorm의 channel rounding 차이가 허용 오차로 가려지는가 | 첫 PixelCopy pass의 alpha-128 `(119,37,66)` vs `(118,37,65)`를 기록했다. UNorm target의 premultiplied source contribution을 8-bit로 양자화하고 strict zero-tolerance comparator로 Android/iOS 모두 exact 재확인했다. |
| 13 | 이미지가 원본 크기 대신 scale/filter로 바뀌어 sample을 우연히 맞추는가 | fixture CSS frame `(8,12) 24×16`, decoded PNG `24×16`, `nearest/clamp`, 80×40 viewport를 log/report/manifest에 결합했다. 10개 표본 중 좌우 띠 경계도 고정 좌표에서 확인했다. scaled sampling은 범위 밖이다. |
| 14 | stale document/image resource를 새 draw에 붙이는가 | runtime image source key는 generation·document·render-tree·intrinsic-size revision을 보존한다. C18.2 report revision 11/2가 draw·manifest와 일치하고 image count/draw count는 1/1이다. |
| 15 | image border/ancestor clip이 alpha 변경 때문에 무시되는가 | C18.2 자체는 clip-free single image다. 기존 C18.1 visible border overpaint와 hidden padding-edge clip 회귀를 양 플랫폼에서 9/9 재실행해 경계를 확인했다. 임의 CSS border 조합은 지원한다고 주장하지 않는다. |
| 16 | PixelCopy/Metal 좌표를 화면 좌표로 잘못 이동시키는가 | Android post-status origin `(435,1291)`, scale `2.625`; iOS origin `(483,1251)` px / `(161,417)` points, scale `3`을 고정했다. comparator가 CSS→surface→screenshot 변환과 실제 PNG pixel을 함께 대조했다. |
| 17 | iOS Metal drawable sample을 compositor 이후 pixel이라고 잘못 보고하는가 | 로그와 계약은 Metal readback을 `before-compositor`로 표기한다. 전체 Simulator screenshot은 별도 후합성 캡처로 비교했다. 실제 display scanout은 검사하지 않았다. |
| 18 | C ABI에 null·잘못된 count·중복 좌표가 들어오면 무효 readback을 예약하는가 | FFI test는 null host/output 및 sample count 0/9를 거부한다. 좌표 중복 검사는 10 pair마다 수행하며 renderer가 drawable bounds를 확인한다. comparator negative controls도 중복·잘못된 좌표를 거부한다. |
| 19 | C18.2가 C18.1-only/device-specific payload를 기본 앱에 끌어오는가 | Cargo, Android APK ARM64 library, iOS app executable의 default/opt-in scan에서 C18.2 ABI 세 개, JS 전체 bytes, PNG 전체 bytes가 default에는 없고 opt-in에만 있음을 확인했다. |
| 20 | 성공 화면이 sample failure 뒤에도 완료라고 표시되거나, C18 전체 지원으로 과장되는가 | 두 앱은 exact 10/10 이후에만 완료 상태를 내고 comparator는 summary와 sample별 `exact=true`를 확인한다. internal contract·STATUS·conformance는 C18.2 fixture slice만 완료로 두며 C18/C14/U05 및 공개 image API는 미완료로 유지한다. |

## 보정 후 확인

- Android C18.2: PixelCopy `10/10`, full-screen `10/10`, outside anchors `4/4`.
- iOS C18.2: Metal drawable `10/10`, full-screen `10/10`, outside anchors `4/4`.
- Android/iOS C18.1 regression: drawable/surface `9/9`, screenshot `9/9`, outside anchors `4/4` 각 통과.
- `cargo test -p spinon-render-wgpu`, C18.2 feature FFI tests, CSS comparator/module-boundary/build-flag Node tests, targeted Clippy, rustfmt 및 diff check 통과.

저장된 원본 run과 comparator 결과는 [Android evidence](./c18-2-alpha-image-android-api37-2026-10-16/README.md), [iOS evidence](./c18-2-alpha-image-ios-simulator-2026-10-16/README.md), [artifact boundary](./c18-2-alpha-image-android-api37-2026-10-16/artifact-boundary/README.md)에 둔다.
