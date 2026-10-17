# C15.3 HostDocument→runtime paint 계획의 실패 관점 검토 20개

검토 대상은 고정 Ahem direct-text를 실제 HostDocument layout snapshot에서 runtime glyph paint까지 전달하는 계획이다. 각 항목은 별도의 입력·불변 조건을 다루며 구현 전 검토다. 이 문서는 구현 코드나 실행 결과를 통과했다고 주장하지 않는다.

1. **Chrome oracle 범위가 달라짐** — C15.1 `long-word`의 기존 HTML, 관찰 JSON, Ahem hash를 고정하고 새 CSS 입력을 만들지 않는다. 원 artifact와 입력 SHA가 다르면 실행을 시작하지 않는다.
2. **line advance를 box width로 잘라냄** — parent는 `15×10`, 한 줄 text advance는 `20px`로 각각 판정한다. `20→15` clamp 변형이 별도 negative control에서 드러나야 한다.
3. **text source가 수동 `TextSource`로 대체됨** — HostDocument에서 direct Text node를 읽어야 한다. 별도 Rust string으로 shaping하면 해당 gate의 pass가 아니다.
4. **Text node를 element style key로 잘못 사용함** — style map은 visible element에만 매핑한다. 원본 `Text` node id는 `TextSource`와 UTF-16 mapping에 보존한다.
5. **HostDocument source order 변경** — paragraph 안에 `X`, 빈 text, `X`를 두는 negative control은 같은 DOM 순서와 합산 advance를 내고 빈 node는 layout source에 glyph를 만들지 않아야 한다.
6. **잘못된 UTF-16을 손실 변환함** — 단독 surrogate mutation은 replacement character로 성공 처리하지 않고 기존 `InvalidTextEncoding` 경계에서 중단해야 한다.
7. **다른 paragraph snapshot 선택** — map의 paragraph id가 실제 Block owner와 다르면 runtime paint 입력을 만들지 않아야 한다.
8. **Taffy measure와 final width 불일치** — width key가 `15px`가 아닌 결과 snapshot은 거부한다. 두 번째 measure callback의 stale result를 재사용하는 변형도 실패해야 한다.
9. **auto height와 line box 분리** — output Block height는 줄 상자 `10px`와 일치해야 하며 단순 hardcoded `10`만으로는 line-layout linkage 증거가 아니다.
10. **content origin과 border-box origin 혼동** — zero border/padding 입력만 허용하고, 이를 바꾼 negative control은 해당 제한을 거부해야 한다. layout origin을 임의 상수로 보정하지 않는다.
11. **baseline을 line top으로 취급** — Ahem baseline과 raster bitmap top을 기존 layout/raster 결과에서 가져온다. renderer가 glyph별 baseline을 0으로 고정하면 내부 geometry 비교가 실패해야 한다.
12. **raster에서 DPR 두 번 곱함** — DPR 1만 지원하고 raster scale·RuntimeRenderText scale을 같은 report에서 비교한다. scale을 중복 적용한 변형은 glyph bounds가 달라져야 한다.
13. **paint glyph 순서를 재정렬함** — raster glyph order와 source ranges를 보존한다. 역순 glyph mutation은 per-glyph x와 `XX` 이미지 검증에서 실패해야 한다.
14. **owner clip이 overflow-visible ink를 자름** — paragraph 밖으로 나가는 두 번째 X의 opaque sample을 확인한다. box frame 자체를 implicit clip으로 처리하는 negative control이 실패해야 한다.
15. **viewport 밖을 overflow 증거로 오인함** — Block을 viewport 안쪽에 둬 두 번째 glyph가 viewport 안이면서 parent 밖에 오도록 고정한다. viewport clip은 parent overflow 동작과 구분한다.
16. **runtime paint order를 생략함** — 테스트 scene의 Block 배경/텍스트 순서를 고정하고 `DrawText`가 동일 owner의 배경 뒤에 한 번만 배치되는지 검증한다.
17. **layout revision과 runtime render key가 엇갈림** — HostDocument generation/document/render-tree와 style/environment revision이 어긋나면 paint 이전에 거부한다. 같은 NodeId를 재사용한 다른 generation을 stale control로 둔다.
18. **font registry 변경 뒤 이전 glyph bitmap 사용** — Ahem 등록 revision을 layout/raster source와 연결한다. 레지스트리 revision 변경 뒤 이전 raster를 새 layout에 재사용할 수 없어야 한다.
19. **feature 전용 자료가 기본 artifact로 누출됨** — 기본 build와 opt-in build의 symbol·embedded Ahem payload를 각각 검사한다. fixture 위치만 확인하고 binary 경계를 추정하지 않는다.
20. **Simulator screenshot을 일반 적합성으로 확대함** — Android AVD PixelCopy와 iOS Simulator drawable phase, 기기/OS, source revision을 분리 기록한다. 전체 CSS, 실기기, iOS compositor/scanout 주장은 금지한다.

## 계획 검토 결론

위 실패 조건을 계획의 gate와 evidence 형식에 반영했다. 다만 일반 CSS style projection이 계속 Text node를 거부하는 한계는 의도적으로 유지하며, 이 slice는 C15/C15.3 완료 근거가 아니다. 구현은 계획 amendment PR이 병합된 뒤에만 시작한다.
