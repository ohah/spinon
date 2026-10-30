# C11.4 direct-text aspect-ratio 병합 상태·Tailnet 로드맵 동기화

## 공식 상태

구현 PR [#834](https://github.com/ohah/spinon/pull/834)은 2026-10-30T02:49:15Z Rebase and merge됐고 main merge SHA는 `31361a9b0b01532fb0baa019f8c30d8b75caeb8a`다. PR check rollup은 비어 있었고 `main` required status checks도 설정되지 않아 CI 통과로 기록하지 않는다.

`spec/STATUS.md`, C11.4 집중 계획·상위 계획, 공개 `spec/public/roadmap.html`의 PR 상태를 동기화했다. C11·C11.3·C11.4 공식 체크는 미완료이고 전체 공식 완료는 `72/213`으로 유지한다. 변경은 Ahem 직접 텍스트 2개 항목, 고정 4개 장면의 fixture-test opt-in slice다. 일반 CSS admission, Spinon WPT 적합성 또는 전체 CSS Grid 동작을 주장하지 않는다.

## 문서 빌드

- RSPress v2.0.22 전체 문서 빌드는 변경과 무관한 기존 dead-link 오류로 실패했다. 부분 output 경로는 `build/spinon/c114-aspect-roadmap-sync-site-2026-10-30`이다. 전체 빌드 실패를 C11.4 문서 변경 탓으로 볼 근거는 없다.
- 첫 선택 build 뒤 상위 계획 diff에서 새 항목과 다음 제목의 줄바꿈 누락을 발견했다. 원본을 고쳐 최종 pass02를 새 경로로 빌드했다. 최종 pass02 선택 root는 642개 파일, 20,419,362 bytes이며 RSPress v2.0.22 selected build가 통과했다. 출력은 252개 파일, 13,237,232 bytes다. source 목록·SHA-256은 `selected-root-pass02.json`, 필수 route·output manifest는 `selected-build-verification-pass02.json`에 있다.
- 선택 build는 dead-link checking을 끄고 대상 route와 자산을 별도로 검사했다. 전체 문서 사이트의 링크 적합성 검증으로 확대하지 않는다.

## Tailnet 미리보기 동기화

대상은 기존 `/Users/yoonhb/Library/Application Support/SpinonPreview` 디렉터리이며 Tailnet의 `/spinon/` URL prefix로 제공된다. listener는 기존 `tailnet-preview-server.mjs`였고 재시작하거나 종료하지 않았다.

첫 copy 호출은 Agent Browser가 연 추가 연결 때문에 listener handle이 2개로 관찰되어 copy 전 단계에서 멈췄다. 파일은 바뀌지 않았다([실패 출력](./tailnet-copy-initial-failure.log)). 브라우저를 닫고 listener 외 handle이 없음을 확인한 뒤 최종 pass02를 만들었다. pass02 사전 점검은 신규 19, 교체 17, 동일 216 route였다. 복사는 36개 파일, 교체 전 백업은 17개이며 삭제는 0개다. 백업 위치는 `SpinonPreviewBackups/c11-4-aspect-ratio-roadmap-sync-2026-10-30/pass02/`다.

pass02 사후 검증은 252/252 local target, 17/17 backup, 252/252 Tailnet HTTP 200 및 source SHA-256 일치를 기록한다. preflight·copy·verify 상세는 `tailnet-preflight-pass02.json`, `tailnet-copy-pass02.json`, `tailnet-verify-pass02.json`에 있다.

## Agent Browser 표시와 종료

터미널 브라우저는 사용하지 않았다. 최종 동기화 후 Agent Browser 세션 `spinon-c114-pr834-roadmap-final-pass02`에서 로드맵, 상태 대장, C11.4 상위 계획을 렌더링했다.

- 로드맵에는 PR #834, merge SHA, 4개 scene, Android/iOS 4/4 및 12/12 표면 표본, C11 미완료와 `72/213`이 보인다. [최종 화면](./tailnet-roadmap-agent-browser-pass02.png)
- 상태 대장에서는 C11, C11.3, C11.3.3, C11.4가 unchecked이고 C11.2.2·C11.2.3만 checked다. 공식 완료 수는 `72/213`이다. [최종 화면](./tailnet-status-c11-agent-browser-pass02.png)
- C11.4 상위 계획에서 #834 heading과 뒤의 #811 heading이 별도 heading으로 렌더링되고 merge SHA·공식 수가 보이는 것을 확인했다.
- URL, title, DOM·checkbox 판정은 `agent-browser-verification-pass02.json`에 기록했다. `agent-browser close --all` 후 active session과 프로세스가 모두 0개임을 확인했다([종료 기록](./browser-close-pass02.json)).

Tailnet은 개발 미리보기만 갱신했다. GitHub Pages나 공식 사이트 배포는 수행하지 않았다. PR diff의 별도 30개 실패 관점 검토는 [pr-diff-review-30.md](./pr-diff-review-30.md)에 있다.
