# C11.4 one-fit row-span mixed minimum · 구현/runtime 실패 관점 20개

**대상:** style projection, Root HTML Grid layout admission/normalization, 1/64px track trace 복원, 기본 RuntimeSession mixed matrix. 구현 전 계획 검토 20개와 구분해 실제 코드를 대조했다.

| # | 실패 관점 | 코드·실행 대조와 판정 |
| ---: | --- | --- |
| 1 | 지원 경계가 one-fit 외 track list로 확장됨 | Style projection은 기존 six-member list의 fit 위치가 하나인 경우만 새 minimum mask predicate에 전달한다. Layout도 six-member span list와 one-fit count를 확인하며 0/two-fit 및 다른 fit count 회귀를 따로 둔다. |
| 2 | three-row와 six-row profile의 위치 허용이 뒤섞임 | 기존 `repeated_span_fit_positions_are_supported`가 three-row `1–3`, six-row `4–6`을 구분한다. one-fit positions 단위 검사를 추가했고 5/6 fit endpoint를 포함한 기존 profile 회귀도 실행했다. |
| 3 | 첫 explicit-row empty control이 `auto` minimum을 받음 | Projection과 layout 모두 첫 visible item에 computed/layout `min-height:0`을 요구한다. RuntimeSession은 control의 zero minimum과 frame을 각 scene에서 확인한다. |
| 4 | span item의 0/auto 밖 최소 크기가 통과함 | 지원 predicate는 각 후속 item을 `0px|auto`로 제한한다. `1px`, `min-content`, 자동 control과 position 4의 rejection을 확인하고 same-session recovery를 보존했다. |
| 5 | minimum mask가 DOM 순서 변경에 따라 다른 item에 붙음 | mask는 Chrome observation의 semantic item ID와 연결하고 forward/reverse topology 및 source-order pair를 전 scene에서 비교한다. |
| 6 | 미지원 span 배치가 topology matcher에 섞임 | layout matcher는 exact three-row 및 six-row placement multisets와 column lines를 요구한다. 추가·누락·변형 placement는 기존 rejection 경로에 남는다. |
| 7 | typed cap, tail 순서, fit 위치가 sizing trace와 불일치 | fixture는 900개 고유 six-member list를 순회하고 computed track order/cap과 owner-scoped row trace를 대조한다. 120,960 scene 전체가 통과했다. |
| 8 | indefinite percentage, calc 또는 음수 cap을 새 predicate가 수용함 | replay는 고정 definite grid height와 직접 px/percentage cap만 계산하고 `calc()`를 거부한다. 기존 cap/position negative checks를 유지했다. |
| 9 | scrollable overflow 또는 flexible track에서 auto minimum 예외가 누락됨 | Runtime trace 검사는 `crosses_flexible_track`을 거부하고 contribution이 `spans_flexible_max_track:false`, `spans_auto_min_track:true`인지 확인한다. 입력 profile은 visible overflow로 제한된다. |
| 10 | auto minimum을 eligible하지 않은 item에도 적용함 | 실제 row pass의 item별 minimum contribution에서 content-based automatic-minimum provenance를 요구하고, auto item의 computed minimum 및 Chrome geometry를 비교한다. eligible occurrence는 169,344개다. |
| 11 | stale/다른 revision trace를 현재 layout에 결합함 | completed layout과 cascade snapshot의 key equality, exact grid owner, 최신 row pass sequence 및 owner-scoped trace를 scene마다 확인한다. |
| 12 | 재생한 sizing phase가 Taffy stage와 어긋남 | 각 span batch의 minimum, content-based minimum, max-content minimum, intrinsic maximum, max-content maximum 단계를 적용한 뒤 stage base size를 trace와 대조한다. 불일치는 지원하지 않는 입력으로 반환한다. |
| 13 | 1/64px 양자화가 전체 row sum을 바꿈 | Mixed one-fit 경로는 target total과 trace-matching simulation/fallback을 확인한다. focused residual scene 110과 전체 matrix를 통과했다. |
| 14 | residual fallback이 특정 row로 임의 이동하거나 stretch 합을 깨뜨림 | phase replay를 우선하고 residual allocator는 같은 total과 final trace를 만족할 때만 사용한다. singleton auto tail stretch는 non-auto rows를 보존하는 residual 판정을 별도로 거친다. |
| 15 | fit-content cap이 auto minimum content floor를 잘라냄 | 각 item의 line count와 auto-min contribution value를 비교하며 payload 높이가 text content floor 미만이 되지 않도록 검사한다. Chrome precomparison도 eligible item 169,344회에서 위반 0을 기록했다. |
| 16 | item frame은 맞지만 payload 또는 text line이 잘림 | item 및 nested payload frame, runtime glyph baseline의 고유 line 수를 Chrome 관측과 대조한다. 120,960 scene 모두 통과했다. |
| 17 | Grid content edge 밖 overflow를 성공으로 오인함 | Chrome payload frame이 Grid content edge를 넘으면 실패하고 runtime payload frame은 oracle frame에 0.01px 허용오차로 결합된다. 현재 고정 fixture에서는 oracle overflow 표본이 0이며 이를 overflow 재현으로 주장하지 않는다. |
| 18 | `align-content:stretch`가 non-auto row나 source-order 결과를 바꿈 | start/stretch와 forward/reverse pair를 matrix에서 비교한다. fit-content/non-auto rows, per-item frames, track list order 모두 oracle과 일치한다. |
| 19 | unsupported input rejection이 부분 layout을 공개하거나 이후 revision을 막음 | malformed minimum/position rejection 뒤 completed layout이 없고 revision/frame이 바뀌지 않는지 확인한 뒤 동일 RuntimeSession의 유효 입력 복구를 검사했다. |
| 20 | 잘못된 trace의 missing owner/index가 panic 또는 잘못된 성공이 됨 | missing/out-of-range grid index를 `UnsupportedGridContainerStyle`로 반환하도록 보강했다. 이전 2개 invalid-trace panic이 재현되던 56개 layout 회귀는 수정 후 전부 통과했다. |

