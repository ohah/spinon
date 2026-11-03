# 공개 로드맵 PR #1124 카드 화면 근거

`spec/public/roadmap.html`에 병합된 C11.1 Root HTML Grid item-alignment 구현 PR #1124 카드를 추가했다. 공식 상태는 C11.1/C11 미완료 및 `72/213`이다.

## Agent Browser 확인

- 변경한 HTML 파일을 임시 localhost HTTP 서버로 제공하고 Agent Browser에서 열었다. `GET /roadmap.html`은 HTTP 200이었고 문서 제목은 `스피논 구현 로드맵`이었다.
- 첫 로드맵 카드가 PR #1124와 merge SHA `e11df25732ef1a2c8365287b8ef475b55f6aa408`을 표시했다. 접근성 snapshot에서 계획, 내부 계약, runtime evidence, 구현/runtime 검토, PR diff 검토, 공식 상태 링크를 확인했다.
- Desktop viewport 1280×1000에서 `document.documentElement.scrollWidth=1280`, mobile viewport 390×844에서 `scrollWidth=390`이었다. 두 화면 모두 새로운 카드가 보인다.
- 이 캡처는 변경된 정적 HTML의 로컬 시각 확인이다. Tailnet publication이나 각 문서 링크의 제공 여부를 대신하지 않으며 배포 후 Agent Browser에서 별도로 확인한다.

## 화면

- [Desktop 1280×1000](desktop.png)
- [Mobile 390×844](mobile.png)

## SHA-256

원본 HTML과 두 screenshot의 SHA-256은 [SHA256SUMS.txt](SHA256SUMS.txt)에 있다. Agent Browser session과 임시 localhost server를 종료했다. 기존 8769 listener는 다른 세션의 `serve-local-pass02.mjs`였으므로 그대로 보존했다.
