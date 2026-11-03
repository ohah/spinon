# C11.1 named-area 자동 최소 크기 로드맵 문서 diff 검토 30개

검토 대상은 `spec/STATUS.md`와 `spec/public/roadmap.html`의 PR #1113 병합 기록 추가다. 완료 체크나 공식 완료 수를 변경하지 않는 문서 변경으로 한정해 각 링크·범위·수치를 실제 병합 PR과 실행 근거에 대조했다.

| # | 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | 기록 ID가 C11.1 named-area 자동 최소 크기로 잘못 좁혀지거나 넓혀지는가 | C11.1의 named-area Grid automatic minimum으로 기록했다. |
| 2 | PR 링크가 계획 PR이나 기존 PR을 가리키는가 | 두 문서 모두 구현 PR #1113을 가리킨다. |
| 3 | merge timestamp가 local timezone으로 오표기되는가 | GitHub `mergedAt`의 UTC `2026-11-03T08:21:27Z`를 사용했다. |
| 4 | 기록한 merge SHA가 병합 결과와 다른가 | `21507a692393ae65ec192a1d286b7e25b2898a9d`를 GitHub와 `origin/main`에서 대조했다. |
| 5 | 시각 순서가 앞선 #1109 항목을 가리는가 | #1113을 #1109 다음의 최신 C11.1 기록으로 배치했다. |
| 6 | exact named-area map의 반복·행 구분이 틀리는가 | `header header / side content`를 사용했다. |
| 7 | named-area 제약이 모든 Grid template로 일반화되는가 | 정확한 기존 map 조건으로 한정했다. |
| 8 | definite content-box 조건을 놓치는가 | 해당 조건을 지원 전제로 기록했다. |
| 9 | `minmax(auto, fr)`가 임의 maximum으로 확대되는가 | 유한한 비음수 `fr` maximum으로 제한했다. |
| 10 | 행과 열 중 한 축만 지원하는 것으로 오해되는가 | 둘 다 지원하되 고정 scene 조건으로 한정했다. |
| 11 | zero-min 대조군을 auto-minimum 구현과 혼동하는가 | `minmax(0, fr)`를 대조 경계로 분리했다. |
| 12 | `0fr` 경계가 positive-flex와 동등하다고 오해되는가 | `0fr`를 별도 경계 사례로 구분했다. |
| 13 | 지원 밖 `minmax()`가 implicit하게 지원된다고 읽히는가 | unsupported 입력은 계속 거부한다고 명시했다. |
| 14 | nested 또는 추가 item 범위까지 확장되는가 | 고정 fixture와 direct-item 범위로 유지했다. |
| 15 | 일반 CSS Grid 지원을 선언하는가 | 일반 CSS Grid 지원은 미완료로 표시했다. |
| 16 | WPT 적합성 결과를 잘못 주장하는가 | WPT 적합성은 미검증으로 표시했다. |
| 17 | Android AVD가 물리 기기로 잘못 표기되는가 | API 37 AVD로 명시했다. |
| 18 | Android API 수준·OS 정보를 혼동하는가 | Android 17/API 37로 기록했다. |
| 19 | software renderer가 hardware GPU로 과장되는가 | ANGLE/SwiftShader software renderer라고 명시했다. |
| 20 | PixelCopy scene 수와 표본 수를 혼동하는가 | 6 scenes와 exact PixelCopy 표본 12/12를 분리해 기록했다. |
| 21 | iOS Simulator가 iOS 실기기로 오해되는가 | iPhone 17 Pro Simulator로 표시했다. |
| 22 | iOS Metal 표본을 compositor 이후 결과로 오해하는가 | drawable 표본이며 compositor 제출 전이라고 기록했다. |
| 23 | Metal scene 수와 표본 수를 혼동하는가 | 6 scenes와 exact 표본 12/12를 분리했다. |
| 24 | Chrome DOM 관측이 native pixel 결과로 오해되는가 | Chrome Agent Browser 관측과 native 표본 결과를 별도 문장으로 기록했다. |
| 25 | Chrome 기대값과 구현 후 관측의 역할이 뒤바뀌는가 | 기대값 일치 여부만 기록하고 독립 계획 비교의 내용을 바꾸지 않았다. |
| 26 | scoped Rust/Clippy 결과가 전체 workspace green으로 확대되는가 | focused/scoped 결과만 적고 전체 workspace 통과를 주장하지 않는다. |
| 27 | PR checks 부재가 CI 성공으로 표현되는가 | GitHub check run 미보고를 명시했다. |
| 28 | C11.1 체크박스가 제한 slice 때문에 완료되는가 | C11.1 상위 체크를 미완료로 유지한다. |
| 29 | C11 상위 체크 또는 공식 완료 수가 바뀌는가 | C11 체크와 공식 `72/213`을 유지한다. |
| 30 | 근거 링크의 저장소 상대 경로가 잘못되는가 | 내부 계약 0059, 실행 근거 README, 공식 상태 route를 각 문서 루트 기준으로 확인했다. |

## 결론

검토 중 문서 사실 오류는 발견하지 않았다. 이 추가 내용은 C11.1/C11 완료 또는 공개 CSS 지원을 승격하지 않는다.
