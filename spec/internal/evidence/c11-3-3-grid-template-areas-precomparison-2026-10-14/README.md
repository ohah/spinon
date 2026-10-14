# C11.3.3 grid-template-areas 사전 비교 · 2026-10-14

## 기준과 판정 규칙

Chrome for Testing `154.0.8037.92`, CDP revision `@334b65d254ccc35df4fca82706d1753227b01039`, viewport `320×240 CSS px`, DPR 1·2, `en-US`/UTC, LTR `horizontal-tb`에서 26개 scene·65개 node를 캡처했다. HTML, inventory, 캡처 도구, Chromium 실행 파일의 hash와 모든 node의 computed CSSOM·resolved track·area rectangle·CSS px geometry는 [기준 JSON](../../../../tests/fixtures/css/references/c11-3-3-grid-template-areas-precomparison-v1.json)에 연결했다. DPR 간 CSSOM과 CSS px geometry는 정확히 같아야 한다. 이후 Spinon 비교의 node frame 허용치는 축별 `0.5 CSS px`다.

[고정 inventory](../../../../tests/fixtures/css/c11/grid-template-areas-c1133-inventory.json)와 [HTML 입력](../../../../tests/fixtures/css/c11/grid-template-areas-c1133.html)은 area tokenization·rectangles·null cells·shorthands·line alias·invalid cascade·track mismatch·nested/hidden child를 개별 scene으로 둔다. scene별 CSSOM `grid-template-areas` 문자열을 matrix로 정규화하고 named-area bounds는 이름별 rectangle로 저장한다. area 셀 수에 비례한 dense 복사 버퍼의 크기·상한은 계획에서 정하지 않았으므로 이 Chrome oracle은 fixture 관측만 저장하고 구현의 저장 표현을 결정하지 않는다.

## 주요 관측

- `basic-rectangles`에서 `head`는 `(0,0,100,40)`, `main`은 `(40,40,120,40)`, `foot`은 `(0,80,160,40)`이다. `null-cell-dot-runs`에서는 `....`/`....`가 각각 한 null cell로 계산되고 `hero`는 두 행을 차지한다.
- `bare-area-vs-explicit-one-name`에서 bare `grid-column: foo`는 area `foo`의 시작 line에서 배치되어 x=40이고, 명시 `grid-column: 1 foo`는 authored `foo` line 1에서 시작해 x=0이다. 이는 bare ident의 area alias와 integer+ident line occurrence를 분리하는 기준이다.
- `explicit-line-before-area`와 `explicit-line-after-area`는 authored/generated name 순서에 따른 Chrome의 alias 충돌 결과를 각각 보존한다. area `a-start`/`a-end`와 explicit `[box-start]`/`[box-end]`로만 정의한 implicit named area도 별도 장면이다.
- L자형, 분리형, row 폭 불일치, trash token, 빈 string 선언은 앞선 유효 선언을 유지한다. 런타임 실패가 아니라 Stylo cascade가 선택하는 computed value를 사용해야 한다.
- `area-track-count-mismatch`는 authored explicit columns 2개, area columns 3개를 둔다. Chrome resolved CSSOM에는 0px track이 보이고 마지막 area item은 `(160,0,0,60)`이다. C11.3.3은 이 조건을 추정된 auto track으로 성공시키지 않고 거부한다.
- `unknown-area-implicit-tracks`에서 explicit 2×2 grid가 알 수 없는 area placement로 Chrome CSSOM상 4×4 track이 된다. 이 기능은 현재 계획 밖이므로 Spinon fixture에서 지원 결과로 세지 않는다.

![`foo` area와 명시적 `1 foo` line placement Chrome 기준](c1133-chrome-20261013T235323413Z-1bb2b0db/bare-area-vs-explicit-one-name-dpr1.png)

## 고정 WPT Chrome oracle

원문은 WPT commit `c999c58338ee1d223df5ad62e48be1202ced0d37`에서 읽었다. 후보 source SHA-256 4개는 고정 inventory와 일치한다. Chrome 154 실행은 4 test `OK`, 73 subtest `PASS`, skip 0, failure 0이다. 원문 report, 일반 로그, structured raw log는 각각 [report](./wpt-chrome154.0.8037.92-run01.json), [log](./wpt-chrome154.0.8037.92-run01.log), [raw log](./wpt-chrome154.0.8037.92-run01.raw.log)에 있다. [하위 검사별 분류](./wpt-chrome154.0.8037.92-classified-run01.json)는 이름·상태·fixture candidate/unsupported 범위와 source SHA를 보존한다.

