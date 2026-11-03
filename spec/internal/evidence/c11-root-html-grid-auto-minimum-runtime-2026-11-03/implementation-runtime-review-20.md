# C11.1 Root HTML Grid auto minimum 구현·런타임 실패 관점 검토 20개

검토 대상은 기본 `RuntimeRootHtmlTextPaintV1` 경로의 실제 구현, 0fr·식별자 변경 런타임 회귀, Android/iOS 캡처 계측, 기본·opt-in FFI 산출물이다. 구현 전 계획 검토 20개와 구별되는 코드·실행 검토이며, 계획 검토 표를 재사용하지 않았다.

| # | 구현 실패 경로 | 검사 결과 |
|---:|---|---|
| 1 | computed track에서 `auto` minimum이 Taffy `Auto` 대신 고정 0으로 투영된다 | **통과** — typed `MinMax` shape가 `GridTrackMinSizingFunction::Auto`로 전달되고 양 축 full-frame 테스트가 통과했다. |
| 2 | `minmax(auto, 40px|percentage|auto)`가 지원 breadth로 잘못 수용된다 | **통과** — exact `Auto + finite non-negative Fraction`만 통과하고 나머지는 같은 node/property 오류로 닫힌다. |
| 3 | 음수·NaN·무한 `fr` factor가 projection 또는 layout preflight를 통과한다 | **통과** — profile validator의 finite/non-negative 조건과 negative matrix를 대조했다. |
| 4 | content-derived auto base가 gap preflight에서 확정 0px track으로 취급되어 Taffy 기여 계산을 건너뛴다 | **통과** — preflight는 fixed extent만 세고 `Auto` sizing function은 Taffy에 보존된다. 같은 80px 기여가 실제 track frame으로 확인됐다. |
| 5 | `minmax(auto, 0fr)`에서 0 factor가 auto base를 제거하거나 남은 공간을 차지한다 | **통과** — 구현 검토 중 빠진 기본 runtime 경계를 추가했다. columns/rows 모두 `[80,40]`, item frame, 18px spill 및 cascade/layout key 일치를 검증했다. |
| 6 | columns와 rows가 같은 물리 축 basis를 공유하거나 축 크기 0에서 fallback한다 | **통과** — column width와 row height를 별도 검사하며 0px·auto axis negative cases가 해당 CSS property로 실패한다. |
| 7 | computed min/max size가 definite basis를 덮어 track 결과가 제한되거나 확장된다 | **통과** — min-width/max-width override가 profile 적용 전에 거부되고 positive axis는 고정 computed size로 유지된다. |
| 8 | aspect-ratio가 반대 축에서 자동 minimum의 크기를 전파한다 | **통과** — aspect-ratio 설정은 fail-closed되고 positive scene은 ratio 미설정이다. |
| 9 | item overflow hidden/auto/scroll/clip의 scroll-container minimum을 visible minimum과 혼합한다 | **통과** — 네 overflow 값이 각각 item `overflow-x` unsupported error로 실패한다. |
| 10 | fraction `minmax()` 비교 장면에서만 필요한 root border 허용이 음수·비유한 값이나 item border까지 열어준다 | **통과** — root 예외는 finite non-negative 값과 finite-fr maximum track이 있는 Grid container에 한정된다. fixture의 2px root border와 0px item border가 확인됐다. |
| 11 | fixed 40px neighbor, root border, gap이 자동 minimum 계산에 중복 반영된다 | **통과** — content extent와 border box를 분리해 계산하고 양 축의 root/item frame이 사전 기준과 일치한다. |
| 12 | 두 번째 direct child가 예상한 row auto-placement 위치를 벗어나 비교값을 우연히 맞춘다 | **통과** — explicit supported track 하나와 fixed neighbor, 두 direct child의 row placement를 고정하고 columns/rows full frame을 검사한다. |
| 13 | overflow 길이를 content edge 기준으로 계산해 18px border 바깥 결과를 숨긴다 | **통과** — root border box 기준 runtime frame과 외부 픽셀 표본을 모두 확인했다. control 바깥 표본은 흰색이다. |
| 14 | unsupported sibling 검증 중 앞선 frame 일부가 layout snapshot에 남는다 | **통과** — `minmax(auto,40px)` 입력은 layout 실패, `completed=None`; cascade만 완료된 상태로 남는다. |
| 15 | layout과 computed style이 서로 다른 source/style revision에서 publish된다 | **통과** — normal·0fr·label 변경 장면에서 layout 완료 key와 cascade 완료 key가 같다. |
| 16 | DOM ID나 CSS class 이름이 product success selector로 쓰여 이름 변경 시 결과가 달라진다 | **통과** — fixture의 모든 ID와 class를 바꾼 입력에서 paint frame vectors가 baseline과 같음을 확인했다. |
| 17 | Android 캡처 extra가 기본 root HTML route를 C11 opt-in fixture route로 바꾸거나 복수 캡처 플래그 중 하나를 실행한다 | **통과** — 단일 캡처는 `SPINON_ROOT_HTML_DEFAULT_APP_ROUTE`를 유지한다. auto-minimum+fit-content 충돌 실기기 없는 AVD 실행은 충돌 오류를 남기고 PixelCopy dispatch를 하지 않는다. |
| 18 | Android PixelCopy가 오래된 surface generation·잘못된 density 좌표·control 바깥의 non-white 값을 통과시킨다 | **통과** — API 37 ARM64 AVD에서 generation 2, 1080×2209 surface, 네 RGBA 표본이 4/4 일치했다. 이전 run ID 재사용은 기존 PNG 충돌로 안전하게 거부되어 pass로 세지 않았다. |
| 19 | iOS Metal 표본이 simulator scale을 잘못 변환하거나 compositor 이후 장면인 것처럼 보고된다 | **통과** — iPhone 17 Pro/iOS 26.2 Simulator에서 402×778 CSS viewport, 3× drawable, 1206×2334 Metal surface와 네 RGBA 표본 4/4를 확인했다. 증거는 compositor 제출 전 drawable까지만 의미한다. |
| 20 | C11.1 capture 전용 ABI/payload가 기본 artifact에 포함되거나 Android PixelCopy가 새 FFI route를 요구한다 | **통과** — 기본 Cargo staticlib와 기본 Android native artifact에는 C11 surface-pixel fixture 심볼이 없다. opt-in positive control에만 기존 generic C11 readback 심볼이 있다. Android 표본은 시스템 PixelCopy를 사용하며 C11.1 fixture FFI를 추가하지 않았다. |

