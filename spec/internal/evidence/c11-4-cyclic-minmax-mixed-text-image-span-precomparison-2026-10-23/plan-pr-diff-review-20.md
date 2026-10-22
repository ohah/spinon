# 계획 PR diff 실패 관점 검토 20개

이 검토는 새 계획 PR의 실제 파일 변경을 대상으로 한다. 계획 내용 자체를 검토한 [20개 표](../../../../plan/c11-4-cyclic-minmax-mixed-text-image-span-plan-review-20.md)와 별도다.

| # | 변경분 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | 상위 C11.4를 완료 체크하거나 `77/213`을 올리는 변경이 섞였는가 | 부모 계획에 후보 설명만 추가했고 공식 체크/집계는 수정하지 않았다. |
| 2 | 계획 PR이 런타임 구현 완료로 서술되었는가 | 계획 header와 evidence README 모두 runtime 미실시를 명시한다. |
| 3 | 새 계획이 기존 plan 이름과 충돌하거나 선행 작업을 대체하는가 | text/image, cyclic text, cyclic image 선행 계획을 각각 링크하고 별도 조합임을 설명한다. |
| 4 | relative link가 추가된 HTML oracle을 찾지 못하는가 | plan의 `../tests/fixtures/...`를 실제 저장소 경로와 대조했다. |
| 5 | evidence의 `../../../../` 상대 링크가 저장소 루트를 지나치거나 덜 올라가는가 | fixture 및 plan 링크가 올바른 대상에 해석된다. |
| 6 | source hash가 최종 staged HTML과 다르게 남았는가 | staged HTML SHA-256과 JSON/README의 `e54b658f...`를 다시 계산해 확인했다. |
| 7 | screenshot hash가 다른 scene revision을 가리키는가 | PNG SHA-256 `7573ceea...`가 evidence README와 일치한다. |
| 8 | observation JSON이 CLI가 출력한 문자열로 이중 인코딩되었는가 | staged 파일을 JSON parser로 읽고 최상위 배열·scene 4개를 확인했다. |
| 9 | 기대값 JSON의 rational 값이 JSON number로 잘못 표기됐는가 | 분수는 `"37/3"` 등 문자열, 정수·quantized CSSOM 값은 number로 구분한다. |
| 10 | observation 값이 independent expectation 파일에 복사되어 출처가 섞였는가 | 규범 유도값과 Chrome observation은 별도 JSON 및 별도 README 절로 유지한다. |
| 11 | image PNG 또는 Ahem font revision이 미기록인가 | 두 asset bytes와 natural dimensions/font load를 해시·environment 파일에 기록한다. |
| 12 | image CSS size가 자연 크기와 혼동되는가 | 24×16 natural bytes, 40px width/min-width, auto height와 3:2 ratio를 분리 표기한다. |
| 13 | source-order control이 style이나 item identity까지 바꾸는가 | forward/reverse 입력은 세 keyed item의 DOM 순서만 바꾼다. |
| 14 | root/item frame과 line rectangles의 coordinate space가 혼합되는가 | item frame과 line rectangle은 root-relative임을 observation code에서 확인했다. |
| 15 | definite border overflow가 해당 PR에서 이미 pixel 확인됐다고 과장되는가 | 계획은 추후 PixelCopy/Metal 표면 검사를 요구하고 계획 PR은 Chrome observation만 주장한다. |
| 16 | simulator evidence를 실기기/hardware GPU로 확대하는가 | Android AVD 및 iOS Simulator만 gate로 적고 physical device를 제외했다. |
| 17 | CSS border stroke나 iOS compositor-after pixel이 검증된 것처럼 링크되는가 | 둘 다 명시적으로 미검증 범위다. |
| 18 | fixture source를 제품/default artifact에 넣으라고 해석되는가 | opt-in fixture-test feature 및 default artifact exclusion을 요구한다. |
| 19 | 계획 PR 변경만으로 공개 CSS 지원 상태나 internal contract version을 바꾸는가 | 공개 API/계약 버전/status promotion 변경이 없다. |
| 20 | 계획 review와 계획 PR diff review가 같은 횟수로 뭉쳐 계산되는가 | 별도 20행의 독립 문서 두 개를 plan과 diff에 각각 연결한다. |

## 결론

Diff check와 source/hash/JSON 대조가 통과했다. 계획 PR diff에서 막아야 할 추가 결함은 찾지 못했다. 구현 PR에서는 이 표를 재사용하지 않고 새 코드·runtime 실패 관점 20개와 PR diff 20개를 별도로 작성한다.
