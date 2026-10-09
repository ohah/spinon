# 0028 · C04.7 CSS media 환경 입력

**계약 버전:** `0.1.0` · **상태:** 내부 Rust API 구현 · **제품 API:** 없음 · **제품 runtime 연결:** 미구현

## 목적과 경계

`CssViewport`에 색상 scheme, primary pointer/hover, 모든 pointer capability를 넣어 Stylo 0.22 cascade에서 CSS media query 입력으로 사용한다. 기존 `FlexMarginV1`의 허용 범위를 바꾸지 않도록 새 profile `FlexMediaEnvironmentV1`과 `compute_flex_media_environment_cascade`를 추가한다.

이 계약은 동기 내부 Rust 계산 API다. OS 설정을 읽거나 변경을 관찰하지 않고, JS/CSSOM에 환경 값을 제공하거나 스타일 재계산·Taffy·GPU 표시를 수행하지 않는다. Rust caller는 viewport 또는 media 입력 변경 때 같은 `EnvironmentRevision`을 증가시켜야 한다.

## 타입과 입력

```rust,ignore
pub enum CssColorScheme { Light, Dark }
pub enum CssPrimaryPointer { None, Coarse, Fine }
pub struct CssPointerCapabilities {
    pub coarse: bool,
    pub fine: bool,
    pub hover: bool,
}
pub struct CssMediaEnvironment {
    pub color_scheme: CssColorScheme,
    pub primary_pointer: CssPrimaryPointer,
    pub primary_hover: bool,
    pub all_pointers: CssPointerCapabilities,
}
```

`CssViewport`는 숫자 viewport와 `EnvironmentRevision` 외에 `media_environment`를 포함한다. `CssViewport::C04_FIXTURE`는 결정적인 desktop/light/fine/hover 입력을 사용한다. 모바일 또는 사용자 색상 설정은 caller가 명시해 전달한다.

입력 검증은 cascade 전에 실행한다.

- primary pointer가 coarse/fine이면 전체 capability 집합에도 같은 유형이 있어야 한다.
- primary가 none이면 primary hover는 false여야 한다.
- primary hover가 true이면 전체 hover도 true여야 한다.
- 전체 hover가 true이면 전체 capability에 coarse 또는 fine이 있어야 한다.
- viewport 숫자 입력이 잘못되면 media 입력보다 먼저 `InvalidViewport`를 반환한다.
- 모순된 pointer 입력은 `InvalidMediaEnvironment`를 반환하고 부분 snapshot을 반환하지 않는다.

Primary `pointer`·`hover` query와 전체 `any-pointer`·`any-hover` query는 분리 계산한다. 전체 capability는 coarse와 fine을 모두 포함할 수 있다.

## 제공 함수 및 CSS profile

```rust,ignore
pub fn compute_flex_media_environment_cascade(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<ComputedStyleSnapshot, CssCascadeError>;
```

`ComputedStyleSnapshot.profile`은 `FlexMediaEnvironmentV1`이고 입력 viewport·환경 revision을 그대로 보존한다. 기존 C04.6 Flex margin profile과 같은 CSS property allowlist를 사용한다.

이 profile의 `@media` 조건은 아래 값만 허용한다.

| CSS feature | 입력 |
|---|---|
| `prefers-color-scheme` | `light`, `dark` |
| `pointer`, `any-pointer` | `none`, `coarse`, `fine` |
| `hover`, `any-hover` | `none`, `hover` |

`screen`·`all` media type와 `and`·`not`, comma list의 조합만 허용한다. viewport 크기, orientation, reduced-motion, forced-colors, `or`, supports/layer 등 다른 조건이나 at-rule은 `UnsupportedAuthorCss`로 거부한다. `@media` parsing diagnostic도 매치하지 않는 쿼리로 조용히 처리하지 않고 `UnsupportedAuthorCss`로 거부한다. 다른 허용 입력에서 Stylo가 남긴 recoverable diagnostic은 snapshot에 보존한다.

모든 괄호 조건은 허용된 feature 하나와 그 feature에 대응하는 값 하나를 가져야 한다. 예를 들어 boolean 형식 `(prefers-color-scheme)`나 다른 feature의 값인 `(pointer: light)`는 허용하지 않는다. Stylo가 복구 과정에서 `@media` 또는 알 수 없거나 disallowed된 at-rule을 버린 경우도 parse diagnostic으로 거부한다. 일반 선언의 recoverable 진단과 구분해 판정한다.

## 사용 예

```rust,ignore
let environment = CssMediaEnvironment {
    color_scheme: CssColorScheme::Dark,
    primary_pointer: CssPrimaryPointer::Coarse,
    primary_hover: false,
    all_pointers: CssPointerCapabilities {
        coarse: true,
        fine: false,
        hover: false,
    },
};
let viewport = CssViewport {
    width_css_px: 390.0,
    height_css_px: 844.0,
    device_scale_factor: 3.0,
    environment_revision,
    media_environment: environment,
};
let computed = compute_flex_media_environment_cascade(
    &view,
    &author_stylesheets,
    viewport,
    style_revision,
)?;
```

이 예시는 앱 화면이나 JS API가 아니라 caller가 제공한 입력을 계산하는 내부 Rust 호출이다.

## 비교 기준과 검증 범위

고정 Chromium `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`, macOS `26.5.1` arm64, viewport `390×844 CSS px`, DPR `3`, locale `en-US`, time zone `UTC`를 사용한다. desktop/mobile × light/dark 4 case에서 12개 `matchMedia()` 결과와 대응 CSS probe의 computed `display`를 보존한다. mobile은 Chromium CDP touch emulation 결과가 coarse/no-hover인지 실행 중 확인한다.

[고정 입력](../../tests/fixtures/css/c04/media-environment.v1.json), [CSS/HTML fixture](../../tests/fixtures/css/c04/media-environment.css), [Chromium reference](../../tests/fixtures/css/references/c04-media-environment-v1.json), [구현·검토 근거](evidence/css-c04-media-environment-2026-10-09.md)를 사용한다. mixed pointer와 포인터가 없는 입력은 이 CDP emulation에서 관찰하지 않았으며 Stylo 매핑 테스트일 뿐 Chromium 실측값으로 취급하지 않는다. Android·iOS OS 환경 수집 및 앱 runtime 업데이트는 검증하지 않는다.
