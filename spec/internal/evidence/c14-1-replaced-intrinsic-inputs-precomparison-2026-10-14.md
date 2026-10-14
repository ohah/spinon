# C14.1 · 대체 이미지 자연 크기 구현 전 비교 모델

## 범위와 판정

이 기록은 HTML `img`의 두 CSS 축이 모두 `auto`이고 decoder가 양의 자연 너비·높이를 제공한 경우에 한해, natural size를 레이아웃 leaf measure 입력으로 넘기는 경계를 고정한다. Chromium 출력은 구현과 다른 브라우저 oracle이다. 단일 fixture이며 공개 CSS 지원 선언이 아니다.

예상값은 이미지 natural size `24×16 CSS px`, 사용 크기 `24×16 CSS px`다. 구현은 frame의 `x`, `y`, `width`, `height`를 각각 비교하며 절대 오차가 `0.5 CSS px`를 넘으면 실패다. 이 fixture의 정수 좌표·크기는 수치 오차 없이 일치해야 한다. 현재 precomparison은 DPR 1만 관찰했으므로 DPR 불변성 주장을 포함하지 않는다.

## 입력과 고정 산출물

- HTML: [`replaced-auto-size.html`](../../../tests/fixtures/css/c14/replaced-auto-size.html)
- inventory: [`replaced-auto-size-inventory.json`](../../../tests/fixtures/css/c14/replaced-auto-size-inventory.json)
- 화면 캡처: [`chrome155.png`](c14-1-replaced-intrinsic-inputs/chrome155.png)
- 브라우저 실행: agent-browser CLI `0.31.1`, Google Chrome `155.0.8059.40`, macOS arm64, headless Chrome session `c14-intrinsic`.
- 로컬 `file://` fixture, viewport `120×80 CSS px`, locale/time zone 설정 없음, device scale factor `1`; 네트워크 요청 없이 inline SVG data URL만 사용했다.
- 캡처 session에서 얻은 user agent는 `HeadlessChrome/155.0.0.0`으로 build patch를 포함하지 않는다. build 전체 버전은 같은 날 `agent-browser doctor --offline --quick`의 Chrome inventory가 보고한 `155.0.8059.40`으로 식별했다.

| ID | Natural size | CSS width/height | Chromium 관찰값 | C14.1 사용 여부 |
| --- | --- | --- | --- | --- |
| `image` | `24×16` | `auto / auto` | frame `(0,0) 24×16` | 포함 |
| `sized-width` | `24×16` | `48px / auto` | frame `(0,16) 48×32` | ratio 후속 경계 |
| `sized-height` | `24×16` | `auto / 24px` | frame `(0,48) 36×24` | ratio 후속 경계 |

세 이미지 모두 `complete=true`, natural size `24×16`이었다. `display:block`을 명시해 inline line box 변수를 제거했다. root는 `120×80px`이다.

## 기준 한계

현재 호스트의 Google Chrome은 저장소의 기존 pinned `154.0.8037.98`이 아니라 `155.0.8059.40`이다. 이 기록은 버전을 숨기거나 기존 Chrome 154 reference와 섞지 않는다. fixture에 브라우저 화면·DOM 관찰값은 저장했지만, Chrome 154 재실행은 아직 하지 않았다. agent-browser 화면 캡처는 DOM 관찰의 시각적 교차 확인이며 CSS 수치의 독립 원본은 agent-browser eval 결과다.

이 모델은 resource decoder, 비동기 decode 순서, 실패 이미지, ratio, 지정된 한 축, `min-content`·`max-content`·`fit-content`, 텍스트/버튼 및 실제 모바일 앱 경로를 검증하지 않는다. 후속 reference를 추가할 때는 새 입력·브라우저 버전·수치 관찰을 별도 기록한다.
