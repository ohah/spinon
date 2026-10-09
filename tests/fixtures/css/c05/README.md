# C05.1 런타임 사용자 지정 속성 비교 입력

`runtime-custom-properties.v1.json`은 고정 viewport, 관찰할 CSS 노드, 문서 전이, 비교 기준의 SSOT다. `runtime-custom-properties.html`을 Chrome `154.0.8037.98`에서 실행해 computed style과 CSS px rectangle을 수집하며, reference는 fixture·HTML·앱 JavaScript·capture 도구·Chrome 실행 파일 hash에 묶는다. 캡처 도구는 기존 reference를 덮어쓰지 않는다.

픽스처는 미등록 사용자 지정 속성 상속, 대소문자 구분, 중첩 fallback, `inherit`·`unset`·`initial`·`revert`, self/mutual cycle, 잘못된 대체값, 빈 값, `!important`, `gap:var(...)`, background paint와 조상 값 변경·재부착을 다룬다. Rust 테스트는 고정 Chrome computed style을 정확 비교하고, scene의 부모·자식 geometry 및 runtime frame 전이를 확인한다.

Android·iOS fixture 앱은 C04.10에서 이미 만든 세 요소의 `style`만 변경한다. 이는 앱의 기본 시작 동작이나 공개 CSS 지원을 뜻하지 않는다. 지원 경계와 내부 호출 계약은 [C05.1 명세](../../../../spec/internal/0032-c05-runtime-custom-properties.md)를 따른다.

## C05.2 stylesheet `@property` 등록

`runtime-registered-properties-inventory.json`은 고정 Chromium 관찰 노드와 geometry 기준을 정하고, `runtime-registered-properties.html`은 기준 HTML fixture, `runtime-registered-properties.js`는 HostDocument에 연결할 동일 DOM·stylesheet 입력이다. Node 테스트는 두 stylesheet의 CSS 본문과 등록 순서를 대조하며 reference는 HTML·runtime JavaScript·capture tool·Chromium 실행 파일 hash를 기록한다.

fixture는 `<length>` 지정값·초깃값·잘못된 값 대체, `inherits:false`와 `true`, 사용 규칙 뒤 등록, 유효한 미지 descriptor 무시, inline `!important`, `<color>` paint, 중복 등록 및 stylesheet 이동 전후의 승자를 비교한다. `width:auto`는 computed width 문자열과 used value를 혼동하지 않고 Chromium 및 runtime frame으로 확인한다. 캡처는 pinned Chrome `154.0.8037.98`만 허용하고 기존 reference를 덮어쓰지 않는다.

`runtime-registered-properties-multi-root.html`은 첫 HostRoot 안의 연결 `<style>` 등록이 두 번째 HostRoot의 `@property` 계산에 전역 적용되는지 고립해 확인한다. Chromium 기준은 computed `width:19px`와 `19×14` CSS px rectangle이다. Rust runtime 테스트는 둘째 root의 cascade 값을 검증하고, 다중 HostRoot layout이 기존처럼 `multiple_host_roots`로 거부되는 경계도 확인한다.

```sh
node tools/css-reference/capture-c05-runtime-registered-properties.mjs
node --test tools/css-reference/c05-runtime-registered-properties.test.mjs
node tools/css-reference/verify-c05-runtime-registered-properties-multi-root.mjs
```

내부 profile 계약은 [C05.2 명세](../../../../spec/internal/0034-c05-runtime-registered-properties.md), 범위는 [구현 계획](../../../../plan/c05-runtime-registered-properties.md)을 따른다. 이는 `CSS.registerProperty()`, CSSOM, 전체 CSS 지원 또는 실기기 검증을 뜻하지 않는다.

## C05.3 재계산 cache 사전 비교

`bun run css:verify:c05-result-cache-precomparison`은 기존 C05.2 HTML에서 문서에 연결하지 않은 노드의 style을 바꿔도 연결된 세 노드의 computed width와 geometry가 고정되는지 Chromium으로 확인한다. 이는 cache 도입 전 CSS 기준이며 runtime cache가 구현되었다는 뜻은 아니다.
