# C11.4 `text-indent` flags 병합 후 상태 대장 검토 30개

**범위:** PR #959 병합 뒤 `spec/STATUS.md`, 계획 및 실행 근거의 상태 동기화. 이 문서는 구현/runtime 20개 검토나 PR diff 30개 검토를 대체하지 않으며, C11 완료 판정도 아니다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | C11 상위 체크가 부분 slice 때문에 완료되는가 | C11 parent checkbox는 미완료다. |
| 2 | C11.4 intrinsic track sizing 전체가 완료되는가 | C11.4 checkbox는 미완료다. |
| 3 | 다른 C11 subitem이 이 변경 때문에 바뀌는가 | PR diff는 새 dated evidence 항목만 추가한다. |
| 4 | 공식 완료 수 72/213이 증가하는가 | 새 항목에 공식 `72/213` 유지가 명시돼 있다. |
| 5 | 구현 slice가 별도 완료 ID로 오인되는가 | entry는 `제한 opt-in runtime 확장`으로 표시하고 상위 완료를 주장하지 않는다. |
| 6 | 기록이 잘못된 roadmap section에 들어가는가 | entry는 기존 C11.4 dated C11.4 notes 다음, C12 section 앞에 배치됐다. |
| 7 | 동일 slice가 두 번 기록되는가 | PR #959의 단일 dated entry만 추가됐다. |
| 8 | PR 번호나 title이 구현 PR과 다른가 | [PR #959](https://github.com/ohah/spinon/pull/959)의 title과 scope가 맞는다. |
| 9 | merge 방식이 저장소 규칙과 다른가 | PR은 Rebase and merge로 병합됐다. |
| 10 | merge timestamp가 PR 기록과 다른가 | GitHub의 `2026-11-01T02:06:32Z`와 일치한다. |
| 11 | merge SHA가 다른 head/base commit으로 기록되는가 | `590f5946e97e0252113abf34b23ae31b13819acf`가 PR API의 mergeCommit과 일치한다. |
| 12 | PR check 결과가 초록으로 과장되는가 | GitHub statusCheckRollup은 비었고 `gh pr checks --required`는 no checks reported를 반환했다. |
| 13 | CI 미실행/미보고가 CI success로 기록되는가 | status note는 CI 통과로 세지 않는다고 적는다. |
| 14 | 새 flags matrix scene 수가 다른가 | 고정 flags matrix는 20 scene이다. |
| 15 | 기존 양수 회귀 범위가 빠지는가 | positive `text-indent` matrix 18/18을 별도 유지한다. |
| 16 | negative profile 회귀가 양수 통제로 오인되는가 | positive controls 포함 30/30로 명시돼 있다. |
| 17 | 136 total이 계산 오류인가 | `(20 + 18 + 30) × 2 platforms = 136`이다. |
| 18 | Android/iOS 결과가 합쳐져 한 플랫폼만 가리는가 | 각 행렬의 두 플랫폼 수가 별도 칸으로 기록돼 있다. |
| 19 | emulator 결과가 실기기 또는 hardware GPU로 표현되는가 | Android API 37 ARM64 AVD로 적고 실기기 주장은 evidence에서 제외한다. |
| 20 | iOS simulator가 물리 iPhone 검증으로 표현되는가 | iPhone 17 Pro / iOS 26.2 Simulator라고 적는다. |
| 21 | layout summary만으로 draw/present를 주장하는가 | 링크된 matrix는 `presented boxes=1` 조건과 예상 root frame을 기록한다. |
| 22 | superseded capture를 성공 집계에 포함하는가 | 최신 여섯 matrix 모두 retry count 0이다. |
| 23 | build hash가 다른 artifact 실행을 가리키는가 | 각 simulator receipt의 installed hash는 최종 APK/executable hash와 일치한다. |
| 24 | 일반 CSS admission으로 범위가 넓어지는가 | 기록은 fixed opt-in anonymous direct-text Grid profile로 제한한다. |
| 25 | 임의 `text-indent` 값과 상호작용을 완료로 해석하는가 | arbitrary value/font/language/writing mode는 미지원 경계에 둔다. |
| 26 | Chrome/WPT oracle가 Spinon WPT 결과로 표현되는가 | WPT 적합성은 명시적으로 범위 밖이다. |
| 27 | screenshot이 glyph raster equality를 증명하는가 | black renderer 영역과 glyph pixel 한계를 설명한다. |
| 28 | iOS Simulator가 compositor 이후 pixels를 증명하는가 | compositor 이후 화면을 검증했다고 주장하지 않는다. |
| 29 | versioned contract가 누락되거나 version이 올라가는가 | 내부 계약 0059 링크를 유지하고 버전은 `0.1.0`이다. |
| 30 | 새 plan/evidence/review 링크가 깨지거나 scope가 충돌하는가 | 저장소 상대 링크를 확인했고 계획·run evidence·implementation 20·PR diff 30 파일이 일치한다. |

## 판정

30개 상태 관점에서 공식 완료 수, 상위 checkbox, merge/CI 사실 또는 검증 범위가 과장되는 문제를 찾지 못했다. C11/C11.4 전체 완료는 미정이며 `72/213`을 유지한다.
