# 공개 로드맵 동기화 PR 변경 검토 30개

검토 대상은 `spec/public/roadmap.html`에 추가한 C11.4 pinned WPT `fit-content(calc(10% + 5px))` implicit-row 기록 한 문단이다. 기준 구현/runtime 동작은 병합된 PR #778과 공식 상태 대장 및 연결된 실행 근거이며, 이 PR은 문서 표시를 동기화한다. 아래 각 항목은 서로 다른 오기·범위 확대·링크 손상 경로를 확인한다.

| # | 실패 관점 | 확인 및 판정 |
|---:|---|---|
| 1 | 최신 이력이 과거 C11.4 항목 뒤에 묻힌다. | 새 기록이 로드맵 첫 진행 문단으로 배치된다. 통과. |
| 2 | 기록이 C11.4 전체 완료로 읽힌다. | 문단은 제한 opt-in fixture라고 명시하고 전체 C11.4 미완료를 유지한다. 통과. |
| 3 | 공식 체크박스가 완료로 바뀐 것으로 읽힌다. | 체크박스 유지 문구를 명시하고 `STATUS.md`를 수정하지 않는다. 통과. |
| 4 | 공식 완료 수가 늘어난 것으로 읽힌다. | 공식 `72/213` 유지 문구가 있고 수치 갱신을 주장하지 않는다. 통과. |
| 5 | assertion의 CSS 식이 다른 값으로 오기된다. | `fit-content(calc(10% + 5px))`를 그대로 기록한다. 통과. |
| 6 | WPT의 행 assertion이 열 assertion과 혼동된다. | `implicit-row assertion`으로 명시한다. 통과. |
| 7 | basis가 indefinite이거나 다른 축으로 읽힌다. | definite 300px block basis라고 적는다. 통과. |
| 8 | calc 결과 35px의 산술이 틀리다. | 300px의 10%인 30px에 5px를 더한 최종 row 35px이며 runtime evidence와 일치한다. 통과. |
| 9 | root/item/child 크기가 서로 뒤바뀐다. | root 60×300px, item 60×35px, child 30×400px 순으로 연결한다. 통과. |
| 10 | child overflow 관찰이 일반 overflow 지원 약속으로 확대된다. | 문단은 고정 fixture geometry만 서술하고 전체 CSS 지원을 부인한다. 통과. |
| 11 | runtime 검증이 브라우저 WPT 실행으로 읽힌다. | Spinon WPT harness가 아니라고 명시한다. 통과. |
| 12 | calc 결과만 확인하고 연결된 runtime identity를 숨긴다. | computed calc AST와 projected `LayoutCalcId` 연결을 적는다. 통과. |
| 13 | AST/registry 관계가 부정확하다. | root node, `GridTemplateRows`, 정확한 `10% + 5px` registry entry를 모두 나열한다. 통과. |
| 14 | Android 관찰을 실기기 결과로 오해한다. | Android API 37 ARM64 AVD라고 명시하고 software renderer 제한도 적는다. 통과. |
| 15 | iOS 관찰을 실기기 또는 최종 compositor pixel로 오해한다. | iPhone 17 Pro/iOS 26.2 Simulator 및 compositor 제출 전 표본이라고 명시한다. 통과. |
| 16 | 기존 percentage scene 결과가 생략되어 회귀 비교 범위가 넓어 보인다. | 기존 percentage와 새 calc scene 각각 2/2로 제한한다. 통과. |
| 17 | frame/trace와 pixel 표본 수가 혼합된다. | frame·trace와 PixelCopy/Metal surface sample 각각 4/4 exact로 기록한다. 통과. |
| 18 | 기본 제품 archive와 fixture artifact 경계가 흐려진다. | 기본 static FFI archive marker 부재와 fixture-test positive control 존재를 분리한다. 통과. |
| 19 | PR checks가 비어 있는데 CI 성공으로 표현한다. | check rollup 공란 및 main required checks 미설정을 기록한다. 통과. |
| 20 | 구현 PR 링크가 잘못된 번호를 가리킨다. | 링크는 `ohah/spinon/pull/778`이다. 통과. |
| 21 | 구현 PR 병합 시각이 오기된다. | `2026-10-29T00:00:16Z`는 병합 메타데이터와 일치한다. 통과. |
| 22 | 구현 merge SHA가 잘리거나 다른 커밋을 가리킨다. | `4ce3d4697c3505e39e945a452bdda3a07a411797` 전체 SHA가 공식 상태 기록과 일치한다. 통과. |
| 23 | 문서 동기화 PR 링크가 구현 PR과 혼동된다. | 문서 PR은 별도 `#779` 링크다. 통과. |
| 24 | 문서 PR 병합 시각 또는 SHA가 부정확하다. | `2026-10-29T00:03:26Z` 및 `59297c1e1f85d4bbd01e74908b70cdc8f2725d92`는 병합 메타데이터와 일치한다. 통과. |
| 25 | 집중 계획 링크가 다른 C11.4 계획으로 연결된다. | GitHub main 링크는 `plan/c11-4-fit-content-calc-argument.md`다. 통과. |
| 26 | 사전 비교 링크가 runtime 근거로 오인된다. | `...precomparison.../README.md`를 별도 링크로 둔다. 통과. |
| 27 | runtime 증거 링크가 잘못된 날짜나 slice를 가리킨다. | `...runtime-2026-10-29/README.md` 경로가 맞다. 통과. |
| 28 | 공식 상태 링크가 공개 상태 파일 대신 로컬 임의 경로를 가리킨다. | GitHub main의 `spec/STATUS.md#css-구현-체크리스트`에 연결한다. 통과. |
| 29 | 링크 URL에 HTML 파싱을 깨뜨리는 따옴표·앰퍼샌드가 들어간다. | 추가한 네 링크는 일반 HTTPS URL이며 속성 값을 깨는 문자가 없다. 통과. |
| 30 | 모바일 화면에서 긴 한글·코드 문단이 기존 레이아웃 밖으로 나간다. | Agent Browser 390×844에서 문서 폭 390px, 가로 overflow 없음, 최신 기록·PR #778·`72/213` 존재를 확인했다. 모바일 화면을 함께 보존한다. 통과. |

이 문서 변경은 정적 HTML 한 문단이며 API, runtime code, fixture, 상태 체크박스, 공식 완료 수를 수정하지 않는다. 사전 Agent Browser 확인은 로컬 파일 URL의 제목·최신 기록·PR 번호·지원 링크를 대상으로 했다. `roadmap-source-agent-browser.png`와 `roadmap-source-agent-browser-mobile.png`는 해당 파일 렌더의 화면이다. Tailnet 반영 뒤 공개 URL도 별도로 확인한다.
