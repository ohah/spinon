# C13.1 WPT 후보 분류 · 2026-10-14

## 사전 비교와 판정 기준

고정 upstream은 [web-platform-tests/wpt의 css/css-overflow](https://github.com/web-platform-tests/wpt/tree/a73e14cfbaeba97744ff40cbc34d1ca96e5a92c7/css/css-overflow) commit `a73e14cfbaeba97744ff40cbc34d1ca96e5a92c7` (2026-10-14T10:07:55Z)다. agent-browser로 GitHub API의 css/css-overflow 목록과 각 선택 소스 파일을 확인했다. 해당 directory의 C13.1 이름 범위에서 reference 파일을 뺀 HTML test source는 58개다. 각 source path는 commit SHA 아래에 고정된다. exact commit, 정렬된 path 목록 SHA-256 `de618ed3e3d8bc67afc86528c33950bd7a31a6d7c9981ebf9ddc599c5365ff4a`, path/category/reason tuple 목록 SHA-256 `b271e4e1fb7330d3ff9aa7c1f9b7cfa24f7a49364b04c3d257b1b1722ed641f0`를 validator에 pin했다.

WPT 실행 자체는 하지 않았다. 저장소에 Spinon WPT runner가 없으며 이 분류는 실행 통과나 공개 CSS 적합성 주장이 아니다.

통과 기준은 58개 source file을 중복·누락 없이 각각 하나의 범주에 배치하고, feature owner와 기존 C13.1 profile에 근거를 남기는 것이다. 고정 fixture candidate도 WPT testcase 실행 없이 source triage만 한 상태로 기록한다.

## 분류 결과

- Spinon fixture candidate: 13개. 현재 rectangle profile에 맞게 로컬 fixed CSS/query fixture로 축약할 수 있다. hit-test 후보는 box query에 한정하며 DOM event dispatch pass가 아니다.
- Chrome oracle-only: 17개. overflow 동작과 직접 연관되나 opacity/outline/ink effect, dynamic style revision, 또는 Chrome과 고정 WD의 negative-value 차이로 현재 runtime fixture 실행값으로 취급하지 않는다.
- unsupported: 28개. C13.2/C13.4/C17/C22/C23/C29 등 선행 범위나 현재 HTML box profile 밖의 기능이 요구된다.

상세 path별 근거와 source filter는 classification.json에 고정했다. 범위와 digest 검사는 tools/css-reference/c13-1-overflow-clip-wpt-candidate-classification.test.mjs에서 3/3 통과했다. count를 보존하는 category swap과 inventory 전체 교체도 negative control로 거부한다. [구현 검토 20개](implementation-review-20.md)와 [PR 변경 검토 30개](pr-delta-review-30.md)는 각각 별도 관점이다.

## 남은 C13.1 관문

이 분류는 전체 precomparison matrix, paint/hit/event negative-control 폭, Chrome oracle 일치 여부를 닫지 않는다. Chrome-only 및 unsupported 항목을 제품 동작으로 승격하지 않는다. C13.1과 상위 C13은 미완료다.
