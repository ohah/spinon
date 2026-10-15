# C13.3 computed scroll policy 구현 실패 관점 검토 20개 · 2026-10-15

검토 대상은 computed style → `RuntimeRenderSnapshot` scroll policy 연결과 이를 소비하는 opt-in fixture host다. 구현 결과를 완료로 승격하지 않고, 범위 밖 동작은 미확인으로 분리했다.

1. **기본 Android 코드에서 listener가 켜지는가:** `c133GestureFixtureRequested`만 검사하면 기본 DEX에도 marker 경로가 남았다. 기본 APK `classes2.dex`에서 ready marker가 검출된 것을 실제로 찾아 `BuildConfig.SPINON_C133_GESTURE_FIXTURE_TEST` guard를 listener와 ready 검사 모두에 추가했다. 재빌드 기본 APK는 marker 0건이다. [artifact 결과](./feature-boundary.md).
2. **기본 host FFI에 opt-in symbol/payload가 섞이는가:** feature-off staticlib의 symbol, ready marker, batch marker는 각각 0건이다.
3. **기본 iOS executable에 opt-in 분기가 남는가:** feature-off archive/executable byte 검사에서 세 전용 문자열이 모두 없다.
4. **feature forwarding이 한 단계에서 끊기는가:** FFI→runtime→style-to-render→render feature dependency를 설정했고 host와 mobile opt-in 산출물에 양성 대조가 존재한다.
5. **feature-off runtime이 새 cascade/profile을 선택하는가:** 신규 runtime selector, cascade dispatch, renderer metadata는 fixture feature로 컴파일된다. 기본 앱 artifact에서 전용 marker가 없다.
6. **C13.3 profile이 renderer route에서 거부되는가:** `is_runtime_stacking_profile`과 overflow/clip route에 feature-gated profile을 추가했다. focused profile fixture test가 실제 snapshot을 만든다.
7. **computed `overscroll-behavior-x/y`가 뒤바뀌거나 shorthand로 뭉개지는가:** 두 물리 축을 따로 읽어 policy에 기록하며 contain-X/auto-Y test는 X stop과 남은 Y ancestor 소비를 구분한다.
8. **scroll range가 없는 축을 직접 입력이 소비하는가:** policy의 direct-scroll bit는 해당 축의 max range가 0이면 false가 된다. 빈 축의 실기기·simulator 행렬은 남아 있다.
9. **overflow가 visible/clip인 노드가 owner로 입력을 소비하는가:** C13.2 owner 판정을 공유해 이런 노드는 policy owner가 되지 않는다. 전체 visibility 조합 행렬은 이번 follow-up의 simulator 증거가 아니다.
10. **`overflow:hidden`을 direct gesture로 움직이는가:** `auto/scroll`만 direct-scroll bit를 받으므로 hidden은 false로 남는다. 기존 C13.2 규칙과 일치한다.
11. **viewport/root scroll owner가 임의 element policy를 읽는가:** 현재 root owner는 root computed style에서 policy를 얻는다. HTML/body 특수 전파와 body가 root를 대체하는 경우는 이 scene에서 확인하지 않았고 완료 범위에 넣지 않았다.
12. **computed overscroll 값 누락이 기본값으로 조용히 바뀌는가:** C13.3 profile에서 축 값 누락 시 명시적 `MissingComputedProperty` 오류를 반환한다.
13. **target 자신의 `touch-action`이 교집합에서 빠지는가:** hit node에서 ancestry를 시작하고 현재 node 값을 먼저 합산한다. `pan-x` leaf test가 수직 pan을 거부한다.
14. **가장 가까운 scroll owner의 touch action을 빠뜨리는가:** owner 값을 포함한 뒤 ancestry를 중단한다. owner의 조상에 둔 `none`은 inner scroll 시작 정책을 바꾸지 않는 경계 test가 있다.
15. **중간 ancestor action을 누락하는가:** 모든 element 값을 순서대로 추가한다. target `pan-x`와 중간/owner `pan-y`가 최종 `none`이 되는 test가 있다.
16. **지원하지 않는 값이 `auto`로 열리는가:** `Unsupported`가 snapshot 조회 결과로 남고 session pan을 거부한다. `manipulation`을 사용한 fail-closed test가 있다.
17. **clip 바깥 hit 또는 miss가 잘못된 node에 action을 적용하는가:** 조회는 snapshot의 clip-aware hit-test를 먼저 호출한다. adapter는 miss 시 `Unsupported`로 시작을 거부한다. 별도 outside-border app swipe matrix는 남아 있다.
18. **metadata와 render box index가 서로 밀리는가:** 한 render loop에서 box와 metadata를 함께 push하고 동일 snapshot constructor로 전달한다. 노드 identity 조회가 실패하면 action 조회는 `None`이다.
19. **조상 순환, 누락 style 또는 할당 실패가 무한 loop/기본값으로 숨는가:** ancestry 길이 상한, missing-style 오류, 매 push 전 fallible reserve가 있다. 전용 allocator fault injection은 이번 follow-up에 추가하지 않았다.
20. **gesture 중 computed style 변경이 기존 action을 바꾸는가:** metadata는 immutable snapshot에 있고 Begin에 해석한 action을 session으로 전달한다. mid-gesture style/policy invalidation과 다음 sample admission은 아직 end-to-end 검증하지 않았다.

## 판정

- 실제 결함 1건(기본 Android DEX marker)이 발견되어 compile-time guard로 수정했고, 새 기본 APK에서 음성 대조를 통과했다.
- 그 외 검사 가능한 feature 경계, axis mapping, owner traversal, unsupported fail-closed는 통과했다.
- root/body 특수 전파, non-default simulator CSS matrix, `none`/`contain` local affordance, mid-gesture invalidation과 displayed-frame targeting은 미확인/미완료로 유지한다. 상세 실행 수치는 [실행 근거](./README.md)와 [artifact 경계](./feature-boundary.md)에 있다.
