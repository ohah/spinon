# C15.2 기준 계획 PR 변경 검토 30개

**검토 대상:** C15.2 oracle·fixture·계획/상태/공개 roadmap 변경 diff<br>
**검토 시점:** PR 생성 전 · 2026-10-17<br>
**판정:** P1/P2 결함 없음. C15/C16 완료 상태·제품 구현·플랫폼 실행 근거는 변경하지 않았다.

| # | 별도 diff 실패 관점 | 변경 검토와 판정 |
| ---: | --- | --- |
| 1 | C15.2 observation 링크가 기존 C15.1 비교 링크를 대체 | plan 상단에 두 문서를 따로 남겼다. C15.1 original fixture 안내도 유지된다. 통과. |
| 2 | 새 acceptance table이 C15.2 외부 위치에 붙어 다른 단계로 읽힘 | `nested inline formatting` C15.2 섹션 안에 table과 implementation boundary를 추가했다. 통과. |
| 3 | `nested-short` 입력의 style 합계와 기대 rect 간 모순 | 고정 HTML의 `border/padding` 실제 값과 `[0,-3,36,16]`, `[3,-5,30,20]`, `[8,0,20,10]`를 observation JSON에서 대조했다. 통과. |
| 4 | `nested-wrap` text line 순서와 expected y 값 역전 | JSON은 text y=0/10, outer/inner fragment y=-2/8이다. plan row의 순서가 동일하다. 통과. |
| 5 | paragraph box와 inline box union을 같은 크기로 읽도록 변경 | plan과 README는 paragraph `30×20`과 line별 fragment array를 구분한다. 통과. |
| 6 | fragment negative y가 검증 전 zero clamp되어 실측 조건이 사라짐 | plan은 음수 relative y를 그대로 명시하고 union rect 대체를 금한다. 통과. |
| 7 | inline-start/end border/padding가 continuation line에도 중복 | nested-wrap 첫/마지막 edge rect를 명시하고 해당 edge contribution만 해당 line fragment에 반영한다. 통과. |
| 8 | fragment height가 line box height로 전파 | short span과 wrapped span의 DOM fragment height와 paragraph line-height를 별도 표기했다. 통과. |
| 9 | `ahem-baseline` marker offset이 기존 C15.1 baseline과 상충 | 두 값 모두 top+8이며 original baseline fixture를 수정하지 않았다. 통과. |
| 10 | Roboto metrics로 line-height를 계산할 때 ascent+descent 값이 24와 불일치 | recorded font bounding metrics는 19+5=24, explicit CSS line-height도 24다. 통과. |
| 11 | Roboto actual glyph bounds가 font bounding metrics와 합산 | table은 둘을 각각 named field로 보존하고 line box와 ink bounds를 혼동하지 않는다. 통과. |
| 12 | Canvas advance가 DOM Range advance와 바뀌어 테스트가 불안정 | 양 값과 의미 차이를 plan/README에 분리했다. 수용 기준은 DOM range advance와 기존 0.05px tolerance다. 통과. |
| 13 | `mixed-baseline`이 C15.2a 구현 범위를 몰래 넓힘 | 해당 case는 C15.2b로 명시하고 C15.2a required list에서 제외했다. 통과. |
| 14 | anonymous block topology가 C15.2a에 포함되어 제품 코드 범위가 불명확 | `anonymous-flow`는 oracle로 고정하되 C15.2b에 연결하고 그전까지 diagnostic을 요구한다. 통과. |
| 15 | one direct block child result를 일반 anonymous block algorithm 보장으로 표현 | README·plan의 범위는 단일 fixed topology라고 한정한다. 통과. |
| 16 | C15.2a의 nesting depth limit와 original one-span C15.1 사이 모순 | C15.2a는 최대 두 단계 span, C15.1은 한 단계 plain span이며 기존 요구가 보존된다. 통과. |
| 17 | plan이 support-outside input을 default style로 받아들이게 함 | `auto`/0/default 강등을 금하고 node/stage diagnostic을 요구한다. 통과. |
| 18 | available width가 달라진 후 stale fragment 재사용 | revision/width gate를 amendment에 다시 적고 existing snapshot rule을 유지한다. 통과. |
| 19 | C15.2 `border 1px` fixture와 기존 C15.1 `plain-inline-fragment` fixture가 혼동 | 새 `inline-text-c15-2-precomparison.html`과 evidence directory를 만들고 기존 파일은 수정하지 않았다. 통과. |
| 20 | source fixture가 번들 font 경로에서 404 | HTML 상대 URL `../../resources/c15/...`가 `tests/fixtures/resources/c15`를 가리킨다. 두 font는 Agent Browser에서 실제 load true였다. 통과. |
| 21 | browser screenshot이 observation data와 다른 page content | observation JSON은 screenshot 전에 저장됐고 screenshot은 같은 fixture의 visible case를 담는다. body CSS zoom 1.75는 metadata/README에 visual-only 조정으로 기록했다. 통과. |
| 22 | browser screenshot과 font/font-family label이 보이지 않아 시각 자료가 무의미 | screenshot은 Ahem box sample, wrapped border, Roboto ascender/descender 및 baseline markers를 label과 함께 보여준다. 통과. |
| 23 | browser metadata에 ephemeral CDP websocket URL을 저장해 live session을 노출 | metadata는 Browser.getVersion 필드만 저장하고 WebSocket URL은 제외했다. 통과. |
| 24 | agent-browser/Chrome 또는 local fixture server가 계속 남음 | 저장 후 `agent-browser session list`가 비었고 process 목록에 agent-browser/Chrome이 없었다. Python server도 Ctrl-C 종료했다. 통과. |
| 25 | screenshot or metadata SHA가 후속 편집과 어긋남 | HTML, observation, screenshot, font files의 SHA-256을 캡처 후 계산했다. HTML은 캡처 뒤 수정하지 않았으며 JSON/screenshot hash가 metadata와 일치한다. 통과. |
| 26 | fixture observation page가 product/runtime output처럼 표시 | evidence README·plan·roadmap은 이를 Chrome DOM oracle로만 정의하고 Spinon runtime/paint/platform 결과를 명시적으로 부정한다. 통과. |
| 27 | roadmap을 업데이트하며 C15 완료 수나 global count가 증가 | 공개 note는 신규 Chrome reference만 추가하고 C15/C16 미완료·runtime 미검증을 적는다. STATUS checkbox와 공식 수치는 바꾸지 않았다. 통과. |
| 28 | STATUS에서 계획/fixture와 새 review evidence가 분리 | C15 status row는 기존 체크를 유지하고 C15.2 oracle, amendment review 20개, PR diff review 30개를 연결한다. 통과. |
| 29 | C15 plan 변경과 맞지 않는 공개 roadmap 설명 | roadmap note는 같은 C15.2 Chrome reference와 미완료 경계를 참조하고 plan, evidence, 두 review artifacts를 직접 연결한다. 통과. |
| 30 | 변경 문서에서 팀 언어, artifact 경로, local Markdown 링크 규칙 위반 | plan/evidence/STATUS/roadmap 문장은 한국어이고 identifier·CSS value는 원어를 유지했다. 새 screenshot·JSON은 evidence folder 안에 있다. 통과. |

## 남은 경계

- 이 diff에는 C15.2 implementation, Simulator, renderer, glyph paint, public CSS support 변경이 없다.
- C15.2a implementation은 이 계획 PR 병합 후 시작한다. C15.2b, C15.3 및 C15/C16 상위 완료는 별도 gates다.
