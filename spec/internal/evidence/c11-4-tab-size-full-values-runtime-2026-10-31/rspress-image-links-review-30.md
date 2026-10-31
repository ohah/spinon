# C11.4 simulator screenshot RSPress link fix · 30개 실패 관점 검토

**범위:** 실행 근거 README의 Android/iOS PNG 두 개에 `./` 상대 경로 접두사를 추가한 변경이다. 이 검토는 Markdown asset resolution과 문서 범위만 다루며 CSS 동작이나 C11 완료를 판정하지 않는다.

| # | 실패 관점 | 판정 |
| ---: | --- | --- |
| 1 | Android PNG 파일명이 바뀌는가 | **PASS** — `representative.png` 이름을 유지했다. |
| 2 | iOS PNG 파일명이 바뀌는가 | **PASS** — `representative.png` 이름을 유지했다. |
| 3 | Android 실행 디렉터리 경로가 바뀌는가 | **PASS** — `c114-tab-values-20261031024357017-869b04d0/android-avd/`를 유지했다. |
| 4 | iOS 실행 디렉터리 경로가 바뀌는가 | **PASS** — `c114-tab-values-20261031024357017-869b04d0/ios-simulator/`를 유지했다. |
| 5 | 경로가 README의 디렉터리 기준 상대경로인가 | **PASS** — `./`로 시작해 sibling subdirectory를 명시한다. |
| 6 | 경로가 package import처럼 해석될 수 있는가 | **PASS** — RSPress의 bare specifier 해석을 피한다. |
| 7 | Android와 iOS 이미지가 서로 바뀌는가 | **PASS** — 각 alt text와 platform directory 대응을 보존했다. |
| 8 | alt text가 제거되거나 바뀌는가 | **PASS** — 두 설명 문구를 그대로 유지했다. |
| 9 | PNG 확장자가 손상되는가 | **PASS** — 두 경로 모두 `.png`로 끝난다. |
| 10 | 실행 ID가 오타로 바뀌는가 | **PASS** — 두 링크의 공통 run ID가 보존됐다. |
| 11 | 이미지가 Git 기준 소스에 존재하지 않는가 | **PASS** — 두 PNG는 기준 main tree에서 추적된다. |
| 12 | 선택 문서 root가 evidence subtree를 생략하는가 | **PASS** — sync selection은 해당 tracked evidence subtree를 포함한다. |
| 13 | 공개 문서 output이 원본 asset을 내보내지 않는가 | **PASS** — RSPress build와 publication manifest에서 두 PNG route를 요구한다. |
| 14 | public HTML roadmap의 PNG URL과 evidence README URL이 달라지는가 | **PASS** — HTML의 절대 preview route와 README의 source-relative asset은 같은 두 파일을 가리킨다. |
| 15 | 화면이 glyph pixel 증거라고 과장되는가 | **PASS** — README의 검은 render surface 한계 문구는 변경하지 않았다. |
| 16 | 한 플랫폼만 고쳐 나머지 링크가 깨지는가 | **PASS** — Android와 iOS 두 destination 모두 수정했다. |
| 17 | Markdown image syntax가 손상되는가 | **PASS** — `![alt](destination)` 형태를 유지했다. |
| 18 | 제목·표·실행 결과 내용이 변경되는가 | **PASS** — 변경은 이미지 destination 두 곳뿐이다. |
| 19 | runtime scene 수가 변경되는가 | **PASS** — 140/140 행렬이나 scene 목록은 수정하지 않았다. |
| 20 | 공식 완료 수가 변경되는가 | **PASS** — `72/213`을 그대로 둔다. |
| 21 | C11.4 상태를 완료로 바꾸는가 | **PASS** — 지원 상태나 checklist는 변경하지 않았다. |
| 22 | C11 상태를 완료로 바꾸는가 | **PASS** — 상위 C11은 미완료로 유지된다. |
| 23 | Android Simulator를 실기기로 표현하는가 | **PASS** — target 설명은 변경하지 않았다. |
| 24 | iOS Simulator를 실기기로 표현하는가 | **PASS** — target 설명은 변경하지 않았다. |
| 25 | 기존 참고 문서 링크에 영향을 주는가 | **PASS** — JSON, plan, contract 링크는 수정하지 않았다. |
| 26 | 이미지 bytes나 screenshot 내용을 바꾸는가 | **PASS** — PNG source bytes는 변경 범위 밖이다. |
| 27 | `./`가 상위 디렉터리 traversal을 허용하는가 | **PASS** — 목적지는 동일 evidence subtree 안에 남는다. |
| 28 | RSPress의 path resolution 실패가 검증 없이 완료 처리되는가 | **PASS** — 최종 selected-root build가 병합된 source에서 재실행되어야 한다. |
| 29 | 변경 검토가 CSS runtime 적합성으로 오인되는가 | **PASS** — 검토 범위를 문서 asset resolution으로 한정했다. |
| 30 | 미지원 범위가 새 동작 약속으로 바뀌는가 | **PASS** — 지원 계약과 사용자-facing feature claim은 바뀌지 않는다. |

**처리:** source 변경은 두 Markdown destination에 `./`를 붙이는 것뿐이다. PR 병합 뒤 pinned RSPress 빌드에서 import resolution과 최종 두 asset route를 다시 확인한다.
