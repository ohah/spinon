# C13 실제 코드와 로드맵 동기화

## 변경 전 Agent Browser 관찰

- URL: `https://macstudio.tailed42f2.ts.net/spinon/roadmap.html`
- 표시 집계: 공식 완료 `72/213`; 체크박스 213개 중 72개가 선택됐다.
- `overflow·scroll·클리핑`은 비활성화된 unchecked 항목이다. 카드에는 C13.1 clip runtime 근거만 요약되어 있고, 현재 대장과 코드에 이미 기록된 C13.2 scroll-state 및 C13.3 gesture 구현 범위가 보이지 않았다.
- 실제 화면과 DOM 관찰은 [Agent Browser 캡처](./roadmap-before-agent-browser.png), [관찰 JSON](./roadmap-before-observation.json)에 보존했다.

## 코드와 공식 대조

- 공식 원본은 [구현 상태 대장](../../../STATUS.md)의 C13 항목이다. 부모 C13은 미완료로 유지한다. 같은 대장의 하위 기록은 C13.2 scroll owner·offset snapshot, root/body 전파·lifecycle과 visible-overflow 표면 표본, C13.3 typed policy·delta router·gesture session/queue 및 플랫폼 fixture 결과를 기록한다.
- [`spinon-ffi` feature 선언](../../../../crates/spinon-ffi/Cargo.toml)은 `c13-2-scroll-fixture-test`와 `c13-3-gesture-fixture-test`를 별도로 둔다. [`runtime_gpu.rs`](../../../../crates/spinon-ffi/src/runtime_gpu.rs)는 해당 adapter state와 재적용 경로를 이 feature들로 조건부 컴파일한다.
- C13.2의 제한 경로는 [`c13_2_scroll_host.rs`](../../../../crates/spinon-ffi/src/runtime_gpu/fixtures/c13_2_scroll_host.rs), C13.3의 앱 검증 경로는 [`c13_3_scroll_gesture_host.rs`](../../../../crates/spinon-ffi/src/runtime_gpu/fixtures/c13_3_scroll_gesture_host.rs)에 있다. Gesture style projection과 Rust routing/session은 [`runtime_scroll_gesture.rs`](../../../../crates/spinon-style/src/stylo_dom/cascade/runtime_scroll_gesture.rs), [`scroll_gesture.rs`](../../../../crates/spinon-render/src/runtime/scroll_gesture.rs), [`scroll_gesture_session.rs`](../../../../crates/spinon-render/src/runtime/scroll_gesture_session.rs), [`scroll_input_queue.rs`](../../../../crates/spinon-render/src/runtime/scroll_input_queue.rs)에 있다.
- 남은 경계는 ink-only `box-shadow` overflow, 플랫폼별 native wheel의 부분 callback 근거, 공개 DOM scroll event/CSSOM과 displayed-frame input 연결, scrollbar/gutter·scroll snap·logical-axis gate다. 이 동기화는 구현 완료 체크를 추가하지 않는다.

## 동기화할 문서

- `spec/STATUS.md`의 C13 요약에 C13.2·C13.3의 제한 구현과 남은 경계를 추가한다. C13 checkbox와 집계는 유지한다.
- `plan/c13-overflow-scroll-clipping.md`의 현재 상태를 고치고 C13.2/C13.3 단계 요약을 실제 코드와 맞춘다. 2026-10-15 실행 기록은 역사적 근거로 표시한다.
- Tailnet roadmap은 `roadmap.html`에서 `main`의 `spec/STATUS.md`를 읽기 전용으로 다시 가져온다. PR 병합 뒤 Agent Browser로 같은 항목과 `72/213`을 다시 확인하고 캡처를 추가한다.

`SHA256SUMS`는 변경 전 화면과 DOM 관찰 파일의 무결성을 기록한다.

## 병합 후 Tailnet 재확인

- 공식 상태 동기화 PR [#565](https://github.com/ohah/spinon/pull/565)은 2026-10-23 11:41:31 UTC에 병합됐고 merge SHA는 `af8baa7bbed5a415f078f084cac323f77804d143`다.
- 같은 Tailnet `/spinon/roadmap.html`을 Agent Browser에서 다시 열었다. [화면](./roadmap-after-agent-browser.png)에는 갱신된 C13.2/C13.3 요약이 보이고, C13 checkbox는 disabled·unchecked로 남았다. 집계는 `72/213`이다. [DOM 관찰](./roadmap-after-observation.json)은 2026-10-23T11:42:28.687Z, viewport 1280×633 CSS px, DPR 1을 기록한다.
- `spec/public/roadmap.html`은 GitHub `main`의 `spec/STATUS.md`를 가져오며 요청에 `cache: 'no-store'`를 사용한다. Tailnet preview server나 정적 사이트 파일을 교체하지 않고, 기존 route의 실제 재요청 결과를 확인했다.
- before/after 캡처와 DOM JSON의 SHA-256은 `SHA256SUMS`에 있다.

## 현재 진행 칸과 공식 상태의 재대조 · 2026-10-23

- 확인 결과 `main`의 `spec/public/roadmap.html`에는 `execution.inProgress: ['C13']`가 남아 있었지만, C13.3 computed-policy simulator matrix PR #185는 이미 병합됐고 C13 전체는 미완료다. 이 수동 칸반 설정이 C13을 여전히 진행 중처럼 보이게 한 원인이다.
- 보드 설정을 현재 작업 lane인 C11.4로 바꾸고, C13 계획·상태 대장에는 C13이 비활성임을 명시했다. C13과 C11.4의 공식 체크박스는 모두 미완료이며 집계는 `72/213`으로 유지한다.
- Agent Browser가 같은 Tailnet route를 2026-10-23T13:33:55.387Z에 확인했다. 칸반의 진행 중 열은 `1/2`와 `C11.4 · ACTIVE`, C13은 disabled·unchecked, 완료는 `72`였다. [현재 화면](./roadmap-current-agent-browser.png) · [DOM 관찰](./roadmap-current-observation.json).
- 이 캡처는 Tailnet preview의 현재 작업 표시를 검증한다. 현재 변경이 GitHub `main`에 반영된 뒤에는 새로고침으로 동일한 상태를 다시 확인한다. browser process는 검증 직후 종료했고 `agent-browser session info --json`에서 `active: false`를 확인했다.
- 새 화면과 DOM 관찰의 SHA-256은 `SHA256SUMS`에 있다.
