# C11.1 parsed Root HTML `fit-content(%)` 계획 검토 20개

검토 대상은 새 후속 계획과 Chrome 156 percentage/over-border precomparison이다. 기존 `fit-content(px)` 계획 검토를 복사하지 않고 percentage basis·overflow 경계에 맞춘 별도 failure mode를 사용한다.

| # | 독립 실패 관점 | 계획상 방어·판정 |
|---:|---|---|
| 1 | `fit-content(%)`를 기존 `fit-content(px)` 계획에 이미 포함된 것으로 오인한다 | 새 후속 계획임을 표시하고 #1093 계획의 percentage 제외 문구를 연결한다. |
| 2 | Chrome 관찰값을 독립 기대값 생성에 사용한다 | Ahem probes와 §7.2.1 계산을 JSON에 먼저 고정하고 Chrome observation은 분리한다. |
| 3 | computed percentage의 fraction과 percent 숫자 단위를 혼동한다 | 입력 퍼센트와 resolved cap을 함께 기록하며 20% of 200px = 40px로 대조한다. |
| 4 | column percentage가 height basis로 resolve된다 | 200px content width의 column cap cases와 별도 100px block-size row scene을 쓴다. |
| 5 | border-box/viewport/gap 차감값으로 cap basis를 잡는다 | cap matrix는 edge/gap 0으로 고정하고 basis를 definite content-box로 명시한다. |
| 6 | indefinite 축을 viewport나 0으로 임의 resolve한다 | definite finite 해당 축만 지원하고 indefinite/cyclic case는 fail-closed로 둔다. |
| 7 | definite zero basis를 indefinite와 동일 처리한다 | 0px basis와 100% cap을 별도 positive case로 둔다. |
| 8 | >100% cap을 100%로 clamp한다 | 120% case에 resolved 240px과 110px max-content 결과를 고정한다. |
| 9 | cap이 min-content보다 낮으면 track이 cap 아래로 내려간다 | 5%/10px limit에서 20px minimum floor를 확인한다. |
| 10 | cap이 max-content보다 높아도 track이 cap만큼 커진다 | 60%와 120% cases를 110px max-content와 비교한다. |
| 11 | repeat expansion이 percentage argument 또는 seam을 바꾼다 | fixed `repeat(2, fit-content(20%))`의 두 track `[40,40]`과 item origins를 각각 확인한다. |
| 12 | authored fixed size floor가 fit-content cap에 눌린다 | 40px cap 아래 60px fixed item case를 별도로 둔다. |
| 13 | text intrinsic min/max와 fixed box runtime을 같은 oracle로 섞는다 | Ahem layout matrix와 system-font-independent Root HTML runtime fixture를 분리한다. |
| 14 | percentage fit-content를 named-area path로 확장한다 | `grid-template-areas` 조합을 명시적으로 제외하고 기존 px named-area 회귀만 유지한다. |
| 15 | explicit allowlist 변경이 `grid-auto-*`/implicit tracks까지 전파된다 | `grid-auto-rows`/`grid-auto-columns`의 fit-content는 계속 거부한다. |
| 16 | `calc()`, `minmax()`, auto-repeat, line names가 같이 열려 범위가 넓어진다 | 각 문법 variant를 별도 negative admission으로 두고 fixed integer repeat만 긍정한다. |
| 17 | negative/non-finite raw value가 Taffy까지 넘어간다 | parser invalid input과 typed layout negative/non-finite injection을 서로 다른 단계에서 거부한다. |
| 18 | cap은 일치해도 Grid 바깥 overflow를 clip하거나 border를 잘못 셈한다 | 별도 Chrome control에서 root right 54, child right 82, 28px overflow와 border-marker pixel을 고정한다. |
| 19 | unsupported track이 한 축에서 실패한 뒤 다른 축 frame을 부분 발행한다 | atomic scene publication 부재, node/property 진단, stale revision 회귀를 acceptance에 둔다. |
| 20 | AVD/simulator 또는 bounded slice를 실기기·WPT·C11 전체 완료로 승격한다 | PixelCopy/Metal 제출 전 표본과 나머지 한계를 분리하고 C11 상태와 `72/213`을 유지한다. |

코드 구현 전 계획 검토 20개를 모두 반영했다. 이 검토는 구현/runtime review가 아니며 구현 뒤 재사용하지 않는다.
