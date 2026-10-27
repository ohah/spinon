# PR #710 로드맵 Tailnet 동기화 근거 · 2026-10-28

## 목적과 범위

PR #710 병합 내용을 상태 대장, C11.4 계획, 공개 로드맵에 반영한 문서 커밋 `610fa07c42d6bcacd3aa419f552c19aac9d08f89`를 개발용 Tailnet 미리보기에 게시했다. 이 동기화는 미리보기만 갱신하며 공식 구현 완료 수나 CI 상태를 바꾸지 않는다.

## 원본과 게시 결과

- 원본 commit: `610fa07c42d6bcacd3aa419f552c19aac9d08f89`
- 원본 tree: `9fb7e0c52f481f39c7d96bbc7ebe49c1d974bc01`
- 선별 원본: 194개 파일, 390개 staged route, 8,452,188 bytes
- 게시 집합: 431개 route, 12,800,039 bytes
- 사전 상태: 409개 생성, 7개 교체, 15개 동일
- 교체 전 백업: 7개 파일, 3,176,637 bytes; route 삭제 없음
- 로컬 미리보기 `http://127.0.0.1:8768/spinon/roadmap.html`과 Tailnet `https://macstudio.tailed42f2.ts.net/spinon/roadmap.html`이 모두 HTTP 200으로 응답했다. 서버는 검증 중 PID 28040으로 `127.0.0.1:8768`에서 수신 중이었다.
- 431개 모든 route를 로컬과 Tailnet 양쪽에서 HTTP 200, 예상 크기, publication manifest SHA-256과 대조했다. 전부 일치한다.
- 백업 7개를 다시 읽어 크기와 SHA-256을 확인했다. 전부 일치한다.

## Agent Browser 확인

로드맵에서 PR #710 항목, 병합 SHA, C11.4 미완료, 공식 `72/213`, CI 통과를 주장하지 않는다는 문구를 확인했다. 집중 계획, 실행 근거, 상태 대장 링크도 열렸다. 연결된 Android 이미지(1080×2400)와 iOS 이미지(1206×2622) 모두 Tailnet 주소에서 로드됐다. 페이지 캡처는 [tailnet-roadmap-pr710-agent-browser.png](tailnet-roadmap-pr710-agent-browser.png)이며 브라우저 관찰값은 [browser-verification.json](browser-verification.json)에 있다.

## PR #710 및 한계

PR #710은 `58cca2afa6086a6e4b8517ecd1d5d5a4012c6a94`로 Rebase and merge 됐다. PR check rollup은 비어 있고 main required status checks도 설정되지 않아 CI 성공으로 표시하지 않았다. Android 8/8 및 iOS 8/8 simulator scene 검증은 기존 static scene과 generic mixed scene의 입력·trace·frame·draw/present 결과다. 화면 캡처는 host surface와 layout boxes를 보여 주며 glyph/image texel paint, 일반 CSS admission, WPT, 실기기 동작을 증명하지 않는다. C11.4는 미완료이고 공식 완료 수는 계속 `72/213`이다.

## 원본 자료

- [Tailnet 사전 검사](tailnet-preflight.json)
- [동기화 결과와 백업 목록](tailnet-copy.json)
- [백업 재검증 결과](backup-verification.json)
- [431개 route 로컬·Tailnet 검증](tailnet-verify.json)
- [동기화 스크립트](sync-tailnet.mjs)
- [게시 검증 스크립트](verify-tailnet.mjs)
- [백업 검증 스크립트](verify-backups.mjs)
- [후속 문서 PR diff 검토 30개](pr-diff-review-30.md)
