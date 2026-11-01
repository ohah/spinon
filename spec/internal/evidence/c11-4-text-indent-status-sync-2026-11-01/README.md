# C11.4 text-indent flags 상태 대장 동기화

2026-11-01에 병합된 구현 PR #959는 anonymous Grid direct text의 `text-indent: hanging`·`each-line` 제한 opt-in profile을 구현했다. `spec/STATUS.md`에는 아직 같은 내용을 계획 후보이며 제품 구현이 없다고 기록하고 있어, 공식 상태 문구를 PR 본문 및 병합된 구현·실행 근거에 맞춰 바로잡는다.

- PR: https://github.com/ohah/spinon/pull/959
- merge SHA: `590f5946e97e0252113abf34b23ae31b13819acf`
- 상태 기준: 공식 `72/213`, C11 및 C11.4 상위 항목 미완료
- 변경 범위: `spec/STATUS.md`의 text-indent flags 이력 한 단락과 본 README 및 상태 변경 검토 기록
- 기존 계획·비교·실행 증거: `plan/c11-4-anonymous-grid-text-indent-flags.md`, `spec/internal/evidence/c11-4-anonymous-grid-text-indent-flags-precomparison-2026-11-01/README.md`, `spec/internal/evidence/c11-4-anonymous-grid-text-indent-flags-runtime-2026-11-01/README.md`

## 판정

상태 문구의 병합·검증·범위·집계 연결은 [서로 다른 실패 관점 30개](status-sync-review-30.md)로 다시 대조했다. PR #959의 본문은 AVD와 iOS Simulator에서 20개 flags scene, 18개 positive scene, positive control을 포함한 30개 negative matrix를 각각 통과했다고 기록한다. 136개 scene의 draw/present, zero superseded retry, 설치 artifact hash와 제한을 기존 runtime evidence가 설명한다. 이 동기화는 그 제한 결과를 공식 상태 대장에 옮길 뿐이며 새 제품 동작이나 전체 C11.4 완료를 주장하지 않는다.

PR #959에는 GitHub check run이 보고되지 않았다. CI 성공으로 기재하지 않는다. 일반 CSS admission, 전체 WPT, 글리프 픽셀, 실기기/hardware GPU 및 iOS compositor 이후 화면도 검증한 것으로 표시하지 않는다.

## Tailnet 미리보기

Tailscale은 `Running`/online이고 기존 preview server PID 28040이 `127.0.0.1:8768`에서 동작했다. 고정 RSPress 2.0.22 selected-root build는 434개 source file을 사용했고 성공했다. 전체 저장소 dead-link 검사는 실행하지 않았으며 이 선택 build에서는 dead-link/image 검사 설정이 꺼져 있다.

생성된 160개 대상 route에서 새 파일 72개, 기존 교체 72개, 동일 16개를 예상했다. 기존 README route 하나는 PID 28040이 열어 둔 것을 확인했다. 새 HTML은 old HTML과 비교해 본문은 같고 hashed JavaScript 참조만 달라 해당 route는 보존했다. 그 결과 72개 생성, 71개 교체, 16개 동일, 1개 open route 보존이다. 교체 71개 backup의 hash를 검증했고 삭제나 server restart는 없다. 160개 route 모두 local target과 HTTPS 응답 200 및 예상 hash를 확인했다. 검증 바이트는 15,344,958이다. [route delta](status-route-delta-pass02.json) · [게시 적대 검토 30개](tailnet-publication-review-30.md) · [preflight](tailnet-preflight.json) · [copy/backup](tailnet-copy-and-backup.json) · [served verification](tailnet-served-verification.json).

Agent Browser에서 Tailnet `STATUS.html#c11-css-grid`의 PR #959 merge SHA, C11 미완료와 `72/213`을 확인했다. 공개 roadmap에서도 PR #959·#962, `72/213` 및 stale text-indent plan-candidate 문구의 부재를 확인했고 문서 너비 1280px이었다. [브라우저 결과](browser-verification.json) · [상태 화면](status-agent-browser.png) · [로드맵 화면](roadmap-agent-browser.png).

Agent Browser 세션과 전용 프로세스를 종료했다. 기존 Tailnet preview server는 실행 상태를 유지한다.
