# C11.4 반복 텍스트 행 span 구현 PR diff 검토 30개

검토 대상은 구현 PR의 실제 Rust diff, 해당 변경으로 연결된 Root HTML runtime 경로와 완료 증거다. 별도 문서인 `implementation-adversarial-review-20.md`는 입력 반례와 관찰 결과를 다루며, 이 검토는 모듈 사이의 데이터 흐름·소유권·오류 경계를 살핀다. 새 공개 CSS 적합성 선언이나 C11.4 전체 완료 판정은 검토 범위에 포함하지 않는다.

1. **기존 비-span Grid가 새 profile로 잘못 분류되지 않는가?** Style projection은 `RuntimeRootHtmlTextPaintV1`와 기존 fit-content candidate를 모두 요구한다. 기존 auto-placement profile은 기존 후보 검사 뒤에 남는다. 810-scene runtime 회귀가 통과했다.
2. **placement-mode 선택이 다른 Root HTML 경로보다 먼저 무조건 덮어쓰지 않는가?** fit-content 경로에서만 `RootHtmlFitContentAutoRowsV1`을 선택하고, explicit-placement 재분류는 이 경로를 건너뛴다. 나머지 mixed-explicit 분류는 기존 순서에 남는다.
3. **layout 공개 진입점이 내부 지원 판정을 그대로 전달하는가?** `grid.rs`는 새 topology 검사에 `pub(crate)` 경계만 제공한다. 새로운 public API나 fixture FFI가 생기지 않았다.
4. **Style projection과 Layout validator가 같은 topology 문법을 사용하는가?** 양쪽 모두 첫 direct item의 고정 placement와 세 가지 텍스트 span 형식을 확인하며, 3개·4개 child 형태의 허용 수를 일치시킨다.
5. **다른 Grid의 item ID를 topology 검사에 넘겨도 성립하지 않는가?** 호출자는 해당 Grid의 ordered direct children만 넘기며, Layout 쪽 반복 item은 그 child ID의 node와 placement를 찾아 검증한다.
6. **첫 explicit-row item이 일반 텍스트 item으로 빠져나갈 수 있는가?** 첫 item의 `children`이 비어 있고 row·column line이 모두 `1 / 2`인 경우만 통과한다.
7. **두 span-2 배치의 중복 또는 누락이 허용되는가?** `(row 2, span 2)`와 `(row 3, span 2)`의 카운트를 각각 하나로 고정하고, 4-item topology에만 `(row 2, span 3)` 하나를 허용한다.
8. **span 텍스트 wrapper에 payload가 둘 이상 들어갈 수 있는가?** Layout topology는 wrapper children을 단일 payload 패턴으로 받고, Style validation도 direct div child가 하나인지 확인한다.
9. **payload 내부 descendant가 별도 레이아웃 콘텐츠를 주입하는가?** 기존 `direct_element_children` 및 반복 텍스트 검사로 중첩 element를 배제하고 payload의 text grammar를 검증한다.
10. **텍스트 측정 경로와 CSS computed style 검사가 분리되어 서로 다른 profile을 허용하는가?** Style projection은 monospace 10px, 10px line-height, LTR·수평 writing mode 등 기존 제한을 유지한다. layout의 `has_supported_text`와 실제 runtime matrix가 같은 텍스트 입력 경로를 사용한다.
11. **HTML item 순서와 span topology 검사가 서로 다른 형상을 고르는가?** topology는 source 순서를 카운트에 사용하고 정규화는 CSS placement를 사용한다. DOM 순서를 바꾼 네 topology가 matrix에서 Chrome oracle과 일치한다.
12. **추가 explicit row·column을 기존 six-member list 경로가 허용하는가?** Style/Layout 계약은 기존 1 explicit row·1 explicit column을 요구하며, runtime 거부 matrix에서 extra row/column을 거부했다.
13. **새 220px 높이 예외가 기존 fit-content 사례에 넓게 적용되는가?** 220px 예외는 기존 repeated text profile 또는 새 span profile에만 결합된다. 다른 fit-content candidate는 기존 100px 계약을 유지한다.
14. **반복 텍스트 span에서 28px min-height floor가 의도치 않게 허용되는가?** span profile은 기존 text item's fixed 0px minimum만 허용하도록 분기하며, 28px floor는 기존 singleton case에 한정된다.
15. **미지원 partial-auto placement가 implicit auto-placement로 섞이는가?** Grid placement 양 경로에서 전체 auto인 기존 입력만 기존 sparse normalizer로 보내고, 일부만 명시된 span 입력은 topology가 일치하지 않으면 오류를 낸다.
16. **두 placement 정규화 단계가 한쪽에서만 span을 검사하는가?** `normalize_grid_layout`과 `normalize_grid_item_placements`가 모두 같은 topology 검사를 거친다. 후자는 원래 line/span 값을 보존해 normalized map에 넣는다.
17. **Grid item placement 복사가 CSS line 표현을 변형하는가?** supported topology 이후 `.clone()`으로 검증된 `GridItemInput`을 저장하므로 resolved line 및 span 값이 보존된다.
18. **새 explicit placement 경로가 기존 all-auto placement를 바꾸는가?** 새 분기는 `all_auto`인 경우 바로 기존 처리로 되돌아간다. 810-scene profile 회귀가 통과했다.
19. **Layout container validator가 span profile을 잘못된 track list에 연결하는가?** 새 profile flag는 six-member list와 `supports_span_topology`가 함께 성립할 때만 true이며, 기존 track grammar 검증을 건너뛰지 않는다.
20. **visible child filtering이 item owner 수 검사를 무력화하는가?** Style 검증은 visible direct item 수와 `grid_item_owners`에서 해당 Grid를 소유한 item 수를 비교한다. hidden child 변형도 거부 결과를 낸다.
21. **반복 text frame normalization이 기존 단일행 추정과 섞이는가?** 새 topology를 인식하면 전용 span normalizer에서 처리한 뒤 `continue`한다. 기존 source-order 기반 반복 텍스트 정규화는 기존 topology에만 적용된다.
22. **Grid trace가 다른 owner 또는 복수 trace를 섞어 쓰는가?** final row track 선택은 Grid node ID와 일치하는 trace가 하나뿐인지 확인한다. 누락·stale owner 입력 단위 테스트는 frame map을 보존하며 실패한다.
23. **row axis의 중간 sizing pass가 최종 결과로 오인되는가?** row pass 중 가장 높은 sequence와 그 pass의 마지막 stage를 사용한다. feature trace runtime 검사는 960개 scene의 expected final rows와 맞춘다.
24. **gutter 또는 collapsed track이 일반 row로 계산되는가?** gutter를 제외한 뒤 정확히 4개 track과 내부 인덱스 `1,3,5,7`, non-collapsed 상태를 요구한다. 불연속 index 단위 테스트는 거부한다.
25. **NaN·무한대·음수 base size가 frame 산식에 들어가는가?** 각 최종 row base size는 finite이고 0 이상이어야 한다. 이 조건을 만족하지 않으면 unsupported trace 오류로 끝난다.
26. **used-row metadata와 final sizing stage가 명백히 다른 track 개수를 말해도 정상화되는가?** explicit 1, negative implicit 0, positive implicit 3, collapsed 0 metadata와 final non-gutter 4개를 교차 검사한다.
27. **item frame 보정이 실제 trace geometry와 무관하게 적용되는가?** 보정 전에 item의 raw y와 height가 row prefix sum에서 유도한 span 영역에 0.01px 이내로 맞아야 한다.
28. **1/64px 정규화가 겹치는 item y 순서로 row 크기를 추측하는가?** 전용 함수는 trace에서 얻은 row size와 기존 Blink 분배 helper를 사용하고 placement line으로 row offset을 계산한다.
29. **item 외부 frame만 이동해 payload 또는 flow tree가 어긋나는가?** span subtree 순회는 `frames`와 `flow_frames`의 descendant y를 함께 보정하고, grid item block height만 snapped span 높이로 설정한다.
30. **trace 검증 오류가 완성된 부분 결과로 노출되는가?** `compute_with_measurements`는 normalization 오류를 `?`로 반환해 `LayoutOutput`을 반환하지 않는다. Runtime 거부·stale revision 테스트도 `completed=None`을 확인한다.

## 판정

검토 중 수정이 필요한 구현 PR diff 결함은 발견하지 않았다. 변경 crate Clippy와 `cargo fmt --all -- --check`는 통과했다. 전체 `spinon-layout --lib` 실행은 506개 통과, 변경 없는 C15 `host_document_collects_direct_text_and_inherited_spans_in_source_order` 테스트 1개 실패로 끝났다. 이 실패가 baseline에서도 재현되는지 별도 비교는 하지 않았으므로 baseline 원인으로 확정하지 않는다. 세부 실행 로그는 이 디렉터리에 보존했다.
