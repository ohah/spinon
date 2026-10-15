# C13.3 pure Rust gesture session 구현 실패 관점 20개

검토 대상은 `spinon-render`의 `RuntimeScrollGestureSession`, geometry key, typed input/error 및 focused tests다. 목표는 단일 pointer 상태가 stale snapshot·잘못된 입력·revision 경계에서 offset을 부분 반영하지 않는지 확인하는 것이다. 이 검토는 native callback이나 앱 화면을 검증하지 않는다.

| # | 실패 관점 | 확인과 판정 |
| --- | --- | --- |
| 1 | Begin이 다른 render key를 가리킴 | `snapshot.key()` 비교 전에 상태를 만들지 않는다. stale Begin test에서 `StaleExpectedSnapshot`, `Idle`을 확인했다. 통과. |
| 2 | 시작 좌표에 NaN/무한대 또는 viewport 밖 값 전달 | 좌표 유한성 및 clip-aware `hit_test_css_point`를 검사한다. malformed Begin과 full viewport miss test가 통과했다. |
| 3 | hit point가 clip에 가려져도 뒤쪽 target을 잘못 고정 | snapshot hit test의 반환 target만 사용하고 뒤늦은 Move에서 retarget하지 않는다. clip change cancellation test가 통과했다. |
| 4 | coordinate scale이 0·음수·무한대 | Begin을 거부해 active state를 만들지 않는다. 0 scale test가 통과했고 finite/positive 검사는 source 확인했다. |
| 5 | 아직 cascade가 지원하지 않는 touch-action을 `auto`처럼 해석 | `Unsupported`는 `UnsupportedTouchAction`으로 거부한다. test에서 상태 변화 없이 거부했다. |
| 6 | target이 scroll owner인 경우 target owner를 누락 | geometry key는 target 자체가 owner이면 chain 맨 앞에 추가한 뒤 ancestor chain을 붙인다. owner 순서와 router target은 source 대조를 통과했다. |
| 7 | owner chain이 중복되거나 참조한 owner geometry가 없음 | 중복 NodeId 또는 container 누락은 `InvalidGeometry`로 닫힌다. geometry builder의 set/Option 수집 분기를 확인했다. |
| 8 | gesture/pointer가 다른 Pan sample 유입 | identity 검증이 scroll router 호출 전에 수행된다. sample/terminal identity 분기를 확인했다. |
| 9 | 같은 active pointer의 중복 Begin | 새 gesture를 대체하지 않고 `GestureAlreadyActive`를 반환한다. 동일 sequence 재사용 뒤 원래 session이 계속 작동하는 test가 통과했다. |
| 10 | 두 번째 pointer 좌표/action이 잘못돼도 기존 pan을 계속함 | active 상태를 먼저 취소하고 두 contact를 억제한다. NaN point와 `Unsupported` action인 두 번째 pointer test가 통과했다. |
| 11 | 억제 contact의 임의 큰 gesture ID가 이후 정상 ID를 고갈 | review에서 suppressed contact가 `last_gesture_id`를 갱신해 이후 Begin을 막는 결함을 찾았다. suppressed 입력은 새 gesture ID를 예약하지 않도록 수정하고 `u64::MAX` contact 뒤 정상 ID 복구 test를 추가했다. 통과. |
| 12 | native slop을 넘기지 않은 tracking 입력이 offset을 이동 | Tracking의 `pan_moved`는 delta 없이 sequence만 처리하며 scroll은 `pan_began`에서 시작한다. native recognizer 경계는 adapter 소유로 남겨둔 API 문서와 source를 확인했다. |
| 13 | `none` 또는 제한 방향에서 scroll 발생 | 허용 축 교집합이 비면 `PanRejected`가 되고 offset/revision은 유지된다. `pan-x`의 세로 방향과 `none` test가 통과했다. |
| 14 | 대각선 동률에서 허용되지 않은 축까지 추가 | 동률은 두 축으로 판정한 뒤 시작 action과 교집합한다. `pan-x` 대각선 test는 X만 선택했다. |
| 15 | Panning 진입 뒤 두 번째 PanBegan으로 축을 다시 선택 | `PanAlreadyStarted`로 거부하며 같은 sequence의 유효 Move를 이어서 받아 상태·sequence를 보존한다. test가 통과했다. |
| 16 | cumulative translation 반복 또는 NaN이 두 번 소비됨 | 이전 accepted 누적값의 차이만 전달한다. 반복 값의 no-op/revision 보존과 비 finite 입력 뒤 같은 sequence 재시도 test가 통과했다. |
| 17 | 자체 scroll commit의 revision 갱신 또는 외부 revision 구분 실패 | delta outcome의 snapshot revision을 다음 baseline으로 쓴다. 외부 scroll revision에서는 late delta 적용 없이 cancel되는 test가 통과했다. |
| 18 | style-only 변화가 gesture action을 바꾸거나 최신 overscroll policy를 무시 | style revision은 geometry key에서 제외하고 action은 Begin 값으로 고정한다. 바뀐 overscroll policy를 적용한 delta test가 통과했다. |
| 19 | document/environment/surface/clip/range 변경 후 stale delta 반영 | key와 frozen geometry를 비교해 먼저 취소한다. generation, environment, surface/scale, clip, owner range test가 통과했다. |
| 20 | terminal 중복·identity 불일치·stale surface·cancel 뒤 늦은 Move | 재현 테스트에서 다른 surface generation의 terminal이 억제 contact를 풀 수 있었다. suppressed pointer에 gesture ID와 시작 surface generation을 함께 보관하고 둘 중 하나가 다르면 거부하도록 수정했다. mismatched terminal은 상태를 유지하고, 정상 terminal은 한 번만 소비하며, cancel 뒤 늦은 Move는 delta를 만들지 않는다. tests가 통과했다. |

검토 중 suppressed contact ID가 다음 gesture ID를 고갈하는 결함과 stale surface terminal이 contact 억제를 풀 수 있는 결함을 찾았다. 각각 복구·격리 test를 추가해 수정했다. 남은 경계는 native queue의 terminal reservation/coalescing, style cascade에서 resolved action을 만드는 연결, Android/iOS adapter, displayed-frame target 및 앱 표면 검증이다.
