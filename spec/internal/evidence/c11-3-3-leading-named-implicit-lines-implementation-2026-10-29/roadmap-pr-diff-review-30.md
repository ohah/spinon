# C11.3.3 public roadmap 변경 실패 관점 검토 30개

## 검토 범위

`spec/public/roadmap.html` 최상단에 PR #790 구현과 PR #791 상태 동기화 결과를 추가한 한 단락을 검토한다. 공식 완료 상태는 바꾸지 않고, Tailnet 화면 검증은 병합 뒤 별도 수행한다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 구현 PR 번호를 잘못 링크 | PR #790 링크와 제목을 확인했다. |
| 2 | 상태 동기화 PR 번호를 구현 PR로 오인 | PR #791을 별도 링크로 썼다. |
| 3 | merge timestamp 시간대 혼동 | 두 시각 모두 GitHub UTC `mergedAt` 값과 대조했다. |
| 4 | merge SHA와 head SHA를 바꿔 적음 | 두 merge SHA를 GitHub API 결과와 비교했다. |
| 5 | 최신 작업보다 C11.4 카드를 먼저 표시 | PR #790 카드가 기존 PR #787 카드보다 앞선 hero 순서다. |
| 6 | C11.3.3 전체 완료로 오해 | 문구에 제한 fixture라고 명시한다. |
| 7 | 상위 C11 완료로 읽힐 수 있음 | C11.3.3 candidate와 미완료 C11 문맥을 유지한다. |
| 8 | 공식 완료 수를 올림 | `72/213`을 그대로 표시한다. |
| 9 | 새 leading fixture 수 오기 | 신규 장면 7개로 기록했다. |
| 10 | 이전 baseline을 새 표본에 합침 | 26 baseline, unknown-area overflow 1, 새 장면 7을 별도로 표기했다. |
| 11 | Android geometry 결과 오기 | 새 장면 최대 geometry 오차 0 CSS px만 주장한다. |
| 12 | iOS geometry 결과 오기 | 같은 0 CSS px 결과를 iOS Simulator evidence와 대조했다. |
| 13 | Android PixelCopy를 전체 화면 검증으로 확대 | 7개 scene 표본 7/7 exact로 한정한다. |
| 14 | Android AVD를 실기기로 오해 | API 37 ARM64 AVD라고 명시한다. |
| 15 | software AVD를 hardware GPU로 오해 | SwiftShader software renderer라고 표시한다. |
| 16 | iOS Metal drawable을 compositor 이후 결과로 오해 | compositor 제출 전이라고 명시한다. |
| 17 | 시뮬레이터를 실기기로 오해 | 두 플랫폼의 simulator 성격을 제목과 설명에 유지한다. |
| 18 | full `spinon-style` suite를 green으로 말함 | 246/247과 기존 실패 하나를 그대로 표시한다. |
| 19 | 실패 test의 다른 경계를 감춤 | TrackSizing percentage auto-repeat 실패임을 명시한다. |
| 20 | 실패를 이번 코드의 새 regression으로 오인 | 구현 evidence에서 선행 동일 실패였음을 함께 연결한다. |
| 21 | 빈 PR checks를 CI 성공으로 표현 | check rollup이 비어 있다고 기록하고 pass 주장을 하지 않는다. |
| 22 | 고정 fixture를 WPT pass로 확대 | Spinon WPT 적합성 미확인을 명시한다. |
| 23 | 제한 profile을 general CSS admission으로 확대 | 일반 CSS admission이 아님을 표시한다. |
| 24 | screenshot link가 다른 run을 가리킴 | Android/iOS 최종 matched run의 PNG 경로를 사용한다. |
| 25 | runtime 근거 링크가 plan 대신 다른 문서를 가리킴 | 구현 README 경로가 실제 tracked evidence 디렉터리와 일치한다. |
| 26 | 계획 링크가 다른 C11.3 plan을 가리킴 | leading named implicit lines 전용 plan route를 쓴다. |
| 27 | review 링크가 없는 파일을 가리킴 | 이 PR의 30-point review 파일을 같은 경로에 추가한다. |
| 28 | STATUS anchor가 잘못됨 | 기존 `css-구현-체크리스트` anchor를 사용한다. |
| 29 | 새 카드 HTML이 기존 hero 카드와 다른 구조를 사용 | 기존 `kanban-note hero-progress` paragraph 구조를 따른다. |
| 30 | public HTML 변경이 상태 대장을 별도로 바꿀 수 있음 | 이 PR은 roadmap HTML과 검토 문서만 변경하며 STATUS 체크/집계는 건드리지 않는다. |

## 판정

고정 근거 및 이미 병합된 PR metadata와 일치한다. Tailnet에서 실제 렌더링된 카드와 링크 해시는 동기화 뒤 Agent Browser로 확인한다.
