# C11.4 fixed-repeat `minmax(px, fr)` 계획 검토 · 실패 관점 20개

**검토 기준:** [계획](./c11-4-fixed-repeat-minmax-flex.md), [사전 비교와 규범 기대값](../spec/internal/evidence/c11-4-fixed-repeat-minmax-flex-precomparison-2026-10-25/README.md), [기대값 JSON](../spec/internal/evidence/c11-4-fixed-repeat-minmax-flex-precomparison-2026-10-25/spec-derived-expectations.json), `auto_repeat.rs`, `grid.rs::track_size()`와 `track_size_is_valid()`, CSS Grid 2025 CRD §7.2.3.1–2·§7.2.4·§11.7–§11.7.1. 계획 hash `c66b5a54fc2c86d6c5d07d72b826833a4651d4adce0a5bb04cbc44522c3a0b97`, 기대값 JSON hash `cbae18cee32348eefdaa6b54c33b50fea39d0ab97f1fe851061cfd560704181b`인 초안을 검토했다. 코드·runtime 변경, 테스트, Agent Browser 세션은 실행하지 않았다.

| # | 검토 경계 | 구체적 실패 경로 | 근거·영향 | 판정 |
|---:|---|---|---|---|
| 1 | fixed-repeat 문법과 `minmax(px, fr)`의 규범 유효성 | adapter가 fixed-repeat 안의 `Fraction`을 syntax 오류처럼 전부 거부하면 이 slice의 유일한 새 입력이 계속 실패한다. | CRD §7.2.3.1의 `<fixed-size>`는 `minmax(<fixed-breadth>, <track-breadth>)`를 포함하고 `<track-breadth>`는 flex를 허용한다. 계획은 min `LengthPx`, max `Fraction`으로 좁혀 이 경계를 맞게 정했다. Chrome `CSS.supports()`도 fixture 입력을 허용한다. | **COVERED** |
| 2 | auto-repeat 개수 산정에서 min/max 선택과 gutter 수 | fixed `fr` maximum이나 gap 하나를 반복 횟수 산식에 넣어 columns 6·rows 3이 아닌 다른 count를 얻는다. | CRD §7.2.3.2는 max sizing function이 definite면 max, 아니면 definite min을 사용한다. 고정 최소값과 gap을 이용한 `28m+28`은 200px에서 6, 120px에서 3이다. 두 fixed group 장면의 `28m+92<=300`은 7이다. 독립 계산 JSON은 Chrome 관측과 분리돼 있다. | **COVERED** |
| 3 | 1fr·2fr의 freeze와 잔여 공간 분배 | auto min 24px track을 freeze하지 않거나 작은 base인 prefix flex track을 잘못 freeze해 남은 공간을 suffix에 배분하지 못한다. | CRD §11.7·§11.7.1의 hypothetical fr/restart 순서를 반영한 `columns-fill`, `rows-fill`, `columns-groups`가 있다. 관측/기대값은 각각 auto 24px freeze, 두 1fr 14px/16px, 마지막 2fr tracks 22px다. | **COVERED** |
| 4 | 0fr minimum track의 sizing | 허용 목록은 `Fraction`이 0 이상이라 `minmax(12px, 0fr)`도 수용하지만, layout이 0fr에 남은 공간을 나눠 줘 12px base를 바꾸거나 0을 나눗셈 분모로 처리한다. | CRD의 `<flex [0,∞]>`는 0을 포함한다. 현재 fixture에는 0fr가 없고 모두 1fr·2fr여서 지원 문구의 하한 동작을 판정할 수 없다. 지원을 주장하려면 0fr 기대값을 고정하거나 허용 범위를 좁혀야 한다. | **BLOCK · 계획 수정 필요** |
| 5 | 0과 1 사이 flex factor의 partial-fill 규칙 | 구현이 flex 합을 항상 1로 normalize해 `0.25fr`/`0.5fr`가 잔여 공간 전부를 차지한다. | CRD §7.2.4는 factor 합이 1 미만이면 요청된 비율만 채우고 나머지를 남긴다고 규정한다. 계획의 nonnegative 범위는 sub-1 값을 포함하지만 관측은 1fr·2fr뿐이다. 대표적인 합계 `<1` positive scene과 independent expectation을 추가하거나 범위를 좁혀야 한다. | **BLOCK · 계획 수정 필요** |
| 6 | 개별 유한 factor와 aggregate f32 overflow | 각 `Fraction`은 finite지만 여러 큰 factor의 합이 Taffy의 f32 accumulator에서 `∞`가 되거나 factor×fraction 곱이 overflow해 재시작 판정과 track size가 규범값에서 벗어난다. | 현재 Taffy `find_hypothetical_fr_size()`는 factor sum과 product를 f32로 계산하고, 코드 주석도 non-finite 입력이 종료 불변식을 깨뜨릴 수 있다고 경고한다. 계획은 각 수의 finite 조건만 두고 aggregate bound·checked accumulation·거부 기대값을 정하지 않았다. | **BLOCK · 계획 수정 필요** |
| 7 | Chrome CSSOM fractional quantization과 runtime 허용오차 | Chrome의 직렬화값을 독립 수학값으로 취급해 `rows-fit`을 실패 처리하거나 1/64 허용을 넘어 오차를 숨긴다. | `112/3=37.333…`과 Chrome 관측 `37.328125`의 차이는 약 0.00521px로 1/64 이내이며 runtime 0.02px 안에도 든다. JSON은 정수·collapsed zero를 exact로 분리한다. | **COVERED** |
| 8 | 유효하지만 profile 밖인 percentage minimum | `minmax(20%,1fr)`을 새 fixed-minimum allowlist에 잘못 포함하거나, Chrome의 CSS 지원만으로 Spinon profile 지원을 주장한다. | 사전 비교에서 Chrome `CSS.supports()`는 true이고 profile 결과는 `UnsupportedComputedGridValue`로 고정했다. 계획은 node/property 진단 확인을 요구한다. | **COVERED** |
| 9 | 유효하지만 profile 밖인 intrinsic maximum | `minmax(12px,max-content)`가 flex 허용 분기로 새어 들어가거나 값은 거부하지만 node/property 문맥을 잃는다. | Chrome 관측은 CSS valid임을 보여 주며 JSON은 이 control을 profile rejection으로 분리한다. 계획의 typed allowlist와 세 rejection control gate가 동작 차이를 보존한다. | **COVERED** |
| 10 | 문법상 invalid flex minimum의 parser 경계 | `minmax(1fr,12px)`이 cascade의 지원 밖 값 오류로 기록되거나, 반대로 Grid profile fixture까지 전달된다. | CRD §7.2.1은 flex를 minimum으로 쓸 수 없다고 하고 Chrome `CSS.supports()`는 false다. 계획과 JSON은 CSS parse rejection으로 분리한다. | **COVERED** |
| 11 | absolute `calc()`의 computed normalization과 reject 범위 | `calc(20px)`가 `LengthPx(20)`로 normalize돼 typed allowlist상 수용 가능한데 계획은 `minmax(calc(...),fr)` 전체를 거부한다고 써 모순된 결과를 낸다. 반대로 unresolved calc까지 허용할 수도 있다. | 계획 지원 절은 computed `LengthPx`로 바뀐 `calc(20px)`을 허용하고, 거부 절은 `minmax(calc(...),fr)`를 일괄 거부한다. `track_size()`는 computed `Length`와 `Calc` variant를 구분하므로 authored source가 아닌 typed computed shape로 판정하도록 문구와 control을 정리해야 한다. | **BLOCK · 계획 수정 필요** |
| 12 | 음수·비유한 typed operand의 fail-closed 경계 | parse/cascade 입력이 negative 또는 non-finite px/factor가 되면 반복 확장 전에 거부하지 않고 track vector에 들어가 NaN/∞ geometry를 낸다. | `track_size_is_valid()`는 computed breadth의 finite/nonnegative를 검사하지만 새 MinMax 전용 allowlist가 두 변의 조건을 모두 보존해야 한다. 계획은 그 invariant를 선언했으나 JSON에는 이 negative/overflow control이 없다. typed fixture rejection/assertion을 구체화해야 한다. | **BLOCK · 계획 수정 필요** |
| 13 | fixed-repeat group 개수 한도 | minmax allowlist가 바뀌면서 각 group은 검사하지만 `fixed_repeat_group_count` cap 또는 list component cap을 우회해 과도한 반복 metadata를 할당한다. | 현재 `auto_repeat.rs`는 list 길이와 fixed-repeat group 수를 `MAX_COMPUTED_GRID_TRACKS`로 제한한다. 계획은 “existing group 수 유지”를 말하고 있지만 새 track type이 포함된 경계 입력은 scene/control에 없다. 허용 최댓값과 바로 위 거부값을 명시해야 회귀 검증이 가능하다. | **BLOCK · 계획 수정 필요** |
| 14 | expanded track 총량 10,000 경계 | 2-track pattern의 `count × pattern length` 또는 auto/ordinary neighbor를 합친 total을 검사하지 않아 10,001개 이상을 만들거나, 10,000개 적합 입력을 거부한다. | 현 코드는 checked multiplication과 누적 `MAX_COMPUTED_GRID_TRACKS` 제한을 갖지만 계획은 어느 경계값을 테스트할지 적지 않았다. 2-track minmax와 auto-repeat 하나를 포함한 total 10,000 통과·10,001 거부를 고정해야 한다. | **BLOCK · 계획 수정 필요** |
| 15 | expanded line metadata 1 MiB 예산과 seam | flex-bearing group 허용 분기에서 repeated authored names가 line-vector/header/string byte 예산 밖으로 확장되거나 repeat boundary에서 이름이 합쳐지지 않는다. | 다섯 사전 장면은 line names가 비어 있어 metadata 예산을 건드리지 않는다. 계획은 seam을 유지한다고 하지만 구체적인 named MinMax case 및 cap 전후 결과가 없다. named seam 회귀와 1 MiB 이하/초과 입력을 고정해야 한다. | **BLOCK · 계획 수정 필요** |
| 16 | reservation 실패의 error atomicity와 재호출 | track 또는 expanded name reservation 실패 뒤 일부 tracks/line groups가 반환되거나 같은 cascade worker 다음 입력까지 failpoint/상태가 남는다. | 현재 source는 fixed groups·tracks·line groups·line names·문자열을 fallible reserve로 만들며 테스트 failpoint 종류도 갖는다. 계획은 failure atomicity를 요구하므로 그 실패의 오류 종류, partial output 부재, 다음 호출 복구를 실제 MinMax 입력에 적용하면 통제된다. | **COVERED · 구현 게이트에서 명시 실행** |
| 17 | auto-fit 점유 판정, collapsed track과 인접 gutter | fixed suffix item을 점유된 auto-repeat track으로 계산하거나 placement 전 collapse해 line index와 item starts가 달라진다. | columns/rows `fit` scenes는 repeated index 0만 점유하고 나머지를 collapse하며 fixed suffix 위치도 검사한다. JSON은 zero track과 starts를 exact 비교한다. CRD §7.2.3.2의 collapse-after-placement 경계와 일치한다. | **COVERED** |
| 18 | fixture 전용 ABI와 제품 artifact 분리 | 새 scene ID·host symbol·payload가 기본 product archive로 유입되어 opt-in validation code가 제품 경로에 노출된다. | 계획은 전용 fixture-test feature의 opt-in positive control과 기본 product archive의 negative control을 각각 요구하고, archive 근거를 APK/IPA 전체 부재로 확대하지 말라고 명시한다. | **COVERED** |
| 19 | Android/iOS runtime 증거, source identity와 PR 화면 | 다른 source revision으로 만든 APK/app을 실행하거나 track/frame만 확인하고 draw/present를 누락한다. 또는 simulator 결과를 실기기·hardware GPU·pixel equality로 과장한다. PR에는 실행 화면을 넣을 수 있는데 계획은 screenshot 첨부·이미지 표시 확인을 정하지 않는다. | 계획은 양 simulator에서 5 scene의 source/revision, used tracks, frames, trace, draw/present를 확인하고 device 한계를 표시하지만 캡처/화면 첨부 단계가 없다. 앞선 사용자 요청은 가능한 동작 화면을 PR 본문에 올리는 것이다. 해당 화면 저장·첨부와 PR에서 이미지 로드 확인을 게이트에 추가해야 한다. | **BLOCK · 계획 수정 필요** |
| 20 | 상위 C11.4 완료 상태와 공식 집계 | 제한된 `minmax(px,fr)` fixture 병합을 전체 C11.4 지원 또는 체크리스트 완료로 승격한다. | 계획은 구현·계획 PR 모두 완료 승격이 아니며 C11.4 checkbox와 `72/213`을 그대로 두도록 명시했다. STATUS 최신 C11.4 line도 계획-only·72/213으로 기록한다. 구현 뒤에도 전체 CSS/WPT 등 남은 범위가 닫히기 전에는 부모 체크를 바꾸지 않는다. | **COVERED** |

