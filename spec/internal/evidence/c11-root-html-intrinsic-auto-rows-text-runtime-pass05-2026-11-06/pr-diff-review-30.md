# C11 Root HTML text min-content rows: PR 변경 적대 검토 30개

- 대상 PR: [#1190](https://github.com/ohah/spinon/pull/1190)
- 기준 diff: `origin/main...6a8134650f5692d5a78a83f64dc7c7912bcee107`
- 검토 범위: PR의 실제 코드·테스트·계획·Chrome observation/expected 값·Android/iOS simulator 증거 변경분
- 판정: 아래 30개 경계에서 병합 blocker를 찾지 못했다. 이 기록은 전체 C11/C11.4 적합성이나 CI 통과를 뜻하지 않는다.

## 실패 관점

1. **기존 empty-item 경로 회귀** — empty direct HTML `div`는 이전의 고정 크기·auto placement 조건을 그대로 요구한다. 기존 one/two-row regression test도 남아 있다. **PASS**
2. **Grid owner 혼합** — 후보가 여러 개이거나 visible Grid owner가 둘 이상이면 거부한다. owner-count 조건을 넓히지 않았다. **PASS**
3. **다른 intrinsic-track route와 충돌** — explicit intrinsic tracks와 이 implicit-row 후보는 같은 route로 섞이지 않도록 candidate와 route 조건에서 제외한다. **PASS**
4. **지원하지 않는 auto-row track list** — typed track list가 단일 `min-content`가 아니면 선택되지 않으며 기존 unsupported 경로가 유지된다. `max-content`와 복수 track rejection control이 있다. **PASS**
5. **Direct Grid whitespace** — 기존처럼 ASCII CSS whitespace만 무시하며 다른 direct text는 unsupported 처리한다. **PASS**
6. **잘못된 item namespace/tag** — HTML namespace의 direct `div`만 인정하고 다른 tag·foreign namespace는 거부한다. **PASS**
7. **Nested Grid item** — Grid container인 direct child는 item 하위 집합에서 제외된다. **PASS**
8. **중첩 markup과 추가 child** — text-bearing item은 direct text child 하나만 허용한다. nested span과 추가 child를 layout 실패/미게시 결과로 확인하는 negative control이 있다. **PASS**
9. **text source 변조** — text는 `A A A A A A`와 정확히 일치해야 한다. 다른 문자는 실제 측정하지 않고 거부한다. **PASS**
10. **텍스트 item 수** — 두 번째 text-bearing item은 checked counter를 넘긴 뒤 거부한다. 기존 빈 item은 여럿 유지할 수 있다. **PASS**
11. **숨김 item 배치** — `display:none` child는 `nodes.hidden` 검사에서 건너뛰며 별도 runtime test에서 text paint 부재와 이후 item의 다음 row 배치를 확인한다. **PASS**
12. **text node identity/revision** — style/layout test는 contribution에 text owner NodeId와 source text를, runtime test는 paint source의 generation·document/tree/style revision 일치를 확인한다. **PASS**
13. **text item width/height 변경** — text item은 `auto` width와 height만 받고 fixed width/height mutation은 atomically reject한다. **PASS**
14. **empty item 크기 회귀** — empty item은 finite nonnegative fixed width/height를 계속 요구한다. auto-size mutation rejection이 있다. **PASS**
15. **min/max·ratio·position 제약** — min/max는 initial auto/none만, aspect ratio 없음, static positioning만 허용한다. placement/style projection에서 위반을 거부한다. **PASS**
16. **명시적 placement** — row/column start/end 네 값이 모두 typed `Auto`여야 한다. 명시적 row placement negative control이 거부된다. **PASS**
17. **font family/weight/style 변조** — text gate는 computed monospace, 400, normal을 확인하고 다른 family를 reject하는 matrix가 있다. **PASS**
18. **font size/line-height 변조** — computed 10px size와 10px line-height 조건을 확인하며 잘못된 line-height를 reject한다. **PASS**
19. **white-space/wrapping** — computed collapse/wrap와 normal word-break·overflow-wrap만 허용하고 nowrap 변조가 reject된다. **PASS**
20. **inline 방향·writing mode·변환** — LTR, horizontal-tb, no text transform, start alignment를 확인한다. text gate에서 허용 값을 바꾸지 않는다. **PASS**
21. **간격·들여쓰기 속성** — default Root HTML paragraph-style 경로는 letter-spacing, word-spacing, text-indent의 zero/normal 외 값을 거부하므로 이 text route에서 미반영 값이 조용히 측정에 들어가지 않는다. **PASS**
22. **auto width의 stretch 전제** — Chrome 관측은 authored `auto`와 resolved 40px를 별도 보존한다. Grid projection은 normal justify-items를 사용하고 테스트가 40px frame을 확인한다. **PASS**
23. **고정 추정치로 대체하는 오류** — style/layout 테스트는 실제 `TextLayoutEngine` 결과의 두 line과 20px height를 inspect하고, trace에 text item의 min-content contribution 20px와 NodeId를 확인한다. **PASS**
24. **row 계산 및 sibling 간섭** — 한 개/두 개 implicit row 사례에서 `[10,20,34]` track 및 item frame을 비교하고 마지막 item의 overflow를 유지한다. **PASS**
25. **trace 범위 확장** — `track_sizing_trace_enabled`는 exact Root HTML intrinsic auto-row route flag에만 연결했다. 다른 Grid profile의 trace default를 바꾸지 않았다. **PASS**
26. **partial publication** — 미지원 text source/topology/style matrix는 `layout.completed == None`을 요구한다. 오류 전에 일부 frame/render snapshot이 보이는 성공 결과를 허용하지 않는다. **PASS**
27. **paint ordering** — runtime test는 DOM item order와 box/text command order를 검사해 owner 배경 뒤 text, 다음 item 배경 앞 text를 확인한다. **PASS**
28. **paint glyph/source 누락** — 여섯 A glyph, 두 baseline, text owner frame과 source revision이 모두 있는지 검사한다. **PASS**
29. **Chrome oracle/fixture 혼동** — fixture hash가 observation 및 expected JSON에 연결되고, 관측 line/frame과 독립 계산을 분리한다. Chrome 결과를 Spinon 또는 WPT 적합성으로 확대하지 않는다. **PASS**
30. **Simulator 증거 과장** — Android/iOS는 동일 fixture source hash, `layout=ready`, 7 boxes와 화면을 보였으나 README가 PixelCopy/Metal readback 부재를 명시한다. 스크린샷만으로 최종 compositor 픽셀 동등성을 주장하지 않는다. **PASS**

## 병합 게이트 상태

- `main` 보호 설정 조회에서 required status checks는 `null`, required approvals는 `0`이었다.
- PR #1190의 GitHub check rollup은 비어 있고 `gh pr checks 1190`은 “no checks reported”를 반환했다. CI 통과로 간주하지 않는다.
- PR 상태는 검토 시점 `OPEN`, merge state `CLEAN`; 별도 review는 아직 등록되지 않았다.
- 이 검토는 소스/diff 검토다. 이 turn에서는 테스트·앱 빌드를 재실행하지 않았다.
