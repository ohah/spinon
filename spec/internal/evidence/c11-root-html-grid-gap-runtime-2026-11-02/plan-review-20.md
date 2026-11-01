# C11.1 parsed Root HTML fixed-px gap 계획 검토 20개

대상은 `plan/c11-root-html-grid-gap.md`, 독립 expected JSON, Chrome 156 precomparison이다. 제품 코드는 아직 변경하지 않았다. 서로 다른 실패 경로와 계약 경계를 한 번씩 검사했다.

| # | 실패 관점 | 판정 |
| --- | --- | --- |
| 1 | RootHtml entry와 무관한 fixture-only 경로를 기본 제품 경로라 부를 수 있음 | PASS — `new_runtime_gpu_with_root_html` 및 `RuntimeRootHtmlTextPaintV1`로 소유 경계를 고정한다. |
| 2 | 모든 C11 Grid profile에 gap을 암묵 승인할 수 있음 | PASS — admission을 Root HTML profile에만 적용하고 다른 profile은 유지한다. |
| 3 | shorthand의 첫/두 번째 값이 뒤바뀔 수 있음 | PASS — `gap:5px 10px`에서 row=5, column=10을 CSS 관측으로 고정한다. |
| 4 | 두 축에 같은 gap을 가정해 매핑 오류를 숨김 | PASS — 축별 크기를 다르게 두어 frame으로 검출한다. |
| 5 | gap을 track 크기에 더하지 않아 root extent가 모자랄 수 있음 | PASS — 130/65px independent sum과 Chrome root rect를 고정한다. |
| 6 | gap을 첫 track 앞/마지막 track 뒤에도 추가할 수 있음 | PASS — 시작 좌표와 마지막 item/root edge를 각각 비교한다. |
| 7 | source-order auto-placement 대신 selector/NodeId에 scene을 맞춤 | PASS — 구현 관문에 id/selector 변경 positive control과 source order 검증을 요구한다. |
| 8 | root Grid width/height가 indefinite여서 다른 sizing 동작이 섞일 수 있음 | PASS — 양 축을 definite px로 고정한다. |
| 9 | implicit track 또는 item 부족/초과가 결과에 섞일 수 있음 | PASS — explicit 2×2 용량에 네 item만 두고 implicit track을 제외한다. |
| 10 | child size가 track을 벗어나 gap/frame을 오염시킬 수 있음 | PASS — 네 child의 definite size를 각 해당 track과 맞춘다. |
| 11 | percentage gap이 definite root에서 계산되지만 지원으로 오인될 수 있음 | PASS — 유효 percentage는 현재 범위 밖으로 거부하며 별도 negative control을 둔다. |
| 12 | `calc()` gap이 layout math table를 우회할 수 있음 | PASS — math-backed property key는 명시 거부 관문이다. |
| 13 | CSS invalid negative declaration과 런타임 computed rejection을 혼동할 수 있음 | PASS — cascade가 무시한 invalid declaration은 런타임 지원 성공으로 주장하지 않는다. |
| 14 | `normal` 기본값이 기존 0 gap을 깨뜨릴 수 있음 | PASS — normal/0은 현재 동작과 zero-gap regression으로 보존한다. |
| 15 | margin/padding/border와 gap을 한꺼번에 풀어 차이를 찾기 어려움 | PASS — 모든 box edge를 0으로 유지하고 이번 범위에서 풀지 않는다. |
| 16 | alignment 분산이 gap 자체와 합쳐질 수 있음 | PASS — 기본 alignment를 유지하고 공간 분배 keyword를 범위 밖으로 둔다. |
| 17 | typed gap을 layout에 두 번 적용할 수 있음 | PASS — 같은 snapshot의 typed `LayoutGap`을 한 번 전달하고 frame을 확인한다. |
| 18 | stale style/layout revision을 가진 scene을 gap 결과로 승인할 수 있음 | PASS — existing same-snapshot key 불변식과 revision regression을 유지한다. |
| 19 | draw/present 캡처를 pixel readback이나 하드웨어 GPU 증거로 확대할 수 있음 | PASS — 캡처 한계를 쓰고 PixelCopy/Metal readback과 hardware claim을 금지한다. |
| 20 | 제한 profile 결과로 C11 또는 전체 CSS Grid를 완료 처리할 수 있음 | PASS — 공식 체크와 `72/213`을 그대로 둔다. |

**판정: 계획 검토 20/20 PASS.** 이는 구현 정확성, runtime 실행 또는 PR diff 검토를 대신하지 않는다.
