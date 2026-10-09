# C04.11 런타임 문서 author stylesheet 내부 계약

- **계약 버전:** `0.1.0` · 출시 전 고정
- **상태:** 고정 Chromium 비교, Rust 검증, Android·iOS Simulator 실행 완료 · 공개 CSS 지원 미완료
- **구현 계획:** [`plan/c04-runtime-author-stylesheets.md`](../../plan/c04-runtime-author-stylesheets.md)
- **구현 전 Chromium 기준:** [`C04.11 precomparison`](evidence/c04-runtime-author-stylesheets-precomparison-2026-10-10.md)
- **고정 Chromium reference:** [`c04-runtime-author-stylesheets-v1.json`](../../tests/fixtures/css/references/c04-runtime-author-stylesheets-v1.json)
- **실행·구현 근거:** [`C04.11 구현 검토와 플랫폼 증거`](evidence/c04-runtime-author-stylesheets-2026-10-10.md)

## 목적과 소유권

이 계약은 현재 `HostDocumentSnapshot`에 연결된 HTML `<style>` 요소를 runtime CSS 입력으로 바꾸는 내부 규칙을 정한다. 기존 JavaScript DOM façade가 노드와 UTF-16 텍스트를 만들고, `spinon-runtime`이 불변 snapshot에서 stylesheet source 목록을 수집한다. `spinon-style`은 UA·author·inline origin을 Stylo cascade에 전달하며, 결과는 기존 Taffy layout과 WGPU scene 경로를 따른다.

새 공개 JavaScript API는 추가하지 않는다. 문서 입력은 기존 `document.createElement`, `document.createTextNode`, `appendChild`, `setAttribute` 호출로 구성한다. `spinon_runtime_gpu_host_eval_author_stylesheets_fixture`는 앱 샘플·플랫폼 실행에서 쓰는 저장소 전용 검증 진입점이지 제품 API가 아니다.

## 입력과 수집

1. 수집은 `HostDocumentSnapshot::root_children()`에서 시작해 연결된 요소를 DOM 전위 순회한다. 연결되지 않은 노드의 규칙은 포함하지 않는다.
2. HTML namespace이며 ASCII 대소문자를 무시한 local name이 `style`인 요소가 후보이다. 비 HTML namespace의 `style`은 해당 runtime 계산을 실패시킨다.
3. `type`이 없거나 공백뿐이면 CSS로 취급한다. MIME essence가 `text/css`이면 매개변수가 있어도 적용하고, 다른 essence는 HTML data block처럼 무시한다.
4. `media`는 생략·공백·`all`·`screen`만 허용한다. 다른 조건 문자열은 무조건 적용하지 않고 계산을 실패시킨다.
5. CSS 원문은 해당 style 노드의 연결된 자손 텍스트를 DOM 순서로 이어 붙인다. 유효하지 않은 UTF-16은 오류로 처리하며 대체 문자를 삽입하지 않는다. 새로운 별도 CSS byte limit은 없으며 HostDocument의 기존 document budget을 따른다.
6. source ID는 `host-style:{document_generation}:{node_id}`이고 source order는 현재 snapshot의 연결 DOM 순서다. source base URL은 `https://spinon.invalid/document.html`이다. URL 자원 요청에 사용할 네트워크 loader는 존재하지 않는다.
7. HTML `<link>`의 `rel` 공백 구분 토큰에 대소문자 무관 `stylesheet`가 있으면 외부 CSS가 미지원임을 오류로 반환한다. href를 읽거나 요청하지 않는다.

## Cascade·layout 동작

