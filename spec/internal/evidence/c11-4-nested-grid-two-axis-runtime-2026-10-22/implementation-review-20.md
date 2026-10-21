# C11.4 nested Grid 양 축 구현·런타임 실패 관점 검토 20개

구현을 완료한 뒤 계획 검토와 구분해 코드 경계, 런타임 trace, Android/iOS 실행을 각각 다시 대조했다. 검토 때까지 해결되지 않은 기능 결함은 찾지 못했다.

1. **Scene allowlist** — runtime과 FFI가 같은 네 새 ID만 허용하며 자유 문자열에서 scene을 만들지 않는다.
2. **depth-one 구분** — root→inner 두 Grid control이 depth-two 세 장면과 같은 sizing 결과를 낸다.
3. **depth-two topology** — root→middle→inner 세 Grid와 단일 text leaf가 고정된다.
4. **자식 수와 순서** — 예상하지 않은 sibling, 누락 노드, 순서 변경은 adapter 검증에서 거부된다.
5. **width·height 대응** — 양 축 scene의 각 Grid width와 height가 동일한 min/max constraint인지 검사한다.
6. **Columns·Rows 대응** — 각 Grid의 단일 column과 row track이 해당 constraint와 일치하며 auto-repeat·areas는 없다.
7. **지원 조합 최소화** — projection gate는 Grid의 `(min,min)` 또는 `(max,max)`만 받으며 교차 조합을 추가하지 않는다.
8. **기존 경로 보존** — 일반 text projection은 새 gate를 `false`로 호출하고 기존 네 inline-only scene도 false로 남는다.
9. **다른 fixture 경계** — anonymous text/image, implicit track, nested Grid intrinsic 등 다른 adapter가 명시적으로 false다.
10. **text metric 전달** — Ahem direct text source가 각 inner Grid에 연결되고 min-content width·height는 70×20이다.
11. **max-content 전달** — 같은 font/source에서 max-content width·height는 100×10이다.
12. **축 계산 순서** — column 결과에 따른 줄바꿈이 row contribution으로 이어지는지 trace에서 Columns와 Rows를 각각 확인한다.
13. **기여 대상** — 각 부모 Grid가 바로 아래 Grid의 contribution을 받고 leaf input을 중간 Grid에 잘못 붙이지 않는다.
14. **전체 trace 깊이** — depth-two에서는 세 Grid 각각에 Columns·Rows pass와 final track이 존재한다.
15. **정밀도** — Rust assertion은 track contribution에 1/64 CSS px tolerance를 적용하고 정수 기대 frame을 별도로 검사한다.
16. **node frame** — root-relative node ID/order 및 70×20 또는 100×10 크기를 전 노드에서 대조한다.
17. **잘못된 입력 실패** — adapter style/tree 검사 이전에 새 projection 경로로 진입할 수 없고 오류 경로가 유효 layout을 반환하지 않는다.
18. **Android host mapper** — 네 scene이 AVD host allowlist에 있고 기존 기대 root/item/node-frame mapper가 접미사를 정규화한다.
19. **iOS host mapper** — 네 scene이 Simulator allowlist와 동일한 frame/count/intrinsic 검사를 통과한다.
20. **실행 증거 경계** — Android 4/4, iOS 4/4에서 trace와 draw/present를 확인했다. screenshot은 glyph paint, hardware GPU, iOS compositor 이후 pixels를 입증하지 않는다.

**결론:** 두 축 intrinsic sizing은 opt-in fixed fixture 경계 안에서 예상 수치와 맞는다. arbitrary nested Grid, 일반 CSS projection, WPT 적합성으로 확장해 판단하지 않는다.
