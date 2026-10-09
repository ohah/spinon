# 0026 · C04.5 지원 HTML UA 계산 스냅샷

**상태:** 내부 Rust API 구현 계약 · **버전:** `0.1.0-draft` · **profile:** `SupportedElementsUaV1`
**범위:** 내장 UA CSS와 author CSS의 제한 computed-style snapshot · **비범위:** 공개 CSSOM, Taffy layout, GPU, V8/runtime 실행기

`SupportedElementsUaV1`은 snapshot이 반환하는 computed property 집합의 내부 profile 식별자다. stylesheet 자원 식별자 `spinon-html-ua/0.1.0-draft`와 버전 공간이 다르며 서로를 대신하지 않는다. 이 API는 호출마다 stylesheet를 파싱·등록하고 subtree를 계산한다. 캐시, 증분 무효화 또는 성능 보장은 계약하지 않는다.

## 목적

컴파일 시 `spinon-style`에 포함되는 `supported-elements-v0.css`를 Stylo의 `UserAgent` origin에 등록하고, 호출자가 고른 HostRoot 직속 요소 하나와 그 하위 요소들의 CSS 계산값을 반환한다. 기존 C04.1 `BasicCascadeV1`은 fixture 전용으로 유지하고, 이 계약은 다른 property whitelist와 profile ID를 가진 호출 가능한 Rust 함수로 분리한다.

이 API는 내부 스타일 계산 단계다. `HostDocumentSnapshot`을 생산하거나 저장하지 않으며 호출 시 전달된 `StyloDocumentView`가 보유한 불변 snapshot만 읽는다. 계산은 호출 스레드에서 동기 실행된다. V8 owner/UI thread에서 실행해도 된다는 보장은 없으며, 실행 thread 선택과 취소·재예약은 호출자/상위 runtime의 책임이다.

## Rust 인터페이스

```rust,ignore
pub fn compute_supported_elements_ua_cascade(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<ComputedStyleSnapshot, CssCascadeError>;
```

crate re-export는 `spinon_style::compute_supported_elements_ua_cascade`다. `StyloDocumentView`는 먼저 절대 document URL, HTML/XML mode, quirks mode와 HostRoot 직속 요소를 지정해 만든다. View가 거부하는 root, generation 또는 URL 오류는 `StyloDomError`다.

## 입력 계약

| 입력 | 의미와 검증 |
| --- | --- |
| `view` | 하나의 불변 `HostDocumentSnapshot`과 HostRoot 직속 요소 subtree를 고정한다. 문서 root 선택·OwnerId 발급 정책은 이 API에서 바꾸지 않는다. |
| `author_stylesheets` | 순서가 의미 있는 인메모리 `Author` stylesheet 목록이다. 각 항목은 고유 ID와 절대 base URL을 가진다. `UserAgent` origin은 전달할 수 없으며 `InvalidStylesheetOrigin`으로 거부한다. 현재 `CssOrigin`에 `User` variant는 없어 user origin 입력은 표현할 수 없다. 저작자 CSS 전체를 cascade 입력으로 받되 결과 snapshot에는 아래 7개 property만 쓴다. |
| `viewport` | 유한한 양수 CSS 폭·높이와 device scale, 환경 revision이다. 값이 유효하지 않으면 `InvalidViewport`를 반환한다. fixture 800×600 상수는 API 기본값이 아니다. |
| `style_revision` | 호출자가 보유한 스타일 입력 revision이며 결과 snapshot에 그대로 보존한다. |

`StyloDocumentView`가 제공하는 요소의 inline `style` 속성도 같은 cascade에서 author stylesheet보다 높은 우선순위로 반영한다. 이 함수는 호출 시점의 immutable view만 읽으며, 문서 변경·새 CSS 입력은 새 view와 재계산으로 반영한다.

## media environment 경계

이 API는 `CssViewport`의 크기·배율만 Stylo viewport로 사용한다. 현재 `Device`는 `screen`, `light` color scheme, 고정 font metrics를 사용하고 pointer capability는 pinned Stylo의 target 기본값을 따른다(Android/iOS는 coarse, 데스크톱은 fine·hover). `environment_revision`은 결과에 echo할 뿐 실제 OS 값을 조회하거나 복원하지 않는다. C01 UA 기본 규칙은 media query를 포함하지 않고 Chromium 기준 author sheet도 비어 있으므로 19개 기본값 비교에는 이 차이가 개입하지 않는다.

함수는 author stylesheet를 입력받지만 target별 pointer 환경 및 이 API가 받지 않는 사용자 선호·지역 환경을 이용한 `@media` parity를 보장하지 않는다. 제품 runtime이 환경 의존 CSS를 연결할 때 media type, primary/any pointer·hover, color scheme, motion/contrast 선호, font metrics 등 입력 계약과 revision 갱신을 별도 결정해야 한다.

