# 구현 PR diff 실패 관점 30개

검토 대상은 six-fit row-span 구현, five-fit simulator 정리, 내부 계약·계획·상태 대장 및 실행 증거다. 각 항목은 서로 다른 경계나 실패 경로를 확인한다. 코드를 실행한 증거와 정적 diff 확인은 따로 표시했다.

| # | PR diff에서 확인할 실패 경로 | 확인 결과 |
|---:|---|---|
| 1 | 새 style predicate가 auto-placement의 six-fit admission을 넓혀 버리는가 | Row-span 전용 module과 호출 지점만 추가했다. 기존 auto-placement predicate는 변경하지 않았다. 통과. |
| 2 | Six-fit cap 순서 중 일부가 새 predicate에서 누락되는가 | Style·layout tests가 720개 typed cap order를 모두 통과했다. 통과. |
| 3 | Cap multiset이 중복·누락인데 길이 6이라는 이유로 허용되는가 | `caps_seen` 배열의 중복 검사와 정확한 multiset 검사를 확인했다. Invalid unit cases 통과. |
| 4 | 20% cap의 typed Percentage가 같은 숫자의 px로 혼동되는가 | Style과 layout은 Percentage(0.2) 위치를 별도로 구분하고 cap order tests에 포함한다. 통과. |
| 5 | Unsupported direct cap, Calc, negative 또는 non-finite 값이 exact profile에 섞이는가 | Exact-match branches 외 입력은 즉시 거부한다. Layout/style invalid tests 및 RuntimeSession rejection 통과. |
| 6 | Parsed CSS projection이 layout profile과 서로 다른 six-fit 목록을 허용하는가 | 두 경계의 typed profile을 각각 검증하고, 720 order tests가 일치한다. 통과. |
| 7 | Six fit-position set이 six rows가 아닌 span profile에서 재사용되는가 | Style helper는 positions `[1,2,3,4,5,6]`와 positive implicit rows 6을 함께 요구한다. Boundary tests 통과. |
| 8 | Layout track-list broad gate가 exact six-fit predicate를 건너뛰는가 | Exact six-fit predicate를 five-fit span logic보다 먼저 확인하고 6 implicit rows에서만 통과시킨다. 통과. |
| 9 | Mixed `min-height`나 visible control child가 all-auto span item과 섞이는가 | 모든 active span child의 uniform minimum 후보와 `min-height:0` control 조건을 확인한다. Mixed-profile runtime rejection/recovery 통과. |
| 10 | Explicit row 1과 implicit rows 2–7의 index가 한 칸 밀리는가 | Trace test가 explicit 1 + positive implicit 6 = 7 used rows 및 typed cap 위치를 대조한다. 통과. |
| 11 | Percentage cap basis가 viewport나 auto-height로 잘못 결정되는가 | Runtime trace의 definite 220px height와 44px growth limit을 확인한다. Height auto 변이는 거부했다. 통과. |
| 12 | Stage replay가 다른 Grid owner 또는 이전 layout pass를 골라 쓰는가 | Trace node 유일성, row axis 최신 sequence, initialize stage와 revision-bound contribution source를 확인한다. 통과. |
| 13 | Gutter track가 user-visible row로 세어져 span range를 바꾸는가 | Nonzero gutter 상태는 거부하고 visible track 수가 정확히 7인지 검증한다. 통과. |
| 14 | Contribution의 node placement line/span이 owner trace와 다르게 매핑되는가 | 각 contribution item의 start line, end line, span 길이와 track count를 검증한다. 통과. |
| 15 | Minimum/content-based/max-content/intrinsic-maximum phase 순서가 뒤섞이는가 | 다섯 contribution phase를 분리 적용하고 각 batch 뒤 stage trace와 대조한다. Runtime trace/matrix 통과. |
| 16 | Fit-content growth limit 또는 infinite potential 처리로 limit을 넘기는가 | Direct length/definite percentage limit을 fixed units로 적용하고 final growth state를 대조한다. Percentage trace와 전체 matrix 통과. |
| 17 | Extra space가 bounded track들에 잘못 나눠지거나 residual 1/64px가 DOM 순서에 의존하는가 | Phase distribution은 potential 순서와 capped share를 재현한다. 720 cap orders의 forward/reverse frame invariants 통과. |
| 18 | Flexible track 또는 flexible-crossing contribution이 replay에 포함되는가 | Track parser는 unsupported type을 거부하고 intrinsic batch가 flexible track을 가로지르면 fail-closed한다. Runtime flexible-track rejection 통과. |
| 19 | Simulator가 Taffy trace와 어긋나도 six-fit에서 final f32 rows로 fallback하는가 | Six-fit replay `None`은 `None`으로 fail-closed한다. Five-fit fallback 분기는 보존된다. 정적 diff 확인 통과. |
| 20 | Generic simulator rename 중 기존 five-fit regression이나 import가 끊기는가 | Five-fit non-matrix regression 9/9 통과, workspace Clippy 및 format 통과. 통과. |
| 21 | Final track count만 맞고 intermediate base/growth states는 다른 계산이 통과하는가 | 각 intrinsic item batch stage를 0.02px 이내로 대조하고 final pre-alignment state도 대조한다. 전체 matrix 통과. |
| 22 | Row rounding 결과와 original unrounded Taffy track frame이 불일치하는가 | Snapped units가 final track base와 맞는지 확인하고 mismatch 시 fail-closed한다. Six-fit full matrix 통과. |
| 23 | Oracle file의 scene pairing이 baseline/candidate 사이 다른 style/topology를 포함하는가 | 7개 scene identity field를 pair별로 비교한다. Metadata 11,520 scenes / 720 orders 통과. |
| 24 | Matrix 실행이 cap order나 한 개 scene topology를 빠뜨리는가 | 4 sequential chunks가 각각 180 orders ×16 scenes를 실행했고 4/4 통과했다. 총 11,520 scenes. |
| 25 | Candidate가 payload floor를 맞추지만 baseline pair와 geometry가 같아 실제 변화가 없는가 | 모든 5,760 pair가 geometry change를 보였고 candidate floor violations는 0이다. 통과. |
| 26 | DOM order 또는 `start/stretch` pair가 row/item/payload frame을 바꾸는가 | Matrix group 비교가 rows, item frames, payload frames를 모두 대조했다. 통과. |
| 27 | Invalid revision이 partial layout을 게시하거나 같은 session을 영구적으로 망가뜨리는가 | Atomic rejection은 completed layout이 없음을 확인하고 valid revision recovery를 실행했다. 통과. |
| 28 | 5-fit rejection fixture가 신규 six-fit admission으로 더 이상 invalid가 아니게 되는가 | 기존 negative input을 unsupported 51px cap으로 바꿔 의미를 유지했다. Atomic rejection test 통과. |
| 29 | Simulator screenshot이나 default route smoke를 six-fit fixture geometry로 과장하는가 | Evidence와 PR template에 기본 route/light-dark smoke로 한정하고 fixture, PixelCopy, Metal readback 및 pixel parity 주장을 제외했다. 통과. |
| 30 | 제한 profile을 C11.4 완료, 공식 완료 수 증가, public API 또는 내부 버전 변경으로 해석하는가 | STATUS checkbox 및 78/214, C11.4/C11 incomplete state와 0.1.0을 유지했고 public API/FFI 변경은 없다. 통과. |

최종 push 후에는 PR의 base/head, 파일 목록, 로컬 tree와 원격 PR diff가 같은지도 확인한다. 수정 사항이 생기면 해당 검토 항목을 다시 실행한다.
