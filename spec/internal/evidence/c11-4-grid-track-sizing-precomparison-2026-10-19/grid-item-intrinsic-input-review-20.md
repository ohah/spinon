# Grid item raw intrinsic input 구현 적대 검토 20개

**검토일:** 2026-10-19
**검토 대상:** PR 후보의 `LayoutOutput.grid_item_intrinsic_inputs` 수집, metadata 구성, text/replaced source, 오류 전파 및 관련 문서. 이 검토는 기존 callback trace 구현 검토와 별도다.
**검토 범위:** Rust `spinon-layout` 경계. FFI·Android/iOS 앱 경로·Grid track sizing 구현은 이번 변경에 포함되지 않는다.

## 독립 실패 관점

1. **callback trace를 canonical input으로 오인하는 경로** — Taffy가 기록한 definite-size 응답을 min/max-content source로 취급할 수 있는지 확인했다. callback trace와 원본 목록이 별도 필드이고, 출력 통합 테스트는 definite callback trace만 있어도 Ahem min-content `70×20`, max-content `100×10`을 보존한다. 통과.
2. **callback 또는 cache hit이 없는 text source 수집** — measure callback이 호출되지 않은 상태에서 원본 수집이 실패하거나 빈 trace를 입력으로 삼는지 확인했다. 저수준 tree 테스트에서 `intrinsic_measurements`가 비어 있는 채 수집기가 Parley intrinsic sizes를 계산·반환한다. 통과.
3. **callback을 건너뛴 replaced source 수집** — Taffy 측정 결과만으로 image natural size를 얻는지 확인했다. 수집기는 measure trace가 아닌 `ReplacedSizingInput.natural`을 읽는다. 출력 테스트는 고정 크기 image에 callback이 definite space를 기록해도 natural `24×16` 입력을 따로 보존한다. 코드 경계 통과; 이 profile에서 callback 완전 생략은 재현되지 않아 그 동작을 주장하지 않는다.
4. **Grid container 소유권 혼합** — 두 container의 item이 잘못 연결되는 경로를 확인했다. metadata 생성 시 direct parent ID를 저장하고 출력 항목에 그대로 복사한다. image/text 테스트가 container ID를 확인한다. 통과.
5. **숨겨진 Grid container 처리** — `display:none`인 조상 아래의 Grid가 입력을 내보낼 수 있는지 확인했다. 반복형 DFS가 숨김 조상 상태를 전달하며 숨겨진 container와 자식을 source map에서 제외한다. 같은 검사가 visual layout tree에서 사용된다. 통과.
6. **숨겨진 direct child 처리** — 숨긴 item이 raw input 또는 다른 visible item의 순서를 밀어내는지 확인했다. source index를 먼저 열거한 다음 숨긴 item을 제외한다. text regression은 첫 hidden item을 건너뛰고 다음 visible item의 원래 순서 `1`을 확인한다. 통과.
7. **absolute/fixed child 분류** — out-of-flow child가 in-flow source input으로 들어갈 수 있는지 확인했다. metadata 생성 시 두 position을 제외한다. 현재 Grid profile은 positioned Grid item을 layout 전에 `UnsupportedGridPositionedItem`으로 거부하므로 성공 출력의 동적 positive case는 없다. 정적 경계 확인, 제한을 기록.
8. **out-of-flow 재배치 뒤 leaf 오판** — absolute descendant가 reparent된 뒤 non-leaf Grid item을 Taffy leaf로 바꾸는지 확인했다. 처음에는 Taffy child vector를 사용해 결함 가능성이 있었다. leaf 여부를 원본 `LayoutNode.children`에서 구성 metadata에 저장하도록 수정했고 수집기는 그 값을 사용한다. 수정 후 전체 layout tests 통과.
9. **non-leaf item의 잘못된 aggregate 주장** — 자식이 있는 item에서 직접 text/replaced source를 대표값으로 내보내는지 확인했다. 원본 graph 기준 `is_leaf`가 false면 수집을 생략한다. non-leaf aggregate/provenance는 명세에서 계속 미지원으로 둔다. 통과.
10. **NodeId 중복 및 다중 parent 충돌** — source map이 한 item을 여러 container에 조용히 덮어쓸 수 있는지 확인했다. map 생성은 입력 검증 뒤 실행하며 입력 graph 검증이 중복 ID·다중 parent를 먼저 거부한다. 모든 값은 하나의 metadata entry에 묶인다. 통과.
11. **source order와 CSS `order` 혼동** — 원본 child 위치를 order-modified sorting 결과로 잘못 기록하는지 확인했다. metadata는 source index만 보존하고 입력 `LayoutStyle.order`도 별도 보존한다. 비영 `order`의 현재 Grid 제한은 그대로 fail closed한다. 통과.
12. **item style 불일치** — Taffy 변환 후 값이 원본 style을 바꾸거나 다른 child style을 붙이는지 확인했다. metadata를 입력 node에서 직접 생성해 callback trace와 raw input이 동일한 item style을 쓴다. text/image regression이 style equality를 확인한다. 통과.
13. **layout/image revision 누락** — snapshot revision을 최종 결과에 반영하기 전에 source input을 찍는지 확인했다. collection은 `output.revision` 확정 뒤 호출된다. image regression에서 raw input revision이 output과 같고 intrinsic-size revision도 snapshot과 같다. 통과.
14. **text font registry identity/revision 소실** — 단순 numeric width만 저장해 서로 다른 font registry 결과를 섞을 수 있는지 확인했다. `TextIntrinsicInlineSizes` 전체를 clone해 private registry token과 revision을 유지한다. 출력에서 font revision을 확인하며 비교 API도 같은 registry token을 요구한다. 통과.
15. **text inline/block 축 또는 min/max 값 누락** — width만 계산하거나 min/max 쌍을 덮어쓰는지 확인했다. Parley 타입의 min/max inline size와 각 block height를 그대로 보존하고 Ahem `70×20`/`100×10` 수치를 고정 테스트한다. 통과.
16. **replaced natural size의 box edge 오해** — image natural size에 padding/border를 미리 더하거나 item border-box로 잘못 기록하는지 확인했다. `LayoutIntrinsicSize` content-box natural 값만 넣고 원래 style은 별도로 남긴다. 실제 contribution에서 edge를 한 번 적용할 후속 경계도 명세에 남겼다. 통과.
17. **text definite layout cache 오염** — intrinsic probe가 최종 definite-width text snapshot을 교체할 수 있는지 확인했다. intrinsic 결과는 `text_intrinsic_layouts`, definite snapshot은 `text_layouts`에 분리 저장한다. 저수준 test가 definite 폭 `30px` snapshot과 intrinsic cache를 별도로 확인한다. 통과.
18. **할당 실패와 부분 LayoutOutput** — raw input 벡터 reserve 실패가 빈/부분 성공을 반환하는지 확인했다. `try_reserve(1)` 오류를 `LayoutError`로 전파하며 compute는 output을 반환하지 않는다. failpoint test 뒤 같은 worker의 재계산 성공도 확인했다. 통과.
19. **오류 상태 누수와 다음 호출 오염** — failpoint가 thread-local 상태를 남겨 이후 계산을 실패시키는지 확인했다. RAII guard가 failpoint와 allocator 관측 상태를 초기화한다. 오류 호출 다음 같은 worker의 layout이 성공하고 미관측 상태가 비어 있는지 테스트한다. 통과.
20. **추가 map 비용·metadata 간 불일치** — node membership, leaf set, order, style이 별도 map/set에서 서로 다른 상태가 될 수 있는지 확인했다. 검토 중 발견해 container/order/leaf/style을 한 metadata map으로 통합했고 flow-only pass에서는 map 생성 자체를 건너뛴다. 통합 후 `spinon-layout` 161개 테스트와 scoped Clippy가 통과했다. 통과.

## 남은 경계

이 검토는 raw source input을 확인한 것이며 CSS Grid contribution 정규화, box edge 조정, specified/transferred/content suggestion, span, automatic minimum, track sizing, anonymous Grid text item, nested non-leaf aggregate, auto-repeat 또는 mobile runtime 검증을 대신하지 않는다. PR의 최종 전체 diff는 별도의 적대 관점 검토 대상이다.
