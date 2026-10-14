# C13.2 앱 오류 원자성 구현 실패 검토 20개

- **대상:** opt-in C13.2 FFI reserve failpoint, stale presentation 거부와 Android/iOS app gate.
- **독립성:** 계획 검토와 이전 C13.2 geometry/root-body 구현 검토를 재사용하지 않는다. 이번 변경의 새로운 실패 조건을 따로 확인했다.
- **근거:** feature FFI test, workspace test/Clippy, feature Clippy, Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator의 최종 실행.
- **판정:** 발견한 정확한 오류 marker 누락 검사를 고쳤다. 결정적 app failure controls는 양 플랫폼에서 통과했다. OS OOM과 concurrent race는 이 검토의 pass 주장이 아니다.

| # | 구현 실패 관점 | 확인 및 판정 |
|---|---|---|
| 1 | failpoint AtomicBool이 `c04-runtime-gpu` 기본 build에 포함돼 fixture state가 제품 host에 노출된다. | host field, import, use가 `c13-2-scroll-fixture-test` cfg 안에 있다. 기본 archive에서 C13.2 symbol·payload·marker가 absent이고 opt-in positive control에는 present다. **PASS** |
| 2 | atomic failpoint가 반복해서 남아 이후 정상 evaluation까지 계속 실패시킨다. | `reapply_c13_2_scroll_state`에서 `swap(false, AcqRel)`로 한 번만 소비한다. Android/iOS route는 실패 뒤 다음 같은-host eval과 offset publish를 통과했다. **PASS** |
| 3 | 기존 snapshot이 없거나 다른 generation인데 설정된 failpoint를 잘못된 scene에 소비한다. | failpoint 전에 prior snapshot 존재와 generation equality를 요구한다. route는 `observed=false` 또는 exact marker 없음이면 실패하므로 우회가 pass가 되지 않는다. **PASS** |
| 4 | 실제 allocation이 끝난 뒤 failpoint가 발생해 request vector 또는 candidate state 일부가 바뀐다. | 주입 분기는 `requests.try_reserve_exact` 바로 앞이며 request push·offset 재계산보다 먼저 반환한다. 저장 snapshot equality negative control이 두 Simulator에서 통과했다. **PASS** |
| 5 | failpoint가 다른 오류 status를 반환해 앱이 reservation error로 취급한다. | `verify_c13_2_failure_controls`는 `ERR_LAYOUT`만 예상한다. 다른 status는 즉시 상세 오류로 반환한다. **PASS** |
| 6 | unrelated `ERR_LAYOUT`이 failpoint pass로 오인된다. | 첫 구현은 observed flag와 status만 봤다. app review에서 report가 `c13_2_injected_reserve_failure`를 포함하는지 확인하지 않는 false positive를 찾았다. exact marker gate와 `reserve_failure_marker=matched` 앱 assertion을 추가하고 두 Simulator를 재실행했다. **FIXED · PASS** |
| 7 | `eval`이 failpoint 전에 syntax/layout error를 내고 다음 호출에도 failpoint가 남는다. | 호출 직후 failpoint를 clear하고 observed flag를 `swap(false, AcqRel)`로 수거한다. marker 없는 오류는 통과하지 않는다. **PASS** |
| 8 | 이전 실행의 observed flag가 다음 실행 결과를 오염시킨다. | 매 실행 시작 때 observed flag를 false로 reset하고 결과 판정 후 다시 consume한다. 하나의 앱 route가 반복되는 negative control을 통과했다. **PASS** |
| 9 | 예약 오류 뒤 저장 offset snapshot 일부가 변경되지만 owner count만 맞아 통과한다. | before/after `RuntimeRenderSnapshot`을 `PartialEq` exact로 비교한다. failpoint 뒤 equality를 양 플랫폼에서 통과했다. **PASS** |
| 10 | error route에서 화면에 이전 document의 stale presentation이 계속 유효한 것으로 과장된다. | 정상 `eval`은 시작 시 presentation sequence를 무효화한다. 구현·evidence는 stored scroll-state equality와 현재 draw 가능 scene 보존을 분리하고, 후자는 보장한다고 주장하지 않는다. **범위 제한 명시** |
| 11 | reservation 오류 뒤 다른 host를 만들어 성공시킨 뒤 same-host recovery라 표시한다. | control은 `self.eval`을 같은 `RuntimeGpuHost` 인스턴스에 두 번 호출하며 create/new host 경로가 없다. 앱 report의 `recovery=same_host`가 양 플랫폼에서 통과했다. **PASS** |
| 12 | 첫 복구 후 layout이 준비되지 않았는데 stale publish 시험으로 진행한다. | 첫 recovery `Result`와 `layout=ready`를 확인한 뒤에만 stale sequence 단계로 간다. error는 상태 code와 report를 보존한다. **PASS** |
| 13 | C13.2 state가 없는 상황을 정상 빈 상태로 취급해 snapshot 보존 검사가 무력화된다. | `c13_2_scroll_state_snapshot()`은 `None`을 `ERR_LAYOUT`로 반환한다. baseline은 9 owners인지 먼저 확인한다. **PASS** |
| 14 | 잘못된 sequence를 계산해 최신 presentation도 stale publish처럼 거부한다. | current sequence를 캡처한 직후 `invalidate_scene()`가 checked increment를 수행하고, 이전 값을 publish에 넘긴다. 실제 최신 상태에서 만든 값만 negative control에 사용한다. **PASS** |
| 15 | sequence exhaustion이 wrap-around되어 stale publication 검사가 우연히 성공한다. | `PresentationScenes::invalidate`는 `checked_add` 기반이며 소진 시 오류를 전달한다. control은 그 오류를 `ERR_STALE`로 실패 처리한다. **PASS** |
| 16 | old presentation sequence publish가 성공해도 테스트가 결과를 무시한다. | `presentation.publish(stale_sequence, ...) == true`는 즉시 `ERR_STALE`로 끝난다. 두 앱 runtime은 `stale_publish=rejected`를 보고했다. **PASS** |
| 17 | stale rejection이 scroll-state map을 수정하는데 상태를 검사하지 않는다. | publish 거부 직후 `before_stale`와 저장 state를 다시 `PartialEq` 비교한다. 양 Simulator에서 통과했다. **PASS** |
| 18 | stale rejection 후 새 host 또는 옛 scene으로 성공한다. | 같은 host에서 matrix를 재평가하고 final offset을 새로 적용한다. 최종 draw는 26 boxes와 scroll revision 8을 보고했다. **PASS** |
| 19 | final presentation sequence 또는 snapshot이 state store와 다르지만 success marker가 출력된다. | presentation 존재, `published.sequence == sequence`, snapshot equality를 모두 검사한 다음 success report를 만든다. Android/iOS app gate가 final success report를 요구한다. **PASS** |
| 20 | app matrix 재평가가 global `const` 충돌 또는 이전 root 누수로 실패/오염되지만 failure control이 계속된다. | JS source를 IIFE로 격리하고 기존 matrix/root-body root 및 lifecycle 참조를 제거한다. 초기 syntax·multiple-root 실패를 각각 재현해 수정했으며 최종 양 플랫폼 실행이 통과했다. **PASS** |

## 남은 경계

실제 OS allocator OOM, 예약 실패 중 기존 presentation이 draw 가능한지, concurrent scheduler race, Android PixelCopy/iOS Metal drawable readback, ink-only overflow 및 공개 CSSOM은 별도 관문이다. 이 구현 검토는 결정적 opt-in fixture 동작으로 범위를 한정한다.
