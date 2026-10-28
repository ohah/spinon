# C11.4 two-track span 계획 PR diff · 30개 독립 실패 관점

검토 대상은 이 계획 PR의 집중 계획, 20-view plan review, parent plan/상태대장/public roadmap 변경, CSS fixture, 독립 expected JSON, Chrome observation/screenshot 및 이 리뷰 기록이다. 제품 구현은 포함하지 않는다.

| # | 독립 실패 관점 | 확인 결과 | 판정 |
| ---: | --- | --- | --- |
| 1 | 계획이 #752와 겹쳐 실질적인 새 경계가 없을 수 있음 | #752는 한 track만 사용하며 새 계획은 두-track span만 추가한다. | 통과 |
| 2 | 공식 구현 완료 상태가 잘못 승격될 수 있음 | `spec/STATUS.md`의 C11.4 checkbox는 그대로 미완료다. | 통과 |
| 3 | 전체 공식 완료 수가 변경될 수 있음 | 상태대장과 공개 roadmap 모두 `72/213`을 그대로 유지한다. | 통과 |
| 4 | 상위 계획이 candidate와 실행 완료를 혼동할 수 있음 | 새 항목은 명시적으로 “다음 계획”이며 계획 검토와 사전 비교만 연결한다. | 통과 |
| 5 | 향후 변경의 대상이 single-track 코드로 잘못 한정될 수 있음 | focused plan은 별도 two-track span submodule로 분리하고 기존 matrix를 보존한다. | 통과 |
| 6 | fixture의 track 개수/함수가 계획 기대값과 다를 수 있음 | CSS는 `repeat(2, minmax(auto, max-content))`이고 item placement는 `1 / span 2`다. | 통과 |
| 7 | auto placement 때문에 item의 span이 우연히 달라질 수 있음 | explicit column lines와 고정 row를 사용한다. | 통과 |
| 8 | 두 `auto` minimum이 아닌 fixed/intrinsic min track이 섞일 수 있음 | 두 column 정의가 모두 `minmax(auto,max-content)`로 고정된다. | 통과 |
| 9 | flexible track 때문에 §6.6 multi-span 조건이 달라질 수 있음 | `1fr`이 없으며 계획은 `spans_flexible_max_track=false`를 요구한다. | 통과 |
| 10 | definite free-space 단계가 계획에서 빠질 수 있음 | root width 80px과 `[40,40]px` scrollable maximize 결과를 별도 설명한다. | 통과 |
| 11 | 100px contribution이 container 80px과 섞일 수 있음 | root frame, item frame, scrollWidth, track sizes가 독립 필드로 분리되어 있다. | 통과 |
| 12 | `visible`/`clip` class가 잘못 분류될 수 있음 | CSS Grid §6.6과 CSS Overflow §3.1을 링크하고 `[50,50]px`와 100px minimum을 기대한다. | 통과 |
| 13 | `hidden`/`auto`/`scroll` class 중 하나가 빠질 수 있음 | 다섯 distinct case IDs가 있고 세 scrollable values 각각의 0px source/value를 요구한다. | 통과 |
| 14 | `overflow-x`/`overflow-y` coupling이 evidence에서 숨을 수 있음 | HTML fixture의 computed Y를 저장하고 JSON에는 visible 또는 auto를 분리 기록한다. | 통과 |
| 15 | font fallback이 수치 재현성을 깨뜨릴 수 있음 | 저장소 Ahem fixture URL과 font readiness check를 모두 관측한다. | 통과 |
| 16 | text wrapping이 min-content 100px을 바꿀 수 있음 | Ahem 고정 advance, 10 `X`, no explicit break를 계획과 fixture에 고정한다. | 통과 |
| 17 | CSS fixture가 비교와 다른 implicit browser margin을 가질 수 있음 | `html, body` margin/padding 0, Grid box sizing, gap, border, padding을 명시한다. | 통과 |
| 18 | outline이 Grid sizing geometry에 섞일 수 있음 | Grid outline은 paint-only이고 browser width/track observation은 80px root를 확인한다. | 통과 |
| 19 | Chrome expected values가 관측 후 역산됐을 수 있음 | normative `expected.json`과 browser `observation.json`이 분리되고 README가 독립 계산 순서를 명시한다. | 통과 |
| 20 | browser output이 Chrome 156 이외 버전으로 오해될 수 있음 | JSON은 HeadlessChrome 156 user agent, viewport, DPR, font readiness를 보존한다. | 통과 |
| 21 | screenshot이 Spinon output으로 오인될 수 있음 | 계획, README, 링크 제목 모두 Chrome baseline이라고 밝힌다. | 통과 |
| 22 | viewport screenshot이 case/차이를 식별하지 못할 수 있음 | 1280×633 screenshot에는 5 label, 80px root outline, visible overflow 차이가 보이며 JSON에 수치가 있다. | 통과 |
| 23 | screenshot 이후 observation JSON이 stale할 수 있음 | screenshot view에서만 `pre`를 숨겼고 fixture, observation JSON, screenshot의 SHA-256을 README 표에 고정한다. | 통과 |
| 24 | aggregate width만 맞고 track 분배가 틀릴 수 있음 | expected JSON은 per-track `[50,50]`/`[40,40]` 배열이고 계획은 각 track base assert를 요구한다. | 통과 |
| 25 | minimum trace source가 맞지 않아도 frame만 통과할 수 있음 | planned test는 `Minimum` source/value와 `MaxContent` contribution을 각각 검사한다. | 통과 |
| 26 | span flag가 regression을 숨길 수 있음 | `spans_multiple_tracks=true`, `spans_auto_min_track=true` 등 정확한 content-based trace flags를 요구한다. | 통과 |
| 27 | clip/scroll paint 또는 사용자 조작 결과를 과장할 수 있음 | README와 plan scope에서 clipping pixels, scroll offsets, scrollbars, paint를 제외한다. | 통과 |
| 28 | app/simulator 결과가 없는 상태를 생략할 수 있음 | 이 PR은 plan/evidence-only이며 app route가 없어 simulator gate가 아니라고 명시한다. | 통과 |
| 29 | private evidence가 PR에서 재현되지 않을 수 있음 | fixture, observation, expected JSON, screenshot, source hashes를 PR 변경 파일로 포함한다. | 통과 |
| 30 | PR 본문에서 visual evidence 누락 또는 잘못된 이미지 host가 있을 수 있음 | 계획 PR 본문에 이 저장소의 raw screenshot을 Chrome-only label과 함께 임베드할 예정이다. | 통과 |

검토 후 blocker는 없다. staged diff의 `git diff --check`가 통과했고, expected JSON과 Chrome observation은 다섯 case 모두 track, frame, root size에서 정확히 일치한다. 구현 테스트 결과로 해석하지 않는다.
