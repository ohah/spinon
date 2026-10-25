# C13.4 scrollbar 사전 비교 · 2026-10-26

## 실행

- 웹 자동화는 Agent Browser `0.31.1`을 사용했다. 실행된 Chrome은 `HeadlessChrome/156.0.0.0`, macOS arm64, DPR 1이다.
- Agent Browser 전용 세션에 `--hide-scrollbars false`를 지정해 native scrollbar가 보이도록 했다. 캡처 뒤 해당 세션만 닫았다.
- 입력: [chrome156-mode-probe.html](chrome156-mode-probe.html)
- 구조화된 결과: [chrome156-result.json](chrome156-result.json)
- 화면: [chrome156-mode-probe.png](chrome156-mode-probe.png)
- 고정 명세: [CSS Overflow Level 3 §4.2 (2025-10-07 WD)](https://www.w3.org/TR/2025/WD-css-overflow-3-20251007/#scrollbar-gutter)

## 관찰

각 상자는 border-box `220×130px`, border `2px`, overflow content `380×280px`다. Chrome DOM의 `clientWidth/clientHeight`, `offsetWidth/offsetHeight`, `scrollWidth/scrollHeight`와 computed `scrollbar-gutter`/`scrollbar-width`를 읽었다.

| ID | gutter | width | client 크기 | offset 크기 | scroll area |
| --- | --- | --- | --- | --- | --- |
| `auto` | `auto` | `auto` | `201×111` | `220×130` | `380×280` |
| `stable` | `stable` | `auto` | `201×111` | `220×130` | `380×280` |
| `both` | `stable both-edges` | `auto` | `186×111` | `220×130` | `380×280` |
| `thin` | `auto` | `thin` | `205×115` | `220×130` | `380×280` |
| `none` | `auto` | `none` | `216×126` | `220×130` | `380×280` |

두 축이 모두 overflow인 조건에서 Chrome 156은 `auto`에 15px씩, `thin`에 11px씩, `none`에 0px의 너비/높이 감소를 적용했다. `stable`은 overflow 상태에서 `auto`와 같은 geometry를 유지했다. `stable both-edges`는 inline 방향 반대쪽에 15px gutter를 하나 더 예약했다. 화면에는 classic scrollbar track과 예약 영역이 보인다.

## 해석 제한

저장소 C13 계획의 기준은 Chrome for Testing `155.0.8059.40`이다. 설치된 Chrome framework에는 `155.0.8059.85`와 `156.0.8078.48`이 있지만 정확히 고정된 patch는 찾지 못했다. 이 실행은 HeadlessChrome 156이며 pinned oracle가 아니다. 관찰한 scrollbar 폭은 UA·빌드·환경 값이지 Android/iOS 상수가 아니다. CSS Overflow 초안은 classic scrollbar가 gutter를 쓰고 overlay scrollbar에는 gutter가 없으며, `stable`은 scrollbar 가시성을 바꾸지 않는다고 규정한다. 이번 사전 비교에는 Android/iOS Simulator mode와 Spinon의 실제 사용자 표시가 포함되지 않는다. 이 파일은 구현이나 CSS 지원 선언의 근거가 아니다.
