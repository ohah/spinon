# C12.5 sticky precomparison · 2026-10-26

## 실행 조건

- 브라우저 자동화: Agent Browser 0.31.1, session `spinon-c125-12a2b46193b3`; 실제 상호작용은 Agent Browser로 수행했다. `terminal-browser`는 사용하지 않았다.
- Browser UA: `HeadlessChrome/156.0.0.0`; macOS host `26.5.1 (25F80)`, Agent Browser-reported viewport `1280×633`, DPR 1.
- fixture: `tests/fixtures/css/c12/position-sticky.html`, SHA-256 `9b41d3fa3fcae300337a67be568b9eb71ba26828cbcca380434c9797b177be8e`.
- unchanged base JSON sample values remain recorded alongside the added parent content-edge comparison.

## 관찰한 값

Agent Browser에서 고정 fixture를 열고 다음 순서로 `getBoundingClientRect()`와 두 scroller의 `scrollTop`을 읽었다: 최초 `0/0`, outer `24/0`, outer 복귀 뒤 inner `20`, `36`, `48`, `96`, `128`, `160`, `212`. 출력 JSON의 수치는 첫 관찰 호출 stdout에서 옮겼으며 key만 JSON snake_case로 정리했다.

| outer / inner scrollTop | inner top | section top | sticky top | sticky 판정 |
| ---: | ---: | ---: | ---: | --- |
| 0 / 0 | 74 | 118 | 118 | natural frame |
| 24 / 0 | 50 | 94 | 94 | inner offset 0이라 outer와 함께 이동 |
| 0 / 20 | 74 | 98 | 98 | 아직 top 제약 전 |
| 0 / 36 | 74 | 82 | 90 | top 제약에 도달 |
| 0 / 48 | 74 | 70 | 90 | sticky |
| 0 / 96 | 74 | 22 | 90 | sticky |
| 0 / 128 | 74 | -10 | 90 | sticky |
| 0 / 160 | 74 | -42 | 90 | sticky |
| 0 / 212 | 74 | -94 | 54 | containing block 끝 제약 |

`inner` top 74, border top 2, padding top 6, `top:8px`에서 pinned top은 90px였다. C13.2의 scrollport record는 border 안쪽 y=76에서 시작하므로, 이번 관측에서는 scrollport origin에 padding top과 CSS sticky inset을 더해야 같은 지점을 얻는다. 이 계산은 고정 Chrome sample에만 한정한다.

## Parent content-edge comparison

같은 HeadlessChrome 156 fixture의 추가 [고정 variant](../../../../tests/fixtures/css/c12/position-sticky-parent-content-edge.html)에 `#section { padding-bottom:12px; border-bottom:2px solid #333 }`를 넣었다 (SHA-256 `5a00b8a86ca7a2e301cec1c646a603aa0a5627ae08d8873b8d9a750dc333a141`). `outer.scrollTop=0`, `inner.scrollTop=212`에서 section border box는 top `-94`, bottom `82`, height `176`; content edge bottom은 `68`; sticky border box는 top `40`, bottom `68`, height `28`이었다. `elementFromPoint(200,78)`는 section을, `(240,100)`은 section-after를 반환했다. Agent Browser 화면 PNG에서도 두 표본 RGBA는 각각 `(245,245,245,255)`와 `(204,221,238,255)`였다. [PNG](./chrome156-parent-content-edge.png)와 해시는 아래 JSON에서 연결한다.

Runtime gate는 Chrome에서 직접 sample한 `(200,78)` section 배경 `(245,245,245,255)`와 `(240,100)` section-after `(204,221,238,255)`를 두 표본으로 사용한다. sticky top `40`은 inner scrollport client clip top `76` 위에 있어 마지막 offset에서는 sticky paint가 clip된다는 점도 함께 검사한다.

같은 고정 viewport에서 `outer.scrollTop=0`, `inner.scrollTop=212`인 종료 sample도 Agent Browser로 다시 열어 실제 Chrome screenshot pixel을 읽었다. fixture viewport pixel `(200,78)`의 sticky 색은 `(219,51,68,255)`, `(240,78)`의 section 배경은 `(245,245,245,255)`였다. DOM hit target과 computed background는 각각 `sticky`/`rgb(219, 51, 68)`, `section`/`rgb(245, 245, 245)`다. screenshot, coordinate, raw RGBA와 SHA-256은 [색상 관찰 JSON](./chrome156-color-observation.json)에 기록했다.

## 보존·재현 한계

첫 Agent Browser eval의 전체 수치 응답은 이 대화의 tool output에 보존됐고, 이 JSON은 그 stdout을 사람이 artifact로 옮긴 사본이다. 이후 같은 세션에서 scroll offsets를 초기화하지 않은 채 한 번 더 실행한 stdout은 첫 `initial` sample도 `inner.scrollTop=212`였으므로 사용하지 않았다. 그 때문에 JSON의 초기 관찰을 오염시키지 않도록 해당 artifact를 첫 stdout 수치로 다시 작성했다. screenshot은 두 동작이 겹친 시점에 저장되어 UI 상태를 확정할 수 없어 제거했고 어떤 비교 기준으로도 사용하지 않는다.

이 precomparison은 base geometry oracle, nested scrollport 및 parent content-edge 경계를 고정한다. Chrome JSON SHA-256: `8f3a9893a841422fde4679a3dd2d79dc559815c13f6b3203ef2f61f8ac70fc3b`; variant screenshot SHA-256: `16a6e256b15a68bf8dce745c0243b312bd295c78bd585ae2440cabfb9c2c4f61`. Android/iOS output, reverse offset input, product CSS admission, WPT 결과는 포함하지 않는다. 구현 후에는 현재 scroll offset이 같을 때 역방향 이동 뒤 frame이 같은 oracle state로 복구되는지 runtime invariant로 확인한다. Chrome input/output path를 추가로 캡처하기 전까지 역방향 동작은 Chrome-observed 사실로 표현하지 않는다.
