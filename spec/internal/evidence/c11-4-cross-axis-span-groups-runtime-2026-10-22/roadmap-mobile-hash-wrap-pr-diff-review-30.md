# 공개 로드맵 모바일 해시 줄바꿈 PR diff 실패 관점 검토 30개

**대상:** `spec/public/roadmap.html`의 `.kanban-note code` 줄바꿈 한 규칙과 연결한 C11.4 runtime evidence 인덱스·SHA-256 목록. 390px 모바일 Agent Browser 측정에서 확인한 기존 long-hash overflow에 한정해 검토했다.

| # | 실패 관점 | diff 판정 |
|---:|---|---|
| 1 | 모바일에서 넘침을 만든 긴 문자열이 현재 #510 요약의 수치나 링크인가? | DOM에서 overflow 폭을 만든 64자 APK SHA는 이전 anonymous Grid 행이며 새 #510 행의 해시가 아니다. |
| 2 | 선택자가 코드가 아닌 일반 로드맵 문장 전체에 줄바꿈을 강제하는가? | `.kanban-note code`만 선택해 일반 문장과 링크 텍스트는 그대로 둔다. |
| 3 | 규칙이 요약 외의 code block, `<pre>` 또는 문서 전체에 적용되는가? | note 안의 inline `code` 요소로 범위를 제한했다. |
| 4 | 한 줄에 들어가는 짧은 코드 값도 항상 낱글자로 분해되는가? | `overflow-wrap:anywhere`는 필요한 경우에만 추가 break opportunity를 허용한다. |
| 5 | 공백과 하이픈이 있는 기존 코드·식별자 줄바꿈을 우선 무시하는가? | 기본 `word-break`를 바꾸지 않아 기존 break 지점을 보존한다. |
| 6 | SHA 원문 자체를 편집하거나 복사 가능한 글자를 제거하는가? | HTML 텍스트 바이트는 그대로 두고 렌더링 시 줄바꿈만 허용한다. |
| 7 | 긴 줄 때문에 body의 scroll width가 계속 viewport보다 커지는가? | 390px Agent Browser 검증에서 전체 `documentWidth`가 viewport와 같고 대상 hash가 note 안에서 줄바꿈한다. |
| 8 | 이번 #510 note 자체가 모바일 폭을 넘는다고 잘못 판정하는가? | 새 note의 client/scroll width는 362px로 같고 right edge는 390px viewport 안이다. |
| 9 | 화면이 390px보다 넓은 경우에도 콘텐츠가 불필요하게 한 글자씩 끊기는가? | `anywhere`는 연속 문자열에만 보조 break를 주며 보통 단어 간격은 유지한다. |
| 10 | desktop 1440px 화면에서 글줄이 바뀌거나 열 폭이 줄어드는가? | 폭이 충분하면 추가 break point를 사용하지 않는다. |
| 11 | note의 padding 또는 좌우 gutter를 수정해 기존 카드 폭을 바꾸는가? | `.shell`, `.kanban-note`의 크기 계산은 변경하지 않는다. |
| 12 | 계획·근거·PR 링크의 hit target이나 URL이 변하는가? | anchor selector와 href는 변경하지 않는다. |
| 13 | `<code>`의 텍스트 선택·접근성 이름을 바꾸는가? | DOM text와 요소 순서를 그대로 보존한다. |
| 14 | `white-space:nowrap` 같은 주변 규칙과 충돌해 줄바꿈을 막는가? | note inline code에 별도 `nowrap` 규칙이 없고 새 선언이 overflow wrap을 허용한다. |
| 15 | sticky top bar, 검색창 또는 필터 컨트롤을 잘못 줄바꿈하는가? | 다른 selector 범위와 레이아웃 속성은 손대지 않는다. |
| 16 | 좁은 화면의 top navigation 가로 스크롤 영역에 영향이 있는가? | `.topnav` 규칙과 분리되어 있다. |
| 17 | 고정 1120px roadmap SVG를 잘라 그림의 선·도형을 바꾸는가? | SVG 크기는 변경하지 않으며 기존 `.kanban-frame`의 내부 가로 스크롤을 유지한다. |
| 18 | 페이지의 86px overflow를 SVG 때문이라고 잘못 진단하는가? | Agent Browser에서 viewport 밖 non-SVG 요소는 이전 note의 64자 code hash였다. |
| 19 | hash 줄바꿈 수정 때문에 앱 페이지의 런타임 동작이나 제품 CSS가 바뀌는가? | 정적 `spec/public/roadmap.html`에만 적용한다. |
| 20 | body 텍스트 검색이나 카드 필터의 문자열을 수정하는가? | 검색 대상 텍스트는 그대로라 필터 로직 입력이 변하지 않는다. |
| 21 | C11.4의 77/213 완료 수나 checkbox를 완료로 바꾸는가? | 공식 상태 문구와 대장은 수정하지 않는다. |
| 22 | #510 merge SHA 또는 simulator 수치가 바뀌는가? | 기존 note의 런타임 문구·수치는 그대로 둔다. |
| 23 | linked plan, README, review, screenshot route가 바뀌는가? | 기존 href 문자열을 유지한다. |
| 24 | Android AVD와 iOS Simulator의 경계 설명이 누락되는가? | runtime README를 통해 같은 화면·경계 설명을 유지한다. |
| 25 | CSS parser가 선언을 다른 속성 값으로 읽는가? | 표준 CSS `overflow-wrap:anywhere` 한 선언만 추가했다. |
| 26 | selector specificity가 주변 note 색상·폰트 규칙을 덮는가? | 새 selector는 `overflow-wrap`만 설정해 다른 computed style과 무관하다. |
| 27 | 줄바꿈 규칙이 모든 `<code>`의 whitespace 처리를 바꾸는가? | `white-space`, `word-break`, font, line-height는 바꾸지 않는다. |
| 28 | 작은 note 안에서 긴 hash가 잘리고 복구 불가능해지는가? | clipping이나 ellipsis를 추가하지 않고 전체 값을 표시한다. |
| 29 | 근거 페이지에 이 responsive 보정의 별도 검토 기록이 남는가? | 30개 관점을 이 evidence 디렉터리에 추가하고 SHA-256 목록에 반영한다. |
| 30 | 수정 뒤 모바일 가로 폭 결과를 실제 브라우저 대신 정적 계산으로 주장하는가? | 최종 PR 검증에서 Tailnet 제공 페이지를 Agent Browser 390px viewport로 다시 측정한다. |

**결론:** 변경은 이전 roadmap note의 긴 인라인 hash가 좁은 화면의 document width를 넓히지 않게 하는 표시 규칙 하나다. 공식 구현 상태, simulator 결과와 C11.4 완료 판정은 이 PR에서 바꾸지 않는다.
