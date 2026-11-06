# C11 #1220 병합 후 상태·로드맵 문서 동기화 · 2026-11-06

구현 PR [#1220](https://github.com/ohah/spinon/pull/1220)은 `2e022c9589eacb49ec2d04c5ea0dd1cb3ca575af`로 `main`에 Rebase and merge됐다. 이 문서 변경은 해당 merge commit 위에서 C11.1 percentage auto-repeat minimum 구현 결과와 검증 경계를 계획·내부 계약 0059·공식 상태표·공개 roadmap에 반영한다. C11.1/C11 체크박스와 공식 전체 집계 `78/214`는 변경하지 않았다.

## 동기화 범위

- `plan/c11-css-grid.md`: 최신 C11.1 bounded slice가 더 이상 구현 전 후보가 아님을 고치고 결과·한계를 연결했다.
- `plan/c11-root-html-grid-auto-repeat-percentage-min.md`: 구현 전 상태를 병합 결과로 바꾸고 실제 focused/broad suite 결과와 플랫폼 한계를 기록했다.
- `plan/c11-root-html-grid-auto-repeat-zero-min.md`: 이미 병합된 PR #1216이 열려 있다는 오래된 문구를 정정했다.
- `plan/c11-root-html-grid-auto-repeat.md`: PR #1216/#1220 후속을 초기 PR #1179 범위와 구분해 현재 profile 문서에 연결했다.
- `spec/internal/0059-c11-grid-layout.md`, `spec/STATUS.md`: merge SHA, 수락 범위, focused 결과, 두 style-suite 실패, CI 부재, Android/iOS 검증 경계를 기록했다. 내부 숫자 버전은 `0.1.0`을 유지한다.
- `spec/public/roadmap.html`: 최신 #1220 카드를 #1216 카드 앞에 추가했다.

## Agent Browser 확인

사용한 페이지는 변경된 `spec/public/roadmap.html`의 local `file://` route다. 1440×1000 desktop과 390×844 mobile viewport에서 최신 카드의 제목, PR #1220, merge SHA, `78/214`, C11.1/C11 미완료 문구를 렌더링해 확인했다. Mobile에서 document/body width가 390px로 viewport와 같아 가로 overflow가 없었다.

- [Desktop 캡처](./roadmap-local-desktop-agent-browser.png)
- [Mobile 캡처](./roadmap-local-mobile-agent-browser.png)
- [viewport/DOM observation](./agent-browser-observation.json)
- 검증 후 이 실행의 Agent Browser session을 닫았다. `agent-browser session list`는 active session이 없다고 반환했고 `pgrep -x agent-browser`도 비었다. 별도 HWPJS 작업의 Chrome process는 건드리지 않았다.

## 범위와 한계

이 변경은 문서 동기화이며 C11 구현 완료를 선언하지 않는다. `78/214`는 저장소 전체의 공식 완료 집계이지 C11 진행률이 아니다. GitHub PR #1220 checks rollup은 비어 있고 `main` required status checks도 설정되어 있지 않아 CI 통과로 표현하지 않는다. `spinon-style c11_1_root_html` 29개 중 두 auto-row rejection 기대값 실패도 그대로 기록했다. Tailnet preview는 아직 갱신하지 않았으며 문서 PR 병합 후 별도로 제공본 hash와 Agent Browser 화면을 확인한다.

## 변경 검토

서로 다른 실패 관점 30개는 [검토 기록](./docs-pr-diff-review-30.md)에 남겼다.
