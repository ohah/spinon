# C11.4 four-fit row-span implementation/runtime 검토: 20개 실패 관점

**대상:** `codex/c11-4-repeated-text-four-fit-auto-min-20261111`의 제한 profile 구현, Chrome 157 관측과 비교하는 RuntimeSession matrix, rejection/recovery, trace 및 시뮬레이터 smoke. 계획 검토 20개와 계획 PR diff 검토 30개를 재사용하지 않았다. 별도의 구현 PR diff 30개 검토는 실제 PR diff 고정 후 수행한다.

| # | 실패 관점 | 코드·실행 확인과 결과 |
| ---: | --- | --- |
| 1 | 새 예외가 일반 auto-placement 또는 모든 Grid 입력까지 넓어짐 | Style projection과 layout은 기존 repeated-text row-span route 안에 four-fit count 분기를 두며, four-fit auto-placement predicate는 별도다. Style 테스트는 네-fit row-span list를 auto-placement admission에서 거부한다. |
| 2 | list 길이가 6이 아니어도 일부 값만 보고 허용 | Four-fit style predicate가 list 길이 6을 확인하고, layout의 exact fit/tail 개수도 검사한다. Style 단위 테스트는 길이 5를 거부한다. |
| 3 | 네 fit cap이 임의의 값이나 cap 중복으로 통과 | 양 계층 모두 `0px`, `10px`, `20%`, `30px`의 typed cap을 각각 한 번씩 요구한다. Projection/layout 테스트는 중복, 미등록 cap, NaN·calc typed input을 거부한다. |
| 4 | `20%`와 숫자가 같은 px 값을 혼동 | 퍼센트 분기는 `Percentage(0.2)`만 받는다. Runtime matrix는 computed track type/order와 Chrome 관측을 장면별 대조한다. |
| 5 | 네 fit 위치 중 특정 authored 순서만 처리 | 위치 validator가 여섯 row 중 strictly increasing 네 위치를 확인한다. 각 계층 테스트가 15 위치 quadruple × 24 cap order × 2 tail order = 720 profile을 모두 실행한다. |
| 6 | tail에 `max-content`, 중복 `min-content` 또는 `auto` 둘이 허용 | Profile multiplicity는 `min-content ×1`, `auto ×1`이다. Style/layout 단위 테스트가 잘못된 tail 구성을 거부하고 RuntimeSession은 `auto` 누락 mutation을 거부한다. |
| 7 | span이 일곱 번째 positive implicit row까지 확장됨 | Four-fit row-span profile은 positive implicit row 6과 네 span 위치 범위를 함께 요구한다. RuntimeSession mutation으로 span 3을 만들어 일곱 번째 row를 요구하면 atomic failure다. |
| 8 | baseline의 `min-height:0`까지 automatic minimum 보정 적용 | Baseline과 candidate는 같은 matrix key의 서로 다른 style revision으로 실행한다. Baseline floor 결과와 candidate floor를 분리해 비교하고 residual policy도 candidate 여부에 따라 분기한다. |
| 9 | 일부 span item만 `min-height:auto`인 mixed profile 수용 | 모든 span text item의 minimum이 동일한 profile이어야 한다. 다크 color-scheme media query로 한 item만 `0`으로 바꾸면 completed layout 없이 실패하고 원래 환경으로 같은 session을 복구한다. |
| 10 | 첫 explicit-row 빈 control이 자동 최소로 영향을 줌 | Candidate predicate에서 첫 control은 제외해 `min-height:0`으로 고정한다. Runtime 관측은 control과 span item의 frames/payload를 각각 검증한다. |
| 11 | 적격/비적격 automatic minimum 대상이 뒤섞여 floor 집계가 잘못됨 | Runtime matrix는 item마다 eligibility와 payload line count를 검사한다. 25,920 eligible candidate occurrence의 floor 위반은 0이며 baseline under-floor는 별도 3,648 occurrence다. |
| 12 | 한 cap 또는 track 순서가 Chrome 관측 뒤바뀜 | 720 track list 각각을 16 scene(정방향·역방향, start·stretch, zero·auto)으로 실행한다. typed track list와 각 row를 직접 대조한다. |
| 13 | source-order 반전이 다른 item에 frame을 대응 | 각 frame 비교는 Chrome scene의 안정된 item ID를 키로 사용한다. forward/reverse pair의 row sizes와 item frames도 별도 비교하며 의미 geometry 차이는 0 CSS px다. |
| 14 | span item frame은 맞지만 occupied rows 또는 line box가 다름 | Matrix observation helper는 item frame, payload frame, line count, used row와 span 합을 개별 scene에서 확인한다. 허용 오차는 고정 0.01 CSS px다. |
| 15 | fractional residual을 마지막 track에 무조건 더하거나 floor를 넘김 | Rounding test는 candidate auto-minimum의 후속 growable track residual, baseline fractional-auto의 auto-priority ceiling, 그 외 baseline nearest total을 각각 고정한다. Fit cap 상한과 auto track 성장 가능성을 반영한다. |
| 16 | `align-content:stretch`가 non-auto fit/min-content row를 변형 | Matrix는 5,760개 start/stretch pair를 비교하고, authored auto row만 증가할 수 있다고 단언한다. |
| 17 | fractional 여부를 stretch 뒤의 크기로 판단해 경계 장면 오분류 | 분기 입력은 `pre_alignment_sizes`에서 읽는다. 전 stretch 상태의 auto row가 fractional인 회귀 장면에서 expected Chrome frames와 row를 대조한다. |
| 18 | 지원 밖 CSS가 일부 결과를 남긴 채 성공하거나 stale frame을 게시 | 잘못된 cap/tail/count, calc, dense flow, scrollable item, unsupported alignment, indefinite basis는 `Failed`와 `completed=None`을 검사한다. |
| 19 | 거부 뒤 세션이 영구 실패하거나 이전 revision을 다시 게시 | Media mutation 거부 뒤 valid environment로 복귀해 key가 바뀐 completed snapshot과 기대 frame을 같은 session에서 받는다. |
| 20 | Simulator 캡처를 four-fit geometry 검증으로 확대 해석 | iOS/Android 로그는 default Root HTML route의 `layout=ready`, draw/present와 OS 화면만 증명한다. 앱 route는 four-fit fixture를 로드하지 않으며 feature geometry 근거는 11,520-scene RuntimeSession matrix다. Android renderer는 SwiftShader다. |

**결과:** 검토 관점에서 새 결함은 발견하지 않았다. RuntimeSession row-span matrix 11,520/11,520과 four-fit auto-placement regression 5,760/5,760, projection/layout focused tests, trace·rejection/recovery, rustfmt, workspace Clippy가 통과했다. PR diff는 별도 30개 실패 관점으로 검토한다. C11.4/C11 공식 체크는 이 bounded slice만으로 완료 처리하지 않는다.
