# C13.3 computed scroll policy PR 변경 관점 검토 30개 · 2026-10-15

검토 범위는 이번 PR diff 15개 source/config 파일과 C13.3 runtime evidence/doc 변경이다. 앞 단계의 router, style projection, session, queue, native adapter 리뷰를 반복하지 않고 이번 computed-style→snapshot 접점과 feature 경계를 중심으로 확인했다.

1. **Cargo feature edge 누락:** FFI가 runtime opt-in을 활성화하고 runtime이 style-to-render, style-to-render가 render feature를 전달한다. 기본 edge에는 강제 활성화가 없다.
2. **기본 빌드의 metadata layout 변경:** `touch_action_chain` field와 accessor가 render feature로 gated되어 기본 `RuntimeScrollRenderMetadata`에 포함되지 않는다.
3. **default FFI ABI leak:** host staticlib의 C13.3 symbol/ready/batch payload가 `0/0/0`이다. [검사 결과](./feature-boundary.md).
4. **Android Java source에서 marker leak:** DEX의 ready marker를 실제 기본 APK에서 발견했다. `BuildConfig` guard를 touch listener와 ready 판정 양쪽에 추가한 뒤 기본 DEX/APK에서 `0/0/0`을 확인했다.
5. **Android opt-in positive control:** 같은 APK 검사에서 opt-in payload/symbol은 `2/1/1`로 존재한다.
6. **iOS compile condition mismatch:** project build settings는 `SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST`와 Swift/C macro를 같은 값을 쓴다. 기본 app은 0, opt-in app은 1로 다시 빌드했다.
7. **new runtime constructor misroutes profile:** 새 constructor가 독립 gesture fixture cascade coordinator를 선택하고 다른 CSS fixture constructors를 바꾸지 않는다.
8. **runtime enum exhaustiveness:** `#[cfg]` profile arms가 type construction과 coordinator dispatch 양쪽에 짝으로 있다.
9. **cascade error swallowed as default:** 새로운 calculation path는 기존 `Result` 전파를 사용한다. unsupported profile diagnostics를 다른 C13 profile처럼 삼키지 않는다.
10. **layout admission omits new profile:** runtime layout validation의 unsupported-inline-property selector가 C13.3 profile-specific 검사에 연결된다.
11. **top-level renderer dispatch:** `build_runtime_render_snapshot`는 C13.3 profile을 stacking renderer로 보내며 feature-off profile enum에 새 arm을 요구하지 않는다.
12. **legacy snapshot entry point broadened unintentionally:** C13.2 `build_runtime_render_snapshot_with_scroll_requests`는 overflow-clip profile에만 허용한다. C13.3 gesture는 current snapshot router를 쓰므로 이 API를 넓히지 않았다.
13. **stacking profile validation mismatch:** C13.3 profile을 stacking builder allowlist에 넣고 같은 profile predicate를 주요 clip/paint paths에서 재사용한다.
14. **overflow clipping lost in C13.3 profile:** C13.3는 overflow-clip profile path를 공유해 scroll clips·hit clips·clip layers를 만든다.
15. **auto-positioned subtree handling differs:** 기존 overflow-clip profile과 같은 predicate가 subtree painting과 hit order에 적용된다.
16. **metadata omitted on non-C13.3 profiles:** touch-action chain을 붙이는 분기는 정확히 C13.3 computed profile에서만 실행된다.
17. **metadata/render-box ordering shifts:** metadata와 box가 같은 loop에서 같은 node 순서로 push된다. 별도 map 정렬 순서에 의존하지 않는다.
18. **nested owner parent chain invalid:** policy attachment는 이미 구성된 scroll owner list와 현재 owner id를 사용하며 owner parent 연결을 재계산하지 않는다.
19. **viewport policy reads unrelated layout owner:** viewport container만 document root computed style을 읽고 element owner는 그 element의 style을 읽는다. HTML/body special propagation은 이번 mobile fixture에서 검사하지 않았다.
20. **axis policy copies the wrong computed field:** X/Y computed overscroll values를 서로 독립적으로 가져와 같은 axis setter에 매핑한다.
21. **missing computed axis silently becomes auto:** 각 축 property 누락을 `MissingComputedProperty` 오류로 반환한다.
22. **hidden owner directly scrolls:** direct bit는 `overflow:auto/scroll` 조건과 snapshot max range를 모두 확인한다. hidden은 false다.
23. **zero-range axis consumes direct input:** max range가 없는 축은 direct bit false이고 existing delta router가 ancestor 처리한다. 이번 fixture test는 zero-range negative control이 아니다.
24. **touch-action ancestry excludes leaf target:** traversal cursor가 hit node부터 시작해 해당 node computed action을 첫 값으로 추가한다.
25. **touch-action ancestry includes unrelated root ancestors:** scroll owner의 action을 추가하고 그 지점에서 멈춘다. `touch_action_intersects_target_to_nearest_owner` test가 owner 너머의 `none`을 배제한다.
26. **intermediate style gets skipped:** parent map을 따라 각 단계 action을 더한다. `pan-x`/`pan-y` intersection test가 제한 값을 증명한다.
27. **unsupported token broadens gestures:** enum `Unsupported`를 그대로 renderer metadata와 snapshot 조회에 보내고 `auto`로 coercion하지 않는다.
28. **hit target and touch-action index diverge:** clip-aware hit test의 node id로 snapshot boxes를 찾고 같은 index metadata chain을 조회한다. hit miss와 invalid node는 action을 제공하지 않는다.
29. **allocation/depth failure turns into partial chain:** 각 ancestry push 전에 fallible reserve를 호출하고 길이 상한/누락 computed style은 오류다. 전용 allocator failure injection은 남은 검증이다.
30. **host bypasses renderer policy with hard-coded input:** Begin은 snapshot의 hit/action lookup 결과를 전달하고 fixture 준비 단계는 각 owner snapshot policy가 computed direct-scroll을 허용하는지 검증한다. 이후 simulator trace는 inner owner만 움직이고 captured bounds 안에 marker가 남는 것을 보여준다.

## 판정

- PR delta에서 발견한 기본 Android DEX marker leak 1건을 compile-time guard로 수정했다. 최종 기본·opt-in app 산출물은 [feature 경계 기록](./feature-boundary.md)에서 각각 negative/positive control을 통과한다.
- 그 외 code-level 접점은 focused tests, workspace suite, Clippy, formatter, diff check를 통과했다.
- 실제 `auto/contain/none` app scene 행렬, body/root 전파, style invalidation, physical multi-touch, wheel/trackpad, S04.10/S05.2 및 WPT는 PR 완료 근거가 아니며 C13.3 미완료 사유로 유지한다. [실행 범위와 화면](./README.md).