## 발견해 수정한 항목

- Invalid trace test에서 layout node index를 직접 조회하다 panic하던 경로를 fail-closed 오류 반환으로 바꿨다. 관련 회귀 56/56 통과.
- Shared projection helper가 지원하는 five-fit row position에 대한 기존 단위 테스트 기대값이 낡아 있었다. 제품 동작은 유지하고 다섯·여섯 position 검사를 함께 표현하도록 테스트를 고쳤다. 관련 projection 5/5 통과.
- 전체 matrix의 한 장면이 5초 내 `Pending`으로 돌아온 실행기 한계를 확인했다. 해당 scene 단독 재현은 1.78초에 통과했고, per-scene wait를 30초로 늘려 matrix 120,960/120,960을 완료했다. 기록은 [matrix wait 회귀](runtime-matrix-timeout-regression.md)에 둔다.
- 새 matrix 검증 모듈 747줄을 source 생성, scene 검증, batch/test 책임으로 분리해 각 Rust 파일을 500줄 이하로 유지했다.

## 실행 근거

- [전체 RuntimeSession matrix](runtime-matrix-result.md)
- [mixed acceptance, rejection/recovery, malformed trace 로그](focused-mixed-regressions.log) · [positive](positive-acceptance.log) · [rejection](rejections-recovery.log) · [negative](malformed-negative.log)
- [layout 회귀 56/56](layout-root-html-fit-content-tests-rerun.log) · [projection 회귀 5/5](style-projection-tests-rerun.log)
- [module split 이후 focused RuntimeSession test](mixed-module-split-regression.log)
- [Android/iOS visual smoke와 정리 상태](platform-visual-smoke.md)

**판정:** 위 제한 fixture profile에서 발견한 fail-closed 결함을 고치고 전체 scene matrix를 통과했다. 이는 C11.4/C11 전체 완료, 일반 CSS Grid 지원, 실기기 검증 또는 surface pixel parity 판정이 아니다. PR 병합 전 별도 실제 diff 30관점 검토와 CI/merge 상태 확인이 남아 있다.
