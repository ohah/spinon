# 기본 Root HTML Grid direct-text aspect-ratio 계획 PR diff 검토 20개

검토 대상은 계획·비교 fixture·Chrome evidence·적합성 비교 모델로 구성된 실제 PR 변경분이다. 아래 항목은 계획 자체의 20개 검토와 별도로 다시 확인했다.

| # | diff 실패 관점 | 최종 판정 |
|---:|---|---|
| 1 | C11.4 fixture-only 구현을 default app route로 잘못 설명 | 통과 — plan과 `0001`은 기존 C11.4 profile을 분리하고 기본 parsed `RuntimeRootHtmlTextPaintV1`을 계획 대상으로만 둔다. |
| 2 | 계획 단계 자료가 제품 지원 결과로 읽힘 | 통과 — conformance 문구는 oracle/plan이라고 밝히고 `STATUS.md`나 완료 체크는 수정하지 않았다. |
| 3 | CSS Grid 2025 CRD의 `start` sizing 규칙을 `normal`/stretch 규칙으로 오기 | 통과 — fixture의 start alignment를 기준으로 §6.2의 fit-content 경계를 기술했다. |
| 4 | CSS Sizing Level 4 Working Draft를 확정 표준으로 표현 | 통과 — plan과 README에서 2026 WD라고 명시하고 상호운용성 증거를 Chrome과 구분한다. |
| 5 | fixture body 아래 Grid가 감싼 section에 있어 direct Root HTML route와 불일치 | 통과 — 네 Grid가 body의 direct child이고 각 item은 Grid의 direct HTML block child다. |
| 6 | 두 auto-placed item이 서로 다른 row에 놓여 single-row 기대와 불일치 | 통과 — 두 explicit columns와 `grid-auto-flow:row`에서 한 auto row `[10,60]`에 놓인다. |
| 7 | explicit columns 합계와 Grid width가 달라 available width 산술이 흔들림 | 통과 — `10px + 60px = 70px`, gap은 0이다. |
| 8 | width ratio를 height/width 반대로 계산 | 통과 — 60px width와 2:1에서 30px height를 기대하고 Chrome 관찰도 같다. |
| 9 | ratio 없는 width control이 auto-row 30px를 상속 | 통과 — computed ratio는 `auto`; Ahem line-height 10px라 auto row는 10px다. |
| 10 | height ratio 장면에서 width가 Grid area 전체로 stretch되어 ratio 효과를 가림 | 통과 — `justify-items:start`이며 Chrome target width는 60px다. |
| 11 | height control의 direct-text width를 ratio나 track width와 혼동 | 통과 — start-aligned no-ratio `AAAA`의 Ahem content width는 40px다. |
| 12 | default Root HTML author allowlist에 없는 `font` shorthand가 fixture에 남음 | 통과 — 최종 fixture는 `font-family`, `font-size`, `line-height` longhand만 쓴다. |
| 13 | outline, nonzero edges, shorthand background가 앱 입력 범위를 흐림 | 통과 — decoration은 제거하고 allowlist된 `background-color`만 scene 구분에 쓴다. |
| 14 | CSS background-color가 sizing 결과를 바꿈 | 통과 — 배경은 paint-only이며 frame/track 독립 기대값과 CSS Grid geometry에서 제외한다. |
| 15 | Ahem font fallback 또는 잘못된 상대 경로로 Chrome 기대값 오염 | 통과 — font check가 true이고 resource 경로/hash를 고정했다. |
| 16 | expected JSON이 raw browser 결과를 복사하거나 frame 축 일부를 생략 | 통과 — 독립 산술로 root/item `x,y,width,height`와 row/column tracks를 모두 적고 관찰을 별도 파일로 보존한다. |
| 17 | screenshot에 없는 viewport/browser context를 추측 | 통과 — Chrome observation에 UA·viewport가 있고 PNG는 `1280×633`이다. |
| 18 | screenshot을 pixel-diff나 platform 앱 결과로 과장 | 통과 — README는 화면 구성만 증명하고 pixel-diff/WPT가 아님을 명시한다. |
| 19 | fixture/evidence hash가 변경 뒤 stale하거나 다른 Ahem/font를 가리킴 | 통과 — `SHA256SUMS`에서 fixture, Ahem, expected, observation, PNG를 모두 검증했다. |
| 20 | 병합 시 공식 완료 수 72/213 또는 C11 checkbox가 바뀜 | 통과 — diff에는 `spec/STATUS.md`가 없고 계획이 제한 slice를 전체 완료에서 명시적으로 제외한다. |

수정사항: 초기 draft에서 사용했던 `font` shorthand 및 장식/edge CSS를 제거하고 default profile author-property longhand와 paint-only scene 색으로 고정했다. 변경 후 Chrome observation, screenshot, expected geometry, hash를 다시 맞췄다.
