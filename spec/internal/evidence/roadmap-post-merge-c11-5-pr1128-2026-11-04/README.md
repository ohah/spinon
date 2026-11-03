# C11.5 제한 구현 병합 후 상태·로드맵 동기화 근거

## 목적과 공식 상태

PR [#1128](https://github.com/ohah/spinon/pull/1128)의 제한 구현 병합 뒤 공식 상태 문서와 공개 로드맵을 동기화했다. PR은 2026-11-03T17:52:20Z에 Rebase and merge됐으며 merge SHA는 `0f704151cfd805ad2fef2e12535739f0d3a109dd`다. GitHub의 check rollup은 비어 있고 `main`에 required status check도 설정되어 있지 않으므로 CI 통과로 기록하지 않는다.

이 변경은 기존 eligible fixed-track Grid의 direct empty `div` 고정 크기 item에서 `align-self`·`justify-self` 위치 정렬을 다룬 제한 slice를 기록한다. 전체 C11.5 계약이나 C11 CSS Grid를 완료 처리하지 않는다. 공식 완료 수 `72/213`과 C11 미완료 상태는 그대로다. 광범위 package tests도 green이 아니다: `spinon-layout` 423 passed/1 failed, `spinon-style-to-layout` 372/1, `spinon-style` 283/1, `spinon-runtime` 261 passed/2 failed/3 ignored. 상세는 [구현 실행 근거](../c11-root-html-grid-item-self-alignment-simulator-2026-11-04/README.md)에 있다.

## 로컬 로드맵 화면

저장소의 `spec/public/roadmap.html`을 로컬 정적 서버에서 Agent Browser로 확인했다.

- 데스크톱 1280×1000: 문서 너비 1280px, C11.5 카드 너비 908px. 캡처: [desktop.png](desktop.png), DOM·링크 값: [agent-browser-desktop.json](agent-browser-desktop.json)
- 모바일 390×844: 문서와 body 너비 390px, 카드 너비 362px이며 수평 overflow가 없다. 캡처: [mobile.png](mobile.png), DOM·링크 값: [agent-browser-mobile.json](agent-browser-mobile.json)

## Tailnet 미리보기 동기화

Tailscale backend는 `Running`이었고 기존 미리보기 서버 PID `28040`이 `127.0.0.1:8768`에서 제공 중이었다. 이 서버는 재시작하거나 종료하지 않았다. 대상 `roadmap.html`만 교체했으며 삭제는 없었다.

- pass01은 550,637 bytes, SHA-256 `c0e63998f59cc2de79be2de3b8eee0a98cb450f3ad5d8b96cd9ec3eba4e353f4`를 복사했다. 이전 Tailnet 파일 `a43ec70011dc24a509a874f221516dbe9f000f543a0b285b00fd231a4b763952`는 `~/Library/Application Support/SpinonPreviewBackups/roadmap-post-merge-c11-5-pr1128-2026-11-04/pass01/roadmap.html`에 보존했다.
- Agent Browser 확인 중 Tailnet 게시본의 새 내부 문서 경로가 404인 것을 발견했다. 카드 링크를 GitHub `main`의 해당 문서 경로로 바꿨다.
- pass02는 550,857 bytes, SHA-256 `e953474d1b547abb8d2a8c82ccdabb7fab2e3b3bd117955d126ef2c5812c2c37`를 복사했다. pass01 게시본은 `~/Library/Application Support/SpinonPreviewBackups/roadmap-post-merge-c11-5-pr1128-2026-11-04/pass02/roadmap.html`에 같은 해시 `c0e63998f59cc2de79be2de3b8eee0a98cb450f3ad5d8b96cd9ec3eba4e353f4`로 보존했다.
- 두 교체 모두 atomic rename, 백업 선행, no-delete 방식이었다. [pass01 사전 검사](preflight.json), [pass01 복사 기록](copy-and-backup.json), [pass02 사전 검사](preflight-pass02.json), [pass02 복사 기록](copy-and-backup-pass02.json)에 절차와 해시가 있다.
- pass02 local HTTP와 Tailnet HTTPS 모두 200을 반환했고, 각각 550,857 bytes와 SHA-256 `e953474d1b547abb8d2a8c82ccdabb7fab2e3b3bd117955d126ef2c5812c2c37`로 저장소 원본과 일치했다. [제공 검증 기록](served-verification-pass02.json)

## Tailnet 화면과 한계

Tailnet URL `https://macstudio.tailed42f2.ts.net/spinon/roadmap.html`을 Agent Browser에서 확인했다.

- 데스크톱 1440×1000: 문서 너비 1440px, 카드 너비 928px. 캡처: [tailnet-desktop.png](tailnet-desktop.png), DOM 값: [tailnet-agent-browser-desktop.json](tailnet-agent-browser-desktop.json)
- 모바일 390×844: 문서와 body 너비 390px, 카드 너비 362px이며 수평 overflow가 없다. 캡처: [tailnet-mobile.png](tailnet-mobile.png), DOM 값: [tailnet-agent-browser-mobile.json](tailnet-agent-browser-mobile.json)
- 브라우저 캡처는 공개 로드맵의 표시·반응형 동작만 증명한다. 앱 동작, 전체 C11 적합성 또는 테스트 스위트 통과를 증명하지 않는다.

해시 목록은 이 폴더의 근거 파일 전체를 대상으로 하며 README와 해시 목록 자체는 제외한다.
