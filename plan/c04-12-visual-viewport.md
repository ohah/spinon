# C04.12 visual viewport와 CSS layout 높이

## 목적

루트 HTML `meta viewport` 환경에서 CSS layout viewport와 초기 visual viewport를 별도 값으로 보존한다. 현재 `viewport_height_css_px = screen_height_css_px / page_scale` 계산은 고정 width가 화면 폭보다 넓을 때 layout 높이 대신 visual 높이를 반환한다. Chrome 155 모바일 oracle에서 `width=500, initial-scale=1`은 layout viewport `500×1082`와 visual viewport `390×844`로 관찰됐다. 이 차이는 `@media (height)`와 layout의 기준을 바꾼다.

이 작업은 viewport 환경 계산과 native 반환값을 고친다. Renderer가 page scale을 그리기 transform으로 적용하거나 root HTML body를 layout·paint하는 작업은 포함하지 않는다. C04.12는 이 변경 뒤에도 미완료다.

## 구현 계약

입력은 기존과 같이 surface physical pixel, 양수 DPR, root HTML metadata와 OS/media 환경이다. `screen_css = surface_physical / DPR`로 계산한다. 현재 metadata allowlist와 page-scale 결정 규칙은 바꾸지 않는다.

- `layout_width = max(requested_width, screen_css_width / page_scale)`.
- `layout_height = layout_width × surface_physical_height / surface_physical_width`.
- `visual_width = screen_css_width / page_scale`.
- `visual_height = screen_css_height / page_scale`.
- CSS viewport media query와 cascade 입력은 layout width/height를 사용한다. DPR과 page scale은 서로 곱하거나 덮어쓰지 않는다.
- 초기 visual viewport origin은 `(0, 0)`이다. pinch zoom, scrolling, safe area, orientation event 구독은 별도 범위다. surface 크기가 바뀌어 환경을 다시 제출하면 두 viewport를 새 입력에서 계산한다.
- Rust `RootHtmlEnvironmentSnapshot`, C ABI `SpinonRuntimeRootHtmlEnvironment` 및 Canvas 응답에 visual width/height를 추가한다. Android/iOS의 C ABI 반환 구조와 environment 로그 형식을 함께 갱신한다.
- 잘못된 surface, scale, layout/visual viewport 산술 결과는 기존처럼 cascade 전에 실패한다. 성공 결과 일부만 게시하지 않는다.

## 구현 전 비교 기준

- 기준: Agent Browser CLI `0.31.1`, Chrome `155.0.8059.85`, iPhone 12 emulation, `screen=390×844`, DPR 3, surface `1170×2532`.
- fixtures와 원본 캡처는 [`사전 비교 근거`](../spec/internal/evidence/c04-12-visual-viewport-2026-10-17/README.md)에 둔다.
- 비교값: `document.documentElement.clientWidth/clientHeight`(layout), `visualViewport.width/height/scale`, DPR, CSS width/height media query 결과, 절대 배치 probe의 캡처 pixel bounding box.
- 수치 기준: metadata layout/visual 크기와 media 결과는 fixture 계산과 정확 비교한다. scale 기반 픽셀 bbox는 물리 pixel 정수 경계를 기준으로 비교하고 실수 viewport는 최대 `0.001 CSS px` 차이를 허용한다.
- `innerWidth`는 이 headless emulation에서 `documentElement.clientWidth`와 일치하지 않았다. layout 판단에는 `clientWidth/clientHeight`와 `matchMedia`를 사용한다. viewport meta가 없는 상태에서 emulation 재설정 전후의 live JS 값과 캡처도 서로 불일치했으므로, 그 세션의 no-meta pixel bbox는 oracle로 인용하지 않는다. 기존 no-meta metrics reference는 회귀 기준으로 유지한다.

## 구현 범위와 검증

1. `ResolvedRootHtmlEnvironment`와 공개 Rust snapshot에 visual viewport width/height를 추가한다.
2. layout height를 resolved layout width와 surface aspect ratio에서 계산한다. CSS cascade에는 layout height가 전달되어야 한다.
3. C ABI의 environment와 Canvas nested environment에 visual 크기를 반환하고, C header·iOS·Android bridge를 같은 필드 순서로 갱신한다.
4. 단위 fixture는 `width=device-width` scale `0.5`, `2`, 고정 width `500` scale `1`, 기존 기본·device-width cases를 검사한다. 고정 width `500`의 `@media (min-height: 1000px)`는 참이고 `(max-height: 900px)`는 거짓이어야 한다.
5. opt-in C04.12 runtime fixture는 Chrome 기준으로 layout/visual viewport 및 media 결과를 검사한다. Android API 37 ARM64 AVD와 iOS Simulator에서 같은 fixture를 실행하고 원본 log와 화면을 남긴다.
6. `cargo fmt --all -- --check`, 범위 crate tests/Clippy, Android·iOS simulator builds, 실제 앱 fixture 실행, agent-browser cleanup을 기록한다. 실기기나 root HTML body paint로 결과를 확대 해석하지 않는다.

## 명시적 제외와 완료 표시

이 작업은 page-scale renderer transform, native physical input→CSS coordinate 변환, pinch/scroll 시 visual viewport offset, text rerasterization, `visualViewport` JavaScript API, safe-area 처리, desktop viewport metadata, 일반 HTML layout·paint를 구현하지 않는다. C04.12의 전체 체크를 변경하지 않는다. C04.12 하위 viewport 환경 단계만 별도 근거를 갖고 완료 처리할 수 있다.
