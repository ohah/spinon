# C11.4 auto-repeat named-line implicit fallback 계획 검토 20개

계획 PR을 병합하기 전, implementation 전 실패 관점 20개를 서로 다른 계약·경계로 검토했다. 각 관점의 대응은 계획에 반영했다. 이는 구현 코드나 runtime을 검토한 기록이 아니다.

| # | 실패 관점 | 검토 및 계획의 대응 |
|---:|---|---|
| 1 | 없는 line name을 auto-repeat 전에 숫자로 해석해 repeat 수와 다른 line을 선택 | Symbolic placement를 Taffy expanded metadata 이후까지 보존한다. |
| 2 | 실제 author typo를 자동 보정해 의도치 않은 먼 implicit track을 만든다 | CSS §8.3의 fallback대로만 처리하며 공개 generic CSS admission은 열지 않는다. |
| 3 | explicit name occurrence가 모자란 경우 absent-name과 같은 결과라고 가정한다 | `ghost` 부재와 `cell` occurrence 초과를 별도 positive/negative scenes로 나눈다. |
| 4 | positive occurrence가 explicit grid 끝을 넘을 때 `grid-auto-columns`가 아닌 다른 track size를 쓴다 | Oracle과 runtime에서 implicit track을 typed 20px로 고정한다. |
| 5 | negative occurrence가 explicit grid 시작을 넘을 때 leading implicit line의 index origin을 잘못 잡는다 | Negative absent/insufficient scene과 ordered track list를 함께 비교한다. |
| 6 | named span의 방향을 grid start/end property 이름에 고정해 반대로 탐색한다 | Forward/backward span을 서로 다른 explicit anchor로 시험한다. |
| 7 | named span `count`가 named-line `occurrence`처럼 explicit grid 끝 기준으로 해석된다 | Span expected result는 반대 endpoint와 search direction에서 산출한다. |
| 8 | auto-fill repeated line names의 중복 경계가 occurrence 수에 중복 반영된다 | Auto-fill line group과 computed track serialization을 fixture로 고정한다. |
| 9 | negative index가 전체 expanded grid 끝을 기준으로 바뀐다 | 규범상 explicit-grid end 기준을 명시하고 current layout path를 확인한다. |
| 10 | line resolution은 맞지만 implicit track expansion이 빠져 frame이 맞지 않는다 | Used track list와 item frame을 모두 acceptance gate로 둔다. |
| 11 | row-axis placement에서 column-axis guard만 제거되거나 fallback 방향이 다르다 | Row positive/negative absent-name controls를 포함한다. |
| 12 | fixed prefix/suffix neighbor가 fallback index를 이동시킨다 | Current slice의 no-neighbor oracle을 격리하고 neighbors regression을 별도 보존한다. |
| 13 | named-area generated line이 authored metadata 부재로 fallback에 섞인다 | Named areas 결합을 명시적으로 제외한다. |
| 14 | auto-fit collapse가 placement 후 line index를 바꿔 named fallback 결과가 흔들린다 | Auto-fit/post-placement collapse를 별도 후속으로 남기고 auto-fill로 격리한다. |
| 15 | 두 endpoint 모두 named span인 경우 conflict handling이 fallback 검색 전에 잘못 발생한다 | Existing conflict handling을 바꾸지 않고 paired-span control을 둔다. |
| 16 | zero, i16/u16 overflow와 allocation failure가 membership gate 제거 과정에서 완화된다 | Range/zero/allocation guards를 그대로 유지하고 negative controls를 둔다. |
| 17 | generic non-auto-repeat named placement 경로까지 implicit fallback이 켜진다 | 변경 경계를 auto-repeat path에 한정한다. |
| 18 | unsupported profile에 partial layout이나 auto-placement fallback을 반환한다 | 제한 profile과 fail-closed error semantics를 유지한다. |
| 19 | browser observation만 규범 기대값처럼 취급하거나 반대로 표준만으로 구현을 검증한다 | W3C 계산, Chrome 관찰, 후속 Spinon runtime을 독립 증거로 유지한다. |
| 20 | 좁은 simulator fixture 결과가 WPT·실기기·일반 CSS 지원으로 과장된다 | 문서/PR에서 제외 범위와 C11.4 `77/213` 유지 조건을 명시한다. |

계획 근거는 [사전 비교 README](../spec/internal/evidence/c11-4-auto-repeat-named-line-implicit-fallback-precomparison-2026-10-23/README.md)와 [구현 계획](./c11-4-auto-repeat-named-line-implicit-fallback.md)이다. PR diff review는 이 문서와 별도로 수행한다.
