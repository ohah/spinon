# C11.3.3 unknown-area implicit-line 구현 실패 관점 검토 20개

**대상:** C11.3.3 전용 trailing implicit named-line resolver, `TemplateGridImplicitTracks`, 앱 fixture 진입·표면 캡처. 기존 C11.3.1–3.2와 공개 지원 계약은 범위 밖이다.

| # | 독립 실패 관점 | 확인·수정 결과 |
|---:|---|---|
| 1 | C11.3.3 opt-in이 꺼져도 missing name을 성공 처리한다 | 기존 entry point가 missing name을 거부하고 새 wrapper만 implicit resolver를 호출하는 경계를 확인했다. |
| 2 | explicit authored line name보다 implicit fallback이 먼저 선택된다 | authored name lookup 및 area alias 해석 뒤 fallback하는 precedence와 기존 회귀 테스트를 확인했다. |
| 3 | `grid-template-areas`의 area edge alias가 unknown-name 처리에 가려진다 | `area.rs` 경계와 explicit/implicit area alias fixture를 확인했다. |
| 4 | 마지막 explicit boundary를 implicit track로 오인해 첫 implicit track을 건너뛴다 | line index에 첫 trailing track 경계가 보존되는 Chrome 154 기준과 placement test를 확인했다. |
| 5 | duplicate name occurrence나 대소문자 비교가 authored metadata를 변형한다 | 기존 named-line occurrence 테스트와 case-sensitive identifier 전달을 확인했다. |
| 6 | `grid-area` shorthand의 4개 longhand 중 하나만 missing name으로 누락된다 | shorthand fixture의 row/column start/end 경로와 typed projection을 확인했다. |
| 7 | row 축과 column 축의 search 방향이 서로 다르게 처리된다 | 동일 resolver의 축별 `SearchDirection` 사용과 26-scene matrix의 row/column case를 확인했다. |
| 8 | named span이 반대쪽 definite line을 기준으로 trailing 방향이 아닌 곳에서 확장된다 | span resolver가 opposite definite line에서 검색하고 허용된 forward shortage만 생성하는지 확인했다. |
| 9 | conflict 규칙이 버린 named span이 불필요한 implicit line을 만든다 | 충돌로 버린 endpoint의 implicit 표시를 초기화하는 resolver 분기와 named-line regression을 확인했다. |
| 10 | numeric positive line이 새 mode를 빌려 일반 implicit line을 만든다 | endpoint validator가 explicit boundary 밖 unmarked numeric line을 거부한다. |
| 11 | `line / span`이 명시 grid 밖 track extent를 우회 생성한다 | unmarked extent 검사와 `opt_in_does_not_accept_unmarked_numeric_span_extent_in_a_sibling` 회귀를 확인했다. |
| 12 | `span / line`이 leading implicit track을 우회 생성한다 | leading span guard와 `opt_in_does_not_accept_unmarked_leading_span_extent` 회귀를 확인했다. |
| 13 | 마지막 explicit line + `auto`가 암묵적 한 track을 추가한다 | 자동 endpoint extent guard와 `opt_in_does_not_accept_unmarked_last_line_auto_extent` 회귀를 확인했다. |
| 14 | resolved `span 0`, arithmetic overflow 또는 변환 범위 초과가 wrap한다 | checked add/sub, explicit line normalization, 지원 범위 오류 반환을 확인했다. |
| 15 | `GRID_MAX_TRACKS`/Taffy line 상한을 넘는 입력이 대형 allocation 전에 layout에 전달된다 | 기존 Grid count validation과 checked implicit extent 경계를 확인했다. |
| 16 | row는 고정, column은 auto여도 implicit mode가 성공한다 | `TemplateGridImplicitTracks`가 `grid-auto-rows`와 `grid-auto-columns` 각각의 `Auto`를 거부한다. |
| 17 | implicit named line이 없는 sibling이 다른 item의 mode를 공유한다 | resolver 결과 flag를 실제 generated line에만 설정하고 다른 sibling의 unmarked extent를 별도로 검사한다. |
| 18 | invalid area declaration이 새 implicit 처리 때문에 정상 cascade fallback 대신 부분 layout을 낸다 | 26-scene cascade fallback/expected rejection과 no-partial-output FFI assertion을 확인했다. |
| 19 | hidden/nested sibling이나 paint order가 확장 과정에서 바뀐다 | nested-hidden source-order scene의 frame·paint 결과가 양 플랫폼에서 보존되고 전체 matrix가 통과했다. |
| 20 | Android route 또는 표면 sample allowlist 누락으로 앱 결과가 다른 scene을 측정한다 | 초기 검증에서 Android C11.3.3 intent route와 overflow PixelCopy sample binding 누락을 수정했다. 최종 AVD, SM-S731N Android 16/API 36 실기기, iOS Simulator matrix와 3/3 overflow sample로 재확인했다. 실기기 SurfaceFlinger renderer는 ANGLE/Xclipse 940/Vulkan 1.3.279다. |

## 재현 및 남은 한계

최종 Android API 37 ARM64 AVD, SM-S731N Android 16/API 36 실기기, iPhone 17 Pro/iOS 26.2 Simulator는 각각 26 scene·65 node, visible frame 61개, 최대 geometry 오차 0 CSS px, unknown-area overflow sample 3/3 exact다. AVD는 ANGLE/Vulkan/SwiftShader이고 실기기는 ANGLE/Xclipse 940/Vulkan 1.3.279이며 iOS readback은 compositor 제출 전이다. Chrome의 흰 viewport backdrop과 native host-owned surface 배경은 비교에서 제외했고 border stroke는 검사하지 않았다. 실기기 전체 화면에는 개인 화면이 섞일 수 있어 앱 SurfaceView PixelCopy만 증거로 보존한다.

`mise exec -- bun run test`는 origin/main에도 없는 C04, C11.3.2, C13.3 static source expectation 네 개에서 CSS-reference 단계가 실패했다. 변경하지 않은 assertion 문제로 분리 기록하며 전체 suite 성공이라고 보고하지 않는다. Spinon WPT 적합성, 다른 Android 기기/driver, iOS 실기기·compositor 이후 픽셀, general/leading/negative implicit grid line, auto/intrinsic implicit track은 검증하지 않았다.
