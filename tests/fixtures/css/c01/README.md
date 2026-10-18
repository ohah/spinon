# C01 첫 비교 fixture

`supported-html-ua.html`은 현재 내장 UA 스타일 초안에 선언된 HTML 태그의 Chromium 계산값을 모으는 원본 입력입니다. 비교 대상의 안정 ID·selector·예상 node ID·CSS property는 [`inventory.v1.json`](inventory.v1.json)에 둡니다.

- oracle 수집기: [`tools/css-reference/capture.mjs`](../../../../tools/css-reference/capture.mjs)
- 비교할 선언: 태그 선택자 일치 집합과 내장 프로필이 선언한 computed CSS 값
- 현재 기록 범위: `div`, `span`, `a`, `img`, `button`, `input`, `p`, `ul`, `li`
- 현재 부분 inventory: HTML 요소 9개, computed CSS feature 19개. 전체 지원 목록이 아닙니다.
- 의도적으로 제외: 레이아웃 좌표·텍스트 metrics·폼 컨트롤 모양·GPU 픽셀. 각각의 비교 경로가 구현되기 전에는 기준 결과라고 가장하지 않습니다.
- author stylesheet와 원격 자원은 fixture에 없습니다.

수집기는 JSON inventory를 검증한 뒤 새 문서가 열리기 전에 깊게 동결해 fixture에 주입합니다. fixture selector·feature 목록, Chromium의 실제 node ID·HTML 태그·namespace가 inventory와 정확히 맞는지 검사합니다. 캡처 JSON에는 inventory SHA-256·부분 범위·요소 수·feature 수가 기록되고 reference-id에도 inventory 해시가 포함됩니다. v1 입력의 SHA-256은 단위 테스트에 고정해 실수로 입력 내용을 바꾸면 감지합니다. 브라우저 버전이나 inventory가 바뀌면 기존 결과를 덮어쓰지 않고 새 디렉터리를 만듭니다.

현재 seed inventory는 9개 요소와 19개 값을 다룹니다. 전체 HTML/SVG·CSSWG/WPT 기준·모든 feature 값 조합·레이아웃/텍스트/페인트·모바일 비교는 포함하지 않으므로 `C01` 전체는 미완료로 유지합니다.

## CSSOM 속성 이름 표면

[`cssom-property-surface.html`](cssom-property-surface.html)은 author stylesheet·외부 자원이 없는 HTML namespace `div` 하나를 고정한다. [`property surface 수집기`](../../../../tools/css-reference/capture-property-surface.mjs)는 해당 노드의 `getComputedStyle(element).item(index)` 이름만 별도 snapshot으로 기록한다. 현재 Chrome 154.0.8037.98 관찰 결과는 총 478개(일반 442·prefixed 36·custom 0)다. 이 결과는 전체 속성 registry, 표준 속성 분류, 속성 값 지원, 전체 요소 적용성 또는 제품 CSS 지원을 의미하지 않는다. 실행 환경·provenance·미포함 범위는 [C01.3 근거](../../../../spec/internal/evidence/css-c01-cssom-property-surface-2026-10-09.md)와 고정 reference JSON에서 확인한다.


## Selector·at-rule CSSOM surface

[`selector-at-rule-surface.html`](selector-at-rule-surface.html)와 [`selector-at-rule-inventory.v1.json`](selector-at-rule-inventory.v1.json)은 15개 고정 HTML node, selector 23행, at-rule 13행의 partial 비교 입력이다. [`capture-selector-at-rule-surface.mjs`](../../../../tools/css-reference/capture-selector-at-rule-surface.mjs)는 Chrome `155.0.8059.85` / CDP revision `@0e66b94549a8bd85f03fbdf189b752d6e7b63c32`의 selector match ID, syntax exception, CSSOM rule constructor/type/tree를 수집한다. 재현 기준은 [C01.4 실행 근거](../../../../spec/internal/evidence/css-c01-4-selector-at-rule-surface-2026-10-19/README.md)와 고정 reference JSON에 있다.

```sh
mise exec -- node tools/css-reference/capture-selector-at-rule-surface.mjs
mise exec -- node tools/css-reference/capture-selector-at-rule-surface.mjs --verify-existing
mise exec -- node --test tools/css-reference/selector-at-rule-surface.test.mjs
```

이 표본은 Chromium의 부분 parser/matcher 관찰이며 Spinon의 selector·at-rule·cascade 지원을 의미하지 않는다. 전체 selector/pseudo-state, at-rule·`@import`, computed values, geometry·paint, SVG/Shadow DOM, 앱 runtime은 범위 밖이다. `C01` 부모는 미완료다.
