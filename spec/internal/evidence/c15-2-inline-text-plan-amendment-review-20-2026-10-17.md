# C15.2 사전 비교·수용 기준 amendment 계획 검토 20개

**검토 대상:** [C15 계획 amendment](../../../plan/c15-inline-text-layout.md), [Chrome 사전 비교](./c15-2-inline-text-precomparison-2026-10-17/README.md), fixture와 원본 observation JSON<br>
**검토 시점:** 제품 코드 수정 전 · 2026-10-17<br>
**판정:** 구현 시작을 막는 미해결 blocker 없음. C15.2는 두 구현 slice로 나뉘고, `anonymous-flow`·`mixed-baseline`은 C15.2b 전까지 진단 거부한다. C15/C16 상태는 미완료로 둔다.

| # | 독립 실패 관점 | 적대 확인과 판정 |
| ---: | --- | --- |
| 1 | C15.2 fixture가 C15.1 oracle을 덮어써 plain span과 decorated span의 경계를 잃음 | 원본 `inline-text-precomparison.html`, C15.1 screenshot/JSON SHA를 유지하고, 새 HTML·observation·screenshot은 `c15-2-inline-text-precomparison-2026-10-17/`에 별도로 둔다. 계획이 두 fixture를 다르게 링크한다. 통과. |
| 2 | screenshot의 확대가 DOM 측정 좌표를 바꾸고 oracle에 섞임 | observation JSON은 body CSS zoom 1.0에서 저장했고 screenshot만 `body { zoom: 1.75 }` 적용 후 재촬영했다. README와 browser metadata에 순서를 적었다. 수치 판정은 JSON만 사용한다. 통과. |
| 3 | 브라우저가 Chrome/Chromium 버전만 표기해 engine revision이나 실행 파일 변화를 숨김 | `Browser.getVersion`의 Chrome 155.0.8059.85, protocol 1.3, V8, WebKit revision을 기록하고 launcher/framework byte 수와 SHA-256을 추가했다. 통과. |
| 4 | webfont 다운로드 전 fallback font로 측정 | observation의 `document.fonts.check()` 결과가 Ahem/Roboto 모두 true다. fixture와 두 font SHA도 metadata에 고정했다. 통과. |
| 5 | 상대 좌표 원점이 case 외부 element 또는 page scroll 위치에 의존 | 모든 inline rect를 해당 `.case`의 `getBoundingClientRect()` 원점에서 정규화한다. fixture case에는 border/padding이 없고 outline은 layout에 영향을 주지 않는다. 통과. |
| 6 | 짧은 nested span에서 border/padding contribution 산술이 불일치 | inner content 20px, 양쪽 inner `(3+2)px`, outer 양쪽 `(2+1)px`로 inner 30px·outer 36px가 된다. Chrome observation의 `nested-short`가 동일하다. 통과. |
| 7 | vertical border/padding을 line-height 증가로 잘못 해석 | `nested-short` fragment rect는 세로로 line box보다 크지만 paragraph line-height는 10px다. 계획은 fragment box와 line box를 별도 판정한다. 통과. |
| 8 | nested wrapped content를 하나의 line이나 element union box로 축약 | width 30px의 text range는 y 0/10에 두 개, span owner도 각 line마다 두 fragment다. 계획은 line fragment list를 요구한다. 통과. |
| 9 | 첫/마지막 line fragment의 inline edge 장식이 각 줄에 반복되거나 빠짐 | 첫 line text x=4, 다음 line x=0이며 각 nested border/padding의 시작/끝 contribution이 개별 rect에 남는다. 판정은 line별 rect를 요구한다. 통과. |
| 10 | decorated fragment 바깥 y 좌표를 clipping해 원본 CSS box를 훼손 | wrapped fragment y=-2/8, height 14와 line box y=0/10, height 10을 구분했다. negative relative y를 0으로 clamp하면 oracle과 달라진다. 통과. |
| 11 | Ahem baseline을 font-size 또는 line top으로 대체 | Ahem `10px/10px` marker가 top+8px다. 계획은 8px를 별도 marker oracle로 고정한다. 통과. |
| 12 | Roboto line box baseline을 ascent와 혼동하거나 descent를 버림 | Roboto font bounding ascent/descent 19/5가 line-height 24와 일치하고 baseline은 top+19다. 둘 다 판정 항목에 남는다. 통과. |
| 13 | Canvas `measureText` advance를 DOM range geometry의 대체 oracle로 사용 | Canvas advance 46.1914px와 Range advance 46.2031px가 다르다. README와 plan은 서로 다른 측정 의미를 유지하며 0.05px tolerance도 이를 수용한다. 통과. |
| 14 | glyph ink bounds와 font bounding metrics를 혼합 | Roboto actual glyph bounds 14.2188/4.2676과 font bounding 19/5를 별도로 기록한다. line box는 font metrics, ink geometry는 actual bounds로 분리한다. 통과. |
| 15 | 다른 font size가 섞인 baseline fixture를 C15.2a의 단일 ParagraphStyle로 가장 | `mixed-baseline`은 C15.2b 후속 fixture로 표시하고 C15.2a required cases에서는 뺐다. 구현이 아직 제공하지 않는 per-run metric을 완료 조건으로 숨기지 않는다. 통과. |
| 16 | anonymous-flow 관찰 하나를 전체 block/inline anonymous box 적합성으로 일반화 | 고정 입력은 Ahem, width 30px, direct block child 하나와 inline 앞/뒤 sequence 하나뿐이다. 계획은 그 topology 외의 일반 CSS 지원을 주장하지 않는다. 통과. |
| 17 | mixed block child를 text paragraph에서 조용히 누락하거나 0 크기 처리 | anonymous-flow 지원 전 명시 diagnostic을 반환하도록 계획했다. 조용한 성공·auto/0 fallback은 허용하지 않는다. 통과. |
| 18 | margin, non-baseline vertical-align, replaced inline 등의 unmeasured property를 장식 지원에 포함 | C15.2a 지원 밖 목록에 margin, non-baseline vertical-align, replaced inline, 깊이 3+ nesting, bidi/vertical writing을 적었다. 통과. |
| 19 | fixture의 폭/폰트 revision과 Taffy final layout의 width가 달라도 이전 결과를 재사용 | amendment는 C15 전체의 snapshot revision/available-width 규칙을 유지하고, 최종 layout width가 다른 snapshot 재사용을 금지한다. 통과. |
| 20 | oracle 추가로 roadmap의 구현 완료 수나 C15/C16 status가 증가 | `spec/STATUS.md` checkbox는 그대로 `[ ]`; 공개 roadmap note에도 Chrome observation만 추가하고 C15/C16 미완료·런타임 미검증을 적는다. 통과. |

## 추가 검토 경계

- 이 검토는 계획과 oracle amendment만 다룬다. Parley 구현, Rust unit test, Android/iOS Simulator 또는 glyph paint 통과를 주장하지 않는다.
- 계획 amendment는 별도 PR diff 30 관점 검토와 병합을 거친 뒤에만 C15.2a 제품 구현을 시작한다.
