# C11.1 parsed Root HTML integer repeat 계획 실패 관점 검토 20개

기준 계획: [c11-root-html-grid-integer-repeat.md](./c11-root-html-grid-integer-repeat.md)

판정: 제한 범위의 계획으로 진행 가능. 구현 전 검토이며 코드 동작을 검증한 결과가 아니다.

| # | 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | repeat 횟수를 pattern 안의 track 수로 혼동해 확장 개수를 틀리는가 | 통과. 2회 × 2-track/4-track pattern을 별도 case와 expected 배열에 분리했다. |
| 2 | Rows와 Columns가 동일 경로라고 가정해 basis 축을 잘못 선택하는가 | 통과. definite width와 height 각각에 4-track matrix를 둔다. |
| 3 | percentage repeat를 fraction 또는 authored numeric percentage로 잘못 환산하는가 | 통과. 200px definite width의 각 25%를 독립적으로 50px로 계산한다. |
| 4 | repeated fraction factor를 전체 list 합산 전에 잘못 나누는가 | 통과. 1fr/2fr pattern 두 번의 합 6에 150px을 나눠 [25,50,25,50]을 고정한다. |
| 5 | px, percentage와 fr이 섞인 반복에서 남는 공간을 두 번 빼거나 빠뜨리는가 | 통과. 20px + 40px + 40px 뒤 100px을 두 fr에 나누는 독립값을 고정한다. |
| 6 | repeat 수가 1일 때 track 또는 line 경계가 누락되는가 | 통과. single-count control은 [30,50]과 두 item 위치를 별도 고정한다. |
| 7 | 한 축의 여러 repeat group을 하나로 오인하거나 순서를 합치는가 | 통과. 복수 group의 사이 direct track과 source order를 유지하는 scene이 있다. |
| 8 | CSS parser가 invalid repeat를 정상 typed repeat로 정규화하는가 | 통과. Invalid repeat 뒤의 valid direct-track declaration을 둔 control에서 CSS declaration drop/fallback과 contextual runtime reject를 구분한다. |
| 9 | auto-fill/auto-fit를 fixed integer repeat 허용과 함께 실수로 열어 버리는가 | 통과. cascade 옵션에서 auto-repeat flag를 바꾸지 않고 negative runtime cases로 확인한다. |
| 10 | repeat 안의 minmax/fit-content/intrinsic sizing이 direct-size 허용을 우회하는가 | 통과. intrinsic flag를 바꾸지 않으며 Root HTML validator가 expanded tracks를 검사한다. |
| 11 | repeated line names가 빈 metadata처럼 지나가 authored line placement를 노출하는가 | 통과. 비어 있지 않은 line-name group은 기존 root validator에서 거부되어야 한다. |
| 12 | 10,001개 반복 track이 잘려 성공하거나 partial snapshot을 내는가 | 통과. 기존 10,000 cap을 재사용하고 over-cap cascade/layout error 및 output 부재를 확인한다. |
| 13 | line-name metadata가 track 개수 cap과 별도로 quota를 넘는가 | 통과. 반복 이름이 track cap 이내에서도 1 MiB를 넘는 입력을 cascade 단계에서 거부하고 quota를 유지한다. |
| 14 | track vector reservation 실패 후 stale/partial result를 재사용하는가 | 통과. allocation failure control에서 contextual failure와 후속 정상 계산 복구를 요구한다. |
| 15 | 확장된 repeated track이 원래 authored CSS 문자열을 재파싱하는가 | 통과. Stylo typed track vector만 사용하고 CSSOM serialized string은 오류 진단/관측에 한정한다. |
| 16 | DOM 순서나 NodeId가 repeated track 또는 item identity를 암묵 결정하는가 | 통과. positive controls에서 ID/semantic class 재지정과 source-order geometry를 비교한다. |
| 17 | named-area 또는 implicit-row 경로가 새 fixed-repeat 확장과 모순되는가 | 통과. 기존 named-area exact-shape guard와 implicit-row matrix를 별도 회귀 확인하고 성공 범위를 확대하지 않는다. |
| 18 | 다른 계산 revision의 track/style을 합쳐 stale layout을 publish하는가 | 통과. 동일 document/style/layout revision과 실패 후 scene 부재를 runtime gate로 유지한다. |
| 19 | Android/iOS 화면 캡처가 실제 표면 pixel equality로 과장되는가 | 통과. draw/present와 screenshot만 기록하며 readback을 하지 않으면 픽셀 근거라 주장하지 않는다. |
| 20 | 제한 profile 구현이 C11.1/C11 또는 일반 CSS 지원 완료로 승격되는가 | 통과. 공식 checkbox와 72/213은 유지하고 auto-repeat·named line·WPT 등 남은 범위를 명시한다. |

계획 검토 완료 뒤 fixture source·독립 expected 값·Chrome observation·hash를 연결했다. 별도 계획 PR이 아니므로 이 계획 표는 이후 구현 검토나 최종 PR diff 검토를 대체하지 않는다.
