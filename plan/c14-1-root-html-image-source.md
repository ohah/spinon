# C14.1 · 파싱된 루트 HTML의 PNG 자연 크기 입력

**상위 계획:** [C14.1 앱 runtime fixture adapter](./c14-1-app-runtime-adapter.md) · [live V8 문서 측정](./c14-1-live-v8-image-measurement.md)

**범위:** 기존 Android/iOS opt-in C14.1 report route에 파서가 만든 초기 문서 snapshot 경로를 추가한다. C14.1 및 C14는 미완료로 유지한다.

## 목표

현재 C14.1 앱 fixture는 `RuntimeSession::new()` 뒤 JavaScript `eval`로 `<img>`를 만든다. 이번 slice는 같은 V8 세션의 두 번째 입력 경로로 `RuntimeSession::new_with_root_html()`을 호출한다. HTML5 document parser가 만든 초기 HTML tree에서 연결된 `<main id="root"><img id="image" ...></main>`을 찾는다. Stylo cascade와 제한 C14.1 layout은 모두 실제 parsed `html` documentElement를 root로 사용한다. 현재 layout API는 layout root가 `HostRoot`의 직접 자식이어야 하므로 `main#root`를 layout root로 전달하지 않는다. 고정 fixture는 이 제한 profile 안에서 문서 전체를 투영할 수 있도록 `html, body`를 명시적으로 block, `head`와 그 자손을 명시적으로 none, `main#root`와 `img#image`를 명시적으로 block으로 둔다. 이는 browser UA default나 일반 head/layout 동작 지원을 주장하지 않는다. 전체 computed style snapshot과 document root를 기존 `ImageRequest` → PNG decoder → `ImageResourceSnapshot` → intrinsic layout snapshot → Taffy 경로에 전달한다.

기존 live-V8-eval 경로와 고정 report route는 유지한다. 두 경로를 하나의 opt-in app 실행에서 각각 측정해 비교 가능한 별도 결과로 보고한다. 이 fixture는 root HTML source parsing, V8 초기 document import, parsed `<style>` text 추출, 이미지 source identity와 C14.1 geometry만 확인한다. 제품 resource loader, URL/network fetch, CSSOM, 일반 CSS 지원, 자동 비동기 decode/replacement, WGPU/Metal image paint와 pixel parity는 포함하지 않는다.

## 독립 oracle과 재현 입력

- **독립 oracle:** 기존 Agent Browser Headless Chrome 155 사전 비교와 [C14 PNG reference](../tests/fixtures/resources/c14-1-png-basic-reference.json)를 재사용한다. PNG bytes는 `97` bytes, SHA-256 `5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2`; natural size는 `24×16 CSS px`, CSS frame은 `(0,0) 24×16 CSS px`다.
- **새 고정 입력:** [c14-1-root-html-image.html](../tests/fixtures/html/c14-1-root-html-image.html). UTF-8 no-quirks HTML document이며 고정 `<style>` 안에 기존 제한 layout 규칙을 두고, body 아래에 단 하나의 `main#root`와 `img#image`를 둔다. fixture는 `html, body`와 `main#root`, `img#image`의 `display:block` 및 `head, head * { display:none }`를 명시해 제한 layout profile의 문서 전체 사전 검사에 맞춘다. 현재 layout API는 보이는 text node의 intrinsic measurement를 지원하지 않으므로 고정 source에는 head 바깥의 text node나 구조 사이의 공백이 없다. 이는 일반 browser UA 기본값, 일반 head formatting 또는 일반 text layout 지원을 증명하지 않는다. 이미지 `src`는 정확히 `https://spinon.invalid/c14-1-png-basic.png`다. 현재 HTML 파일 SHA-256은 `56fc0d9c5a444550866eba7ee65fbe3960b27631830294093107c8c20d77b386`이다.
- **Root HTML 입력 경로:** `RuntimeSession::new_with_root_html(source)`가 parse/import를 마친 뒤 opt-in actor snapshot API로 불변 `HostDocumentSnapshot`을 읽는다. HTML의 script는 초기 session 생성 때 실행하지 않는다. root/head/body와 fixture element를 HostDocument tree에서 찾고 HTML namespace, id, 연결 상태와 exact `src`를 확인한다.
- **Stylesheet 입력:** CSS 문자열을 HTML source에서 따로 찾지 않는다. HostDocument snapshot의 실제 연결된 HTML `<style>` element와 Text child에서 CSS를 읽고 기존 Stylo cascade에 전달한다. 이 제한 fixture는 하나의 `<style>` element만 허용하며 외부 stylesheet, CSS URL, 동적 style/script는 거부한다.
- **비교 환경:** CSS viewport `120×80`, DPR `1`; Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator. 물리 device를 조회하거나 선택하지 않는다. Pixel density/screen scale은 CSS px에 곱하지 않는다.

