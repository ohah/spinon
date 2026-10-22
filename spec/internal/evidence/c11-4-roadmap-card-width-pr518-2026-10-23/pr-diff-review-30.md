# C11.4 roadmap 카드 폭 PR diff 적대 검토 30개

**검토 대상:** `spec/public/roadmap.html`의 C11.4 max-content span-groups runtime 카드 class 변경 한 줄. 기존 페이지 CSS 및 같은 `.hero` grid의 다른 진행 카드와 비교했다.

| # | 실패 경로·불변식 | diff와 근거 | 판정 |
| --- | --- | --- | --- |
| 1 | 최신 진행 카드가 desktop 2열 중 좁은 오른쪽 칸에 계속 배치된다. | `.hero > .hero-progress`는 `grid-column: 1 / -1`로 두 열 전체를 차지한다. 새 카드에 기존 `hero-progress` class를 적용한다. | 통과 |
| 2 | 새 class가 기존 CSS 선택자에 매치되지 않는다. | 카드는 `.hero`의 direct child `<p>`이며 선택자 조건을 만족한다. | 통과 |
| 3 | class가 inline child에만 적용돼 grid item 폭이 그대로다. | class는 해당 `<p>` 자체에 있어 부모 grid placement를 결정한다. | 통과 |
| 4 | 넓은 화면에서 카드가 첫 번째 열만 차지한다. | 기존 규칙 `grid-column: 1 / -1`은 첫 선부터 마지막 선까지 확장한다. | 통과 |
| 5 | 두 열 사이 gap이 카드 content에 잘못 포함된다. | CSS Grid가 두 track과 gap을 포함해 spanning item 폭을 계산한다. 별도 폭 산식은 없다. | 통과 |
| 6 | 900px breakpoint의 단일 열에서 span 지정으로 카드가 사라진다. | 한 열의 마지막 grid line까지 확장하는 `1 / -1`은 유효하며 같은 규칙을 기존 full-width 카드가 사용한다. | 통과 |
| 7 | 600px 이하에서 shell 축소 뒤 카드가 viewport를 넘는다. | `.hero > * { min-width: 0 }`와 shell 폭 규칙을 유지하고 grid placement만 바꾼다. | 통과 |
| 8 | 긴 scene ID가 grid 폭을 강제로 넓힌다. | `.kanban-note code { overflow-wrap: anywhere }`가 기존 적용 상태다. | 통과 |
| 9 | 링크 줄이 본문 바깥으로 넘친다. | 링크는 같은 `<p>`의 inline content로 유지되고 폭이 넓어진다. href는 바뀌지 않는다. | 통과 |
| 10 | 계획 링크가 다른 문서로 향한다. | `/spinon/plan/c11-4-cyclic-minmax-calc-cross-axis-max-content-span-groups.html`은 그대로 둔다. | 통과 |
| 11 | runtime 근거 링크가 끊긴다. | README와 두 검토 문서의 href는 diff에서 변경하지 않는다. | 통과 |
| 12 | Android screenshot 경로가 바뀐다. | 성공 AVD PNG URL을 그대로 보존한다. | 통과 |
| 13 | iOS screenshot 경로가 바뀐다. | 성공 Simulator PNG URL을 그대로 보존한다. | 통과 |
| 14 | 구현 PR 링크가 빠지거나 다른 PR로 연결된다. | GitHub PR #517 href가 그대로 남는다. | 통과 |
| 15 | 공식 상태 anchor가 무효화된다. | `/spinon/STATUS.html#c11-css-grid` href를 변경하지 않는다. | 통과 |
| 16 | plan과 runtime 결과 카드 DOM 순서가 바뀐다. | DOM 순서는 그대로 두고 runtime 카드만 spanning한다. | 통과 |
| 17 | 앞 mixed-span 결과 카드가 함께 full-width가 된다. | 앞 카드 class는 변경하지 않아 기존 2열 위치를 유지한다. | 통과 |
| 18 | 다음 overview section이 같은 grid column으로 확장된다. | `.hero-progress`는 새 카드 한 곳에만 붙고 뒤 `section.overview`는 별도 grid child다. | 통과 |
| 19 | 장면 수·geometry·pixel 표본 수가 편집 중 변형된다. | 수정 범위는 class attribute 하나이며 본문 수치와 링크는 그대로다. | 통과 |
| 20 | C11.4 전체 완료 문구로 의미가 바뀐다. | `77/213`과 C11.4 미완료·미검증 범위 문장은 수정하지 않는다. | 통과 |
| 21 | 공식 상태 checkbox가 체크된다. | 이 변경은 `spec/STATUS.md`에 닿지 않는다. | 통과 |
| 22 | accessible 문서 순서나 요소 역할이 바뀐다. | 의미 요소는 `<p>`와 링크로 유지되며 heading 구조도 그대로다. | 통과 |
| 23 | `hero-progress`가 새 JS hook을 실행한다. | 기존 사용처를 검색했으며 이 class의 동작은 CSS grid placement뿐이다. | 통과 |
| 24 | HTML attribute가 중복되어 class가 무시된다. | 하나의 `class` attribute에 `kanban-note hero-progress`를 지정한다. | 통과 |
| 25 | 태그 닫힘 누락으로 뒤 내용이 카드 안에 합쳐진다. | `<p>` markup과 닫는 tag는 수정하지 않는다. | 통과 |
| 26 | 새로운 CSS가 다른 페이지나 card theme에 영향을 준다. | stylesheet는 수정하지 않고 기존 `.hero > .hero-progress` 규칙만 재사용한다. | 통과 |
| 27 | color, type scale, spacing, border를 임의로 바꾼다. | visual placement 외 스타일은 기존 class 규칙 그대로다. | 통과 |
| 28 | RSPress route 수나 정적 asset을 불필요하게 늘린다. | 새 route·asset·dependency가 없으며 기존 HTML 한 파일만 바뀐다. | 통과 |
| 29 | unrelated simulator/Tailnet evidence가 PR에 함께 들어간다. | stage 대상은 roadmap HTML, 이 검토 파일, 표시 확인 screenshot으로 한정한다. | 통과 |
| 30 | 실제 Tailnet 화면에서 최종 카드 폭이 full-width인지 확인하지 않는다. | 1280×633 Agent Browser viewport에서 `.hero` grid는 `908px 288px`, 대상 카드 bounding box는 `x=20px`, `width=1240px`, `grid-column: 1 / -1`이다. 캡처에서도 같은 폭으로 렌더링됨을 확인했다. | 통과 |

## 결론

초기 화면에서 카드 box가 `x=972px, width=288px`로 오른쪽 좁은 열에 놓였다. 기존 전체 폭 카드와 같은 `hero-progress` class를 붙여 넓은 viewport에서 `.hero` 두 열을 가로지르게 했다. 패스53 Tailnet Agent Browser에서 대상 카드의 폭이 shell의 가용 폭 1240px과 일치함을 확인했다. 좁은 viewport에서는 기존 single-column 규칙을 따른다.
