# C13.3 · 활성 gesture의 style invalidation 연결

**상태:** style invalidation 구현 및 iOS/Android Simulator fixture 검증 완료 · C13.3 전체 미완료
**구현 PR:** [#189](https://github.com/ohah/spinon/pull/189)
**상위 계획:** [스크롤 gesture와 축별 chaining](./c13-3-scroll-gestures.md)
**내부 계약:** [0063 C13.3 scroll gestures](../spec/internal/0063-c13-3-scroll-gestures.md)
**구현 전 Chrome 비교:** [관찰·fixture·정확한 trace](../spec/internal/evidence/c13-3-style-invalidation-precomparison-2026-10-15/README.md) · [JSON](../tests/fixtures/css/references/c13-3-style-invalidation-precomparison-v1.json)
**계획 실패 관점 20개:** [검토 기록](../spec/internal/evidence/c13-3-style-invalidation-plan-review-20-2026-10-15.md)
**구현 실패 관점 20개:** [검토 기록](../spec/internal/evidence/c13-3-style-invalidation-2026-10-15/implementation-review-20.md)
**PR 변경 관점 30개:** [검토 기록](../spec/internal/evidence/c13-3-style-invalidation-2026-10-15/pr-delta-review-30.md)
**실행 근거:** [Android/iOS Simulator 빌드·시나리오·화면·pixel·hash](../spec/internal/evidence/c13-3-style-invalidation-2026-10-15/README.md)

## 목표와 범위

C13.3 opt-in runtime fixture에서 실제 inline `style` 갱신이 computed style과 새 render snapshot까지 이어질 때 활성 gesture의 정책·기하 invalidation 경계를 연결한다.

- geometry-neutral paint/style 또는 gesture policy 변화는 활성 gesture를 유지한다. 현재 pointer는 Begin 시 resolve한 `touch-action`을 끝까지 사용한다. 새 `overscroll-behavior`는 해당 style revision 이후 enqueue된 다음 delta부터 사용한다. 새 gesture는 새 `touch-action`을 resolve한다.
- scroll owner, scrollport, target/clip, range 등 geometry가 바뀌면 다음 입력을 소비하기 전에 활성 gesture를 `GeometryChanged`로 취소한다. 이미 반영한 offset은 유효한 새 range에 보존·clamp하고, geometry 변경이 scroll revision을 갱신했다면 다음 입력과 stale queued sample을 구분한다.
- 활성 session에 geometry-neutral한 document/render-tree/style revision 쌍이 도착해도 단순 revision 비교 때문에 취소하지 않는다. inline style attribute 변경이 render-tree revision도 올리는 실제 `HostDocument` 동작을 반영한다. 스타일 계산 revision이 바뀌고 geometry key가 같을 때만 style-only transition으로 승격한다. style revision이 그대로인 임의 document/render-projection mutation은 계속 취소한다.
- 앱 검증은 `c13-3-gesture-fixture-test` opt-in route 안에서만 한다. 제품 DOM event, 공개 CSS 지원, C13.3 전체 완료는 범위 밖이다.

### Chrome 기준과 계약 차이

고정 Chrome 155 touch oracle에서는 active direct-touch 중 `overscroll-behavior-y:auto`를 `contain`으로 바꿔도 같은 gesture의 남은 touch delta가 outer로 전달됐다. 시작/첫 move/final state와 harness timing은 JSON에 기록했다. C13.3 내부 계약은 queue가 최신 revision을 받은 뒤의 다음 delta부터 새 overscroll policy를 쓰도록 한다. 이 PR은 내부 계약을 구현하며 Chrome 155와 동작이 같다고 주장하지 않는다. 별도 공개 compatibility decision이 승인되기 전까지 이 차이는 계획과 evidence에 유지한다.

## 구현과 관찰 순서

1. **코어 revision 분류:** geometry key에서 style-neutral로 갱신 가능한 document/render-tree/style revision을 geometry 수치와 분리한다. active gesture는 마지막으로 받아들인 document/render-tree/style revision을 추적한다. style revision 변화와 동일 geometry key가 함께 확인될 때만 revision을 갱신하고, Begin의 `touch-action` 및 active 축은 고정한다.
2. **snapshot 재적용:** runtime inline style mutation 뒤 새 immutable render snapshot을 만든다. owner identity가 그대로인 scroll offset은 되살리고, geometry-neutral mutation에서는 `ScrollStateRevision`을 보존한다. geometry 변경·owner 삭제·reparent는 같은 경로에서 확인하고, 유효한 새 range 밖 offset은 clamp한다.
3. **ordered fixture barrier:** app probe는 PanBegan을 enqueue한 뒤 fixture 전용 style update가 끝날 때까지 중간 native move를 queue에 넣지 않는다. UI callback은 worker 계산을 기다리지 않는다. 보류한 cumulative sample은 업데이트 완료 뒤 다음 callback으로 전달하며, style update 실패·surface 종료 때는 gesture를 cancel하고 stale sample을 승인하지 않는다. 새로운 style snapshot과 후속 input 사이의 순서는 FFI trace에 revision/sequence로 기록한다.
4. **정책 변경 probe:** scenario 시작 시 inner의 `touch-action:pan-x`, X boundary, `overscroll-behavior-x:auto`를 computed profile로 확인한다. PanBegan 뒤 실제 inline style을 `touch-action:none; overscroll-behavior-x:contain`으로 바꾼다. 이미 시작한 gesture의 후속 수평 delta는 pan-x로 받아들이고, containment 뒤 추가 ancestor 이동을 막는다. 종료 후 새 gesture는 computed `none`으로 수평 scroll을 시작하지 않는다.
5. **기하 변경 probe:** 다른 fresh scenario에서 active pan 도중 inner scrollport 폭을 바꿔 새 render snapshot을 만든다. 다음 sample이 `GeometryChanged`로 취소되고 취소 이후 offset/revision이 변하지 않는지 확인한다. gesture terminal 뒤 같은 host에서 새 geometry를 사용한 fresh pan이 동작해야 한다.
6. **플랫폼·artifact:** Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 같은 CSS coordinate trace, `touch-action`, style mutation timing, revision, offset, screenshot을 확인한다. `c13-3-gesture-fixture-test`를 끈 Android/iOS artifact에 전용 symbol/string/payload가 없는지, 켠 positive control에 있는지 각각 확인한다. 화면 검사는 scrollport/inner border 바깥으로 content marker가 새지 않는 pixel control을 포함한다.

## 합격 기준

- **Style-only revision:** 실제 fixture DOM `style` attribute 갱신으로 document/style revision이 증가하고 geometry key가 유지된다. 새 pan policy가 계산되더라도 active pan은 기존 frozen axis/action을 유지한다. stale old revision input은 새 policy로 조용히 재해석하지 않는다.
- **Overscroll sample epoch:** update 전에 수락된 sample은 기존 revision 순서 뒤에 처리한다. update barrier 뒤의 sample은 새 policy epoch와 같은 snapshot key를 갖는다. containment 후 outer/root offset은 update 당시 값과 같고 inner는 boundary를 넘지 않는다. `touch-action:none`은 후속 새 gesture에만 적용된다.
- **Paint-only control:** geometry를 바꾸지 않는 background-color 변경은 활성 gesture를 취소하지 않고, 실제 pixels는 해당 scrollport clip 안에만 나타난다.
- **Geometry update:** width/scrollport/range 또는 owner 관계 변화는 다음 Move 전에 취소한다. clamp/revision 효과는 한 번만 게시하며, 이후 old gesture input은 offset을 바꾸지 않는다. terminal 뒤 fresh gesture는 새 geometry에서 동작한다.
- **Negative controls:** style update 동안 move가 중복 적용되지 않음, invalid mutation ID/unsupported style은 명시 거부, update 실패 뒤 같은 host가 fresh gesture를 수락, Android second pointer/lifecycle cancel 회귀, iOS cumulative translation no-double-apply를 확인한다.
- **Visual/artifact:** 양 플랫폼 screenshot은 실제 이동된 content와 border/clip을 포함한다. background/marker exact-color 점을 inner scrollport 안과 밖에서 검사한다. 기본 artifact의 fixture 경계와 opt-in 양성 대조를 함께 기록한다.
- **범위 문구:** 결과는 제한 C13.3 internal opt-in runtime fixture의 스타일 invalidation 근거다. Chrome parity, WPT pass, public `PointerEvent`/`scroll`, 제품 CSS 지원, S04.10 displayed frame, compositor/scanout, 실기기 검증으로 확대하지 않는다.

## 구현 진입 조건

- Chrome 155 precomparison JSON과 screenshot hash가 재현되고, style-policy 기대값과 Chrome 관찰 차이를 문서에 명시한다.
- 계획 리뷰의 20개 독립 실패 관점이 기록돼 있으며, unresolved product decision이 없고 각 구현 실패 조건이 fixture/관찰값에 대응한다.
- 이 계획 검토 뒤 별도의 새 구현 실패 관점 20개를 작성한다. PR 직전에는 실제 전체 diff·simulator 실행·artifact 경계를 대상으로 별도 30개 관점을 새로 검토한다.

## 구현 결과 · 2026-10-15

- Runtime cascade key와 style cache는 요청의 `StyleRevision`을 전달한다. 실제 subtree style invalidation만 revision을 한 단계 올리고, environment 갱신은 revision을 보존한다. 빈 layout scene도 요청 revision을 유지한다.
- 새 layout snapshot은 기존 scroll offset을 복원한 다음 geometry key를 비교한다. scroll-adjusted hit-test clip이 offset 0인 임시 scene과 달라 보이는 false geometry change를 막고, geometry-neutral style 변경에서 `ScrollStateRevision`을 유지한다.
- scenario 8은 active pan을 유지하면서 현재 gesture의 `touch-action`을 동결하고, 다음 gesture에 `none`을 적용했다. scenario 9는 실제 scrollport/range 변경으로 active pan을 취소하고 새 geometry의 후속 pan을 허용했다. scenario 10은 paint-only 갱신 중 pan을 유지했다.
- Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 세 시나리오를 각 플랫폼 fresh host로 통과했다. 두 paint screenshot의 inner pixel은 이전 `[23,37,58,255]`에서 새 `[14,86,80,255]`로 바뀌었고, inner scrollport border 밖 `[200,100]` 표본은 양 플랫폼 모두 배경 `[27,52,65,255]`를 유지했다.
- 캡처 검증기는 Android의 동일한 `STYLE_UPDATE`/input batch 중복 보고를 stable payload 기준으로 합치고, logcat이 뒤쪽 진단을 다르게 자른 행에서는 가장 긴 원문을 보존한다. 이 로그 정규화는 별도 unit fixture로 검증했다.
- 결과는 opt-in simulator fixture 근거다. 기본 iOS Simulator `.app`와 Android debug APK에서 새 style ABI·payload·marker 제외, opt-in positive control 포함을 별도 검사했다. iOS background가 style barrier 진행 중 들어오는 timing은 시뮬레이터에서 주입하지 않았다. Chrome 155가 기록한 active-touch policy 관찰과의 차이, public event/CSSOM, real device, hardware GPU, compositor 이후 표시 및 C13.3 전체 미완료 범위는 유지한다. [artifact 경계 기록](../spec/internal/evidence/c13-3-style-invalidation-2026-10-15/artifact-boundary/README.md)
