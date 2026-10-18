# C04.12 기본 root HTML runtime PR diff 실패 관점 검토 · 30개 · 2026-10-19

검토 대상은 현재 브랜치의 구현·FFI·Android/iOS 연결·문서·실행 근거 전체다. 사전 계획 review 20개와 구현 review 20개를 재사용하지 않고, 최종 변경의 별도 실패 경로와 게시 경계를 확인했다. 구현에서 발견한 hidden subtree 실패는 구현 review에서 수정했고, 이 PR 검토에서는 수정 후 코드와 회귀 결과를 다시 확인했다.

| # | 실패 관점 | 확인 및 판정 |
| --- | --- | --- |
| 1 | null, 빈 source, 1 MiB 초과, 잘못된 UTF-8이 raw slice 생성 전에 거부되지 않을 수 있다. | `ffi.rs`는 null·길이 상한·UTF-8을 순서대로 검사한다. fixture-test feature의 새 FFI 회귀 테스트 2개가 네 입력을 확인한다. |
| 2 | source bytes가 `RuntimeGpuHost` 수명까지 보존된다고 가정해 호출 뒤 dangling될 수 있다. | FFI 호출 안에서 `ParsedRootHtml`이 동기 파싱하고 `HostDocument`에 소유된다. Android `byte[]`와 iOS `String`은 native call을 마칠 때까지 살아 있다. |
| 3 | C 헤더의 포인터/길이 타입과 Rust `extern "C"` 시그니처가 다를 수 있다. | 헤더는 `const uint8_t *`와 `size_t`를 선언하고 Rust는 `*const u8`와 `usize`를 받는다. Android JNI와 Objective-C++ bridge도 길이를 명시한다. |
| 4 | 일반 제품 ABI가 fixture feature에만 실리거나 반대로 fixture ABI가 기본 산출물에 들어갈 수 있다. | 제품 archive에는 root HTML 생성 symbol이 있고 C04.12 text fixture symbol/Ahem payload는 없다. fixture-test archive positive control에는 둘 다 있다. 해시와 크기는 `artifact-boundary.txt`에 기록했다. |
| 5 | 일반 constructor가 앱이 쓰던 기존 parsed HostDocument와 다른 document를 만들 수 있다. | 일반 GPU constructor는 `ParsedRootHtml::parse` 결과를 기존 session 초기 문서로 넘긴다. 별도 합성 tree나 selector 기반 text 경로는 없다. |
| 6 | quirks mode나 HTML이 아닌 root가 제한 renderer에 정상 문서처럼 들어갈 수 있다. | no-quirks 조건은 parser 단계에서, `html` root와 HTML namespace 조건은 layout adapter에서 검사한다. 기존 quirks 거부 테스트와 새 scene 검증이 이를 나눈다. |
| 7 | root/document generation이 다른 cascade 결과를 paint에 부착할 수 있다. | `prepare`/`attach`가 generation, document/render-tree/style/environment revision과 viewport bits를 비교하고 mismatch는 scene publication 전에 실패한다. |
| 8 | OS scheme, meta viewport 또는 색 scheme 입력이 매체 cascade와 다른 viewport를 만들 수 있다. | 기존 environment tests가 meta viewport와 OS `prefers-color-scheme`을 확인한다. Android/iOS app은 같은 asset과 scheme 전환을 사용했고 environment revision이 로그에 남는다. |
| 9 | `display:none` subtree의 unsupported style이 visible 문서 전체를 실패시킬 수 있다. | 실제 `word-break: keep-all` 입력으로 재현해 visible element만 style 변환하도록 수정했다. 회귀 테스트에서 hidden text가 없고 visible 형제 scene은 준비된다. |
| 10 | head/meta/style/script 또는 HTML 비표시 요소가 일반 Block으로 그려질 수 있다. | visible tree가 HTML UA display와 비렌더 요소 allowlist를 함께 검사하고 visible unsupported 요소를 fail-closed한다. |
| 11 | 순환·중복 node 또는 문서 밖 text source가 layout 입력에 섞일 수 있다. | tree 수집은 중복 `NodeId`를 거부하고, source node는 snapshot 안의 실제 Text node인지와 HostDocument text를 검증한다. |
| 12 | visible 외부 namespace나 미지원 inline tag가 조용히 빈 텍스트/Block으로 바뀔 수 있다. | HTML namespace와 block/inline tag 범위를 검사해 미지원 visible element에 node ID를 포함한 오류를 반환한다. |
| 13 | computed property가 없거나 지원하지 않는 값이 임의 기본값으로 대체될 수 있다. | 필수 property lookup과 각 enum/length/color 변환이 missing/unsupported 오류를 반환하며 `prepare` 실패는 scene 전체를 막는다. |
| 14 | font family, weight, italic/oblique computed 값이 shaper 요청에서 사라질 수 있다. | paragraph와 inline run style이 family/size/weight/style을 전달하고, Parley shaped run metadata에서 선택 face와 synthesis 정보를 보존한다. |
| 15 | 시스템 font discovery가 꺼져 앱 텍스트가 fixture font나 고정 glyph로 나올 수 있다. | runtime engine은 OS font discovery를 사용하고 fixture engine의 no-system-font 경로는 별도로 유지한다. 양 플랫폼 빌드와 실제 app screen/log에서 source text batch를 확인했다. |
| 16 | `line-height: normal`이 임의 배수로 바뀌어 실제 glyph metrics와 line box가 어긋날 수 있다. | runtime raster snapshot은 run의 ascent/descent/leading를 보존하고 normal line box를 이 metrics에서 계산한다. |
| 17 | CSS whitespace collapse가 UTF-16 원문 위치와 UTF-8 paragraph 범위를 어긋나게 할 수 있다. | normalizer는 source UTF-16 range와 paragraph UTF-8 range를 저장한다. attach는 HostDocument 원문과 source text를 대조하며 UTF-16 변환 오류를 거부한다. |
| 18 | 여러 inline 조상의 font style/color 상속이 바깥 조상으로 덮일 수 있다. | source의 inline ancestor 경로에서 가장 안쪽 owner를 사용한다. 하나의 shaped run이 서로 다른 source colors를 합치면 잘못 섞지 않고 실패시킨다. |
| 19 | bidi/RTL run의 색이나 baseline이 glyph 순서와 독립적으로 연결될 수 있다. | raster glyph는 run index를 유지하고 text source mapping으로 run color를 결정한다. viewport 방향 및 text direction은 computed style에서 전달된다. |
| 20 | emoji/color glyph가 alpha mask pipeline에서 단색 mask로 왜곡될 수 있다. | `GlyphBitmapContent::AlphaMask` 외 raster 결과는 명시적으로 거부한다. 이 제한은 내부 계약과 evidence 범위에 남아 있다. |
| 21 | NaN 좌표, 0 scale, 과대 glyph bitmap 또는 산술 overflow가 raster/scene에 들어갈 수 있다. | scale/frame/glyph 유한성, 크기·snapshot byte limit, checked multiplication과 `Result` 경계를 검사한다. 좌표가 finite가 아니면 attach가 실패한다. |
| 22 | Raster snapshot이 다른 font registry/layout에서 만들어진 뒤 현재 scene에 붙을 수 있다. | layout/raster font registry revision 및 engine token을 비교하고 input layout revision과 scene key가 일치해야 attach한다. |
| 23 | glyph color가 inline owner가 아니라 paragraph fallback color로 전부 칠해질 수 있다. | glyph별 color vector 길이가 glyph 수와 일치해야 하며 shaped source에 연결한 computed color가 실제 `RuntimeRenderText`로 전달된다. |
| 24 | inline layout frame이 element box로 그려져 중복 배경/테두리를 만들 수 있다. | root text profile의 layout frame을 document-contained paint frame 집합으로 줄이고 inline frame은 box renderer에 넘기지 않는다. |
| 25 | text가 owner box paint보다 앞/뒤에 붙어 DOM paint order가 달라질 수 있다. | 기존 paint command 순서에서 각 `DrawBox` 다음에 owner `DrawText`를 추가한다. owner box 누락은 전체 scene attach 실패다. |
| 26 | text clip이 owner overflow clip과 달라 box 바깥 glyph가 노출될 수 있다. | `RuntimeRenderText` 생성 시 owner box clip을 그대로 사용하고 attach 뒤 box/text clip equality를 검사한다. overflow 조합 전체의 별도 readback matrix는 이 PR에서 검증하지 않았다. |
| 27 | CSS px, device scale, surface 좌표 변환이 플랫폼별로 달라 glyph가 잘리거나 이동할 수 있다. | 동일 320×240 CSS viewport와 device scale 환경으로 Android/iOS를 실행했고 WGPU draw 로그 및 Simulator 화면을 보존했다. 화면 캡처만으로 모든 scale/rotation 조합을 증명하지 않는다. |
| 28 | color conversion과 alpha blending이 surface sRGB format에서 이중 인코딩될 수 있다. | glyph pipeline은 surface format의 sRGB 여부에 따라 encoding 경로를 선택한다. CSS text color 입력은 정확히 표현 가능한 8-bit sRGB로 제한하고 wide-gamut은 거부한다. |
| 29 | 앱 route 인수가 다른 fixture route와 동시에 들어와 잘못된 host constructor가 선택될 수 있다. | Android extras와 iOS launch arguments가 `spinon_c*`/`--spinon-c*` 충돌을 확인하고, default route에서는 번들 HTML asset을 읽어 일반 source API만 호출한다. |
| 30 | preview/PR 문서가 캡처를 PixelCopy나 compositor readback으로 과장하거나 오래된 링크를 가리킬 수 있다. | 로드맵의 PixelCopy 표현을 실제 AVD/iOS 전체 화면 캡처와 미실행 readback으로 고쳤다. 최종 문서 동기화 뒤 Agent Browser에서 네 이미지 링크, evidence 상대 링크와 inline image dimensions를 확인하고 결과를 manifest에 기록했다. |

