# Grid item raw intrinsic input PR diff 적대 검토 20개

**검토일:** 2026-10-19
**검토 대상:** C11.4 raw intrinsic input 후보 변경 전체 diff. 기존 callback trace 검토와 raw input 구현 검토와 별도로, 변경 코드·테스트·계약·상태 문서가 PR 단위에서 함께 맞는지 확인했다.
**범위:** Rust `spinon-layout` API와 해당 내부 계약·상태 문서. Android/iOS 앱 runtime 변경은 없다.

## 독립 실패 관점

1. **공개 결과 구조 변경 누락** — `LayoutOutput`에 새 필드를 추가하면서 저장소 내 struct literal을 놓쳤는지 검색했다. 제품 생성 경로의 유일한 literal인 `collect_frames`가 빈 입력 벡터를 초기화하고, 전체 workspace 빌드·테스트가 통과한 기록이 있다. 외부 crate에서의 struct literal 호환성은 별도 공개 API 보장으로 주장하지 않는다.
2. **callback trace 의미 후퇴** — 새 raw input이 callback 관찰 trace를 대체하거나 trace의 의미를 바꾸는지 확인했다. 기존 `intrinsic_measurements`와 새 `grid_item_intrinsic_inputs`는 서로 다른 output 필드이고, 문서도 둘을 구별한다. 통과.
3. **Taffy cache hit/miss 의존** — callback 생략 시 raw input이 사라지는지 확인했다. raw collector는 callback trace를 읽지 않으며, text low-level test가 빈 trace에서 직접 수집을 확인한다. 통과.
4. **source 범위 확장** — Grid descendant나 container 자체가 direct item으로 잘못 기록되는지 확인했다. metadata는 입력 Grid container의 직접 자식만 방문하고 원본 자식 목록으로 leaf 여부를 저장한다. 통과.
5. **out-of-flow 재부모화** — absolute/fixed descendant 이동이 원본 leaf 판정을 바꾸는지 확인했다. leaf 여부는 Taffy 재부모화 전의 `LayoutNode.children`에서 설정한다. 코드 경계 통과; 현재 profile은 positioned Grid item 성공 출력을 만들기 전에 거부한다.
6. **숨겨진 조상 전파** — `display:none` 조상 아래 Grid/item이 수집되는지 확인했다. root DFS가 조상 hidden 상태를 전파하고 metadata 단계가 이를 제외한다. 통과.
7. **숨겨진 형제와 source order** — hidden child를 제거한 뒤 순번을 다시 매겨 CSS 원래 순서를 잃는지 확인했다. 먼저 원본 enumerate 순번을 저장하고 hidden filter를 적용하며 regression은 순번 `1`을 확인한다. 통과.
8. **CSS `order`와 DOM 순서 혼합** — source order가 시각 정렬값으로 대체되는지 확인했다. 입력 child index와 `item_style`을 각각 보존하고, 지원되지 않는 Grid `order`는 기존 fail-closed 경계에 남긴다. 통과.
9. **중첩 Grid 소유권** — 한 노드의 container ID가 외부/내부 Grid 사이에서 뒤섞이는지 확인했다. metadata에 direct parent container를 함께 기록하고, output을 `(container, source_order)`로 정렬한다. text/image assertion이 owner를 확인한다. 통과.
10. **text 측정 source 분기** — text node가 replaced size로 기록되거나 source가 빠지는지 확인했다. `text_paragraphs` membership을 먼저 검사하고 Parley intrinsic-size type을 저장한다. 통과.
11. **text min/max 및 양 축** — inline width만 보존해 block 정보가 손실되는지 확인했다. `TextIntrinsicInlineSizes` 전체를 보존하고 Ahem 회귀가 min `70×20`, max `100×10`을 검사한다. 통과.
12. **font registry 식별 정보** — numeric revision만 남겨 다른 registry 결과와 혼합되는지 확인했다. 전체 intrinsic-size 객체를 clone하고 test에서 revision을 확인한다. 통과.
13. **replaced natural/used size 혼동** — definite constraint 적용 결과를 natural size로 내보내거나 edge를 두 번 적용하는지 확인했다. `replaced_sizing.natural` content-box 크기만 넣고 style을 별도 보존하며 회귀가 `24×16`을 확인한다. 통과.
14. **layout revision 시점** — intrinsic-size snapshot revision을 합치기 전에 결과를 고정하는지 확인했다. collector는 최종 `output.revision`이 결정된 뒤 호출되고 image 회귀가 revision equality를 확인한다. 통과.
15. **item style provenance** — 변환된 Taffy style이나 이웃 노드 style을 원본 style로 노출하는지 확인했다. metadata는 입력 `LayoutNode.style`을 저장하고 raw output은 그 값을 사용한다. text/image 회귀가 equality를 확인한다. 통과.
16. **text definite cache 오염** — intrinsic probe가 최종 definite-width snapshot을 교체하는지 확인했다. 두 캐시는 분리되어 있고 low-level text test가 definite 폭 `30px` snapshot을 보존한다. 통과.
17. **예약 실패의 부분 결과** — raw vector reserve 실패가 빈/부분 success output을 내보내는지 확인했다. `try_reserve(1)` 실패는 `LayoutError`를 반환하고 compute가 `Err`를 전파한다. failpoint 후 같은 worker 재계산 회귀가 있다. 통과.
18. **callback reserve 오류 회귀** — 기존 trace의 할당 실패 처리가 새 오류 문구나 collector 때문에 느슨해지는지 확인했다. callback 경로의 `try_reserve` 및 오류 기록은 그대로 있으며 새 문구가 양쪽 기록을 설명한다. 기존 및 전체 layout regression 통과.
19. **보조 flow-only 계산 오염** — flow-only tree가 intrinsic metadata를 계산하거나 결과를 중복 생성하는지 확인했다. metadata builder가 flow-only에서 즉시 빈 map을 반환한다. full visual tree가 반환하는 경로와 구별된다. 통과.
20. **상태 과대 표기** — 단계 A 입력 보존을 C11.4 contribution/track sizing 완료로 표시하는지 확인했다. `spec/STATUS.md`는 C11.4 미완료를 유지하고 0059·계획 문서는 edge 조정, span, automatic minimum, track sizing, auto-repeat 및 앱/WPT 범위를 남겼다. 통과.

## 검토 결과와 제한

검토에서 고친 추가 결함은 찾지 못했다. 이전 raw-input 구현 검토에서 확인한 leaf provenance와 metadata 통합 수정은 최종 diff에 반영되어 있다. `spinon-layout` 161개 테스트, scoped Clippy, workspace 테스트, rustfmt 및 `git diff --check`가 통과한 기록을 대조했다. 전체 문서 빌드는 현재 저장소의 다수 기존 evidence/plan dead link 때문에 실패한다. 실패 목록에 변경한 raw-input review 링크는 없고, 이 변경에서 문서 빌드 전체 통과를 주장하지 않는다.

PR #340 생성 뒤 추가한 `spec/STATUS.md`의 PR·검토 자료 링크와 C11.4 미완료 상태를 20번 관점에서 다시 확인했다.

이 PR은 Rust layout 입력 수집만 추가한다. 계산된 CSS contribution, padding/border 조정, span/automatic minimum, track sizing, anonymous text item, non-leaf aggregate, FFI, Android/iOS runtime, simulator 또는 WPT 검증을 포함하지 않는다.
