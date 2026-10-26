# C11.4 image mixed-track negative-control targeted re-review

## 판정

**최종 수정분: PASS.** Child plan, fail-closed README, source/test 경계의 요청된 정합성을 확인했다. 전체 negative-control 범위는 명시된 미실행 inherited-gate 후속 때문에 **bounded PARTIAL**이며, 이번 slice의 변경 코드에 대한 blocker는 아니다.

검토 기준은 branch `plan/c11-4-image-auto-max-mixed-track-20261026`, `HEAD=e4570a04f1f8d0595836e3f35d7fa6a88b7c8b83`다. 대상은 [child plan](./c11-4-image-auto-max-mixed-track.md), [adapter](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_image.rs), [tests](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_image/tests.rs), [fail-closed evidence](../spec/internal/evidence/c11-4-image-auto-max-mixed-track-runtime-2026-10-26/fail-closed-controls/README.md)다. 테스트·browser·simulator는 실행하지 않았다.

## 최종 수정분 대조

- Child plan의 row control은 `auto` breadth를 `1px`로 바꾸고 track-list 길이는 유지한다고 정확히 적는다. Test도 `rows[0]`만 `LengthPx(1.0)`로 바꾸어 `unsupported_c114_image_case` / `track functions`를 확인한다. Fail-closed README가 같은 변이를 설명한다. 이전의 “list 변경” 불일치는 해소됐다.
- Stale revision 및 document generation 근거는 [snapshot test #158](../crates/spinon-image/src/snapshot/snapshot_tests.rs#L158)과 [#331](../crates/spinon-image/src/snapshot/snapshot_tests.rs#L331)을 가리킨다. #158은 같은 source의 이전 document revision completion 거부, #331은 새 document generation에서 오래된 success/failure 거부를 검사한다. Plan은 이 lower-level 결과를 C11.4 adapter의 `c114_image_resource_snapshot` branch가 실행됐다는 증거로 주장하지 않는다. 이전 #77 단독 참조 문제는 해소됐다.
- Fail-closed README line 19의 계획 SHA-256은 `c735faceabc5d019dc9f5a82736c1605afbf268df6a44439a6973488d10faa25`이며 child plan 실제 SHA와 일치한다. README의 test-source SHA-256 `4e64a61afc2c000d430b62fc0fa97979b15c2a826b2f3d72048984d55572b704`도 현재 test 파일과 일치한다.
- Validator controls는 실제 `validate_grid_style`/`validate_item_style`가 반환하는 code/detail 경계와 맞는다. Root width/height/border/gap은 `unsupported_c114_image_case` / `container box geometry`; 세 alignment property는 `unsupported_c114_image_style` / 해당 `expected start`; explicit placement/span은 `unsupported_c114_image_case` / `replaced item box or placement`다. 잘못된 root ID는 `unsupported_c114_image_fixture_root`와 입력 ID detail이고, item ID 및 source는 각각 `image tag or id mismatch`, `fixed PNG source required`다.
- Root geometry의 `prepare()` test는 `Err`와 `C11_4ImageInput` 미반환만 보인다. Direct validator tests도 해당 error만 검증한다. Child plan과 README가 downstream frame·trace 부재나 image resource allocation atomicity를 관측하지 않는다고 밝히므로 evidence 경계는 과장되지 않는다.
- Plan은 extra/nested/non-leaf topology, PNG decode/dimension의 adapter translation, stale/foreign completion의 adapter translation, profile rejection을 미실행 inherited-gate 후속으로 구분한다. Lower-level decoder/snapshot coverage를 adapter control 통과로 세지 않는다.

## 남은 범위와 결정

미실행 topology/profile 및 고정 compile-time PNG 때문에 현재 주입할 수 없는 decode/dimension·adapter snapshot 분기는 문서화된 bounded follow-up으로 둘 수 있다. 이번 변경이 해당 공통 gate나 injection 경로를 바꾼 것은 아니므로 이 누락은 이번 slice merge blocker로 보지 않는다. Adapter translation까지 실행 증거가 acceptance 조건이 되면 injection seam과 adapter-level test가 추가로 필요하다. 저장된 focused run은 `8 passed, 0 failed`로 기록돼 있지만 이번 targeted review에서는 재실행하지 않았다. 공식 C11.4 및 `72/213` 상태에는 변경이 없다.
