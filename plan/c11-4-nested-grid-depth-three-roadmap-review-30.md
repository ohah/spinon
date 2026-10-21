# C11.4 depth-three 공개 로드맵 PR 실패 관점 검토 30개

이 문서는 `spec/public/roadmap.html`에 추가한 PR #458 진행 항목을 검토한다. 구현 correctness는 별도 runtime 20개 및 PR diff 30개에서 검토했다.

| # | 독립 실패 관점 | 확인과 판정 |
| --- | --- | --- |
| 1 | 새 항목이 C11.4 영역 밖에 추가되는가? | PR #453 항목이 있는 C11.4 section 닫힘 바로 앞에 추가했다. 통과. |
| 2 | 날짜가 PR merge 시각의 한국 시간과 맞지 않는가? | GitHub `2026-10-21T16:44:41Z`를 `2026-10-22 01:44:41 KST`로 기록했다. 통과. |
| 3 | 계획 PR #457과 구현 PR #458을 혼동하는가? | roadmap 항목은 구현 PR #458을 연결하며 계획은 별도 plan route로 가리킨다. 통과. |
| 4 | head SHA를 merge SHA라고 표기하는가? | merge commit `4df8d09cb2512d5198733e2eda578ba9c8936230`을 썼다. 통과. |
| 5 | 병합 방식을 잘못 적는가? | PR metadata와 동일하게 Rebase and merge라고 적었다. 통과. |
| 6 | 빈 check rollup을 성공한 CI로 포장하는가? | check rollup이 비었다고 쓰며 green check를 주장하지 않는다. 통과. |
| 7 | required checks 상태가 실제 branch 설정과 다른가? | main protection API의 `Required status checks not enabled` 응답과 맞게 적었다. 통과. |
| 8 | 고정 fixture를 제품 지원으로 승격하는가? | “고정 scene” 및 “일반 CSS Grid 지원 미검증” 경계를 명시한다. 통과. |
| 9 | 상위 C11.4를 완료라고 읽을 수 있는가? | C11.4 checkbox가 유지되고 문구도 미완료라고 밝힌다. 통과. |
| 10 | 공식 완료 수가 바뀐 것처럼 보이는가? | `72/213`을 명시적으로 유지한다. 통과. |
| 11 | scene 수가 새 두 개와 전체 실행 수 사이에서 혼동되는가? | 새 scene 두 개와 기존 depth-one/two control 여덟 개를 구분한다. 통과. |
| 12 | Android 10/10이 새 case만의 10개로 오해되는가? | 각 플랫폼 10/10은 control 포함 matrix임을 본문에 적었다. 통과. |
| 13 | Android AVD 결과가 physical device 결과처럼 보이는가? | API 37 ARM64 AVD라고 쓰며 Android 실기기는 미검증으로 남긴다. 통과. |
| 14 | iOS Simulator 결과가 iOS 실기기 결과로 읽히는가? | iPhone 17 Pro/iOS 26.2 Simulator로 환경을 한정한다. 통과. |
| 15 | min-content width와 height를 뒤바꾸는가? | `70×20px`로 독립 기대값과 runtime README를 대조했다. 통과. |
| 16 | min-content track axes를 섞는가? | Columns `[70px]`, Rows `[20px]`를 각각 쓴다. 통과. |
| 17 | max-content width와 height를 뒤바꾸는가? | `100×10px`로 독립 기대값과 runtime README를 대조했다. 통과. |
| 18 | max-content track axes를 섞는가? | Columns `[100px]`, Rows `[10px]`를 각각 쓴다. 통과. |
| 19 | depth-three nesting 역할에서 branch가 빠지는가? | root → middle → branch → inner 순서를 모두 쓴다. 통과. |
| 20 | direct text leaf를 여러 child나 이미지와 혼동하는가? | plan과 fixture 범위는 Ahem direct text 하나로 한정한다. 통과. |
| 21 | draw/present를 pixel equality나 glyph paint로 부풀리는가? | draw/present만 확인하고 glyph pixel equality와 compositor pixels는 미검증으로 쓴다. 통과. |
| 22 | WPT나 일반 nested Grid 지원을 주장하는가? | WPT·일반 CSS Grid·arbitrary nesting은 미검증이라고 적었다. 통과. |
| 23 | 계획 route가 depth-two 페이지를 가리키는가? | `/spinon/plan/c11-4-nested-grid-depth-three.html`로 연결한다. 통과. |
| 24 | 사전 oracle route가 task와 맞지 않는가? | depth-three precomparison README 경로를 연결한다. 통과. |
| 25 | 실행 README route가 실제 platform evidence를 가리키는가? | 신규 depth-three runtime README 경로를 연결한다. 통과. |
| 26 | Android screenshot 링크가 다른 task/date output을 가리키는가? | API37 AVD pass의 min-content PNG 경로가 repository에 존재한다. 통과. |
| 27 | iOS screenshot 링크가 다른 task/date output을 가리키는가? | iPhone 17 Pro Simulator pass의 min-content PNG 경로가 repository에 존재한다. 통과. |
| 28 | #458 URL이나 공식 상태 route가 잘못됐는가? | GitHub PR #458 및 `/spinon/STATUS.html#c11-css-grid`를 확인했다. 통과. |
| 29 | 링크를 추가하면서 다른 roadmap history 항목을 제거하는가? | 기존 PR #453 항목과 section content를 유지하고 새 paragraph만 추가했다. 통과. |
| 30 | raw HTML 구조·문자열이 깨져 새 paragraph가 페이지 전체를 망가뜨리는가? | 추가한 paragraph의 `<p>`, `<strong>`, `<code>`, `<a>` tag와 7개 href target을 검증하고, 선택 build에서 page route를 확인한 뒤 Agent Browser로 PR 화면과 병합 후 Tailnet route를 각각 확인한다. |

## 판정

변경은 공개 roadmap에 한 항목을 추가한다. 공식 완료 체크나 공개 지원 범위는 바꾸지 않는다. 상대 route 및 화면 표시 최종 결과는 Tailnet 미리보기에서 확인한다.