## 판정

규범 grammar와 현재 five-scene 산술·rounding, fail-closed controls, auto-fit collapse, artifact 분리 및 상위 완료 상태는 대체로 분리해 잡혀 있다. 다만 계획을 구현 시작 관문으로 통과시키기 전에 0fr·sub-1fr와 aggregate flex overflow 범위, absolute/unresolved calc 허용 규칙, negative/non-finite rejection, group/expanded track/metadata 정확한 cap 경계, MinMax 입력의 reservation 실패 검증, 그리고 가능한 simulator screenshot의 PR 첨부·표시 확인을 명시해야 한다. 특히 #4–6·#11–15·#19는 현 초안으로는 구현자가 서로 다른 계약을 선택할 수 있어 계획 수정이 필요하다. C11.4 전체 completion은 이 slice와 무관하게 미완료로 유지한다.

## 규범 참고

- [CSS Grid 2025 CRD §7.2.3.1–§7.2.3.2: repeat 문법·반복 수·auto-fit collapse](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#repeat-notation)
- [CSS Grid 2025 CRD §7.2.4: 0–1fr partial fill](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#fr-unit)
- [CSS Grid 2025 CRD §11.7–§11.7.1: flexible track freeze와 hypothetical fr](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-flex-tracks)

## 수정 추적

위 20행은 plan revision 전 초안의 결함을 보존한다. blocking 관점은 계획 및 사전 oracle에 다음처럼 반영했다. 수정된 계획은 두 번째 독립 실패 관점 검토를 거친 뒤 구현 관문으로 넘긴다.

| 원 리뷰 | 수정 |
|---|---|
| #4–6: 0fr, sub-1fr, factor-sum overflow | 계획은 full finite nonnegative `Fraction` 의미를 유지하고, 0fr/0.5fr Chrome 장면과 `1e38fr` CSS-valid overflow control을 추가했다. Taffy 전 conservative expanded-axis f32 upper bound를 요구한다. |
| #11: authored calc 허용/거부 모순 | authored source가 아닌 computed shape를 기준으로 명시했다. `LengthPx`로 fold되면 허용하고 unresolved `Calc`는 거부한다. |
| #12: negative/non-finite values | synthetic negative, NaN, ±infinity typed controls와 node/property를 보존하는 Taffy 전 rejection을 추가했다. |
| #13–15: group/track/metadata ceilings | component 10,000/10,001, descriptors 10,000/10,001, line metadata 1,048,576/1,048,577-byte exact boundary와 MinMax line-name seam을 추가했다. |
| #16: reservation failure | MinMax pattern의 group/track/name reservation failpoint, partial result 없음과 다음 호출 복구를 계획 검증에 추가했다. |
| #19: simulator capture/PR image | 구현 PR 본문에 실행 screenshot을 표시하고 Agent Browser로 실제 이미지 로드를 확인하는 관문을 추가했다. |
