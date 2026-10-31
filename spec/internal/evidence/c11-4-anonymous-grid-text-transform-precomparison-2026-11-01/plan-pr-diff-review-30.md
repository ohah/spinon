# C11.4 computed `text-transform` 계획 PR diff 적대 검토 30개

이 검토는 계획 PR의 실제 변경 문서·fixture·oracle·roadmap delta를 대상으로 한다. 구현 리뷰는 포함하지 않는다.

| # | 적대 관점 | 확인 결과 |
|---:|---|---|
| 1 | 계획 후보가 구현 완료처럼 분류될 수 있음 | **PASS** — STATUS·내부 계약·로드맵 모두 plan-only와 runtime 미시작을 명시한다. |
| 2 | C11/C11.4 상위 체크가 완료로 바뀔 수 있음 | **PASS** — 상위 checkbox는 그대로 미완료다. |
| 3 | 공식 완료 집계가 잘못 증가할 수 있음 | **PASS** — `72/213`을 유지하며 계획 후보를 완료 수에 더하지 않는다. |
| 4 | 계획 문서가 내부 Grid 명세를 잘못 연결할 수 있음 | **PASS** — C11.4 plan과 internal contract 0059로 연결한다. |
| 5 | W3C 문서 버전이 불명확하거나 다른 규범을 참조할 수 있음 | **PASS** — CSS Text Level 3의 2026-08-14 CRD §2.1 직접 fragment를 둔다. |
| 6 | inherited `text-transform` 속성인데 inheritance를 생략할 수 있음 | **PASS** — 계약을 상속 속성으로 적고 exact direct owner 입력만 허용한다. |
| 7 | CSS 변환이 underlying text를 변경한다고 설명할 수 있음 | **PASS** — HostDocument 원문을 보존하고 layout-only buffer 변환으로 한정한다. |
| 8 | Unicode full case mapping 지원을 암시할 수 있음 | **PASS** — non-ASCII와 expansion을 고정 입력 밖으로 명시한다. |
| 9 | 터키어 등 언어별 casing이 ASCII 구현으로 처리될 수 있음 | **PASS** — exact direct `lang=en`만 허용하고 다른 locale을 거부한다. |
| 10 | `capitalize`의 단어 경계 규칙을 잘못 부분 지원할 수 있음 | **PASS** — `capitalize`는 지원 값에서 제외하고 명시 거부한다. |
| 11 | `full-width` 또는 `full-size-kana`를 누락할 수 있음 | **PASS** — 두 값 모두 지원 범위 밖이라고 기록한다. |
| 12 | six-scene matrix cardinality가 문서와 oracle에서 다를 수 있음 | **PASS** — 세 computed keyword × 두 track으로 JSON과 plan 모두 6개다. |
| 13 | test text가 case transform 변화를 관찰하지 못할 수 있음 | **PASS** — `IIII ii`는 upper/lower transform 후 glyph advance가 달라진다. |
| 14 | browser font fallback으로 oracle가 달라질 수 있음 | **PASS** — Noto Sans KR font load 상태와 immutable asset SHA-256을 기록한다. |
| 15 | glyph advances가 independent expectation으로 고정되지 않을 수 있음 | **PASS** — Latin `wght=400`, `lang=en`, 1000 UPEM advances와 scene sum을 별도 JSON으로 기록한다. |
| 16 | shaping language가 font metrics에 영향을 줘 반복 결과가 달라질 수 있음 | **PASS** — owner와 Chrome fixture 둘 다 direct `lang=en`이다. |
| 17 | min-content를 Range rect로 대체할 수 있음 | **PASS** — CSSOM grid track이 primary oracle이며 Range 관찰은 보조다. |
| 18 | max-content scene에 wrapping이 섞여 기대 폭이 바뀔 수 있음 | **PASS** — max-content row는 10px, min-content row는 두 line 20px로 CSSOM에 고정한다. |
| 19 | fractional font quantization 때문에 독립식과 browser 값을 혼합할 수 있음 | **PASS** — raw CSSOM과 glyph-sum 값을 분리하고 0.03125px tolerance를 명시한다. |
| 20 | scene root width/track controls가 미정일 수 있음 | **PASS** — root width 120px, one column, zero gap/edge 및 explicit min/max track이다. |
| 21 | line-height 또는 font-size 변화가 text-transform 효과와 섞일 수 있음 | **PASS** — 10px font/line-height 및 weight400을 고정한다. |
| 22 | whitespace 처리와 case transform 상호작용이 범위를 넘을 수 있음 | **PASS** — normal whitespace 및 동일 source literal만 허용하고 whitespace regression을 계획한다. |
| 23 | text source range가 변환 뒤 깨질 수 있음 | **PASS** — ASCII one-to-one 변환 및 UTF-8/UTF-16/source provenance assertions를 요구한다. |
| 24 | unsupported input이 기본값으로 조용히 수용될 수 있음 | **PASS** — scene/value/lang/owner/revision fail-closed negative control을 요구한다. |
| 25 | general CSS author admission으로 잘못 승격될 수 있음 | **PASS** — opt-in fixture gate 외 author CSS 지원을 제외한다. |
| 26 | WPT 적합성이나 glyph painting을 과장할 수 있음 | **PASS** — WPT·glyph pixels·surface parity는 범위 밖이다. |
| 27 | Noto bytes가 product/default artifact로 유출될 수 있음 | **PASS** — default/product와 fixture positive control archive 경계 확인을 plan에 넣는다. |
| 28 | Android emulator를 physical/hardware 증거로 오인할 수 있음 | **PASS** — API 37 ARM64 AVD 및 iOS Simulator로만 판정하고 제한을 기록한다. |
| 29 | roadmap HTML에서 문서 링크 또는 상태 설명이 어긋날 수 있음 | **PASS** — Rspress 2.0.22가 plan/evidence/review route를 생성했고 Agent Browser가 roadmap row를 desktop 1280px 및 mobile 390px에서 확인했다. mobile horizontal overflow는 0이며, row는 candidate·runtime 미시작·`72/213`을 표시한다. |
| 30 | 계획 review가 PR diff review와 중복 계수될 수 있음 | **PASS** — 독립 20-point plan review와 이 별도 30-point PR diff review를 분리한다. |

PR diff에서 코드 구현이나 public support claim은 추가하지 않았다. Tailnet 배포와 PR image URL 검증은 계획 PR 병합 준비 단계에서 수행하며 이 문서는 해당 실행의 완료를 주장하지 않는다.
