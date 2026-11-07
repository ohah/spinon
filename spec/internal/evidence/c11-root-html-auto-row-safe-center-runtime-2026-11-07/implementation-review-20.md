# C11.4 safe/unsafe center 구현·runtime 실패 관점 검토 20개

대상은 style projection, 두 Root HTML Grid layout validators, Taffy mapping boundary, direct Layout/RuntimeSession tests, Android/iOS simulator outputs다. 각 항목은 별도 입력·불변 조건으로 확인했다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | 허용 문자열을 `safe center`와 다르게 직렬화하거나 토큰 순서를 바꿈 | allowlist는 정확한 computed 문자열 두 개만 연다. RuntimeSession positive/overflow matrix에서 Stylo→projection→layout을 통과한다. |
| 2 | `safe` modifier를 typed `AlignmentSafety::Safe` 대신 plain center로 지움 | Layout frame matrix가 overflow에서 0px start offset을 확인한다. |
| 3 | `unsafe` modifier를 Safe 또는 기본/누락 modifier로 바꿈 | Layout frame matrix가 overflow에서 -41px center offset을 확인한다. |
| 4 | positive free space에서도 safe/unsafe가 서로 다른 정렬을 적용 | positive 3값 결과가 모두 `[5,15,25]px`다. |
| 5 | `safe center` overflow 방향을 end로 처리 | 명세·독립 계산의 start fallback `[0,56,112]px`와 대조했다. |
| 6 | `unsafe center` overflow를 start로 강제 | 기대 `[−41,15,71]px`를 Layout, RuntimeSession에서 검증했다. |
| 7 | 기존 plain `center` overflow 회귀를 safety candidate의 결과로 대체 | 기존 calc-gap suite를 다시 실행했고 Layout/RuntimeSession 결과가 통과했다. |
| 8 | alignment subject에서 gutters를 빠뜨려 overflow 여부를 다르게 판정 | trace는 두 46px gutter와 세 10px rows를 별도로 확인한다. |
| 9 | safe fallback 중 auto rows 자체를 stretch하거나 줄임 | 6 scene trace의 used row sizes는 모두 `[10,10,10]px`다. |
| 10 | percentage basis를 width, viewport 또는 indefinite axis에서 계산 | fixed 40px definite block size와 calc-gap 경계는 기존 profile 및 direct input으로 유지한다. |
| 11 | item topology가 두 direct auto item이 아닌 상태에서 align-content를 허용 | fixed item + A/B source order 및 정확한 세 direct item 조건을 유지한다. |
| 12 | hidden/extra item 수가 fixed profile 검사에서 빠짐 | existing visible item-count gate는 2 또는 3으로 유지되고 이 fixture는 3개다. |
| 13 | 새로운 CSS grammar나 public admission으로 범위가 커짐 | CSS parser, runtime profile, FFI와 public API 변경이 없다. |
| 14 | legacy `RootHtmlIntrinsicTrackSizingV1`에서 safe alignment가 우연히 허용 | 공통 helper는 auto-row profile에서만 두 safety 값을 인정한다. |
| 15 | 두 layout validation 경로의 allowlist가 달라져 한 경로만 통과 | intrinsic Grid validator와 one-axis placement validator가 같은 helper를 호출한다. |
| 16 | safe modifier가 start/end 또는 distributed keyword로 확장됨 | `safe start`, `unsafe end`, `unsafe flex-start`, distributed safety는 rejection 목록에 남는다. |
| 17 | baseline alignment가 center 지원으로 포함 | baseline variants는 style projection에서 거부한다. |
| 18 | unsupported style rejection 뒤 이전 complete frame을 부분 대체 | RuntimeSession 실패 장면은 `completed=None`을 확인한다. |
| 19 | 환경 변경 뒤 valid profile로 돌아오지 못함 | 같은 session light→dark rejection→light recovery가 Ready로 돌아온다. |
| 20 | Simulator screenshot을 surface readback이나 음수 frame의 직접 증거로 과장 | PNG는 색 경계만 보고한다. unsafe 첫 fixed item은 status area에 겹쳐 색 bounds가 없고 음수 좌표는 Layout/RuntimeSession assertions가 증명한다. |

판정: blocking finding 없음. 첫 검증에서 profile별 alignment admission과 one-axis placement validator가 서로 달라진 점을 발견해 shared helper로 통합한 뒤 focused test를 통과했다.
