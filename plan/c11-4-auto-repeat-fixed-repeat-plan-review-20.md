# C11.4 fixed-repeat + auto-repeat 계획 검토 · 실패 관점 20개

**검토 기준:** 계획 [c11-4-auto-repeat-fixed-repeat.md](./c11-4-auto-repeat-fixed-repeat.md), CSS Grid 2025 CRD §7.2.3.1–7.2.3.2, 고정 Chrome 156 fixture 및 독립 계산. 이 검토는 계획 변경 전에 수행하며 구현/runtime 결과를 주장하지 않는다.

| # | 독립 실패 관점 | 검토 결과와 계획상 통제 |
| --- | --- | --- |
| 1 | 표준 문법이 fixed-repeat과 auto-repeat을 같은 list에 허용하지 않을 수 있음 | §7.2.3.1이 fixed-repeat 조합은 허용하고 auto-repeat은 최대 하나로 제한한다고 기록했다. 후보 grammar를 구현 전에 다시 고정한다. |
| 2 | Stylo typed AST가 authored 문법과 다른 repeat shape로 normalize될 수 있음 | 1단계에서 실제 computed AST와 line-name group 길이를 검사하며, 다르면 코드 작성 전에 계획/oracle을 재검토한다. |
| 3 | 정수 repeat 내부에 다른 repeat을 중첩해 grammar 경계를 넘을 수 있음 | CSS Grid의 non-nesting 규칙을 scope와 unsupported input matrix에 두고 중첩 입력은 fail-closed한다. |
| 4 | 숫자 count가 0, 음수 또는 `usize` 변환 불가일 수 있음 | positive count와 checked conversion을 요구한다. CSS parser 거부와 typed adapter 오류를 별도 관찰한다. |
| 5 | prefix fixed-repeat 확장 뒤 auto-repeat 시작 index가 이전 authored index에 머물 수 있음 | 전개된 track 수로 `start/end`를 다시 산정하고 cascade·projection tests가 4-track prefix 경계를 확인한다. |
| 6 | suffix fixed-repeat이 반복 pattern보다 먼저 배치되거나 사라질 수 있음 | suffix 전개 순서를 별도 columns/rows oracle case로 고정한다. |
| 7 | fixed-repeat 확장 후 outer line group이 `N+1` 규칙과 맞지 않을 수 있음 | named lines는 거부하되 빈 metadata group의 component 경계 수를 전개 track 수에 맞춘다. cascade 및 projection에서 count를 검사한다. |
| 8 | authored 또는 repeat 내부 line name이 silently dropped될 수 있음 | 이번 slice는 모든 nonempty line-name group을 거부한다. 이름 merge, collision, alias semantics를 scope 밖에 둔다. |
| 9 | fixed-repeat track에 negative, nonfinite, percentage, calc, intrinsic, flex breadth가 섞일 수 있음 | 직접 finite nonnegative `LengthPx`만 허용한다. 다른 breadth별 fail-closed case를 계획에 포함했다. |
| 10 | fixed-repeat 외부 neighbor validation을 넓혀 이전 거부 경계가 바뀔 수 있음 | 기존 auto-repeat neighbor validator를 재사용하고 이번 변경에서 allowlist를 손대지 않는다. |
| 11 | auto-repeat fragment의 fixed minimum 또는 최대 sizing whitelist가 확장될 수 있음 | 기존 fixed-minimum validation을 그대로 두고 사전 oracle은 기존 허용 `minmax(32px,1fr)`만 사용한다. |
| 12 | auto-fill 개수 계산에서 fixed-repeat breadth 또는 그 gutter가 누락될 수 있음 | spec-derived `108 + 40n` 하한과 prefix/suffix를 모두 넣어 2회/3회 경계를 판정한다. 모든 active gutter를 layout oracle에 포함한다. |
| 13 | auto-fit이 count 계산 전 빈 track을 없애 반복 수가 한 개로 줄 수 있음 | spec 순서상 count 후 placement/collapse임을 계획에 명시하고 empty/occupied case를 분리한다. |
| 14 | 점유 track 옆 collapsed track을 처리하면서 range boundary gutter도 함께 잘못 접힐 수 있음 | Chrome 156의 occupied case `84px` track 및 `x/y=92px` item을 고정하고 시작·끝 boundary behavior를 layout trace로 확인한다. |
| 15 | row 계산이 inline-size를 쓰거나 column 계산이 block-size를 쓸 수 있음 | 200px width와 200px height를 독립 scene으로 실행하고, 한 grid의 양 축 동시 조합은 이번 계획에서 제외한다. |
| 16 | authored count 검사는 실제 expanded fixed tracks가 아니라 component 수만 셀 수 있음 | count×pattern 길이 및 전체 explicit descriptors를 checked arithmetic 후 10,000 한도와 비교한다. Auto-repeat의 realized count는 layout 한도와 별도로 검사한다. |
| 17 | integer overflow 또는 allocation 실패가 부분 track/style을 남길 수 있음 | checked multiply/add, fallible reserve, 기존 contextual allocation error, fault-injection와 no-partial-output 판정을 요구한다. |
| 18 | fixture-only 확장을 일반 앱 CSS 지원으로 표시할 수 있음 | 공개 CSS admission은 제외하며 internal contract·STATUS·roadmap에 제한 profile과 C11.4 미완료/`72/213`을 명시한다. |
| 19 | 앱 fixture가 잘못된 grid revision을 보거나 artifact에 test code가 포함될 수 있음 | 현재-source Android/iOS runtime에서 layout revision을 묶고, default vs opt-in artifact positive/negative controls를 분리한다. |
| 20 | Chrome 결과만 표준 기대값으로 삼거나 과거 계획 검토를 구현 검토에 재사용할 수 있음 | 독립 수식·규범을 함께 쓰고 Chrome을 교차 관찰로 한정한다. 구현/runtime 20개와 PR diff 30개는 새 실패 관점으로 따로 작성한다. |

## 결론

계획은 CSS Grid가 허용하는 fixed-repeat 혼합 중 direct-`px` 그룹 하나만 선택하며, parser shape·line metadata·반복 수·collapse·resource limit·artifact 경계를 구현 전에 확인하도록 한다. 계획 관문은 통과시킬 수 있다. 코드 변경과 simulator 실행은 이 검토에 포함하지 않는다.