## 수정 및 재검증

- 검토에서 0fr의 style projection은 있었지만 default runtime 최종 배치 테스트가 빠진 것을 확인했다. 양 축 0fr runtime 테스트를 추가했고 해당 Rust test 4개가 통과했다.
- Android 캡처 인수에서 auto-minimum과 fit-content percentage가 함께 오면 auto-minimum 요청만 살아날 수 있었다. 충돌 상태에서 두 경로 모두 dispatch하지 않도록 조건을 고쳤고 API 37 AVD의 음성 실행에서 오류 로그만 남고 capture dispatch가 없는 것을 확인했다.
- Android 유효 capture는 새 run ID `c11autominandroid06`로 다시 실행해 PixelCopy 4/4 exact를 확인했다. 같은 run ID의 이전 PNG 충돌 로그와 오래된 좌표 표본 불일치는 유효 pass 증거에서 제외했다.
- 기본 FFI artifact와 opt-in positive control은 각각 다시 빌드해 심볼 부재/존재를 대조했다. Rust 소스 변경의 `cargo fmt --all -- --check`와 당시 source diff whitespace 검사는 통과했다.
- PR diff review에서 auto-minimum Metal capture argument 탐지가 iOS feature-flag `#else`에도 포함된 것을 발견했다. 두 fallback 인식 지점에서 제거한 뒤 default iOS Simulator app을 다시 빌드했고, 8개 bundle 파일 scan에서 argument 문자열 0건을 확인했다. 이 artifact 경계 결과는 새 검토 사항이며 초기 20개 표의 다른 관점을 재사용하지 않는다.
- 완전한 PR evidence diff의 `git diff --check`는 원본 Android logcat trailing spaces, ADB CRLF 및 Xcode build log의 padded export/blank lines를 보고한다. 원본 artifact는 SHA manifest로 고정해 byte-for-byte 보존했고, source·문서의 whitespace 검사는 별도로 통과했다.

이 검토는 병합된 제한 구현의 범위만 판정한다. 구현 PR [#1109](https://github.com/ohah/spinon/pull/1109)은 `2026-11-03T06:35:47Z` Rebase and merge됐고 merge SHA는 `9aa813727c79e274ce4d230ede2b0accd261fd87`다. 별도 PR diff 검토 30개는 [전용 기록](pr-diff-review-30.md)에 완료했다. GitHub checks는 보고되지 않았고 main required status checks도 설정되지 않아 CI 통과로 세지 않는다. 공개 로드맵 후속 동기화는 별도 문서 PR에서 진행하며 C11 상위 항목과 공식 완료 수는 변경하지 않는다.
