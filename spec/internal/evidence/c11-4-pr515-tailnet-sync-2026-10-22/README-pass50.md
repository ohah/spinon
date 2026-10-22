# C11.4 PR #515 Tailnet 미리보기 동기화 · pass50

2026-10-22, C11.4 최종 simulator evidence와 갱신한 계획·상태·내부 계약·공개 roadmap을 기존 Tailnet 미리보기에 반영했다. GitHub Pages는 배포하지 않았다.

## 선택 build

- RSPress `2.0.22` 빌드는 7,033개 route/file, 393,981,667 bytes를 만들었다. deterministic output tree SHA-256은 `53bffe0273df4d55f17856a61a250c869896c7ece421b43ea72e6bf5ccaace5d`다. [build 요약](./selected-build-summary-pass50.json)과 [Rspress log](./selected-build-pass50.log)에 남겼다.
- pass49 selected root를 복제하고 갱신한 문서와 curated C11.4 evidence만 overlay했다. 전체 C11.4 regression 출력 중 263 scene 전체 result JSON과 simulator manifests, representative logs, watchdog 근거 및 두 PR screenshot을 포함했다. 무관한 untracked workspace evidence는 site build에 넣지 않았다. [selected-root 기록](./selected-root-preparation-pass50.json).
- 공개 roadmap, 공식 `STATUS`, 내부 계약 0059, C11.4 계획, runtime evidence, 구현/runtime 검토 20개, PR diff 검토 30개와 새 실행 근거 route를 검사했다. C11.4 unchecked 및 `77/213`은 그대로다.

## 안전 복사와 검증

- 7,033개 source route preflight: 485개 신규, 459개 교체, 6,089개 동일. 교체 전 136,724,115 bytes를 pass50 전용 backup에 보관했다. 삭제는 하지 않았다.
- 복사 후 local source/preview hash는 7,033/7,033, 교체 backup hash는 459/459 일치했다. 상세 summary는 [preflight](./preflight-summary-pass50.json), [copy와 backup](./copy-and-backup-pass50.json), [최종 local verification](./final-verification-pass50.json)에 있다.
- Tailnet origin `https://macstudio.tailed42f2.ts.net/spinon/`에서 전체 7,033 route가 HTTP 200이고 response byte 수와 SHA-256이 build와 모두 일치했다. 실패 0. [원격 검증](./remote-verification-pass50.json).
- 별도 [동기화 실패 관점 검토 30개](./sync-review-30-pass50.md)를 기록했다.

## Agent Browser 검증과 정리

- Agent Browser `spinon-pr515-pass50`에서 roadmap의 현재 C11.4 행, `263/263` draw/present, 표본 `219/219`, 공식 `77/213`을 확인했다. STATUS에서 C11.4 unchecked checkbox를 확인했다. 계획과 실행 근거의 현재 행렬·미지원 경계도 읽었다. GitHub PR #515 본문도 열어 두 raw commit-pinned screenshot이 각각 `1080×2400`, `1206×2622`로 표시되는 것을 확인했다.
- 직접 연 두 runtime image route는 Android AVD `1080×2400`, iPhone 17 Pro/iOS 26.2 Simulator `1206×2622`로 로드됐다. [roadmap 화면](./tailnet-roadmap-agent-browser-pass50.png), [status 화면](./tailnet-status-agent-browser-pass50.png), [실행 근거 화면](./tailnet-evidence-agent-browser-pass50.png), [기계 판독 browser 검증](./agent-browser-verification-pass50.json).
- [PR 본문과 첨부 화면 확인](./pr-agent-browser-pass50.png).
- 확인 후 pass50 Agent Browser session을 닫고 활성 session과 해당 프로세스가 0개인지 확인했다. 기존 preview server PID `79812`는 계속 유지했다. tmux와 사용자 Chrome은 건드리지 않았다.

roadmap의 두 simulator runtime matrix는 제한 fixture evidence다. Android는 Apple M4 Max host GPU를 쓰는 AVD이고, iOS Metal 표본은 compositor submission 전이다. 이 페이지는 일반 CSS admission, Android 실기기/device GPU, iOS compositor 이후 pixel 또는 전체 C11.4 완료를 주장하지 않는다.
