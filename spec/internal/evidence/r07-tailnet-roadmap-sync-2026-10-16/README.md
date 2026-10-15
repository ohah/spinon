# R07 Tailnet 로드맵 동기화 · 2026-10-16

## 변경 범위

[PR #204](https://github.com/ohah/spinon/pull/204)이 `a328a69d8b5904f659b53666c944ae4e76cc8e53`로 main에 리베이스 병합된 뒤, [Tailnet 로드맵](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html)의 R07 설명과 상태 미리보기를 공식 원본에 맞췄다.

동기화 전에도 로드맵은 GitHub main의 `spec/STATUS.md`를 새로 가져오므로 카운터는 62/197로 표시됐다. 정적 HTML의 R07 설명은 이전 내용이었다. 동기화 후 페이지는 62/197, 상위 5/78, 세부 57개 완료를 표시하고, R07 카드를 조사 완료로 표시한다. R03은 미완료이며 제품 지원 완료는 여전히 0개다.

수정한 제공 파일은 root/public 로드맵 HTML, `spec/STATUS.md`, 생성된 `STATUS.html` 사본과 그 페이지가 실제로 참조하는 Rspress 자산이다. 15개 파일을 반영했다. 5개 기존 파일을 교체하기 전에 Tailnet preview 아래의 전용 backup 경로에 복사했고, 새 자산 4개를 만들었으며, 동일 해시 자산 6개는 다시 쓰지 않았다. 파일별 원본·이전·기대 SHA-256과 backup 경로는 [manifest](./manifest.json)에 있다. 삭제나 서비스 재시작은 없었다.

## 생성과 반영

- 저장소 고정 Rspress `2.0.22`로 `STATUS.md` 한 페이지를 격리 staging build했다. 임시 설정에서만 dead-link·dead-image 검사를 끄고, 실제 Tailnet page가 필요로 하는 CSS·JS·SVG asset을 결과물에서 식별했다.
- `bun run docs:build` 전체 사이트 빌드는 기존 저장소 전체의 dead link 및 이미지 참조 오류에서 실패했다. 이 실패를 전체 빌드 성공으로 보고하지 않는다. 실패 로그는 작업 환경의 `/private/tmp/spinon-r07-status-build.log`에 있다.
- 저장소의 Rspress 설정, GitHub Pages workflow, Tailnet 서비스 설정은 수정하거나 실행하지 않았다.

## Agent Browser 확인

- [로드맵](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html)에서 `공식 완료 62/197`, R07 체크 상태와 Vue 3.5.44·Svelte 5.57.2 조사 설명을 확인했다.
- [상태 HTML](https://macstudio.tailed42f2.ts.net/spinon/STATUS.html)에도 R07 항목이 표시된다. `/spinon/spec/STATUS.html`은 파일 GET은 200이지만 Rspress client navigation에서 404 화면으로 바뀐다. 이 경로는 표시 성공으로 주장하지 않는다. 공식 상태 링크는 GitHub main의 `spec/STATUS.md`를 가리킨다.
- 로드맵·상태 HTML과 로드맵에 필요한 asset 등 15개 경로 모두 Agent Browser fetch에서 HTTP 200이었고, 제공한 바이트의 SHA-256이 기대값과 일치했다. 상세 결과는 [served 검증](./served-verification.json)에 있다.
- [화면 캡처](./roadmap-agent-browser.png)는 Agent Browser에서 1680×1200으로 저장했다. 공식 카운터와 R07 카드가 함께 보인다.

## 범위

이 동기화는 문서 미리보기만 갱신한다. 앱, API, Android/iOS Simulator 또는 실기기 동작은 바꾸거나 검증하지 않았다. R07은 제한된 호스트 가능성 조사 완료이며 R03, FFI, 제품 Vue/Svelte adapter, CSS/GPU 화면, 접근성 및 플랫폼 지원은 미완료다.
