# C11.4 PR #1306 병합 후 상태·로드맵 및 Tailnet 동기화

PR [#1307](https://github.com/ohah/spinon/pull/1307)은 2026-11-09T09:50:37Z Rebase and merge로 병합됐다(merge SHA `da5b9c80cc963dded6a56e99bd3c411426058a0d`). 공식 상태·계획·내부 계약·공개 로드맵은 PR #1306의 제한 범위와 merge SHA를 반영한다. C11.4/C11은 미완료, 공식 완료는 `78/214`, 내부 숫자 계약은 `0.1.0`이다. [상태·로드맵 문서 diff 검토 30개](./roadmap-sync-pr-diff-review-30.md)를 보존했다.

## Tailnet 선택 경로 동기화

선택 대상은 `spec/public/roadmap.html`과 `/Users/yoonhb/Library/Application Support/SpinonPreview/roadmap.html`뿐이다. Rspress 전체 사이트 빌드는 기존 evidence 문서에 있는 `.log`, `.json`, 소스 파일 상대 링크의 dead-link 오류로 종료 코드 1을 반환했다. 따라서 전체 문서 빌드 성공을 주장하지 않으며 GitHub Pages도 실행하지 않았다.

첫 동기화 `pass01`은 병합된 PR #1307의 소스를 백업과 SHA 확인 후 제공했다. Agent Browser가 새 카드의 `/spinon/plan/...html`이 Tailnet에서 `Not found`임을 찾아, 카드 내 계획·명세·실행 근거 링크를 GitHub blob Markdown 경로로 바꾸는 후속 수정이 필요했다. `pass01` 원본·복사 기록은 이 변경을 숨기지 않고 보존한다.

후속 수정 `pass02`에서 기존 pass01 제공본을 별도 백업하고, 변경된 로컬 원본과 loopback/Tailnet 응답 모두의 SHA-256을 `968ffa172b5be73de45d9cd904706deb6ca6dfa979e275dabc6c1a464cc7c2f9`로 확인했다. 후보 source의 Git blob SHA-1은 `cb980ec9f8fe81613d7481b56d9a28a3fb59b2b8`이며 [source binding](./candidate-source-binding.json)에 적었다; PR merge 뒤 main tree의 동일성을 재확인한다. Tailscale backend는 `Running`, 예상 server PID `28040`은 `127.0.0.1:8768`에서 응답했다. target open handle은 없었고 같은 디렉터리 임시 파일을 `fsync`한 뒤 atomic rename했다. 서버는 재시작하지 않았다. 자세한 기록은 [pass01 preflight](./preflight-pass01.json), [pass01 복사·백업](./copy-and-backup-pass01.json), [pass02 preflight](./preflight-pass02.json), [pass02 복사·백업](./copy-and-backup-pass02.json), [사용한 스크립트](./sync-tailnet-roadmap.mjs)에 있다.

## Agent Browser 확인

Agent Browser에서 Tailnet 화면의 PR #1306 카드, merge SHA, `78/214`, 다음 계획 미선정 표시를 확인했다. Desktop `1440×1000`과 mobile `390×844` 모두 document/body 가로폭이 viewport와 같고 새 카드가 viewport 안에 표시됐다. 새 카드의 계획·0059·실행 근거·검토 링크는 GitHub blob 페이지로 열렸다. Android 화면은 `1080×2400`, iOS 화면은 `1206×2622`로 실제 로드됐다.

- Desktop: [DOM·폭 기록](./agent-browser-tailnet-desktop-final.json) · [화면](./agent-browser-tailnet-desktop-final.png)
- Mobile: [DOM·폭 기록](./agent-browser-tailnet-mobile-final.json) · [화면](./agent-browser-tailnet-mobile-final.png)
- [링크·이미지 검증](./agent-browser-links-final.json)
- [Browser 종료 상태](./agent-browser-lifecycle.json) · [종료 후 프로세스](./agent-browser-processes-after-close.txt)

Agent Browser session을 닫은 뒤 활성 session이 없음을 확인했다. Nov 8부터 실행 중이던 Chrome과 기존 Terminal Browser daemon은 이번 확인 프로세스가 아니어서 보존했다. Simulator 앱/기기는 이 동기화 작업에서 시작하지 않았다.

## 남은 범위

이 private preview는 선택된 Tailnet roadmap 파일만 갱신한 것이다. 정식 문서 배포, GitHub Pages, 전체 C11.4/C11 완료를 뜻하지 않는다. 공식 완료 `78/214`와 내부 계약 `0.1.0`은 유지한다.
