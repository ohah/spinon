# C11.4 positive percentage auto-repeat minimum PR diff 검토 20개

계획 revision 검토 및 구현/runtime 실패 관점과 별도로, 구현 커밋 `d2b4826e`와 PR evidence diff를 계층 간 일치·범위 누출·근거 오류 기준으로 검토했다. 이 문서는 자기 자신을 제외한 PR 변경을 대상으로 한다.

| # | 실패 관점 | 확인 결과와 근거 |
| --- | --- | --- |
| 1 | 제품 기본 코드가 fixture전용 scene builder를 참조하는가 | 새 FFI 모듈은 기존 `c04-runtime-gpu-fixture-test`로 게이트되는 C11.4 fixture module 안에서만 등록된다. 기본 archive marker scan은 새 ID를 찾지 못했다. |
| 2 | computed style 범위가 opt-in track-sizing profile 밖으로 넓어지는가 | cascade 변경은 기존 `auto_repeat_track_has_fixed_minimum`의 제한된 track-sizing profile 내부 한 arm이며 일반 profile을 바꾸지 않는다. |
| 3 | source syntax를 computed typed value처럼 취급하는가 | Calc zero term 제거 후 `Percentage(0.1)`이 되는 Stylo 관측을 근거로 computed subset을 고정한다. authored calc와 직접 percentage가 구별된다고 문서화하지 않는다. |
| 4 | 기존 LengthPx 또는 supported Calc 처리가 회귀하는가 | 기존 match arms와 definite Calc 조건은 그대로 남아 있고 기존 calc focused tests 12개가 함께 통과한다. |
| 5 | zero/bare percentage나 unsupported maximum이 허용되는가 | positive `MinMax` minimum만 추가한다. 0%, bare %, auto/max-content/% maximum은 fail-closed 회귀로 고정했다. |
| 6 | maximum fixed length와 flex maximum을 중 하나 빠뜨리는가 | cascade 및 layout match arm은 `LengthPx`와 `Fraction` 모두 받으며 고정 12px 대조 장면이 별도로 있다. |
| 7 | layout validator가 existing `definite_axes`의 calc 의미를 바꾸는가 | 새 `AutoRepeatAxisContext`는 style과 기존 axis definite boolean을 묶어 전달한다. Calc branch는 이전 boolean 및 지원 calc 검사를 그대로 사용한다. |
| 8 | columns/rows에서 content dimension·padding·border가 뒤섞이는가 | helper는 `grid-template-columns`와 rows를 분기해 각 축의 dimension 및 면을 읽는다. 양 축의 simulator frames/traces가 맞는다. |
| 9 | BorderBox fixed CSS size를 content basis로 착각하는가 | 지정 border-box size에서 같은 축 padding/border를 빼고 content size를 쓴다. ContentBox/BorderBox fixed control layout 회귀가 있다. |
| 10 | layout support가 clamp나 indefinite resolution에 의존하는가 | minimum fixed zero, maximum none, positive fixed dimension을 요구한다. zero, cyclic, intrinsic, indefinite, nonpositive content basis는 Taffy 진입 전 거부한다. |
| 11 | 비유한 입력이나 `p×basis` overflow가 track allocator에 도달하는가 | finite/positive 조건이 명시되어 있고 `f32::MAX` multiplication case에서 fail-closed한다. |
| 12 | layout tests가 계산값만 확인하고 repeat/collapse를 놓치는가 | 10개 track, auto-fit 9 collapse, item frame, ContentBox/BorderBox 및 양 축을 비교한다. |
| 13 | overflow fixture가 root outer border 기준의 초과량을 잘못 계산하는가 | 180px item이 124px border box를 58px 넘는 값이 Rust frame assertion, JSON expected frame, 두 simulator 결과에서 같다. |
| 14 | JSON fixture와 FFI runtime JS source의 node/style topology가 다른가 | JSON의 root/item style을 source builder가 그대로 넣고, 정적 scene ID catalog와 unknown-ID rejection test가 경로를 고정한다. |
| 15 | Android/iOS scene allowlist가 JSON 또는 capture matrix와 달라 일부 장면이 빠지는가 | capture helper가 양 플랫폼 source allowlist를 읽고 16개 JSON ID 전부 포함되는지 검사한다. Bun boundary test 19개가 통과한다. |
| 16 | capture matrix 전체 수가 fixture 추가분과 맞지 않는가 | 기존 C11.4 348개에 새 16개를 더해 364개를 검사하며 중복 ID와 미등록 선택 ID도 거부한다. |
| 17 | native harness 내부 기대 frame이 없어 로그만으로 성공을 보고하는가 | Android/Swift 양쪽에 root, item, node frame, box count를 추가해 검사한다. 첫 Android 실패가 누락을 잡았고 보완 후 두 platform 각 16/16 재실행했다. |
| 18 | 성공적인 layout trace만 있고 draw/present를 빠뜨리는가 | 최종 결과 JSON과 screenshot이 Android AVD와 iOS Simulator 각각 16개 전부 `drawPresented=true`를 기록한다. screenshot은 border overflow만 보여 주며 pixel equality는 주장하지 않는다. |
| 19 | 실행 근거가 source tree, target 기기 또는 앱과 연결되지 않는가 | 두 `simulator.json` 모두 commit/tree `d2b4826e`/`b5b0116a`와 clean tracked diff를 기록한다. Android API 37 ARM64 AVD 및 iOS 26.2 iPhone 17, 앱 SHA-256을 명시한다. |
| 20 | 부분 구현 PR을 전체 C11.4 완료로 승격하거나 미검증 범위를 숨기는가 | 계획·README·PR 설명은 fixed definite positive percentage subset으로 한정하고, padding/gap, 일반 CSS, WPT, 실기기 및 compositor pixel 범위는 주장하지 않는다. 공식 C11.4 checkbox와 `72/213`은 후속 merge-status 문서 PR 전까지 유지한다. |

**검토 결과:** diff에서 추가 수정이 필요한 불일치는 찾지 못했다. 계획 revision 20개, 구현/runtime 20개, PR diff 20개 검토는 각각 분리 기록했다.
