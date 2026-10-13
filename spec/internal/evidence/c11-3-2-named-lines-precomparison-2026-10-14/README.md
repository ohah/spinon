# C11.3.2 named grid lines 사전 비교 · 2026-10-14

## 기준

Chrome for Testing `154.0.8037.92`, CDP revision `@502bee2426ab`, 고정 fixture 15 scene·30 node, viewport `320×240` CSS px, DPR 1·2다. 전체 scene observation, 실행 정보, screenshot hash는 [reference JSON](../../../../tests/fixtures/css/references/c11-3-2-grid-named-lines-precomparison-v1.json)과 [capture summary](./c1132-chrome-20261013T175533841Z-b14d1e22/capture-summary.json)에 보존했다.

15 scene은 positive/negative occurrence, identifier-integer 순서, forward/backward named span, 같은 line 중복 이름, repeat seam, row 축, 역전·동일 line conflict, 누락 이름/occurrence, span shortage와 lone named span을 다룬다. CSSOM resolved values, metadata observation, geometry를 고정했다.

## WPT oracle

원문 revision은 `c999c58338ee1d223df5ad62e48be1202ced0d37`다. `grid-placement-using-named-grid-lines-001.html`, `...-009.html`은 각 PASS이고 `grid-placement-unknown-named-grid-line-001.html`은 17 subtests PASS/skip 0이다. 두 run의 JSON과 원문 로그는 [run 01](./wpt-chrome154.0.8037.92-run01.json), [run 02](./wpt-chrome154.0.8037.92-run02.json), [로그 01](./wpt-chrome154.0.8037.92-run01.log), [로그 02](./wpt-chrome154.0.8037.92-run02.log)에 둔다. WPT runner는 `headless: false`라고 보고한다. 이 자료는 Chrome oracle 결과이며 Spinon runner나 적합성 결과가 아니다.

Git diff 검사를 위해 plain log 끝부분의 whitespace를 정규화했다. byte-exact 원본은 같은 디렉터리의 `.raw.log.gz`로 보존하며 SHA-256은 [runtime evidence manifest](../c11-3-2-grid-named-lines-runtime-2026-10-14/raw-log-sha256.txt)에 기록했다.
