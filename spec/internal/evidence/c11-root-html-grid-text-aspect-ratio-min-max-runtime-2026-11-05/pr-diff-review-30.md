# C11.1 Root HTML `aspect-ratio` min/max PR diff 적대 검토 30개

**검토 대상:** 현재 구현 변경 전체와 사전 비교·Android/iOS runtime evidence 문서. 계획 자체의 20/30 관점 및 구현/runtime 20개 검토와 중복해서 완료 처리하지 않았다.

| # | 변경 diff에서 확인한 실패 경로 | 판정 |
|---:|---|---|
| 1 | 제한된 구현이 C11.1 또는 C11 전체 완료로 과장되는가 | 통과 — README와 공식 집계는 `78/214` 및 미완료 상태를 유지한다. |
| 2 | 기본 Root HTML 외 style profile에도 min/max 예외가 열리는가 | 통과 — projection 예외는 `RuntimeRootHtmlTextPaintV1`로 제한된다. |
| 3 | opt-in C11.4 fixture 경로를 기본 Root HTML 증거로 대체하는가 | 통과 — runtime test는 `new_runtime_gpu_with_root_html` 기본 경로를 사용한다. |
| 4 | min/max 허용이 ratio 후보 검증보다 먼저 적용되는가 | 통과 — 허용 후에도 기존 고정 owner와 target candidate guard를 통과해야 한다. |
| 5 | owner 자신의 min/max가 target 예외로 들어오는가 | 통과 — owner dimension guard는 unchanged이며 tests reject it. |
| 6 | 첫 direct text sibling의 min/max가 허용되는가 | 통과 — short sibling의 min/max는 거부되고 layout의 예외는 두 번째 child만 본다. |
| 7 | 다른 NodeId가 두 번째 item의 제약을 빌려가는가 | 통과 — projection target identity와 layout child identity를 모두 확인한다. |
| 8 | 음수, 비유한 수 또는 70px 초과가 fixed px로 수용되는가 | 통과 — 값은 finite `0..=70px`로 제한된다. |
| 9 | percentage, `calc()` 또는 intrinsic keyword가 px처럼 처리되는가 | 통과 — typed variant와 `layout_math_values`를 함께 검사하고 fail-closed controls가 있다. |
| 10 | 초기 `auto`/`none`이 명시적 0px와 합쳐지는가 | 통과 — computed defaults와 typed dimensions를 보존하고 0px를 별도 값으로 처리한다. |
| 11 | constrained ratio가 임의 값 또는 serialization에만 의존하는가 | 통과 — computed `2 / 1`과 typed `2.0`이 모두 요구된다. |
| 12 | ratio sizing basis가 기존 box-sizing과 어긋나는가 | 통과 — constrained transfer에서 typed `BoxSizing` basis를 요구한다. |
| 13 | ratio 없는 positive control에 비율 전이가 적용되는가 | 통과 — ratio-free min/max scenes가 별도 기대 크기를 확인한다. |
| 14 | 잘못된 ratio가 min/max gate를 통해 우회되는가 | 통과 — 3:2 constrained negative case를 거부한다. |
| 15 | 두 sizing 방향 중 한 축만 constraint를 검증하는가 | 통과 — width-definite height clamp와 height-definite width clamp를 모두 둔다. |
| 16 | `min > max` 처리 순서가 반대로 되는가 | 통과 — 양 축의 contradictory min/max scene은 min 우선을 확인한다. |
| 17 | explicit `min-height`가 automatic text minimum으로 덮이는가 | 통과 — helper는 초기 `auto`일 때만 automatic minimum을 쓴다. |
| 18 | explicit `min-width`가 paragraph intrinsic minimum으로 덮이는가 | 통과 — 해당 helper에도 초기 `auto` guard가 있다. |
| 19 | 최대 크기 적용 후 intrinsic text가 이전 자동 최소 크기를 누출하는가 | 통과 — positive/negative sizing matrix와 bounded intrinsic control을 함께 검사한다. |
| 20 | grid constraint validation이 다른 placement mode에 영향을 주는가 | 통과 — 예외는 `RootHtmlDirectTextAspectRatioV1`와 두 번째 child에 한정된다. |
| 21 | layout/runtime test가 root 위치나 자동 row 누적을 놓치는가 | 통과 — 10 scene의 root frame, target-relative position, row height를 검사한다. |
| 22 | fixture 수정이 unrelated CSS shorthand 동작을 전제로 하는가 | 통과 — 기본 profile이 지원하는 explicit longhand를 사용하며 fixture hash를 고정한다. |
| 23 | Android pixel capture가 다른 실행에서 새 sampling 동작을 활성화하는가 | 통과 — viewport 전환과 세 표본은 고정 run ID에만 적용된다. |
| 24 | Android 표본이 visual viewport와 layout viewport를 혼동하는가 | 통과 — 해당 run ID에 980px layout width와 surface 비율 높이를 적용한다. |
| 25 | Android 표본이 item overflow 경계의 색을 판별하지 못하는가 | 통과 — track 내부·Grid 밖 item 내부·item 바깥 세 점을 exact RGBA로 검사한다. |
| 26 | iOS Metal capture가 다른 run ID에 scene을 잘못 부여하는가 | 통과 — `C111RMX` marker와 정확한 run ID가 opt-in feature 내부에서 확인된다. |
| 27 | iOS sample 좌표가 UIKit visible viewport 기준으로 계산되는가 | 통과 — layout 폭과 drawable 비율을 이용하며 viewport mismatch 실패 기록이 있다. |
| 28 | Metal drawable 결과를 compositor 최종 출력으로 과장하는가 | 통과 — README가 compositor 제출 전 readback이라고 명시한다. |
| 29 | Android/iOS 앱에 다른 fixture가 들어가거나 bootstrap 수정이 남는가 | 통과 — artifact hash가 같은 fixture/root-app source를 기록하고 runner가 원본 bootstrap hash 복원을 확인한다. |
| 30 | fixture-test/readback 옵션이 기본 앱 빌드에서 빠지는 경계를 놓치는가 | 통과 — Android/iOS 기본 feature-off 빌드와 fixture-enabled positive run을 각각 기록한다. |

**결과:** 검토 대상 diff에서 수정해야 할 기능 결함은 발견되지 않았다. 전체 workspace test, 실제 Android 기기, hardware GPU, iOS compositor 이후 화면, WPT 또는 일반 CSS Grid 지원은 검증하지 않았다.