## 사전 판정

각 경로에서 다음을 모두 확인한다.

1. 기존 eval route는 `document_origin=live-v8-eval`, parsed-root route는 `document_origin=parsed-root-html-v8`로 report된다. parsed-root 경로의 image는 JavaScript `createElement`로 생성되지 않는다.
2. parsed source에서 unique HTML `html`, `head`, `body`, `main#root`, `img#image`를 찾아야 한다. image는 document에 연결되고 고정 `src`가 exact-match여야 한다. 누락·중복·wrong namespace/id·detached node는 실패다.
3. parsed `<style>` text에서 기존 CSS profile을 만든다. cascade와 layout 모두 실제 `html` documentElement root를 사용하고 전체 computed style snapshot을 넘긴다. `html=block`, `head=none`, `main=block`, `img=block` computed display를 모두 pass gate로 기록한다. head descendants를 숨기는 규칙은 이 bounded fixture의 layout profile 요구사항이며 UA style 지원을 주장하지 않는다. style 누락·복수·빈 값은 성공으로 대체하지 않는다.
4. embedded HTML source SHA-256은 `56fc0d9c5a444550866eba7ee65fbe3960b27631830294093107c8c20d77b386`과 exact-match여야 한다. HTML image에서 `ImageRequest`를 만든 뒤 고정 PNG byte length와 digest를 확인한다. request, resource, intrinsic 및 layout source는 generation/document/render-tree revision이 모두 같아야 한다.
5. decoded natural size는 `24×16 CSS px`, 이미지 frame은 `(0,0) 24×16 CSS px`, 네 좌표/크기 차이는 Chrome 155 reference 대비 각각 `≤0.5 CSS px`다.
6. 기존 live-V8-eval stale-src control은 계속 `SourceChanged`로 거부된다. parsed-root 결과는 stale control과 별도의 status로 판정하며 이전 경로의 성공을 대신하지 않는다.
7. Android 및 iOS opt-in app report에 두 경로 결과가 각각 보인다. feature-off route는 disabled로 남고 기본 artifact에는 C14.1 symbol/fixture payload가 없으며 opt-in positive control에는 포함된다.

## 실패 관점 및 실행 순서

1. 구현 전 이 계획을 서로 다른 실패 관점 20개로 검토한다. 이 기록은 기존 C14.1 plan review나 live-V8 implementation review를 재사용하지 않는다.
2. 고정 HTML fixture와 SHA-256을 추가하고 HTML→HostDocument tree의 expected structure를 기록한다.
3. fixture-only Rust 경로를 추가한다. `new_with_root_html` 생성, actor-owned snapshot, unique node discovery, DOM-owned style text, image request/decode와 layout까지 source stamp를 보존한다.
4. 기존 C14.1 app route의 report를 두 독립 결과가 모두 통과할 때만 `status=ok`로 만든다. buffer 부족, parser/session 실패, image/style ambiguity, unsupported source와 non-finite geometry는 성공이 아니다.
5. 계획과 다른 새 20개 실패 관점으로 구현/실행을 검토하고 발견 결함을 고친 뒤 영향 범위를 재검토한다.
6. focused Rust/FFI 검사, formatting, feature boundary와 Android API 37 AVD 및 iOS Simulator app route를 확인한다. 두 화면과 log, artifact hash를 새 evidence directory에 보존한다.
7. PR diff에는 계획·구현 두 review 표를 재사용하지 않는 30개 독립 실패 관점을 남긴다. GitHub PR 이미지는 Agent Browser에서 렌더링·주소·natural/rendered size를 확인한 뒤 해당 실행에서 시작한 Agent Browser process를 닫는다.
8. merge 후 Tailnet static roadmap preview에 근거를 반영하고 Agent Browser에서 served route와 content hash를 확인한다. tmux session은 건드리지 않는다.

## 상태 해석

이 slice가 통과해도 C14.1 product integration, remote/local loader, HTML image loading algorithm, async scheduling, caching, replacement/error event, general text layout, paint/pixel, image overflow, 실기기 동작 또는 C14/U05/X16 완료를 의미하지 않는다. 공식 status checkbox는 계속 미완료다.
