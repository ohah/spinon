# C11.4 PR #1199 병합 상태·roadmap/Tailnet PR diff 검토 30개

검토 대상은 PR #1199 병합 결과를 반영한 `spec/STATUS.md`, `spec/public/roadmap.html`과 선택 파일 Tailnet 제공 evidence다. 기준은 merge SHA `8bf916bc2ff8b922846a38e3c52c0ed298b44616`, 원본 C11.4 precomparison, runtime evidence와 현재 branch protection 설정이다.

| # | 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | 제한된 PR #1199를 C11.4 전체 완료로 승격하는가 | PASS — 상위 checkbox를 미완료로 유지한다. |
| 2 | 공식 완료 수가 `78/214`에서 바뀌는가 | PASS — status와 roadmap 모두 `78/214`를 유지한다. |
| 3 | PR merge SHA가 실제 GitHub merge와 다른가 | PASS — PR #1199 `8bf916bc2ff8b922846a38e3c52c0ed298b44616`와 일치한다. |
| 4 | 날짜·시각 표기가 merge 시각을 과장하는가 | PASS — `2026-11-05T23:18:52Z` UTC 시각과 서울 날짜 2026-11-06을 분리한다. |
| 5 | 기존 #1187/#1190/#1195 history가 수정 또는 제거되는가 | PASS — 새 note만 관련 #1195 기록 다음에 추가한다. |
| 6 | 새 roadmap card가 다른 C11 항목 아래 잘못 놓이는가 | PASS — C11.4 multiple-text note를 #1195 intrinsic columns/text rows card 바로 뒤에 둔다. |
| 7 | HTML card가 문서 구조를 깨거나 p element를 중첩하는가 | PASS — 새 card는 기존 hero-progress `<p>`와 같은 sibling 구조이며 Tailnet browser에서 렌더링됐다. |
| 8 | plan, contract, status의 지원 경계가 서로 다른가 | PASS — 여러 direct ASCII `A` item이라는 제한과 parent incompletion을 맞춘다. |
| 9 | 10,000 track limit이 생략되거나 잘못 설명되는가 | PASS — 10,000 frame 보존과 10,001 사전 거부를 표시한다. |
| 10 | trace 생략을 layout 결과 실패 또는 frame 추정으로 묘사하는가 | PASS — PR evidence와 internal contract의 선택 진단 한계를 그대로 따른다. |
| 11 | focused test 수가 evidence와 다르게 요약되는가 | PASS — style 4, helper 3, runtime 7, regression 1+1을 각 로그와 대조했다. |
| 12 | check rollup 부재를 CI 성공으로 표현하는가 | PASS — check run 및 main required status checks가 없다고 명시한다. |
| 13 | iOS/Android 캡처를 실기기 결과라고 부르는가 | PASS — API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator로 한정한다. |
| 14 | simulator screenshot을 PixelCopy/Metal pixel equality로 확대하는가 | PASS — readback 및 compositor parity가 아니라고 명시한다. |
| 15 | Chrome oracle result와 Spinon runtime result를 혼동하는가 | PASS — Chrome fixture rows와 별도의 runtime/simulator 근거를 구분한다. |
| 16 | public CSS admission, WPT 또는 일반 text/font 범위를 추가로 주장하는가 | PASS — 해당 지원 범위를 남은 작업으로 둔다. |
| 17 | 새 plan 링크가 Tailnet 제공 root에 없는 `.html` route를 가리키는가 | PASS — GitHub `blob/main` Markdown URL을 사용한다. |
| 18 | 새 execution/review links가 404인가 | PASS — GitHub main README와 review URL이 HTTP 200이다. |
| 19 | Tailnet STATUS link가 사라지거나 잘못된 route인가 | PASS — `/spinon/STATUS.html#c11-css-grid`가 HTTP 200이다. |
| 20 | Android/iOS raw screenshot link가 잘못된 commit/path인가 | PASS — merge SHA의 두 raw PNG가 HTTP 200, 1080×2400 및 1206×2622다. |
| 21 | source card가 merge 상태인데 draft PR처럼 표시되는가 | PASS — PR link label은 Rebase and merge이며 exact merge SHA/시각이 있다. |
| 22 | Tailnet status/backend가 죽었거나 다른 프로세스가 8768을 소유하는가 | PASS — Tailscale Running과 예상 server PID 28040을 확인했다. |
| 23 | server path가 repo 파일을 직접 제공한다고 잘못 가정하는가 | PASS — 별도 `SpinonPreview/roadmap.html` target을 검사·교체했다. |
| 24 | 교체 중인 target에 열린 handle이 있어 unsafe overwrite가 되는가 | PASS — preflight에서 target handle PID가 비어 있었다. |
| 25 | 기존 preview를 backup 없이 덮어쓰는가 | PASS — old SHA와 byte 수가 기록된 별도 pass01 backup이 있다. |
| 26 | target 바꿀 때 unrelated file을 지우거나 server를 restart하는가 | PASS — `roadmap.html` 한 파일만 atomic rename했고 deletion/restart는 없었다. |
| 27 | local source와 local HTTP served content가 불일치하는가 | PASS — 두 쪽 모두 SHA `4c56c7e6…dd97e54`, 586,947 bytes다. |
| 28 | Tailnet HTTPS가 stale copy를 제공하는가 | PASS — remote HTTP 200과 source/local hash 일치를 확인했다. |
| 29 | responsive card가 desktop/mobile에서 잘리거나 수평 overflow하는가 | PASS — desktop 1440/928px card, mobile 390/362px card와 no horizontal overflow를 확인했다. |
| 30 | PR 본문의 Agent Browser screenshots가 실제로 표시되는가 | PASS — Agent Browser에서 PR #1200을 열어 두 raw image가 `complete=true`, desktop 1440×1000 및 mobile 390×844로 로드되어 실제 표시됨을 확인했다. `close --all` 뒤 Agent Browser/Chrome process가 남지 않았다. |

## 검토 결과

**PASS 30 / PARTIAL 0 / FAIL 0.** Tailnet에서 source와 served bytes/hash, 화면의 merge note·links 및 desktop/mobile geometry를 확인했다. PR #1200 본문의 두 image 렌더링과 Agent Browser process cleanup까지 확인했다. 전체 docs build와 GitHub Pages 배포는 이 변경의 완료 증거가 아니다.
