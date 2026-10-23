# C11.4 현재 코드·로드맵 재대조 · 2026-10-24

## 기준과 발견

기준 revision은 `1b00ba0d8c5e0014c3b41ff2efb1a093917980b8` (`main`과 `origin/main`이 같은 상태)다. 공식 구현 상태는 [`spec/STATUS.md`](../../../STATUS.md)이며, 공개 구현 목록은 [`spec/public/roadmap.html`](../../../public/roadmap.html)에서 이 상태를 표시한다.

이번 재대조에서 C13 전체는 미완료이고 현재 실행 lane은 C11.4라는 판정이 일치했다. C13.1–C13.3의 제한 opt-in slice는 남아 있지만 public DOM scroll/CSSOM, ink-only shadow overflow, scrollbar/gutter, scroll snap 및 logical axis는 완료되지 않았다. Tailnet Agent Browser에서 C13 checkbox는 unchecked, 공식 집계는 `72/213`, 활성 lane은 C11.4로 확인했다. 따라서 C13 상태 변경은 필요하지 않았다.

C11.4의 상위 계획에는 이미 구현된 고정 경계 두 개가 남은 일처럼 적혀 있었다. 계획 문구를 현재 근거에 맞춰 고쳤다.

- [PR #415](https://github.com/ohah/spinon/pull/415)의 [`spanning-auto-min-flex-tracks` 실행 근거](../c11-4-spanning-auto-min-flex-text-2026-10-21/README.md)는 두 `minmax(0, 1fr)` track의 48px/48px 분배와 두-track span을 확인한다. 이는 해당 고정 scene에 한정된다.
- [PR #385](https://github.com/ohah/spinon/pull/385)의 [실행 근거](../c11-4-negative-implicit-track-sizing-2026-10-20/README.md)는 expanded extent의 9,999/10,000/10,001 산술 경계와 Spinon 소유 reservation 실패 시 partial output 부재 및 동일 worker 재사용을 확인한다. 10,000-track 전체 layout 성공이나 Taffy 내부 allocator OOM 복구는 확인하지 않았다.
- Mixed intrinsic/flexible/fixed의 넓은 조합, empty/hidden item 상호작용, 나머지 `calc()` 조합과 repeat/placement cross-product는 계속 미완료다. C11.4 상위 checkbox와 `72/213`은 유지한다.

## 수정 및 확인

- [`plan/c11-4-grid-track-sizing.md`](../../../../plan/c11-4-grid-track-sizing.md)의 현재 상태 날짜를 2026-10-24로 맞추고, 완료된 고정 fixture와 남은 교차 조합 및 할당 한계를 분리했다.
- [C11.4 계획 diff 적대 검토 20개](./pr-diff-review-20.md)를 남겼다.
- RSPress `2.0.22` 문서 빌드가 11.6초 만에 성공했다. 선택한 7개 Tailnet route는 HTTP `200`과 빌드 산출물 SHA-256 일치로 모두 확인했다. 전체 사이트 link 검사나 모든 route의 remote 검증은 하지 않았다. preflight 당시 listener PID `1191`의 active connection은 `0`이었다. 기존 파일 3개(2,122,241 bytes)를 백업하고 4개 route를 추가했으며 삭제는 없었다.
- Agent Browser에서 Tailnet의 C11.4 계획 route를 열어 PR #415/#385 근거와 trace budget·allocator 한계를 확인했다. 화면은 1280×633이며 SHA-256은 `0362454ca835f950f659b4168127c87f201d838703f21f5e594e108b11c3b0d9`다.
- 이 문서 변경에는 Rust source가 없으므로 Rust tests는 실행하지 않았다. `git diff --check`는 통과했다.

![Agent Browser에서 확인한 C11.4 계획](./tailnet-c114-plan-agent-browser-pass01.png)
