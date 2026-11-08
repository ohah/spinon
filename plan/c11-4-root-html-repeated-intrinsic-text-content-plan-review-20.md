# C11.4 repeated intrinsic text content plan · adversarial review 20

대상: [계획](./c11-4-root-html-repeated-intrinsic-text-content.md), [Chrome 사전 비교](../spec/internal/evidence/c11-4-root-html-repeated-intrinsic-text-content-precomparison-2026-11-09/README.md), fixture 및 810-scene expected JSON.

각 항목은 서로 다른 거부·회귀·과장 실패 경로를 확인했다. 계획은 검토 결과를 반영했으며 제품 구현은 시작하지 않았다.

| # | 실패 관점 | 계획에서 고정한 방어/검증 | 결과 |
|---:|---|---|---|
| 1 | UTF-8 byte 수와 Unicode scalar 수를 혼동 | ASCII-only 문법이라 byte·scalar 길이가 같다. 32-byte 상한을 validator와 fixture에서 검사한다. | 통과 |
| 2 | 길이 상한의 off-by-one | 32자 문자열을 Chrome corpus에 포함하고 33자는 negative case다. | 통과 |
| 3 | 긴 단어의 min-content width가 column보다 커짐 | 각 단어를 6 ASCII자로 제한하고 40px column 기준을 고정한다. | 통과 |
| 4 | 공백 정규화가 구현체마다 바뀜 | 허용 문법은 공백 한 칸만 허용하고 연속·선행·후행 공백은 거부한다. | 통과 |
| 5 | 빈 문자열/공백만 있는 paragraph가 정상 텍스트로 admission | empty text와 whitespace-only input을 음성 검사한다. | 통과 |
| 6 | 소문자·숫자·기호로 font fallback이 개입 | uppercase A–Z만 허용하고 다른 문자군을 거부한다. | 통과 |
| 7 | 탭·개행이 line wrapping 경로를 바꿈 | ASCII space 외 whitespace는 거부하며 tab/newline negative case를 둔다. | 통과 |
| 8 | 여러 TextSource가 단일 paragraph처럼 합쳐짐 | source 개수 1과 허용 문자열 전체를 검사하고 추가/누락 source를 거부한다. | 통과 |
| 9 | inline span/run style이 child style gate를 우회 | inline ancestor/run style 없는 기존 조건을 유지하고 span negative case를 둔다. | 통과 |
| 10 | 폰트 fallback 또는 line-height 차이가 geometry를 바꿈 | font family/size/weight/style/line-height/spacing를 exact profile로 고정한다. | 통과 |
| 11 | `white-space:nowrap`가 줄 수를 고정 문장 기대와 다르게 만듦 | `white-space:normal`만 허용하고 nowrap 거부를 검증한다. | 통과 |
| 12 | 기존 #1290 내용이 회귀 | 별도 `legacy-1290` corpus 270 scene을 새 expected JSON에 포함한다. | 통과 |
| 13 | 새 문자열도 이전 exact 목록만 통과 | varied ASCII와 경계 corpus를 제품 admission에 넣고 문자열 exact match 제거를 확인한다. | 통과 |
| 14 | 30개 tail multiset 순열 중 누락 | 각 corpus에 30 order 전부, expected JSON의 고유 order 수 30을 고정한다. | 통과 |
| 15 | cap의 단위/경계 조건 누락 | `20px`, `20%`, `0%`, `120%`와 start/stretch를 전체 행렬에서 측정한다. | 통과 |
| 16 | `stretch`가 비-auto track까지 늘어남 | CSS used rows와 모든 track order를 exact 대조하고 auto 분배 경계를 확인한다. | 통과 |
| 17 | 바깥 overflow를 row geometry로만 판단 | 100px cohort에서 payload frame bottom을 비교하고 30/30 장면에 overflow가 있는지 검사한다. | 통과 |
| 18 | Range rect y 좌표가 text line 수를 잘못 셈 | capture script는 Range client rect의 고유 y 수를 저장하고 text descriptor를 함께 남긴다. | 통과 |
| 19 | expected JSON이 다른 fixture revision에 묶임 | fixture SHA-256을 expected JSON과 README에 기록한다. | 통과 |
| 20 | Chrome 관측을 Android/iOS font parity나 일반 CSS 지원으로 과장 | oracle와 mobile smoke를 분리하고 grammar·topology·style·플랫폼 한계를 명시한다. | 통과 |

## 판정

계획의 경계와 oracle는 구현을 시작할 만큼 구체적이다. Chrome 관측은 독립 기대값일 뿐 제품 통과 결과가 아니며, 다음 단계에서는 같은 값의 Rust RuntimeSession matrix 및 Android/iOS 실행을 별도로 확인해야 한다.
