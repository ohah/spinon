# `text-transform: full-width` 계획 실패 관점 검토 20개

대상은 [계획](./c11-4-anonymous-grid-text-transform-full-width.md)과 [Chrome 156 사전 비교](../spec/internal/evidence/c11-4-anonymous-grid-text-transform-full-width-precomparison-2026-11-02/README.md)다. 구현 전 계획의 규범, 입력·revision 소유권, 계산값, 런타임 경계를 서로 다른 실패 경로로 검토했다.

| # | 실패 관점 | 위험 | 계획의 차단·판정 |
|---:|---|---|---|
| 1 | at-risk 값이 CRD 다음 단계에서 제거됨 | 구현이 곧 폐기할 수 있는 계약에 의존 | at-risk를 계획·PR·상태 문서에 기록하고 전체 지원으로 승격하지 않는다. |
| 2 | Chrome에서 선언이 `none`으로 계산됨 | target 폭을 변환 동작 기준으로 오인 | CSSOM target을 unsupported control로 분류한다. |
| 3 | Unicode decomposition의 방향을 반대로 적용 | `<wide>` 호환 분해가 full-width source→narrow로 읽힐 수 있음 | narrow ASCII source에서 `<wide>` target을 조회하는 방향을 명시한다. |
| 4 | 모든 ASCII punctuation에도 단순 codepoint offset 적용 | `<wide>` 대응이 없는 문자까지 임의 변환 | source allowlist를 `A`, `B`, `1`, `2`, U+0020으로 제한한다. |
| 5 | lowercase와 uppercase가 혼합됨 | wide mapping 테스트가 잘못된 case profile을 포함 | 고정 입력을 대문자 ASCII로 제한하고 lowercase는 거부한다. |
| 6 | U+0020이 normal whitespace collapse 전에 변환됨 | 세 공백이 세 ideographic spaces로 남음 | Phase I collapse 뒤 transform 순서를 고정하고 `AB   12`를 회귀 입력으로 둔다. |
| 7 | U+3000이 U+0020처럼 추가 collapse됨 | full-width 공백 contribution을 잃음 | 변환 뒤 공백 재정규화를 하지 않으며 50px/70px candidate 차이를 둔다. |
| 8 | transform 뒤 line breaking이 수행되지 않음 | min-content track이 원문 12.65625px을 사용 | `full-width-min`의 20px·두 줄을 검증 기준으로 둔다. |
| 9 | max-content scene이 min-content track을 사용 | 50px 기대값이 20px candidate로 축소 | max/min scene owner와 template track을 각각 검증한다. |
| 10 | font fallback이 바뀜 | 20px/50px 폭이 Noto font가 아닌 fallback에서 생성 | 고정 Noto Sans KR 파일, weight, size, font-loaded 상태와 hash를 확인한다. |
| 11 | font variable axis가 달라짐 | glyph advance가 변함 | `wght=400`과 10px을 fixture style에 고정한다. |
| 12 | anonymous item이 다른 Grid owner에 귀속됨 | root의 style이 아닌 자손 style로 값이 바뀜 | exact owner `NodeId`, tree shape와 computed style snapshot을 확인한다. |
| 13 | `text-transform` scene 이름만으로 mode 결정 | 잘못된 computed style도 성공 | typed mode와 CSSOM serialization의 일치가 필요하다. |
| 14 | `lang=ja`를 상속값으로 오인 | root HTML/ancestor 의존이 계획 범위에 섞임 | direct `lang=ja`만 허용하고 missing/inherited 값을 거부한다. |
| 15 | unsupported source가 부분 변환됨 | 미지원 punctuation/non-ASCII가 일부 적용된 채 publish | 전체 source allowlist 검증을 변환·layout 전에 한다. |
| 16 | inline source range가 분할됨 | normalized UTF-8과 원문 UTF-16 mapping이 틀림 | inline runs와 ancestor-owned source를 fail-closed한다. |
| 17 | UTF-8 byte offset을 UTF-16 source range로 사용 | BMP fullwidth의 byte 길이 증가가 원문 위치를 바꿈 | 각 output scalar에 원문 UTF-16 range를 보존한다. |
| 18 | stale style snapshot을 최신 HostDocument와 결합 | 이전 `full-width` 값이 새 text에 적용됨 | document/style/render revision 동일성을 layout 전에 확인한다. |
| 19 | feature-off artifact에 fixture marker가 포함됨 | opt-in scene이 product archive 경계를 넘음 | default/archive 부재와 opt-in positive control을 별도 검증한다. |
| 20 | simulator screenshot을 glyph·Chrome parity 증거로 확대 | 실제 측정하지 않은 픽셀 또는 호환성을 주장 | screenshot은 capture 상태만 확인하고 Chrome unsupported·glyph-pixel 한계를 분리한다. |

계획 범위에서 발견된 문제는 위 제한 및 negative gates로 계획에 반영했다. 구현 뒤 동일 목록을 재사용하지 않고 실제 코드·runtime을 새 20 관점으로 검토한다.
