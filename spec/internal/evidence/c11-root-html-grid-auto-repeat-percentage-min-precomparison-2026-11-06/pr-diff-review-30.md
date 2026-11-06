# C11.1 auto-repeat percentage minimum 계획 PR diff 적대 검토 30개

## 검토 범위와 수정한 발견

검토 대상은 계획 문서, C11 계획 색인에 추가한 4줄, 계획 자체의 20관점 기록, six-scene HTML fixture, 독립 expected JSON, Chrome 157 observation, 비교 검증기, screenshot, checksum manifest다. 구현 코드는 변경하지 않았다.

첫 검토에서 percentage `0%` 장면이 없어 1px count floor를 percentage track sizing과 분리해서 보이지 못했다. 0% 장면을 추가하고 Agent Browser Chrome 157로 다시 관찰해 6/6으로 비교했다. 검증기의 축 선택이 `40px` 문자열 특례에 묶여 있던 점도 찾아 expected basis의 width/height를 사용하도록 고쳤다. 확장 repeat track 수와 overflow를 검증기에 추가했다.

| # | 별도 실패 관점 | 실제 diff 검토 결과 |
|---:|---|---|
| 1 | 기존 CSS Grid 범위와 새 slice가 혼동되어 전체 C11 진행으로 보임 | **통과.** 계획과 색인은 `RootHtmlAutoRepeatV1` 제한 후보로 쓰고 상위 완료 표시는 유지한다. |
| 2 | 기본 Root HTML route와 C11.4 opt-in fixture가 중복 구현처럼 보임 | **통과.** 문서가 default route와 opt-in route를 각각 지정한다. |
| 3 | 공식 완료 집계가 계획 단계에서 증가 | **통과.** `spec/STATUS.md`를 수정하지 않았고 계획 문서의 공식 수는 `78/214`다. |
| 4 | C11.1/C11 체크가 계획 등록만으로 완료 처리 | **통과.** 색인과 전용 계획이 구현 미시작을 명시한다. |
| 5 | 제한 profile이 일반 CSS 지원을 약속 | **통과.** 일반 author admission, WPT, indefinite percentage, hardware·실기기는 제외한다. |
| 6 | percentage basis가 viewport/parent/border-box로 잘못 선택 | **통과.** 범위와 fixture는 definite repeated-axis content-box를 사용하고 border/padding은 0이다. |
| 7 | `1fr` maximum이 repeat count에 쓰이는 definite sizing function으로 해석 | **통과.** 문서는 flex maximum을 indefinite로 두고 definite minimum 기준을 명시한다. |
| 8 | minimum percentage가 다른 축 크기로 계산 | **통과.** columns 폭과 rows 높이를 따로 고정하고 `basis.axis`를 expected에 남긴다. |
| 9 | 반복 fragment 시작·끝의 gutter를 하나 누락 | **통과.** 산식은 numerator `A+G`, denominator `Σmax(mᵢ,1px)+P×G`로 정리했다. |
| 10 | single-track repeat에서 gap이 0일 때 count가 흔들림 | **통과.** 120% overflow 장면은 zero-gap count 1을 고정한다. |
| 11 | single-track positive percentage count가 잘못 계산 | **통과.** 10%/20% 장면의 독립 count와 used tracks가 Chrome 관찰과 일치한다. |
| 12 | row repeat가 column basis만 쓰도록 구현됨 | **통과.** 160px definite-height row 장면과 expected axis 검증을 추가했다. |
| 13 | 2-track pattern의 내부·반복 경계 gutter가 중복되거나 누락 | **통과.** 5회·10-track 기대값과 browser track 순서를 비교한다. |
| 14 | pair의 1fr sizing이 minimum freeze 규칙을 놓침 | **통과.** `[20,16.4]px` ideal pair는 20px track freeze 뒤 남은 1fr 공간을 나눠 계산했다. |
| 15 | Chrome의 1/64px 분산 반올림을 제품 오차로 오인 | **통과.** frame·used-value는 0.02px 허용, count와 track 수는 exact 비교다. |
| 16 | auto-fit의 empty-track collapse 전에 repeat 수를 낮춰 계산 | **통과.** 반복 8개를 유지하고 occupied track 뒤 7개 collapse 결과를 기록한다. |
| 17 | 0% percentage가 count 때 0으로 나뉘어 과대 반복 | **통과.** 독립 기대값에서 resolved minimum 0과 count minimum 1을 분리하고 34개를 검사한다. |
| 18 | count 전용 1px floor가 실제 typed minimum에 복사 | **통과.** 계획은 `Percentage(0)` 전달을 고정하고 expected에는 sizing minimum 0px가 남는다. |
| 19 | 100%를 넘는 minimum을 조용히 clamp | **통과.** 120% 입력에서 240px track과 40px overflow를 비교한다. |
| 20 | CSS maximum, `calc()`, named line, neighbor 등 지원 밖 입력까지 열림 | **통과.** planned typed grammar와 기존 rejection profile을 문서에 제한한다. |
| 21 | source 문자열을 다시 읽어 cascade typed value/owner provenance 우회 | **통과.** projection 단계에서 computed typed value와 owner identity 유지 조건을 적었다. |
| 22 | definite size가 없거나 유효하지 않은 axis로 percentage를 해석 | **통과.** positive definite fixed width/height, checked finite arithmetic 조건을 명시한다. |
| 23 | large count에서 expansion 전 quota 또는 integer overflow가 검사되지 않음 | **통과.** 10,000-track cap과 checked arithmetic을 expansion 전에 검증하도록 했다. |
| 24 | fixture가 실제와 다른 axis dimension, gap, item order를 기록 | **통과.** HTML input과 observation JSON을 6개 scene ID로 대조했고 독립 verifier가 frames와 track order를 비교한다. |
| 25 | font/network/assets가 관측에 비결정성을 추가 | **통과.** fixture는 system font만 쓰며 expected geometry는 고정 크기의 10px items다. |
| 26 | expected 값이 Chrome observation에서 역산되어 독립 oracle이 아님 | **통과.** expected JSON은 고정 percentage·gap 산식을 출처로 명시하며 Chrome 값은 observation 파일에 별도 보관한다. |
| 27 | 관측 browser version 또는 DPR 누락 | **통과.** HeadlessChrome 157.0.0.0, DPR 1, `CSS.supports` 결과가 저장돼 있다. |
| 28 | verifier가 axis를 fixture 값 문자열에 결합하거나 repeat 수 누락 | **수정·통과.** `basis.axis`로 columns/rows를 고르고 `repeatCount × patternTracks`를 expanded track 수와 exact 비교한다. |
| 29 | overflow가 화면에만 보이고 기계 검증에서는 빠짐 | **수정·통과.** verifier가 observed tracks의 합과 definite size 차이로 40px overflow를 계산한다. |
| 30 | screenshot/checksum이 다른 fixture·expected·observation을 가리키거나 검증 범위를 과장 | **통과.** screenshot은 6 scenes를 표시하고 4개 입력·관측 파일의 SHA-256 검증이 성공했다. README는 이를 Spinon 구현·WPT·실기기 증거로 부르지 않는다. |

## 판정

초기 누락과 검증기 결합 지점을 수정한 뒤 `node .../verify-observation.mjs`는 Chrome 157 기준 6/6 장면을 통과했고, `shasum -a 256 -c`는 네 파일을 모두 통과했다. 이 계획 PR 검토는 구현/runtime 20관점 검토나 향후 implementation PR diff 검토를 대신하지 않는다.