- 수집한 author stylesheet 순서를 각 HostRoot fragment의 Stylo 계산에 전달한다. 적용된 UA·author·inline origin, cascade specificity, `!important`, custom property와 `var()` 해석은 고정 fixture의 Chromium 관찰값으로 비교한다.
- C05.1 runtime custom-property computed-style profile의 기존 layout allowlist를 따른다. stylesheet의 사용자 지정 속성은 해당 profile에서만 허용한다. WGPU paint profile은 추가로 기존 단색 `background-color` typed conversion을 사용한다.
- fixture가 대조하는 selector는 HTML type·ID·class·descendant 조합이다. 이 목록은 전체 CSS selector 호환 선언이 아니다. 각 HostRoot는 기존 fragment 범위 안에서 계산하므로 서로 다른 HostRoot 사이의 sibling 관계를 selector로 보장하지 않는다.
- 일반 선언 allowlist 밖 CSS 속성 및 지원하지 않는 at-rule은 계산 전체를 거부한다. author stylesheet의 모든 Stylo parse diagnostic은 source ID와 0-base 줄, 1-base UTF-16 열을 포함해 layout-only와 WGPU 양 runtime 계산 모두 실패시킨다. 기존 inline `style` 속성 진단은 이 계약에서 바꾸지 않는다.
- `@import`는 Stylo의 import 비허용 파서 설정에서 진단되고 실패한다. 외부 stylesheet, `url()` 자원이나 font를 네트워크로 불러오지 않는다. CSSOM, 동적 stylesheet 객체, 정적 `<link>` loader, 범용 media query, `@property`는 포함하지 않는다.
- 내장 UA 규칙 `style { display: none }`으로 style 요소는 일반 레이아웃 상자를 만들지 않는다. `display:none` 조상 아래 text node는 layout input, runtime frame과 scene preorder에서 제외한다. 보이는 text node는 기존처럼 `unsupported_text_node` 오류다. author rule이 style 요소를 표시하면 그 CSS 본문 텍스트도 보이는 text로 처리되어 실패한다.
- stylesheet source는 매 계산 요청의 현재 snapshot에서 다시 수집한다. 텍스트 수정·삽입 위치 변경·분리·재연결은 새 `DocumentRevision`을 통해 반영한다. 계산 요청 사이 stylesheet registry나 computed-style cache는 보존하지 않는다. stylesheet 전용 `StyleRevision`은 만들지 않는다.

## JavaScript 예제

```js
const root = document.createElement("div");
root.setAttribute("id", "app");
document.appendChild(root);

const sheet = document.createElement("style");
sheet.setAttribute("type", "text/css");
sheet.setAttribute("media", "screen");
sheet.appendChild(document.createTextNode(`
  #app { display:flex; gap:11px; }
  .tile { width:47px; }
`));
root.appendChild(sheet);

const tile = document.createElement("div");
tile.setAttribute("class", "tile");
root.appendChild(tile);
```

예제는 기존 DOM API 호출 형태를 보여준다. CSS 속성은 runtime profile allowlist에 속해야 한다. `gap` 계산은 `row-gap`·`column-gap` layout 입력과 기존 단위 해석 계약을 따른다.

## 실패 조건과 상태

| 입력·상태 | 관찰 결과 |
| --- | --- |
| 비 CSS `type` data block | CSS source로 등록하지 않음 |
| 미지원 `media` 값 | 해당 계산 전체 실패 |
| SVG 등 비 HTML `style` | 해당 계산 전체 실패 |
| 연결된 `link rel=stylesheet` | 외부 fetch 없이 해당 계산 전체 실패 |
| CSS UTF-16 단위 오류 | 대체 변환 없이 해당 계산 전체 실패 |
| author CSS parse diagnostic 또는 `@import` | 두 runtime profile 모두 source 위치를 포함해 계산 전체 실패 |
| 지원 밖 선언·at-rule | profile 밖 입력으로 계산 전체 실패 |
| visible text node | layout `unsupported_text_node` 실패 |
| `display:none` 하위 CSS text | layout·frame·scene 대상에서 제외 |
| 분리된 style 요소 | source 목록에서 제외, 이전 규칙 재사용 없음 |

## 비교 기준과 범위

Chromium 기준은 저장소 고정 Chrome `154.0.8037.98` 실행 파일이다. 상세 OS·viewport·media 환경·실행 파일 hash 및 CSS computed string은 [precomparison](evidence/c04-runtime-author-stylesheets-precomparison-2026-10-10.md)과 JSON reference에 기록한다. 고정 viewport에서 computed string은 정확히 일치해야 하고 좌표별 geometry 오차는 최대 `0.5 CSS px`다. Android API 37 emulator, iPhone 17 Pro / iOS 26.2 Simulator 결과는 별도의 구현 근거에 기록한다. Chromium fixture 통과만으로 플랫폼 WGPU 결과를 대신하지 않는다.

이 계약은 C04.11만 설명한다. 전체 HTML/CSS, 일반 웹 호환성, CSSOM, 외부 자원, SVG 렌더링, text shaping, font loading, incremental style invalidation이나 실기기 성능을 보장하지 않는다. 숫자 계약 버전 `0.1.0`은 출시 또는 호환성 정책이 정해지기 전 올리지 않는다.
