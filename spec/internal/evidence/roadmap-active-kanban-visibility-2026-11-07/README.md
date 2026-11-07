# 공개 로드맵 진행 칸 가시성 · 2026-11-07

## 문제와 변경

PR #1257은 `C11.4 · flex-end 계획`을 진행 카드에 추가했지만, 공개 로드맵의 칸반이 긴 변경 이력 뒤에 있어 데스크톱 최초 화면에서 보이지 않았다. Agent Browser로 당시 Tailnet 제공본을 열어 1440×1000 viewport에서 Kanban y=`46672.66px`, active card y=`46770.66px`을 확인했다([이전 위치 기록](./before-position.json)). 카드가 DOM에 있어도 방문자가 진행 작업을 볼 수 없는 배치였다.

칸반을 페이지 제목 바로 뒤로 옮기고 전체 shell 폭을 사용하게 했다. 모바일에서는 제목과 설명을 세로로 배치하며, 칸반을 렌더링하거나 viewport가 바뀔 때 진행 열로 수평 스크롤을 이동해 active card가 첫 화면에 나오게 했다. 다른 열은 칸반 내부를 좌우로 밀어 확인한다. 진행 카드는 기존 `execution.detailInProgress` 데이터에서 계속 생성된다.

## Agent Browser 후보 확인

Agent Browser 0.31.1에서 로컬 정적 preview를 사용했다. Terminal Browser는 사용하지 않았다.

- 데스크톱 1440×1000: 진행 카드 y=`455.02px`, 칸반 bottom y=`773.02px`; 둘 다 첫 viewport 안이다. 문서 폭은 viewport와 같은 1440px이다. [DOM 관측](./candidate-desktop.json) · [screenshot](./candidate-desktop-agent-browser.png)
- 모바일 390×844: 진행 카드 y=`438.53px`, 칸반 내부 `scrollLeft=272px`로 진행 열을 보인다. 문서 폭은 viewport와 같은 390px이며 page-level horizontal overflow는 없다. [DOM 관측](./candidate-mobile.json) · [screenshot](./candidate-mobile-agent-browser.png)
- 두 viewport에서 공식 상태 동기화 문구는 완료 `78/214`를 표시한다. C11.4/C11 완료 상태는 변경하지 않았다.
- Agent Browser session과 로컬 preview server를 닫았고, 종료 뒤 Agent Browser 및 HeadlessChrome process는 남아 있지 않다. [lifecycle](./candidate-browser-lifecycle.json)

이 screenshot과 DOM 관측은 로컬 후보 변경의 시각 확인이다. PR 병합 후 Tailnet 제공본은 별도로 갱신하고 Agent Browser로 다시 확인해야 한다.

[PR 변경 실패 관점 검토 30개](./pr-diff-review-30.md)
