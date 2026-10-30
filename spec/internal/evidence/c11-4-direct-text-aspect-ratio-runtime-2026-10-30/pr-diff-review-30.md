# C11.4 direct-text aspect-ratio min/max 구현 PR diff 검토 30개

검토 대상은 PR #850의 전체 `main...HEAD` 변경이다. 구현/runtime 20개 검토와 별도로, adapter·projection·공통 Taffy 변경, Android/iOS validator와 capture, evidence 및 공식 상태 diff를 실패 관점 30개로 대조했다. 기존 최종 테스트·시뮬레이터 receipt를 확인했으며 이 문서 작성 과정에서 테스트를 새로 실행하지 않았다.

| # | 독립 실패 관점 | 결과 |
| --- | --- | --- |
| 1 | 기본 build나 다른 profile이 새 fixture API에 의존하는가 | 통과. 새 public fixture entrypoint는 `c11-4-grid-track-sizing-fixture-test`에 한정하고 runtime fixture route는 opt-in C11.4 모듈에 등록한다. |
| 2 | 일반 layout projection에서 min/max 처리 범위가 의도치 않게 넓어지는가 | 통과. CSS admission은 computed `RuntimeBlockFlexGridTrackSizingV1` projection과 고정 direct-text collector로 제한한다. |
| 3 | 임의 profile이 min/max ratio gate를 호출할 수 있는가 | 통과. projection은 profile을 검사하고 정확한 owner NodeId가 없으면 전용 게이트를 열지 않는다. |
| 4 | ratio owner가 2개 이상일 때 임의 하나를 선택하는가 | 통과. snapshot의 typed ratio owner가 하나이고 요청 owner와 같아야 한다. |
| 5 | min/max 제약이 없는 다른 ratio owner까지 허용되는가 | 통과. ratio owner와 제약 owner가 모두 정확히 한 개이며 같은 node여야 한다. |
| 6 | 제약을 두 번째 `A` item 또는 임의 text item에 적용하는가 | 통과. collector는 min/max를 `AAAA`에만 허용하고 다른 직접 text 후보는 profile 검사에서 거부한다. |
| 7 | 제한 scene 외 구조나 text source가 같은 profile을 통과하는가 | 통과. 고정 두-item `AAAA`/`A`, Ahem 및 root 조건 이외는 기존 fail-closed collector가 거부한다. |
| 8 | 숫자 ratio와 `auto <ratio>`의 typed basis가 뒤바뀌는가 | 통과. computed serialization, 유한 `2.0` typed ratio, box-sizing/content-box basis의 조합을 각각 검사한다. |
| 9 | computed 문자열만 바꾸거나 typed ratio만 바꿔 gate를 우회하는가 | 통과. 양쪽 표현이 일치하지 않으면 지원 ratio가 되지 않는다. |
| 10 | percentage, `calc()`, 음수, 비유한 값, 60px 초과가 고정 px로 허용되는가 | 통과. collector와 projection gate가 유한 `0..=60px`만 허용한다. |
| 11 | no-ratio min/max control이 반대 축 ratio 전이를 받는가 | 통과. 전용 ratio owner가 없고 두 축 control은 독립 scene 및 expected frame으로 확인된다. |
| 12 | numeric ratio 제약을 content-box 값으로 잘못 바꾸는가 | 통과. 숫자 ratio는 CSS `box-sizing` basis를 유지하고 대응 border-box scene의 독립 기대값이 있다. |
| 13 | `auto <ratio>` border-box authored constraint를 content-box 환산 없이 쓰는가 | 통과. 전용 projection에서만 edge 합을 빼고, ratio basis는 content box로 보존한다. |
| 14 | padding/border가 min/max 또는 used frame에 누락·중복되는가 | 통과. 양축 edge 10px/6px을 포함한 numeric 및 auto-ratio controls를 oracle과 대조한다. |
| 15 | edge가 constraint보다 커서 음수 내부 크기가 생기는가 | 통과. content-box 변환은 0으로 바닥 처리하고 CSS 음수 min/max 입력은 collector가 거부한다. |
| 16 | aspect ratio 전이가 지정 size가 있는 축까지 적용되는가 | 통과. Taffy helper는 preferred size의 auto 여부에 따라 transfer하며 2축 직접 입력은 transfer하지 않는다. |
| 17 | transferred min/max 교차 축 우선순위가 뒤집히는가 | 통과. destination direct max가 transferred min을 제한하고 destination min이 transferred max를 floor하는 대조 scene이 있다. |
| 18 | 같은 축 `min > max`에서 max가 min을 덮는가 | 통과. width·height 각각의 충돌 scene에서 min 우선과 Chrome geometry를 고정한다. |
| 19 | min-content와 max-content contribution 경로가 다르게 처리되는가 | 통과. 두 Grid contribution call site가 공통 ratio/minmax 경로를 호출하며 runtime trace의 두 축 stage를 검사한다. |
| 20 | 이미 알려진 축 dimension 때문에 intrinsic content contribution이 사라지는가 | 통과. known-dimension 빠른 경로에서도 non-replaced ratio content contribution을 보존하도록 양쪽 경로를 연결한다. |
| 21 | 공통 Taffy 변경이 replaced item ratio 계산을 손상시키는가 | 통과. 기존 replaced-image 양축 transferred-size regression 1/1 및 nested text/replaced regression 1/1의 결과가 README에 기록돼 있다. |
| 22 | non-replaced text에 replaced 전용 transferred suggestion을 적용하는가 | 통과. 변경 경로는 non-replaced content-size contribution이며 기존 replaced 전용 경로는 별도로 유지한다. |
| 23 | text min-content floor와 명시적 `min-width:0` control이 합쳐져 버리는가 | 통과. `max-height`만 둔 scene과 `min-width:0`을 추가한 scene은 별도 oracle과 기대 frame을 갖는다. |
| 24 | continuous px 구간이 일부 고정 정수 사례에만 맞춰져 있는가 | 통과. `45px`와 `17.75px` fractional Chrome 관측을 추가해 연속 범위 경계를 확인하고 두 플랫폼 matrix에 포함한다. |
| 25 | FFI가 임의 scene 문자열을 신규 runtime fixture로 해석하는가 | 통과. 19개 ID만 table에서 찾아 등록하고 미등록 scene은 `None`이다. |
| 26 | Android scene allowlist 또는 expected geometry가 누락되는가 | 통과. 고정 19 scene 기대값, input/style constraints, row/column trace 검사와 실패 식별 로그가 추가됐다. |
| 27 | iOS validator가 Android와 다른 scene·box basis를 승인하는가 | 통과. 같은 19개 ID와 constraint 표를 사용하고 box-edge conversion 및 trace 검사를 별도 수행한다. |
| 28 | 캡처 좌표·화면 readback이 기기나 compositor 증거로 과장되는가 | 통과. receipt는 Android API 37 QEMU/software renderer와 PixelCopy, iOS Simulator의 pre-compositor Metal drawable을 구분하고 129개 표본만 주장한다. |
| 29 | 문서·PR 화면 링크가 다른 scene을 가리키거나 증거 범위를 부풀리는가 | 통과. README와 PR은 min-width 45px 및 fractional max-height 장면을 연결하며 glyph paint·실기기·hardware GPU·전체 WPT는 범위 밖으로 둔다. |
| 30 | 공식 C11/C11.4 상태나 완료 집계가 부분 구현으로 올라가는가 | 통과. 두 checkbox를 미완료로 유지하며 `72/213`을 그대로 둔다. |

**판정:** 현재 diff에서 병합을 막는 결함은 발견하지 못했다. 기존 기록은 두 simulator matrix와 로컬 검증 성공을 증명하지만 GitHub PR #850의 check rollup은 비어 있다. 이를 CI 성공으로 표시하지 않는다. 공식 C11/C11.4 완료 판정과 merge 상태는 이 검토 범위와 별도다.
