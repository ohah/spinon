# C11.4 fixed-repeat 상한 제거 계획 PR diff 검토 · 실패 관점 20개

**검토 범위:** 계획-only 변경의 실제 diff. 구현 코드는 포함하지 않는다. 별도의 계획 실패 관점 20개는 [계획 검토](./c11-4-unbounded-fixed-repeat-groups-plan-review-20.md)에 기록했다.

| # | diff 실패 관점 | 검토 결과 |
| --- | --- | --- |
| 1 | 계획 PR이 구현/runtime 완료로 오해될 수 있음 | 제목·본문·STATUS·로드맵 모두 plan-only이며 runtime 미시작과 72/213 유지를 명시한다. |
| 2 | 잘못된 고정 규범 링크를 근거로 문법 범위를 넓힐 수 있음 | 2025 CSS Grid CRD §7.2.3.1 primary source 링크와 정확한 절 제목을 연결했다. |
| 3 | 규범의 “한 번” 제약이 fixed-repeat에도 적용되는 것처럼 표현될 수 있음 | 문서는 auto-repeat만 1개, fixed-repeat은 복수 가능하다고 분리한다. |
| 4 | 계획 지원 범위가 표준 전체 지원으로 읽힐 수 있음 | finite nonnegative `LengthPx`, 1–2 track pattern 및 10,000 descriptor 제한을 명시한다. |
| 5 | 구현량이 상한 없는 메모리 사용을 유도할 수 있음 | 그룹 수를 10,000 descriptor 한도와 fallible allocation으로 제한한다. |
| 6 | metadata 접근 경로가 그룹 수 제곱에 비례할 수 있음 | index-addressed lookup 또는 선형 traversal 요구를 계획에 포함했다. |
| 7 | 계획 oracle fixture가 다른 test input과 혼동될 수 있음 | 별도 `grid-unbounded-fixed-repeat-auto-repeat-c114-precomparison.html` 파일에 두 scene을 고정했다. |
| 8 | 네 그룹 syntax가 parser에서 실제 허용되는지 관측되지 않았을 수 있음 | Chrome 156 Agent Browser가 두 scene의 authored syntax를 수용하고 CSSOM track list를 반환했다. |
| 9 | CSSOM sequence가 기대값 JSON과 다를 수 있음 | columns/rows 각 14-track 순서를 raw observation과 독립 기대값에 보존했다. |
| 10 | 독립 반복 산식에 fixed track 또는 ordinary neighbor가 빠질 수 있음 | 8 fixed tracks, 1 ordinary track, 94px group breadth와 5px neighbor를 적었다. |
| 11 | 반복 count 경계가 잘못 산정될 수 있음 | 최소 공간 `163 + 28n`, n=5 303px, n=6 331px를 명시했다. |
| 12 | Chrome serialization 오차를 Spinon 결과와 혼동할 수 있음 | 수학값 23.4px, CSSOM 값 23.3906px 및 1/64px 허용치를 구분했다. |
| 13 | row item line index가 authored track 순서와 어긋날 수 있음 | `grid-row: 5 / 6`, 독립 시작점 74px, Chrome frame을 함께 보존했다. |
| 14 | screenshot이 observation JSON과 다른 입력에서 만들어질 수 있음 | screenshot과 JSON은 같은 로컬 fixture/Chrome session에서 생성했고 hash를 기록했다. |
| 15 | fixture 또는 JSON이 capture 뒤 수정됐을 수 있음 | HTML, observation, screenshot, expectation JSON의 SHA-256을 `sha256.txt`와 README에 기록했다. |
| 16 | 상태 대장이 이 계획을 공식 완료로 셀 수 있음 | C11.4 checkbox와 공식 `72/213`은 그대로 두고 계획-only 항목을 별도로 추가했다. |
| 17 | 공개 roadmap에서 계획이 완료 lane에 놓일 수 있음 | 새 row는 `hero-progress`이고 구현 미시작·72/213 유지를 명시한다. |
| 18 | Rspress route links가 계획/evidence 파일명과 다를 수 있음 | 고정된 Rspress 2.0.22 선택 build가 계획·두 review·사전 비교 README route를 생성했다. raw `roadmap.html`은 builder 밖 Tailnet asset이므로 병합 뒤 별도로 sync 확인한다. |
| 19 | 기존 #594 계약/근거를 수정해 과거 결과가 바뀔 수 있음 | 기존 plan, contract 0059, #594 runtime evidence는 편집하지 않았다. |
| 20 | main 기준점이나 PR 범위에 무관한 변경이 섞일 수 있음 | 변경은 계획, 계획 검토, 상태/roadmap 기록, 새 Chrome fixture와 사전 비교 evidence로 한정한다. |

## 판정

계획과 Chrome 사전 비교를 구현 결과로 표시하는 결함은 없다. 기존 #594 완료 범위와 official count를 유지한다. 계획-only PR diff를 병합 검토 단계로 넘길 수 있다.
