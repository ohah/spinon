# C13.3 touch-action simulator matrix 구현 실패 관점 검토 20개

대상은 zero-translation pan 경계, C13.3 scenario 3–7, Android/iOS opt-in fixture route, presentation 재게시 Clippy 수정, 실제 최신 simulator 행렬이다. 계획 문서 검토와 앞선 C13.3 구현 리뷰의 관점·문구를 재사용하지 않았다.

1. **UIKit `.began`의 `(0,0)`이 축을 잠그는 경우** — `beginC133Pan`은 zero cumulative translation에서 Begin만 enqueue하고 `c133PanBegan`을 false로 둔다. 첫 nonzero sample 전에는 Rust axis latch가 시작되지 않는다.
2. **zero `.changed`가 queue를 포화시키는 경우** — 아직 pan이 시작되지 않았고 translation도 0이면 callback에서 반환한다. 무의미한 Move sequence를 queue에 넣지 않는다.
3. **첫 nonzero `.changed`가 Move로 잘못 분류되는 경우** — `startsPan`이 true인 첫 유효 sample은 action 1 (`PanBegan`)으로 enqueue된다. 이후 changed callback만 action 2다.
4. **recognizer가 nonzero `.began`을 처음 주는 경우** — `beginC133Pan`은 해당 누적 translation으로 즉시 `PanBegan`을 enqueue한다. zero-only iOS 경로를 도입하면서 기존 시작 경로를 막지 않는다.
5. **zero session sample이 scroll을 조용히 바꾸는 경우** — `pan_began`은 finite 검증 후 exact-zero면 sequence만 갱신하고 `Moved { delta: None }`를 반환한다. 신규 session 회귀 테스트에서 state와 revision 불변을 확인했다.
6. **첫 nonzero sample 전에 잘못된 phase에서 pan이 시작되는 경우** — 분리된 `start_pan`은 active session이 `Tracking`일 때만 축을 결정한다. 이미 시작/종료된 phase는 기존 오류 경계로 거부된다.
7. **NaN/무한대가 zero 경로를 우회하는 경우** — `validate_pan_translation`이 zero 판정보다 먼저 실행된다. 기존 non-finite 회귀 테스트와 전체 render session 테스트가 통과했다.
8. **중복·역순 sequence가 deferred begin 상태를 덮는 경우** — zero sample도 `last_input_sequence`에 기록한다. 이후 queue/session 입력은 같은 epoch의 순서 검사를 계속 적용한다.
9. **허용되지 않은 pan-x가 zero 이후 horizontal 이동을 허용하는 경우** — `pan-y` session에서 zero 후 nonzero x를 주면 `PanRejected`, revision 0이다. `zero_pan_began_defers_pan_y_axis_lock_until_first_nonzero_sample`이 검증한다.
10. **허용되지 않은 pan-y가 zero 이후 vertical 이동을 허용하는 경우** — `pan-x` session에서 zero 후 nonzero y를 주면 거부되고 revision은 0이다. 대응 Rust 회귀 테스트가 통과했다.
11. **허용된 pan-y가 zero 때문에 계속 tracking에 갇히는 경우** — 첫 nonzero y sample에서 `PanStarted`와 y-only delta가 나온다. x offsets는 시작값과 같고 revision만 증가한다.
12. **축을 거부한 뒤 반대 방향으로 재잠금하는 경우** — 첫 dominant axis에서 거부되면 나중에 허용 축을 보내도 재잠금하지 않는다. 기존 `disallowed_dominant_axis_rejects_pan_and_never_relocks_later`가 통과했다.
13. **`touch-action:none`의 수평/수직 중 한 방향만 새는 경우** — 양 플랫폼 모두 두 방향 swipe에서 inner/root/outer offset과 revision이 0이다. matrix의 scenario 3 네 결과가 거부다.
14. **`pan-x`/`pan-y`의 물리 축이 뒤바뀌는 경우** — 각 플랫폼에서 pan-x horizontal과 pan-y vertical만 허용되고 cross-axis swipe는 거부된다. scenario 4·5의 총 8개 결과를 대조했다.
15. **target descendant `pan-x`가 inner auto에 묻히는 경우** — scenario 6 computed action은 `pan-x`; 양 플랫폼 horizontal만 inner를 이동시킨다. vertical은 revision 0이다.
16. **ancestor/child action 교집합을 무시하는 경우** — scenario 7의 inner `pan-y`와 child `pan-x`는 effective `none`이다. 양 플랫폼의 네 방향 swipe가 모두 무변경이다.
17. **fixture ID 오타가 다른 policy로 조용히 대체되는 경우** — Rust `TryFrom<i32>`는 0–7만 명시적으로 매핑하고 나머지는 오류를 낸다. scenario mapping 단위 테스트가 feature-on 상태에서 통과했다.
18. **fixture report가 CSS policy와 다르지만 앱이 ready가 되는 경우** — host는 fixed point의 computed touch-action과 scenario 기대값을 비교한다. 두 adapter는 report의 scenario, effective action, seed/offset과 첫 draw 결과를 검사한 뒤에만 ready 상태로 전환한다.
19. **offset 변경이 화면에서 안 보이거나 inner border를 넘는 경우** — 최신 앱에서 수평·수직 marker는 swipe 전 0 pixel, swipe 후 exact RGB로 확인됐다. 네 after bbox는 blue inner border 안이고 root/outer marker pixel은 0이다.
20. **fixture가 기본 제품 artifact로 새거나 opt-in 경로가 빠지는 경우** — default 아카이브에는 4개 C ABI 이름과 전용 JS marker가 없고 opt-in positive control에는 모두 있다(`fixture-artifact-boundary.txt`). Default/feature Clippy와 workspace/CSS reference 테스트도 통과했다.

## 판정

이 구현 검토에서 수정이 필요한 실패 경로를 남기지 않았다. 실제 발견된 UIKit zero-translation 경합은 첫 nonzero `.changed`를 `PanBegan`으로 분류하고 Rust session의 zero 시작을 no-op 처리해 수정했다. 이는 고정 fixture의 단일 pointer swipe 범위에 대한 결과다. tap/click activation, iOS multi-touch, wheel/trackpad, style/layout invalidation, public events와 S04.10 displayed-frame targeting은 이 검토로 통과 처리하지 않는다.

근거 파일: `README.md` · `matrix.json` · `pixel-analysis.json` · `workspace-test.log` · `ffi-opt-in-test.log` · `ffi-opt-in-clippy.log`.
