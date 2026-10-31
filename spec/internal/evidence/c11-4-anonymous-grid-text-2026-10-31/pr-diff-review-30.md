# C11.4 numeric author `tab-size` PR diff 실패 관점 30개

구현/runtime 20개 검토와 분리해 최종 코드·명세·capture tool·증거 변경을 30개 실패 관점으로 확인했다. 기대값은 고정 Chrome 156 비교 결과와 CSS Text contract에 대조했고, 플랫폼 동작은 API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 결과에서 확인했다.

| # | 실패 관점 | 판정과 근거 |
| ---: | --- | --- |
| 1 | 변경이 일반 CSS 지원이나 C11.4 전체 완료로 읽히는가 | **PASS** — plan, 0059, evidence README가 고정 opt-in slice로 한정하고 공식 체크를 바꾸지 않는다. |
| 2 | fixture expected 값이 구현 결과에서 역산됐는가 | **PASS** — Chrome 156 관찰과 독립 Ahem 계산을 사전 expected-values 파일에 기록하고 구현 입력으로 사용한다. |
| 3 | oracle 대상이 계획에 적은 numeric profile과 다른가 | **PASS** — expected matrix는 unitless 1·2·4에 초점을 두며 기존 initial 8은 별도 regression control이다. |
| 4 | `TabSize`가 0이나 fractional 값을 나타낼 수 있는가 | **PASS** — enum은 1·2·4·8만 표현한다. 0·1.5·20px negative 장면은 별도 computed-style 검사에서 닫힌다. |
| 5 | computed `tab-size`가 authored text나 scene ID로 생성되는가 | **PASS** — collector가 computed-style snapshot 문자열을 typed 값으로 읽고 장면 ID는 기대값 일치만 확인한다. |
| 6 | 값이 없거나 expected scene과 다를 때 초기값으로 조용히 대체되는가 | **PASS** — unit test는 missing snapshot과 size 4 장면의 size 2 mismatch를 오류로 고정한다. |
| 7 | invalid value가 root frame 또는 partial output을 남기는가 | **PASS** — Android·iOS negative 3/3이 `frames=absent`와 지정 error code/property/node로 끝났고 screenshot은 만들지 않는다. |
| 8 | tab이 현재 inline position이 아닌 문단 index만으로 확장되는가 | **PASS** — normal character와 앞선 tab 확장을 누적한 cursor로 다음 stop을 계산하고 Ahem 1·2·4 사례의 exact advance가 일치한다. |
| 9 | 이미 stop에 닿은 경우 strict next-stop rule을 위반하는가 | **PASS** — size 1·2·4 exact-stop scene이 다음 stop으로 이동하며 size 4의 `XXXX\tX`는 90px다. |
| 10 | 새 줄 tab이 이전 줄의 cursor를 재사용하는가 | **PASS** — LF/CRLF 두 사례가 줄별 advance를 각각 고정하고 20px 높이 frame을 확인한다. |
| 11 | CRLF가 두 개의 output newline을 만들거나 provenance를 잃는가 | **PASS** — CR과 LF source code unit은 하나의 normalized newline UTF-8 범위를 공유한다. Rust와 두 platform validator가 이를 검사한다. |
| 12 | 연속·leading·trailing tab이 다른 값에만 맞춰져 있는가 | **PASS** — matrix가 이 위치를 size 1·2·4별로 분리하고 expected normalized text와 advance를 비교한다. |
| 13 | `min-content`가 preserved tab text를 잘라 soft-wrap하는가 | **PASS** — size 2 min-content/max-content가 모두 60px이며 한 줄이다. |
| 14 | definite 25px track이 paragraph를 wrap해 overflow를 숨기는가 | **PASS** — root track 25px와 text line 60px를 별도 검사하고 soft-wrap은 없다. |
| 15 | `PreWithTabSize`가 text engine의 wrap/collapse policy에서 빠지는가 | **PASS** — text engine builder는 `NoWrap`과 preserved spaces를 적용하고 67개 text-layout test가 통과한다. |
| 16 | 기본 `Pre` 또는 기존 `PreWrap` tab 계산이 numeric path 때문에 바뀌는가 | **PASS** — 기본 8은 explicit default, PreWrap은 기존 분리된 branch를 유지하고 matrix에 기존 scenes가 포함된다. |
| 17 | 다른 font, spacing, direction 또는 inline topology의 tab이 열리는가 | **PASS** — 고정 Ahem/horizontal LTR/zero-spacing/inline-empty validator와 negative unit tests가 범위를 제한한다. |
| 18 | HostDocument collection이 새 typed mode 때문에 기존 order/profile를 깨뜨리는가 | **PASS** — collect와 compute allowlist 두 곳을 함께 갱신하고 HostDocument anonymous-text tests 32개가 통과한다. |
| 19 | fixture collector가 computed value 검증 전에 paragraph를 layout에 전달하는가 | **PASS** — `validate_grid_style`가 typed `TabSize`를 반환한 뒤만 paragraph style을 만들며 실패는 이전에 반환한다. |
| 20 | text source, track contribution, final item frame 기대값이 서로 다른가 | **PASS** — Chrome case와 capture validator가 normalized text·per-line advance·intrinsic width·tracks·frame을 각각 비교한다. |
| 21 | fixture catalog 중복 또는 source 누락이 생기는가 | **PASS** — Rust FFI catalog는 unique 55개이고 catalog boundary test가 exact IDs를 고정한다. |
| 22 | platform adapter 중 하나가 새 positive/negative ID를 놓치는가 | **PASS** — Android와 iOS 모두 동일 55 IDs의 `sceneCount`와 선택 목록을 검증한다. |
| 23 | Android expected root/track 값이 iOS와 달라지는가 | **PASS** — size 1·2·4, LF/CRLF reset, min-content, fixed overflow의 frame·track 기대값이 두 앱 validator에서 일치한다. |
| 24 | platform source-map validator가 expanded tab을 일반 글자 폭으로 취급하는가 | **PASS** — 두 validator가 tab마다 expected space range를 계산하고 CRLF의 두 원본 범위도 별도 검사한다. |
| 25 | capture script가 기존 155-scene section이나 전체 catalog 수를 손상하는가 | **PASS** — local assertion은 anonymous slice 55 및 전체 catalog 672를 검사하고 Node boundary suite 24/24가 통과한다. |
| 26 | screenshot이 assertion을 대신하거나 glyph를 표시한 것처럼 문서화되는가 | **PASS** — README는 이미지가 fixture surface 제출만 보인다고 설명하고 glyph ink는 runtime trace 결과로 분리한다. |
| 27 | 시뮬레이터가 stale/fixture-off binary를 성공으로 오인하는가 | **PASS** — final Android APK와 설치본, iOS executable과 설치본 SHA-256이 각각 일치한다. 초기 fixture-off 실행은 별도 로그 archive에 남겼다. |
| 28 | fixture ABI·scene payload가 default product archive에 포함되는가 | **PASS** — macOS ARM64 archive 비교에서 기본 0/16·ABI 0개, opt-in 16/16·ABI 2개다. |
| 29 | evidence가 simulator를 실기기, pixel 또는 hardware proof처럼 말하는가 | **PASS** — device/OS/AVD hash를 기록하고 physical device·hardware GPU·compositor 이후 pixel 비교를 명시적으로 제외한다. |
| 30 | 내부 contract version, official status, PR 요구 항목이 부정확하게 바뀌는가 | **PASS** — 숫자 버전 `0.1.0`, 상위 C11.4/C11 checkbox, official `72/213`을 유지하고 PR 본문은 status 예외와 evidence를 설명한다. |

초기 검사에서 발견된 fixture feature 미적용, layout allowlist 누락, Android/iOS expected mode 미갱신, size 4 입력 오기, 단위 테스트 `vec!` lint를 각각 수정했다. corrected builds·tests와 두 simulator 최종 행렬은 [실행 근거](./numeric-author-tab-size.md)에 구분해 기록했다. Required CI 결과 및 PR merge state는 PR 생성 후 별도로 확인해야 한다.
