# C11.4 fixed-repeat 두 그룹 PR diff 실패 관점 검토 30개 · 2026-10-24

이 검토는 계획의 사전 검토 및 구현/runtime 20관점 검토와 별개로 실제 PR diff 전체를 대상으로 한다. 검토 기준은 기존 single-group 동작, 명시된 제한 profile, fixture 경계, 플랫폼별 검증 경계, 공식 상태 대장이다.

| # | 별도 실패 관점 | diff 확인과 판정 |
| --- | --- | --- |
| 1 | 기존 fixed-repeat 한 그룹의 동작이 두 칸 배열 때문에 바뀌는가 | 기존 1-group prefix/suffix/auto-fit tests가 그대로 포함되며 11개 fixed-repeat scoped tests 모두 통과했다. |
| 2 | 두 그룹만 허용하려다 두 번째 그룹을 자동 repeat으로 처리하는가 | `RepeatCount::Number`와 auto-fill/auto-fit가 분리된다. 10 scene의 source 및 cascade track expectations가 group 두 개와 auto-repeat 하나를 각각 확인한다. |
| 3 | 세 번째 fixed-repeat을 자동으로 무시하거나 부분 확장하는가 | extra group은 먼저 fail closed 한다. third-group CSS negative test에서 node와 `grid-template-columns` property가 포함된 오류를 확인했다. |
| 4 | 두 auto-repeat 중 하나가 덮어써져 잘못된 입력이 허용되는가 | 중복 발견 시 기존 `replace` guard가 즉시 오류를 반환한다. 새 코드가 그 guard를 바꾸지 않았다. |
| 5 | zero/negative count 변환이 플랫폼별 정수 cast에 의존하는가 | signed-to-`usize`는 checked conversion이고 양수만 통과한다. CSS grammar가 허용하지 않는 raw negative AST는 별도 실행 근거로 확대하지 않았다. |
| 6 | fixed pattern의 값 타입 제한이 느슨해지는가 | finite nonnegative computed `LengthPx`만 허용한다. percentage, mixed calc, minmax, flex와 intrinsic breadth controls가 남아 있다. |
| 7 | 각 반복 그룹 line metadata가 기존 이름을 유실하거나 합치는가 | fixed group이 있는 profile은 outer/fixed/auto 내부 이름이 모두 비어 있어야 한다. 지원 범위를 넓히지 않고 fail closed 한다. |
| 8 | 2-track pattern count 곱이 overflow하는가 | 그룹별 `checked_mul`이며 fixed group 누적도 `checked_add`다. |
| 9 | 전체 authored track descriptor의 경계가 1 group 기준으로 남아 있는가 | 10,000-track 통과와 두 group 합계 10,001 거부 테스트로 경계를 다시 정했다. |
| 10 | ordinary track 또는 auto pattern 길이를 더하면서 descriptor 수가 넘는가 | 전체 `TrackListValue` 순회에서 checked addition과 같은 cap을 적용한다. |
| 11 | auto-repeat pattern breadth를 실제 반복 횟수만큼 고정 repeat 수에 더하는가 | 계산용 authored descriptor 수와 layout의 realized auto-repeat track 수를 분리하고 기존 auto-repeat sizing path를 유지한다. |
| 12 | repeat group 위치가 바뀌어도 fixed track order를 재정렬하는가 | prefix/split/suffix와 reversed group scene의 cascade 배열에서 authored order를 assert한다. |
| 13 | auto-repeat start/end가 원래 AST index로 남아 expanded index를 가리키지 않는가 | tracks vector 길이로 start/end를 기록한다. 여섯 column 배치와 row split trace에서 expected index가 일치한다. |
| 14 | multitrack group 안쪽 순서가 반복마다 바뀌는가 | `repeat(2,20px 12px)`의 기대 배열이 `[20,12,20,12]`이고 auto-repeat 위치 뒤에 후속 group이 유지된다. |
| 15 | outer line-group 수가 두 group의 확장 크기를 반영하지 않는가 | output metadata 수는 ordinary/fixed expanded count와 auto pattern 경계로 계산된다. line-name-empty 범위와 10,000 경계가 테스트에 있다. |
| 16 | 실제 reserve 크기와 반복 push 수가 다르거나 일부 tracks가 반환되는가 | 사전 total 계산 후 reserve하며 group expansion 실패는 `Result`로 빠진다. allocation-fault injection 및 boundary test가 부분 결과를 허용하지 않는다. |
| 17 | 변경된 reserve 순서에서 allocation failure가 다음 계산까지 오염시키는가 | Tracks, LineGroups, LineNames 세 fault site 뒤 recovery 계산이 성공한다. |
| 18 | `has_fixed_repeat`가 한 개만 있다는 뜻으로 downstream 분기를 망가뜨리는가 | 결과에서는 boolean을 “fixed repeat 존재”로 유지하고 group 개수는 외부 API에 노출하지 않는다. downstream representation은 expanded tracks와 auto-repeat boundary다. |
| 19 | 두 축 동시 지원처럼 보이지만 한쪽 처리만 검증하는가 | 8개 columns scene과 3개 rows scene에서 한 축만 repeat을 가지며 반대 축은 control로 고정한다. simultaneous dual-axis는 미지원이라고 기록한다. |
| 20 | empty auto-fit에서 collapse/gutter 처리가 새 fixed suffix까지 적용되는가 | column prefix 및 row split empty scene에서 auto-fit 반복 track만 collapse되고 뒤 fixed tracks와 ordinary neighbor는 그대로다. |
| 21 | occupied auto-fit의 placement index가 fixed group 확장으로 이동했는데 fixture만 old position을 검사하는가 | columns item `(110,0)`, rows item `(0,110)` 및 `186px` used track을 Rust/JSON expectation, AVD, iOS에서 확인한다. |
| 22 | 독립 CSS 계산과 Chrome 숫자 serialization의 부동소수 차이를 기능 오류로 오판하는가 | 규범/독립 값 `30.8px`와 Chrome serialized `30.7969px`를 사전 비교에서 별도로 기록하고 capture tolerance는 1/64px다. |
| 23 | 새 JSON fixture의 IDs가 누락·중복·unlisted 요청을 받는가 | FFI unit test가 정확히 10 IDs의 uniqueness와 unlisted ID rejection을 검사한다. |
| 24 | `include_str!` 데이터가 task fixture 경계 밖에서 다시 포함되는가 | module path가 `runtime_gpu/fixtures/c11_4` 아래이며 `fixture-module-boundary` test가 새 route와 source location을 포함해 통과한다. |
| 25 | fixture ABI와 payload가 기본 library에 들어가는가 | final-source debug archive에서 기본 marker/ABI absent, `c04-runtime-gpu-fixture-test` positive control에서 present다. final hash는 artifact log에 기록했다. |
| 26 | simulator scene catalog count를 올린 뒤 기존 selection의 ID가 바뀌거나 사라지는가 | capture script의 전체 count는 449에서 459로 바뀌고 Node boundary suite 11/11에서 기존 selected IDs 및 신규 10 IDs를 함께 확인한다. |
| 27 | precomparison JSON의 `cases`/`scenes` 구조 차이로 신규 scene에서 다른 코드가 검증되는가 | adapter는 `chrome.cases ?? chrome.scenes`를 사용하고 count/ID/geometry checks를 같은 공통 경로에 적용한다. 두 fixture catalog가 기대 개수로 실행됐다. |
| 28 | Android generic `fixed-repeat-` selector가 opt-in gate 밖의 신규 장면을 허용하는가 | `BuildConfig.SPINON_C04_RUNTIME_GPU_FIXTURE_TEST` guard가 유지되고 fixed-repeat path 내에서 root/item 크기와 sample coordinate를 group/axis에 따라 구분한다. AVD final capture는 10/10이다. |
| 29 | iOS validator가 임의 prefix scene을 허용하거나 pixel을 post-compositor 결과처럼 보고하는가 | iOS ID set에는 신규 10개 exact ID만 추가됐고 root/item sample 좌표를 검증한다. Metal sample은 제출 전이라는 경계를 문서에 반복해 적었다. iOS final capture는 10/10, 표본 12/12다. |
| 30 | 새 동작이 내부 계약 0059와 상태 대장에 기록되지 않거나 C11.4 전체 완료처럼 집계되는가 | 0059에는 기존 한-group 범위를 이전 slice로 표시하고 두-group 제한 profile을 추가했으며 숫자 버전 `0.1.0`은 유지했다. `STATUS.md`와 public roadmap은 구현 slice와 제한을 기록하되 C11.4 parent checkbox와 72/213을 유지한다. 계획, 실행 근거, 두 simulator screenshot 링크를 연결했다. |

## 수정·재검토

구현 20관점 검토에서 발견한 Clippy `needless_range_loop`를 iterator 방식으로 고쳤다. 이후 Rust format, 11개 style tests, fixture unit test, fixture-boundary 11/11, scoped Clippy, final-source Android/iOS rebuild 및 10/10 capture를 다시 확인했다. 마지막 문서 대조에서 내부 계약 0059의 한-group 설명이 current grammar로 읽힐 수 있어 historical 범위를 분리하고 새 제한 profile을 추가했다. 계약 버전 `0.1.0`과 C11.4/72/213 상태는 변경하지 않았다. 최종 runtime metadata 두 개의 dirty-source SHA-256은 같다. 검토 한계는 실행 근거 README와 같다.
