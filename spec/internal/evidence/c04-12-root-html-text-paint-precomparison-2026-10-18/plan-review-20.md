# C04.12 parsed root HTML direct text paint · 계획 실패 관점 검토 20개

계획·Chrome 사전 비교·현재 root HTML/HostDocument/C15 text layout/paint source를 서로 대조했다. 구현 코드는 아직 변경하지 않았다. 4번과 15번에서 계획 경계 결함을 찾아 계획에 반영한 뒤 해당 관점을 다시 읽었다.

1. **문서 입력 identity** — 성공 scene은 `RuntimeSession::new_with_root_html()`의 실제 HostDocument snapshot에서 온 노드만 쓴다. synthetic DOM/fixture document 성공 경로를 금지했고, 현재 C04.12 block profile에서 visible text가 거부되는 기존 경계와 일치한다. **통과.**
2. **HTML parser 중복** — `ParsedRootHtml`가 `html5ever` tree sink 결과를 product HostDocument로 변환하는 기존 흐름을 재사용한다. 별도 tokenizer, reparsing, JS 합성은 계획에서 제외했다. **통과.**
3. **원본 Text owner** — owner `NodeId`와 HostDocument UTF-16 문자열을 text input까지 유지한다. `HostDocumentBridge`가 source text를 UTF-16 code unit으로 기록하는 기존 변환과 맞는다. **통과.**
4. **block 사이 markup whitespace** — 처음 계획은 whitespace-only node를 애매한 입력으로 전부 거부했지만 성공 fixture의 들여쓰기 자체가 이런 Text node를 만든다. Chrome의 두 block y 좌표와 body/main 높이는 이 fixture에서 별도 line box가 생기지 않음을 보인다. C15 HostDocument collector도 collapsible HTML whitespace를 선택적으로 건너뛴다. 계획과 근거 문구를 `white-space:normal`의 고정 성공 계약으로 고쳤다. **수정 후 통과.**
5. **empty Text node** — empty source는 C15 paragraph 수집기에서 shaping source로 추가되지 않는다. 계획은 glyph 또는 line box를 만들지 않는다고 정하고 구현 실패 주입에서 별도로 확인하도록 했다. **통과.**
6. **숨김 text 누출** — `display:none` text가 HostDocument에는 남지만 layout/paint에서 빠지는 조건을 명시했다. Chrome Range가 비고 상자가 `0×0`인 negative control이 있다. **통과.**
7. **head/style/title 비가시성** — 이들의 parser text가 content glyph로 잘못 섞이면 즉시 보일 수 있다. 계획은 비표시 text와 hidden subtree가 glyph/height를 만들지 않도록 요구한다. **통과.**
8. **computed CSS 우회** — fixture 선언만 신뢰하지 않고 Stylo computed style을 typed input으로 확인한다. text shaping·fragment origin·line break에 영향을 주는 미지원 non-initial 값은 fail-closed하도록 범위를 보강했다. **통과.**
9. **Ahem loader 경계** — Chrome fixture는 `@font-face`로 Ahem을 불러오고 runtime fixture에서는 그 loader를 뺀다. 앱 harness는 동일 hash asset을 자체 registry에 등록한다. 이 자료를 앱 `@font-face` 지원 증거로 쓰지 않는다. **통과.**
10. **실제 shaping 결과 사용** — 폭을 font-size나 문자 수로 추정하지 않고 기존 text layout/shaping API 결과를 쓴다. Ahem hash와 Chrome text Range가 oracle을 고정한다. **통과.**
11. **Taffy measure/final width 일치** — available content width `15px`에서 생성한 immutable layout snapshot만 final layout과 paint로 전달한다. width-key가 다른 measure 결과 혼용은 거부한다. **통과.**
12. **긴 단어와 부모 frame 혼동** — `XX` advance `20px`가 `15px` owner를 넘는 것이 정답이다. owner frame을 text advance로 늘리거나 advance를 frame에 맞춰 자르지 않으며, screenshot에서 visible/hidden 차이를 별도로 확인한다. **통과.**
13. **overflow owner clip** — `visible`의 x=16 sample은 잉크, `hidden`의 같은 x=16은 배경이어야 한다. layout geometry는 양쪽 다 `20px` text Range여야 하고 clip은 paint 단계에서만 다르다. **통과.**
14. **box model/profile** — content width, auto height, margin/padding/border, box-sizing을 고정했다. overflow 두 축도 pair `visible`/pair `hidden`으로 확인하므로 scroll overflow나 다른 축 조합이 몰래 허용되지 않는다. **통과.**
15. **revision provenance 위치** — 처음 계획은 raw `TextLayoutSnapshot`과 `RuntimeRenderText`가 문서·style·environment revision까지 직접 보존한다고 썼다. 실제 `LayoutInputRevision`은 source/style/environment를, `LayoutTextSnapshot`이 이를 복사하고, text snapshot은 font registry/width/source를 보존하며, runtime text는 owner/`RuntimeRenderKey`를 보존한다. 계획을 이 실제 타입 경계에 맞춰 수정했다. bridge는 scene 조립 전에 현재 revision 대조를 수행한다. **수정 후 통과.**
16. **UTF-8/UTF-16 offset 혼동** — text engine의 paragraph UTF-8 span과 실제 HTML Text node UTF-16 span을 별도로 검사하고 둘의 source mapping을 같은 node에 결속한다. **통과.**
17. **stale/부분 scene 게시** — document/render-tree, style/environment, font registry 변경은 stale snapshot 거부로 이어지고 실패 scene은 일부 glyph를 게시하지 않아야 한다. **통과.**
18. **fixture ABI 누출** — Cargo feature뿐 아니라 Android build flag/JNI/header, iOS build setting/C header 조건을 맞추고 default archive의 심볼·payload 부재와 opt-in positive control을 각각 확인하도록 했다. **통과.**
19. **surface 표본 좌표** — CSS sample point를 surface 좌표로 복사하지 않고 실제 surface size, DPR, page-scale projection에 넣어 native pixel을 선택하도록 했다. AVD는 PixelCopy, iOS는 Metal drawable 단계로 구분했다. **통과.**
20. **증거 과장/상위 완료 승격** — Chrome oracle은 해당 Chrome 155 geometry/pixel만, Android는 API 37 ARM64 AVD, iOS는 iPhone 17 Pro/iOS 26.2 Simulator의 compositor 전 drawable만 증명한다. 실기기·hardware GPU·전체 CSS/WPT·C04.12/C15 완료로 승격하지 않도록 계획과 STATUS에 제한을 썼다. **통과.**

## 사전 자료 무결성 확인

- `shasum -a 256 -c SHA256SUMS`: 두 HTML fixture, Ahem, observation JSON, PNG 모두 일치.
- `python3 -m json.tool chrome155-observation.json`: JSON 유효.
- PNG 파일 형식 관찰: `320×240`, RGB.
- 두 fixture의 semantic diff는 Chrome 전용 `@font-face` block뿐이다.
- `agent-browser session list`: `No active sessions`.
