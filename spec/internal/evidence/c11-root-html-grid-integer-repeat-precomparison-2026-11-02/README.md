# C11.1 parsed Root HTML integer repeat 사전 비교

이 자료는 기능 구현 전 CSS Grid integer repeat의 track 확장과 배치값을 고정한다. expected.json의 값은 repeat 횟수·pattern 순서·definite axis basis를 사용한 독립 산술이며, Chrome은 같은 입력의 computed track과 item frame을 교차 확인한다.

- 기준: [CSS Grid Layout Module Level 1, 2025-03-26 CRD §7.2.3.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#repeat-syntax), explicit grid track listing과 fixed repeat expansion.
- 브라우저: Agent Browser가 구동한 HeadlessChrome 156.0.0.0.
- viewport: 1280×633 CSS px, devicePixelRatio 1.
- 입력: tests/fixtures/css/c11/root-html-grid-integer-repeat-precomparison.html.
- 입력 SHA-256: f2d0d35e88ce89d64ebaac5d37365ad9621735f54f6399e90a1959a7412ccd5d.
- 독립 기대값: expected.json.
- Chrome CSSOM/DOMRect 관측: chrome-observation.json.
- 화면: chrome-agent-browser.png.
- screenshot SHA-256: c26b0ae0813374de43ee19dac9b71be0fb9c55d45e42343b766edcd4950f1651.
- observation SHA-256: dede1155f09268a98cc51dad76c5caf6d719d1901a35dfb322ac03b5ceb28590.

7개 장면 모두가 일반 document flow로 쌓인 한 HTML 문서다. Chrome의 computed pixel track list와 각 root/item frame이 독립 기대값과 일치했다. 비교에는 direct px, definite-axis percentage, fraction, mixed px/percentage/fraction, 복수 repeat group, row axis, repeat count one이 포함되며 전체 문서 높이는 220 CSS px다. 같은 source를 default Root HTML Rust runtime과 Android/iOS Simulator에서 쓸 수 있도록 scene별 absolute positioning은 사용하지 않는다.

구현 전 런타임 입력 검토에서 `align-items` 및 CSS `background` shorthand가 default Root HTML profile 바깥임을 확인했다. 같은 색과 frame을 유지하는 `background-color` 선언으로 바꾸고, 이 fixture에서 각 item이 track 크기와 같아 불필요한 alignment 선언을 제거했다. Chrome baseline을 새 source로 다시 캡처했으며 independent expected JSON과 7개 scene·열/행 합계 35 tracks·28 items의 CSSOM/DOMRect 값이 모두 일치한다. 이전 source hash `608b0590...f8e85b`의 관측과 screenshot은 `.pre-profile-normalization` 파일로 남겨 두었다.

이는 Chrome oracle 관측이며 Spinon 코드, Spinon WPT 적합성, Android/iOS 동작 또는 공개 CSS 지원의 근거가 아니다. 화면은 baseline CSS Grid 배치만 보여 준다.

로컬 비교에는 Agent Browser를 사용했고 terminal-browser는 사용하지 않았다. 비교 뒤 Agent Browser 세션을 닫고 OS 프로세스 부재를 확인했으며, 임시 HTTP server도 종료했다.