| WPT 원문 | source SHA-256 | Chrome 결과 | 분류 |
| --- | --- | --- | --- |
| `grid-support-grid-template-areas-001.html` | `fdc219b1887a5aaf7e81c4db2cea39e7fb9878274736b06a629adabf06e77b4a` | 45/45 PASS | 각 syntax·cascade 입력은 파생 fixture 후보. 전체 WPT DOM은 Chrome oracle only |
| `grid-placement-named-grid-area-resolution-001.html` | `573b98353618c00c3eefc27ccc56c176dc269fe12ece7bbcd8dd6375dcce6ae7` | 2/2 PASS | area/shorthand·span과 line 조합 fixture 후보. 전체 WPT geometry는 Chrome oracle only |
| `grid-placement-named-lines-and-areas-001.html` | `6e55fadc66203e6d570c61009c5d6a42d2f2db7eb77c7864aced34f86219aaa9` | 22/22 PASS | generated edge·이름 충돌·suffix area·explicit-line implicit named area fixture 후보 |
| `grid-placement-unknown-named-area-001.html` | `7fa795ad262e74399487fa817da1e99a75bbcce2e46b5f4547d55959e2e9ebd4` | 4/4 PASS | unknown area가 생성하는 implicit tracks는 현재 미지원 |

Chrome WPT report의 `run_info.headless`는 `false`였다. 실행 명령에 `--headless`가 있었지만 runner report와 실제 표시 모드는 같은 필드로 확정할 수 없으므로 WPT 표시 모드는 미확정으로 둔다. 별도 custom fixture capture는 `chromium-session.mjs`가 `--headless=new`를 사용하고 Chrome 버전·실행 파일 hash를 기록했다. 모든 위 WPT 결과는 Chrome oracle이며 Spinon WPT 적합성 결과가 아니다.

재현 명령은 고정 WPT checkout에서 실행했다.

```sh
./wpt run --no-manifest-update --no-manifest-download --channel canary \
  --binary='/Users/yoonhb/Documents/workspace/spinon/build/chrome-for-testing-154.0.8037.92/chrome-mac-arm64/Google Chrome for Testing.app/Contents/MacOS/Google Chrome for Testing' \
  --webdriver-binary='/Users/yoonhb/Documents/workspace/spinon/build/chrome-for-testing-154.0.8037.92/chromedriver-mac-arm64/chromedriver' \
  --headless \
  --log-wptreport=/tmp/spinon-c1133-wpt.json \
  --log-raw=/tmp/spinon-c1133-wpt.raw.log \
  --log-mach=/tmp/spinon-c1133-wpt.log \
  chrome \
  css/css-grid/grid-definition/grid-support-grid-template-areas-001.html \
  css/css-grid/placement/grid-placement-named-grid-area-resolution-001.html \
  css/css-grid/placement/grid-placement-named-lines-and-areas-001.html \
  css/css-grid/placement/grid-placement-unknown-named-area-001.html
```

재현 capture는 저장소 루트에서 `SPINON_CHROMIUM_BIN`에 같은 Chromium 절대 경로를 지정한 뒤 `node tools/css-reference/capture-c11-3-3-grid-template-areas.mjs`로 실행한다. WPT report subtest를 inventory hash와 결합하는 도구는 `tools/css-reference/classify-c11-3-3-grid-template-areas-wpt-report.mjs`다.

## 완료 경계

이 디렉터리는 구현 전 Chromium CSSOM·geometry 기준, screenshots, 고정 WPT source hash 및 개별 subtest Chrome oracle 분류를 보존한다. 이후 sparse area projection/resolution 후보의 검토와 실행은 [구현 실패 관점 검토](../c11-3-3-grid-template-areas-implementation-review-2026-10-14.md)와 [현재 runtime 근거](../c11-3-3-grid-template-areas-runtime-2026-10-14/README.md)에 기록했다. 후속 Android API 37 AVD 실행에서 26 scene·65 node 전체 matrix의 지정 검증이 통과했다. iOS Simulator는 success/rejection 2 scene이다. Android SM-S731N·hardware GPU, Spinon WPT 적합성, 공개 CSS 지원 및 전체 CSS Grid 지원은 여전히 검증하지 않았다.
