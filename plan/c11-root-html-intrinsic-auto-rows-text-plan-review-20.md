# C11.4 Root HTML min-content implicit row text 계획: 실패 관점 검토 20개

범위는 `RuntimeRootHtmlTextPaintV1` 기본 경로에서 단일 `grid-auto-rows:min-content` pattern에 text-bearing direct block item 하나를 허용하는 계획이다. 이 검토는 구현 전 계획의 입력·기준·거부 경계를 검토한다.

1. **Track function identity:** 지원 값은 singleton `min-content` 하나뿐이며 `auto`, `max-content`, `minmax()`를 성공 범위에 넣지 않았다.
2. **Explicit·implicit 행 구분:** 첫 item은 explicit 행 하나에 고정하고 뒤 item은 implicit 행에 자동 배치해 대상 기여가 explicit track과 섞이지 않게 했다.
3. **줄바꿈 입력:** 40px 열에서 공백으로 분리된 `A` 여섯 개를 사용해 한 줄짜리 smoke가 아닌 구체적 줄바꿈 경계를 둔다.
4. **Line-height 산술:** 기대 text 기여는 10px line-height 두 줄인 20px이며 테스트에서 track trace와 item frame을 모두 확인해야 한다.
5. **글꼴 지표 범위:** 일반 `monospace` advance는 플랫폼마다 다르므로 pinned Chrome 관측과 Android/iOS 측정을 분리하고 browser glyph advance를 규범값으로 부르지 않는다.
6. **고정 크기 대조:** explicit 10px 행과 마지막 item의 고정 34px 크기로 intrinsic text row가 explicit 행에서 복사되거나 sibling 크기로 대체되지 않는지 검사한다.
7. **행·source 순서:** 첫 item, text, 마지막 item의 DOM 순서와 paint 순서를 유지하며 ID/class 기반 배치 shortcut을 허용하지 않는다.
8. **Text 소유자 정체성:** text는 direct HTML block item 하나에만 속한다. Text node를 Grid owner로 옮기면 anonymous item이 되므로 지원 밖이다.
9. **단일 text run:** 출처와 shaping 계획이 마련되기 전에는 여러 text child, run을 나누는 주석, nested inline node를 제외한다.
10. **Computed 값:** font family/size, line-height, white-space와 Grid auto rows는 동일 style revision과 typed cascade snapshot에서 가져와야 한다.
11. **Automatic minimum:** 대상은 `min-height:auto`를 유지한다. 이를 고정 또는 0으로 바꾸어 intrinsic contribution을 가리면 안 된다.
12. **Text wrapping mode:** computed `white-space:normal`만 허용하며 nowrap/pre/overflow-wrap/word-break 동작이 의도치 않게 포함되면 안 된다.
13. **Definite inline 크기:** 40px explicit column과 `justify-items:normal` stretch가 auto-width item의 resolved inline size를 40px로 고정한다. Indefinite column과 percentage 크기는 제외한다.
14. **Visible overflow:** 마지막 행이 40px container 아래 y=64px까지 이른다. Clipping이나 viewport 경계가 track 크기를 바꾸거나 item layout을 감추면 안 된다.
15. **원자적 거부:** 잘못되거나 지원하지 않는 text item이 sibling box 또는 stale scene을 성공 결과로 게시하면 안 된다.
16. **Revision 무효화:** Text source나 font style이 바뀌면 같은 Root HTML owner를 invalidate하고 새 snapshot 게시 전에 대상 행을 재측정해야 한다.
17. **Empty·hidden 대조:** 비어 있거나 `display:none`인 child가 text contribution을 만들어내면 안 되며 기존 의미를 회귀 검사한다.
18. **자원 한계:** 고정 implicit 행 두 개는 기존 10,000-track·checked-count 상한 안에 있고, 고정 text 길이가 무제한 측정을 정당화하지 않는다.
19. **Simulator 해석:** 앱 근거는 기기/시뮬레이터, 빌드와 source hash를 기록한다. Harness가 직접 제공하지 않는 exact glyph pixel이나 track trace를 screenshot만으로 주장하지 않는다.
20. **완료 집계:** 이 slice는 제한 동작만 추가한다. C11.4/C11을 완료 처리하거나 공식 `78/214`를 증가시킬 수 없다.
