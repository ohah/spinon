# C11.4 Root HTML text intrinsic row: implementation/runtime 적대 검토 20개

이 검토는 [계획 실패 관점 검토 20개](./c11-root-html-intrinsic-auto-rows-text-plan-review-20.md)와 분리해, 구현 diff, Rust runtime/layout 결과, simulator route를 대상으로 한다. 기준 범위는 direct text child `A A A A A A`가 있는 visible HTML `div` Grid item 하나와 단일 `grid-auto-rows:min-content` pattern이다.

1. **Route 경계:** Root HTML intrinsic auto-row classifier만 확장하고 다른 Grid mode의 우선순위와 admission은 바꾸지 않았다. Track trace도 선택된 route에서만 켠다.
2. **Direct child 유형:** 빈 HTML `div`의 기존 경로와 direct text child 한 개 경로를 분리하고 다른 element·anonymous text를 허용하지 않는다.
3. **Text source identity:** text source가 item의 유일한 direct child인지 확인하고, nested markup과 추가 child는 item node/property가 붙은 오류로 닫는다.
4. **Text content 경계:** ASCII `A A A A A A` 외의 text source 변형을 atomically 거부하며 partial layout을 게시하지 않는다.
5. **Text item 개수:** 한 Grid에 두 번째 text-bearing item을 허용하지 않고, 기존 여러 empty-item 동작은 유지한다.
6. **Fixed item 회귀:** empty item은 유한한 nonnegative width/height를 계속 요구한다. text 지원이 기존 empty item 검사를 느슨하게 만들지 않는다.
7. **Auto width의 definite 사용값:** text item의 authored `width:auto`는 40px explicit column의 normal stretch로 40px가 되며, runtime frame에서 `[0,10,40,20]`을 확인한다.
8. **Auto block sizing:** text item의 height/min-height/max-height 계약을 검사하고 fixed height mutation을 fail-closed 한다.
9. **Computed text flow:** font family/weight/style/size, line-height, whitespace, word-break, overflow-wrap, direction, writing mode, transform, alignment를 computed values로 제한한다. 기존 renderer의 spacing, line-break와 indent 검증도 계속 적용된다.
10. **실측 기여:** 추정값으로 row를 키우지 않고 text layout engine의 동일 revision 측정에서 2 line와 20px height를 사용한다.
11. **Contribution trace:** Rows trace의 item identity와 `MinContent=20` 및 sizing completion의 `[10,20,34]` track base를 검사한다.
12. **Explicit track 보존:** 첫 explicit row는 10px이고 뒤 implicit rows는 text 20px 및 마지막 item 34px 기여를 각각 반영한다.
13. **Overflow:** 마지막 item은 y=30부터 34px를 차지해 y=64까지 이른다. 40px Grid 내부에 clip되었다고 보고하지 않는다.
14. **Hidden child:** `display:none` text item은 scene에서 빠지고 다음 visible item은 y=10에 놓이며 text paint가 없다.
15. **Empty child 및 one-row 대조:** text가 없거나 hidden인 경우 기존 empty direct item route의 explicit/implicit geometry 회귀를 포함한다.
16. **Atomic failure:** nested/multiple child, 다른 source, family·line-height·nowrap·fixed-size mutation은 `completed=None`을 확인한다.
17. **Revision ownership:** cascade/layout key가 일치하고 published text source의 generation, document, render-tree, style revision이 완료 layout과 일치한다.
18. **Paint contract:** 한 text owner에 six visible glyph bitmap과 두 baseline을 확인하고, owner box 다음이자 다음 item box 전에 text command가 온다.
19. **Platform comparison:** API 37 ARM64 Android AVD와 iPhone 17 Pro/iOS 26.2 모두 같은 HTML asset SHA, 기본 app route, `layout=ready`, `boxes=7`, renderer log와 screenshot을 남긴다. Android 4+2, iOS 3+3 줄바꿈은 platform font metric 차이로 기록한다.
20. **증거와 완료 판정:** screenshot은 화면 draw 증거이며 PixelCopy/Metal drawable readback을 주장하지 않는다. 이 bounded slice는 C11.4/C11이나 공식 집계 `78/214`를 완료시키지 않는다.

검토에서 확인한 초기 simulator 시도는 기존 `root-app.js`의 `#app` 계약 불일치와 fixture 교체 누락으로 실패했다. 최종 pass05에서는 fixture index asset SHA를 양 앱 bundle 안에서 확인하고 DOM API를 사용하지 않는 비어 있지 않은 smoke script로 실행해 두 플랫폼 모두 통과했다. 실패 시도는 성공 증거와 섞지 않았다.
