# 계획 PR diff 실패 관점 검토 30개

대상 diff: `plan/c11-4-anonymous-grid-text-transform-full-width.md`, 20관점 계획 검토, 상위 C11.4 계획 항목, 고정 HTML fixture와 precomparison JSON/README/PNG. 계획 PR을 병합하기 전에 지원 범위, 측정 의미, 문서 연결과 이후 구현 게이트를 독립 관점으로 재확인했다.

| # | diff 관점 | 검토 결과 |
|---:|---|---|
| 1 | 제목이 CSS Grid parent와 C11.4 slice를 식별하는가 | C11.4 anonymous Grid text로 범위를 표시한다. |
| 2 | 현재 문서가 구현 완료라고 말하는가 | 상태를 계획 후보로 명시한다. |
| 3 | at-risk 표기가 누락됐는가 | CSS Text CRD at-risk 상태와 후속 재판정 조건을 적었다. |
| 4 | Chrome이 unsupported value를 지원하는 듯 표시되는가 | computed `none` 관측과 Chrome 미지원 판정을 명시한다. |
| 5 | Chrome target 폭이 변환 expected로 혼용되는가 | normative mapping과 literal candidate measurement를 분리했다. |
| 6 | CSS Text anchor가 명시적인가 | §2.1.1 Mapping Rules 링크를 고정 CRD URL에 연결했다. |
| 7 | UAX #11이 mapping 기준에 연결되는가 | wide/narrow mapping source 링크를 포함한다. |
| 8 | Unicode mapping 방향이 모호한가 | narrow ASCII source에서 `<wide>` output으로 읽도록 기술했다. |
| 9 | fixed source가 unsupported 문자까지 포함하는가 | A, B, 1, 2와 U+0020 외 입력을 거부하도록 제한했다. |
| 10 | expected output의 code point가 source와 일치하는가 | `ＡＢ　１２`에 대응하는 고정 5개 mapping을 기록했다. |
| 11 | U+0020 handling이 CSS white-space와 맞는가 | Phase I collapse 뒤 단일 U+3000 mapping을 고정했다. |
| 12 | candidate normal-space와 ideographic-space 개수가 뒤섞이는가 | 50px 단일 공간과 70px 세 공간의 구별을 기록했다. |
| 13 | max-content candidate가 실제로 max intrinsic width를 재는가 | inline candidate width 50px와 source control을 함께 기록했다. |
| 14 | min-content candidate가 transformed word width를 재는가 | min-content Grid candidate 20px와 두 줄 높이를 기록했다. |
| 15 | CSSOM width quantization/허용오차가 불분명한가 | fixed browser measurements와 0.03125px tolerance를 적었다. |
| 16 | Noto Sans KR font load가 실패해 fallback될 수 있는가 | loaded/status, family, weight, source path와 hash를 보존했다. |
| 17 | fixture asset URL이 파일 경로와 맞는가 | HTML의 상대 URL은 `tests/fixtures/resources/...`를 가리킨다. |
| 18 | DPR와 viewport가 빠져 측정 재현성이 없는가 | Agent Browser user agent, DPR 1, 1280×633을 고정했다. |
| 19 | target style serialization이 HTML declaration과 어긋나는가 | `declaredTextTransform`과 computed value를 별도 field로 기록했다. |
| 20 | screenshot이 변환 성공을 시사하는가 | README와 plan에서 screenshot은 unsupported CSS 선언 화면이라고 제한한다. |
| 21 | direct language owner가 ambiguous한가 | runtime plan은 exact Grid owner의 direct `lang=ja`를 요구한다. |
| 22 | inherited language를 제품 지원으로 끌어오는가 | missing/inherited/다른 언어를 fail-closed 범위로 분리했다. |
| 23 | source revision stale data를 받아들일 수 있는가 | HostDocument/style/render revision 일치를 gate로 둔다. |
| 24 | scene ID만으로 style 값을 추정하는가 | computed serialization과 typed mode를 같이 검증하도록 명시한다. |
| 25 | source map이 UTF-8 byte 변화에 무너질 수 있는가 | output scalar와 원래 UTF-16 range의 대응을 확인한다. |
| 26 | transform이 line break 이후에 적용될 수 있는가 | normalization 뒤, line breaking·intrinsic sizing 전에 적용한다. |
| 27 | default CSS author support가 추가된 것처럼 보이는가 | opt-in fixture이며 일반 CSS admission은 제외한다. |
| 28 | default archive exclusion이 빠지는가 | default marker absence와 opt-in positive control을 요구한다. |
| 29 | simulator 근거를 실기기나 glyph pixel로 확대할 수 있는가 | AVD/Simulator와 screenshot 한계를 분리한다. |
| 30 | 상태 대장 count나 C11 completion이 과장되는가 | C11/C11.4 미완료와 `72/213` 유지 조건을 반복해 명시한다. |

이 계획 diff 검토는 구현/runtime 실패 관점 20개를 대신하지 않는다. 구현이 승인·진행되면 코드와 실제 runtime을 대상으로 별도 검토한다.