내장 `supported-elements-v0.css`는 항상 `Origin::UserAgent`로 author sheets보다 먼저 등록된다. 따라서 일반 origin 우선순위에 따라 author declaration이 UA 기본값을 덮어쓴다. CSS fetch는 없다. `@import` 등 Stylo 파서 진단은 결과 `diagnostics`에 남고 원격 자원을 불러오지 않는다. stylesheet ID/base URL 오류는 `CssCascadeError::StylesheetRegistry`로 반환한다.

## 출력 계약

결과 `ComputedStyleSnapshot.profile`은 `SupportedElementsUaV1`이며, 각 요소의 `properties`에는 아래 7개 이름의 computed CSS serialization을 넣는다.

| CSS property | UA 규칙에서 관찰하는 주요 요소 |
| --- | --- |
| `display` | `div`, `span`, `a`, `img`, `button`, `input`, `p`, `ul`, `li` |
| `list-style-type` | `ul` |
| `margin-block-start` | `p`, `ul` |
| `margin-block-end` | `p`, `ul` |
| `margin-inline-start` | `p`, `ul` |
| `margin-inline-end` | `p`, `ul` |
| `padding-inline-start` | `ul` |

이 profile은 HostRoot 직속 선택 요소부터 view subtree의 모든 요소에 동일한 7개 whitelist property를 직렬화한다. C01 inventory의 19개 feature는 그 중 요소별 선언을 실제 Chromium oracle과 비교하는 대상이다. 비교 대상 9개 외 요소가 반환되지 않는다는 뜻은 아니다. text node에는 computed element style이 없다. 결과는 `DocumentGeneration`, `DocumentRevision`, `RenderTreeRevision`, 입력 `StyleRevision`, viewport 및 environment revision을 보존한다.

## 실패·진단

- 잘못된 viewport는 cascade 시작 전에 `InvalidViewport`로 거부한다.
- author 목록에 비-author origin이 있으면 해당 source ID와 함께 `InvalidStylesheetOrigin`으로 거부한다.
- stylesheet 등록 문제(빈/중복 ID, 잘못된 절대 base URL)는 registry 오류로 반환한다.
- CSS 문법 또는 로더 부재 진단은 별도 예외로 승격하지 않고 결과 `diagnostics`에 source ID·가능한 경우 HostNode ID와 함께 보존한다. 이 API는 진단이 없음을 보장하지 않는다.
- 이전 snapshot에서 만든 view는 이후 HostDocument 변경을 반영하지 않는다. 새 문서 revision에는 새 view를 만들고 새 계산을 제출해야 한다.
- `inline`, `inline-block`, `list-item` 값 계산은 line box·replaced element·목록 marker 레이아웃을 구현하지 않는다. 이 출력만으로 화면 배치를 주장하지 않는다.

## 사용 예

```rust,ignore
let view = StyloDocumentView::new_with_base_url(
    document_snapshot,
    app_root,
    true,
    QuirksMode::NoQuirks,
    "https://app.invalid/document.html",
)?;
let viewport = CssViewport {
    width_css_px: 390.0,
    height_css_px: 844.0,
    device_scale_factor: 3.0,
    environment_revision,
};
let computed = compute_supported_elements_ua_cascade(
    &view,
    &author_stylesheets,
    viewport,
    style_revision,
)?;
```

위 코드는 Rust 내부 API 예시다. V8·Swift·Kotlin 호출, 앱 크기 전달, asynchronous worker, Taffy projection, GPU scene admission을 제공하지 않는다.

## 비교 기준과 근거

C01 inventory의 HTML 요소 9개·19 feature ID, Chromium `154.0.8037.95`, revision `@05d469856e75794131cc2e5d9b2f6b6f10a70388` 및 `800×600` CSS px / scale 1 reference를 사용한다. 정확한 실행 hash·OS·fixture 환경은 [UA baseline 실행 근거](evidence/css-c04-ua-baseline-snapshot-2026-10-09.md)에 둔다. 합격은 C01 `chromiumDefault`의 19개 computed string 정확 일치, author override, origin·viewport 오류 거부, namespace 경계, 진단 보존과 revision echo다.

이 구현은 C04 parent, 제품 UA stylesheet 적용 전체, C01 전체, 공개 CSS 지원을 완료하지 않는다. 다음 단계는 owner/root 및 viewport를 실제 runtime에서 제공하고, 적절한 계산 worker를 거쳐 supported style subset을 layout 경로와 연결하는 별도 작업이다.
