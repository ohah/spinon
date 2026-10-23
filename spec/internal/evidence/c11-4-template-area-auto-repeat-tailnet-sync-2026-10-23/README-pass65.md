# C11.4 Tailnet roadmap 동기화 · pass65

2026-10-23 KST, PR #539와 #540이 `main`에 병합된 SHA `92b447f687e751cfc2c333bd17e22c4271c36536` 기준으로 Tailnet preview를 갱신했다. 기존 pass64 선택 루트에서 시작했고 미추적 workspace 증거는 입력에 포함하지 않았다. Tailnet preview server PID 1191과 tmux는 유지했다.

## 선택 문서 build

- pass64 선택 루트에서 `plan/`과 `spec/` 변경을 적용했다. main diff의 문서 경로 131개를 515개 route alias로 overlay했다. 선택 루트는 16,989 files / 623,979,370 bytes, tree SHA-256 `3ecf12c0fa32b3963ed271fe80b23cb6a6c018ea75c7a4186c4b77c940778f89`다. [준비 manifest](./selected-root-preparation-pass65.json).
- RSPress 2.0.22 선택 build가 성공했다. 로그의 표 출력 정렬용 끝 공백만 제거해 저장했다. 결과는 9,640 files / 540,457,131 bytes, tree SHA-256 `5ae4914feb71dfb8bbca505ee071c9b5b3e47409ba62d2e59f436942c676248b`다. [build log](./selected-build-pass65.log) · [요약](./selected-build-summary-pass65.json).
- `public/` 8,424개 파일 중 8,423개를 동일 볼륨 hardlink로 연결했고 한 파일은 생성된 파일과 hash가 같았다. 300,245,765 bytes의 public asset payload를 중복 복사하지 않았다.

## Tailnet 복사와 hash 검증

- Preview base URL은 `https://macstudio.tailed42f2.ts.net/spinon/`이고 roadmap은 [`/spinon/roadmap.html`](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html)이다.
- 사전검사에서 9,640 files: 신규 835, 교체 553, 동일 8,252, 삭제 0이었다. 교체 전 553개는 `SpinonPreviewBackups/c11-4-pr540-tailnet-sync-2026-10-23/pass65`에 같은 볼륨 hardlink로 보존했다.
- 로컬 대상 9,640/9,640, 백업 553/553이 size와 SHA-256으로 일치한다. Tailnet HTTPS 응답도 9,640/9,640개 HTTP 200 및 build-source byte hash와 일치했고 실패는 0이다. [preflight](./preflight-pass65.json) · [copy/backup](./copy-and-backup-pass65.json) · [local verification](./final-verification-pass65.json) · [remote verification](./remote-verification-pass65.json).

## Agent Browser 확인

- roadmap에 #539 병합 SHA와 범위가 표시되고, 공식 완료 수 `77/213`과 C11.4 미완료가 유지된다. [roadmap screenshot](./tailnet-roadmap-agent-browser-pass65.png).
- PR #540의 로드맵 HTML 검토 페이지에서 ordered list 30개가 렌더된다. 집중 계획은 PR #539를 보여주고, 내부 계약 0059는 `0.1.0`이다.
- runtime README의 Android·iOS 화면은 각각 `complete=true`, `1080×2400`, `1206×2622`로 표시됐다. scene `13/13`, 표면 표본 `66/66`, APK/IPA 한계와 `visibility:hidden` 한계도 페이지 텍스트에 있다.
- 전체 관찰은 [Agent Browser DOM 검증](./agent-browser-verification-pass65.json)에 보존했다.

## 후속 수정

Agent Browser가 로드맵의 기존 상태 링크 `#c11-css-grid`가 pass65 `STATUS.html` DOM에 없음을 확인했다. 같은 문서에서 `#css-구현-체크리스트`는 실제 heading으로 열리고 브라우저에서 확인됐다. 따라서 pass65는 파일·hash 동기화에는 성공했지만 로드맵의 해당 fragment는 깨져 있었다. 소스는 후속 PR #541에서 올바른 heading으로 바꾸며, 병합 뒤 roadmap 한 파일을 pass66으로 갱신하고 최종 URL·DOM anchor를 재검증한다.

PR #539의 simulator 결과는 PR 본문과 runtime README에 있는 Android API 37 ARM64 AVD·iPhone 17 Pro/iOS 26.2 Simulator 결과다. pass65는 문서 build·preview 검증이며 새 앱·기기 테스트를 수행하지 않았다. C11.4와 공식 완료 수 `77/213`은 미완료·유지다.
