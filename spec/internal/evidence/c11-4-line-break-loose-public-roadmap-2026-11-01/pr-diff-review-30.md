# C11.4 PR #959/#962 공개 로드맵 동기화 diff 적대적 검토 30개

## 검토 대상

`spec/public/roadmap.html`의 두 stale C11.4 카드를 현재 병합 결과와 맞춘 diff다. 신규 PR #962 `line-break: loose` 카드를 추가하고 PR #959 `text-indent` flags 계획 후보를 제한 구현 완료로 바꾼다. 공식 상태 수, C11/C11.4 체크박스, 실행 evidence 원본은 수정하지 않는다. 각 항목은 다른 오표기·누락 경계를 확인한다.

| # | 실패 관점 | 확인과 판정 |
| ---: | --- | --- |
| 1 | 잘못된 구현 PR 번호를 노출하는가 | GitHub의 병합 PR #962와 일치한다. |
| 2 | 계획 PR #961을 구현 PR로 혼동하는가 | 카드 링크와 제목은 구현 PR #962다. |
| 3 | rebase merge SHA를 원본 PR head SHA로 적는가 | merge SHA `5831014125e26079748f0b99cb99d3c21613358c`를 기록한다. |
| 4 | merge 시각을 로컬 시간대로 변환하는가 | GitHub UTC `2026-11-01T02:41:05Z`를 그대로 쓴다. |
| 5 | GitHub check 부재를 CI 통과로 확대하는가 | `no checks reported`와 CI 미주장을 명시한다. |
| 6 | `MERGEABLE`을 테스트 결과로 표현하는가 | mergeability 또는 CI 판정을 카드에 넣지 않는다. |
| 7 | 공식 완료 수를 올리는가 | `72/213`을 그대로 기록한다. |
| 8 | C11 또는 C11.4를 완료로 표시하는가 | 두 상위 범주가 미완료라고 명시한다. |
| 9 | 제한 fixture를 author CSS 지원으로 읽을 수 있는가 | `제한 opt-in fixture`와 일반 CSS admission 제외를 기록한다. |
| 10 | 고정 입력 행렬을 임의 문자열 지원으로 넓히는가 | 임의 text/font/locale 제외를 명시한다. |
| 11 | Japanese loose와 English control을 혼동하는가 | 새 네 scene의 locale/mode 역할을 구별한다. |
| 12 | 기존 회귀 scene 수를 잘못 세는가 | 기존 5 + 신규 4 = 9 scene으로 쓴다. |
| 13 | Android 실행 수치를 iOS에 대신 적용하는가 | 두 시뮬레이터 각각 9/9라고 쓴다. |
| 14 | AVD를 실기기로 표현하는가 | `Android API 37 ARM64 AVD`로 한정한다. |
| 15 | iOS Simulator를 iOS 실기기로 표현하는가 | `iPhone 17 Pro/iOS 26.2 Simulator`를 명시한다. |
| 16 | layout status 화면을 glyph paint로 표현하는가 | 캡처가 glyph paint를 증명하지 않는다고 명시한다. |
| 17 | drawable 결과를 compositor 이후 pixel로 확대하는가 | iOS compositor 이후 pixels 제외를 명시한다. |
| 18 | 시뮬레이터 결과를 hardware GPU 결과로 부르는가 | hardware GPU 검증 제외를 명시한다. |
| 19 | CSS/WPT 적합성을 주장하는가 | WPT와 전체 CSS 지원 제외를 명시한다. |
| 20 | static roadmap만 바꾸고 상세 계획 링크를 누락하는가 | focused plan 링크를 넣는다. |
| 21 | internal contract 경로가 오기되는가 | 계약 0059 경로를 현재 파일과 대조한다. |
| 22 | precomparison 링크가 normal/strict 문서를 가리키는가 | loose 전용 Chrome 사전 비교 경로다. |
| 23 | runtime README 링크가 이전 5-scene evidence를 가리키는가 | loose 전용 9-scene 실행 README 경로다. |
| 24 | PR diff 검토 링크가 다른 slice 검토로 연결되는가 | loose runtime의 PR diff review 경로와 대조한다. |
| 25 | post-merge review 링크가 빠져 상태 동기화를 확인할 수 없는가 | loose slice post-merge 30개 검토 경로를 추가한다. |
| 26 | 병합된 PR #959를 계획 후보로 남겨 놓는가 | stale `text-indent` flags 계획 카드가 #959의 제한 구현 slice로 바뀐다. |
| 27 | 기존 공식 `72/213` 표기를 삭제하는가 | 기존 진행률과 상태 링크를 보존한다. |
| 28 | 화면의 링크가 `/spinon/` base 경로를 벗어나는가 | 내부 링크는 동일한 site base 아래의 선택 route를 쓴다. |
| 29 | 신규 카드 순서가 병합 시각과 다른가 | #962, #959, #952 순서로 최신 병합부터 나열한다. |
| 30 | 무관한 untracked evidence/cache가 PR에 포함되는가 | 공개 roadmap과 전용 evidence 폴더 안의 변경만 포함한다. |

## 결론

카드의 merge metadata, 제한 범위, platform evidence와 route 연결을 실제 상태 대장·계약·계획·evidence 경로에 대조했다. 상위 완료 수와 지원 범위를 넓히는 표현은 없다.
