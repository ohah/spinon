# C11.4 intrinsic explicit track + implicit text row 계획 PR 변경 검토 30개

계획 PR 실제 변경(계획, 사전 비교 HTML, independent expected, Chrome observation, screenshot/hash, plan review)을 다음 별개의 실패 경로로 검토했다.

| # | 적대 관점 | 실제 diff 확인 결과 |
| ---: | --- | --- |
| 1 | 계획이 기존 standalone gate를 잘못 설명 | `grid_values.rs`의 fixed-only implicit-row gate와 intrinsic-track `auto_rows` 거부 조건을 구체적으로 지목한다. |
| 2 | 새 feature가 별도 product profile로 숨음 | 기존 `RuntimeRootHtmlTextPaintV1`와 `RootHtmlIntrinsicTrackSizingV1`의 결합으로 서술하고 새 profile/FFI를 금지한다. |
| 3 | 열 sizing과 행 sizing의 순서 역전 | min-content 6.03125px column이 6행 text를 만드는 장면을 expected와 review에 고정했다. |
| 4 | Max-content overflow가 test viewport에서 사라짐 | 66.234375px used column과 root 40px 대비 넘침을 fixture/expected에 보존했다. |
| 5 | Auto track positive control 누락 | `auto-track` scene에서 40px column·20px text row를 고정했다. |
| 6 | Flexible maximum을 auto track과 동일하게 검증 | 별도 `minmax-flex` scene과 track expectation을 기록했다. |
| 7 | Row-only baseline 없어 새 열 영향 분리 불가 | `rows-control`의 fixed 40px column을 유지한다. |
| 8 | browser의 font readiness 미확인 | JSON은 `document.fonts.check('10px monospace')` 성공과 `fontCheck`를 기록한다. |
| 9 | font metrics를 grid result에서 순환 추정 | 별도 `width:min-content`/`width:max-content` probe가 grid scene과 분리되어 있다. |
| 10 | screenshot과 observation JSON이 서로 다른 실행 | screenshot과 JSON은 같은 fixture/browser session에서 수집했고 해시를 연결했다. |
| 11 | screenshot이 DPR 또는 source hash를 잃음 | fixture SHA, screenshot SHA, DPR 1, PNG dimensions를 보존한다. |
| 12 | 파일 URI가 다른 fixture를 가리킴 | observation의 file URL과 README/expected fixture path가 동일하다. |
| 13 | 화면에 보이는 label이 sizing scene을 바꿈 | Grid는 각 label의 다음 block이며 computed frame을 root-local로 정규화해 기록한다. |
| 14 | Range rectangle 높이를 line-height로 사용 | Range 값은 line count로만 observation에 두고 sizing expected는 frame/track 크기로 둔다. |
| 15 | CSSOM track string rounding을 exact geometry로 사용 | max-content computed string과 fractional used frame `66.234375`를 분리한다. |
| 16 | gap 5% 사용값을 property serialization만 보고 판단 | `row-gap:5%` computed 값과 observed 2px item offset을 별도 기록한다. |
| 17 | gap을 track base에 합산 | Expected는 row tracks와 `usedRowGaps`를 별도 배열로 기록한다. |
| 18 | positive gap의 percentage basis가 indefinite | Fixture root width/height는 모두 definite 40px다. |
| 19 | direct text가 multiple nodes 또는 inline descendants로 바뀜 | 모든 positive scene은 같은 direct `<div>`와 단일 text child를 사용한다. |
| 20 | empty sibling이 intrinsic contribution을 만들어 oracle 오염 | 빈 item width를 0으로 고정하고 text가 기여하는 column을 분리했다. |
| 21 | target text style이 controls 사이에 달라짐 | 모든 장면이 같은 10px monospace, 10px line-height, `white-space:normal`을 공유한다. |
| 22 | Explicit row와 implicit rows 구분이 사라짐 | Fixture/expected는 explicit 10px row와 이후 `[60,34]` 등의 implicit rows를 명시한다. |
| 23 | auto-placement order가 각 장면에서 달라짐 | 세 형제의 source order와 grid-local y positions를 고정한다. |
| 24 | W3C wording을 Chrome screenshot만으로 뒷받침 | Independent expected를 §7.6/§11.1/§11.5 rule과 intrinsic probe로 분리했다. |
| 25 | 모든 existing intrinsic grammar를 새로 허용한다고 과장 | Plan은 기존 explicit-track admission만 재사용하며 track grammar expansion을 금지한다. |
| 26 | `fit-content`, area, line, repeat이 우회로로 열림 | 해당 값들을 제외 범위와 구현 negative controls에 각각 적었다. |
| 27 | 기존 aspect-ratio candidate mode가 먼저/나중 선택됨 | 별도 review 항목으로 mode priority와 ratio controls를 확인하도록 요구한다. |
| 28 | Taffy trace가 paint/output proof로 과장 | Plan은 track trace, frames, `layout=ready`, draw/present와 screenshot을 각각 구분한다. |
| 29 | Simulator evidence가 physical hardware 또는 post-compositor로 과장 | PixelCopy/Metal readback은 실제 수행 때만 기록하고 AVD/iOS limits를 plan에 명시한다. |
| 30 | bounded plan이 official completion을 자동 증가 | C11/C11.4 checkbox와 `78/214` 유지 조건을 계획과 expected scope에 고정한다. |

발견한 계획 blocking 결함은 없다. 구현 전 검토 20개는 이 계획 diff 검토와 별개의 artifact로 유지한다.
