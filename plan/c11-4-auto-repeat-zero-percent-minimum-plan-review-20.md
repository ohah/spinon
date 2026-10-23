# C11.4 zero-percent auto-repeat minimum 계획 실패 관점 20개

계획, CSS Grid 2025 CRD에 근거한 독립 기대값, Chrome 156 및 Agent Browser 관찰, 현재 `spinon-layout` preflight와 vendored Taffy count 경로를 교차 검토했다. 서로 다른 20개 입력·불변조건을 확인했다. 구현/runtime 검토와 계획 PR diff 검토는 별도로 수행한다.

| # | 적대적 관점 | 검토 결과와 판정 |
|---:|---|---|
| 1 | `0%` minimum을 auto-repeat grammar에서 unsupported라고 계속 거부한다. | **통과.** CSS Grid 2025 CRD fixed-size grammar와 §7.2.3.2의 count floor를 인용했다. 구현은 cascade가 만든 `Percentage(0.0)`만 기존 제한 profile 안에서 받도록 계획했다. |
| 2 | count floor가 실제 minimum sizing function을 1px로 변경한다. | **통과.** floor를 반복 개수 계산에만 적용하고 실제 track initialization·flex distribution에는 전달하지 않는다고 분리했다. |
| 3 | zero minimum이 `0/0`, NaN 또는 `u32::MAX` clamp로 이어진다. | **통과.** count denominator에 들어가는 각 repeated contribution을 먼저 floor하며 0/NaN/Infinity 방지 조건을 별도 Taffy test gate로 뒀다. |
| 4 | 두 track 이상인 repeat group에서 그룹 합계만 1px로 floor한다. | **보완 완료.** 원래 계획에는 multi-track 반복 불변조건이 구체적이지 않았다. 120px·gap 0의 `minmax(0%, 1fr) 0.5px` group이 두 track 각각 floor되어 60회 반복되는 Taffy unit test를 추가했다. |
| 5 | definite maximum이 있을 때 minimum으로 count를 산정한다. | **통과.** 현재 `track_definite_value`의 max-if-definite, min fallback, definite max를 min으로 floor하는 순서를 유지한 뒤 UA floor를 적용하도록 계획했다. positive maximum control을 unit test에 포함했다. |
| 6 | repeat 앞뒤의 0.5px explicit track에도 1px floor를 적용한다. | **통과.** floor scope를 반복 fragment 내부 track으로 한정했고 이웃 explicit track은 관측·기대 histogram에서 각 0.5px로 고정했다. |
| 7 | repeat fragment 내부 gap과 fragment 사이 gap 개수를 혼동한다. | **통과.** gap은 track floor와 분리하고 120px·gap 0/4에서 각각 120/24 repetitions, prefix/suffix case는 total 25 tracks·24 gaps로 고정했다. |
| 8 | columns percentage basis로 viewport 또는 border-box를 사용한다. | **통과.** columns는 positive definite inner inline content size만 사용한다. 모든 120px columns scene에서 content basis를 검증기 기대값에 추가했다. |
| 9 | rows percentage basis로 inline size 또는 외부 viewport를 사용한다. | **통과.** rows는 positive definite inner block content size를 사용하고 row scene도 120px basis를 observation validator에서 확인한다. |
| 10 | border-box axis의 padding/border 공제를 누락한다. | **보완 완료.** fixture 공통 `.case`에는 좌우 border 2px와 padding 8px가 있다. 계획 표·사전 비교 README·독립 기대값 JSON이 border를 빠뜨려 136−16−4=116px를 충분히 재현하지 못했다. 셋 모두 border 2px를 명시했다. |
| 11 | `auto-fit`이 placement 전에 empty track 수를 줄여 repeat count 자체를 바꾼다. | **통과.** `auto-fill`과 동일한 pre-placement count를 먼저 만들도록 계획하고 빈/단일/span-2 장면을 분리했다. |
| 12 | collapse 단계에서 empty 0px track을 다시 1px로 유지한다. | **통과.** empty `auto-fit` 120/120, 한 item 1×120px+119×0px, span-2 2×58px+22×0px 기대값을 고정했다. |
| 13 | span-2 점유 track 또는 두 track 사이 gap을 잘못 계산한다. | **통과.** 58+4+58=120px item frame과 남은 22개 empty track collapse를 독립 관찰값으로 둔다. 검증기는 item width/height를 직접 확인한다. |
| 14 | 0.5px/0.5% control을 제외해 1px floor가 0% 특례인지 확인하지 못한다. | **통과.** 0%, 0px, 0.5px, 0.5%를 gap 0/4 조건과 비교해 subpixel minimum 일반 floor 경로를 고정했다. |
| 15 | definite content basis가 0px일 때도 양수 basis처럼 허용한다. | **통과.** 기존 positive-axis gate를 유지하고 zero basis 거부를 layout/preflight negative control로 명시했다. 이는 이 제한 profile의 범위다. |
| 16 | indefinite 또는 cyclic percentage basis를 새로 resolve하려 한다. | **통과.** indefinite/cyclic basis는 기존 typed error로 fail-closed하며 새 evaluator나 cyclic sizing 지원을 추가하지 않는다. |
| 17 | negative, NaN, Infinity percentage를 `max(1px)`가 정상화한다. | **통과.** finite/nonnegative 입력 검사를 floor보다 먼저 하며 invalid typed input은 거부하도록 했다. |
| 18 | 기존 `min-size: 0`, `max-size: auto`, supported max와 single-repeat gate가 느슨해진다. | **통과.** 기존 min/max/axis/profile 조건을 유지한다. nonzero unsupported percentage, extra calc grammar, overflow와 unrelated repeat syntax는 계속 거부한다. |
| 19 | 내부 계약이 구현과 달라지거나 출시에 앞서 numeric version을 임의로 올린다. | **보완 완료.** 원래 계획에 내부 계약 변경이 빠져 있었다. 구현 단계에서 계약 0059에 0% gate·count-only floor·collapse·실패 경계를 반영하고 숫자 버전 `0.1.0`은 유지하도록 추가했다. |
| 20 | opt-in fixture 결과를 기본 artifact·일반 CSS·전체 C11.4 완료 주장으로 확대한다. | **통과.** default/product와 fixture-test positive control, AVD와 Simulator 및 화면/readback 범위를 분리한다. 검증기는 두 observation의 exact scene ID set·중복 ID·basis·histogram·item presence/dimensions를 확인하고, C11.4 unchecked와 `72/213` 유지 조건을 둔다. |

네 건의 계획·검증 결함을 발견해 반영했다: multi-track count-floor unit case 구체화, border-box fixture의 2px border 명시, 사전 비교 검증기의 scene/basis/item 검사 보강, 구현 시 내부 계약 0059 갱신 조건 추가. 계획은 CSS 전체 지원이나 상위 C11.4 완료를 주장하지 않으며, 구현 PR에서 새로운 실패 관점을 따로 검토한다.