## 검토에서 반영한 구조·문서 수정

- 862줄 단일 adapter를 scene 연결, computed-style 투영, DOM tree 검증 모듈로 분리했다. 동작 변경 없이 `cargo fmt`와 focused runtime test를 다시 확인했다.
- C ABI 입력 경계 회귀 테스트를 추가했다. null/empty, 1 MiB 초과, malformed UTF-8을 생성 전에 거부한다.
- 공개 roadmap의 Android PixelCopy 주장을 실제 캡처 방식에 맞게 수정했다. 본 PR은 simulator screen capture와 runtime log를 남기며 PixelCopy/Metal drawable readback 증거는 주장하지 않는다.

현재 통과: text-layout package 37 tests, 관련 layout/render/style/runtime/FFI package tests, runtime default 2 tests, fixture positive-control 1 test, FFI invalid-input 2 tests, product-feature `spinon-ffi` cargo check, fixture module boundary 6 tests, `cargo fmt --check`, `git diff --check`, 최종 소스 Android/iOS Simulator app 재빌드와 light/dark 재실행. PixelCopy/Metal drawable readback과 physical-device coverage는 하지 않았다.

## 30개 검토 뒤 최종 코드 델타 확인

- `TextRun::PartialEq`는 Parley Blob ID 대신 실제 font bytes와 face index를 비교한다. 이로써 독립 engine에서 같은 font resource를 shape해도 stable equality를 유지하면서, 다른 font resource를 같다고 처리하지 않는다. 기존 snapshot/raster equality 회귀를 포함한 text-layout 37개와 관련 package suites가 통과했다.
- corrupt font index 거부 assertion이 실제 요청 family `Roboto`를 확인하도록 바로잡혔다.
- 최종 소스로 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 앱을 다시 빌드·실행하고 light/dark 화면을 recapture했다. 업데이트한 screenshot hash, apk hash, test/build logs는 README와 `SHA256SUMS`에서 대조한다.
