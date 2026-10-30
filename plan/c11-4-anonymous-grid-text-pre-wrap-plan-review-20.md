# C11.4 anonymous Grid text `white-space:pre-wrap` 계획 실패 관점 검토 20개

대상: [`c11-4-anonymous-grid-text-pre-wrap.md`](./c11-4-anonymous-grid-text-pre-wrap.md)와 연결된 HeadlessChrome 156 사전 비교 자료. 각 항목은 별도 계약 실패 경로를 검토한다.

| # | 실패 관점 | 판정과 계획의 대응 |
| ---: | --- | --- |
| 1 | `normal`의 공백 접기 회귀가 pre-wrap 개선으로 잘못 처리될 수 있음 | 통과 — 같은 `XX  XX` 입력의 normal max-content 50px control을 유지한다. |
| 2 | PreWrap max-content에서 두 연속 U+0020 중 하나가 누락될 수 있음 | 통과 — 독립 합 20+20+20=60px과 Chrome 60px을 비교한다. |
| 3 | source 앞 공백이 일반 normal처럼 제거될 수 있음 | 통과 — text-engine unit test에 leading spaces 보존을 요구한다. |
| 4 | source 끝 공백이 normalized paragraph에서 제거되거나 intrinsic width를 잘못 늘릴 수 있음 | 통과 — trailing spaces의 normalized content와 max-content/hanging 동작을 검증 항목에 넣었다. |
| 5 | 탭이 ASCII space로 접혀 tab stop 의미를 잃을 수 있음 | 통과 — tab 보존 unit case를 독립적으로 추가한다. |
| 6 | LF가 ordinary collapsible space로 처리될 수 있음 | 통과 — forced-break scene에서 20px/20px 결과를 고정한다. |
| 7 | CR/CRLF/form-feed break가 누락되거나 두 CRLF code unit의 source provenance가 끊길 수 있음 | 통과 — segment-break 정규화는 하나의 LF output에 각 원본 code unit mapping을 보존한다. |
| 8 | comment가 나눈 두 DOM Text node가 두 Grid item이 되거나 공백 provenance를 합쳐 잃을 수 있음 | 통과 — split source pre-wrap scene은 하나의 60px anonymous item과 각 원본 node mapping을 요구한다. |
| 9 | line-end preserved space가 min-content track을 늘릴 수 있음 | 통과 — `pre-wrap-min-content`는 20px track을 유지해야 한다. |
| 10 | 25px soft-wrap가 공백 포함 30px line으로 잘못 계산될 수 있음 | 통과 — root/track 25px, 두 줄, hanging separator fragment를 독립 기대값과 대조한다. |
| 11 | hanging Range rect를 glyph paint 또는 visible surface pixel로 과대해석할 수 있음 | 통과 — Range fragment와 glyph/pixel 증거를 문서에서 분리한다. |
| 12 | 공백-only sequence가 rendered anonymous item이 되어 auto-placement cell을 소비할 수 있음 | 통과 — CSS Grid §6 negative control에서 marker가 첫 cell에 있어야 한다. |
| 13 | empty text paragraph가 zero-sized Grid item으로 변환될 수 있음 | 통과 — whitespace-only content가 paragraph/item을 만들지 않는다고 요구한다. |
| 14 | CSS Text `break-spaces` 동작이 PreWrap에 섞일 수 있음 | 통과 — 값은 지원하지 않고 끝 공백 hanging을 pre-wrap 규칙으로 고정한다. |
| 15 | 기존 PreLine의 segment-break 보존이 변할 수 있음 | 통과 — 기존 9-scene matrix와 PreLine runtime scene을 regression control로 포함한다. |
| 16 | NoWrap 또는 Normal에서 새 preserved behavior가 켜질 수 있음 | 통과 — 기존 scenes와 정상 mode 별도 unit tests를 유지한다. |
| 17 | Ahem fast path가 whitespace normalization을 건너뛰고 잘못된 크기를 반환할 수 있음 | 통과 — fast path에서 PreWrap을 명시 제외하고 text engine의 일반 path를 검증한다. |
| 18 | opt-in fixture allowlist 확장으로 일반 CSS/root HTML admission이 넓어질 수 있음 | 통과 — `RuntimeBlockFlexGridTrackSizingV1` 전용 allowlist만 바꾸고 root HTML adapter 제외를 명시했다. |
| 19 | FFI fixture marker/payload가 product artifact에 들어갈 수 있음 | 통과 — default/fixture-test artifact 경계와 positive control을 검증한다. |
| 20 | 계획 결과를 C11.4 또는 C11 전체 완료나 `72/213` 증가로 계산할 수 있음 | 통과 — 체크박스·공식 집계를 유지하고 WPT/general CSS 범위를 제외한다. |

계획 검토는 이 문서의 범위·oracle·구현 계약을 평가한다. 구현 뒤 runtime 검토 20개와 구현 PR diff 30개는 새 항목으로 별도 작성한다.
