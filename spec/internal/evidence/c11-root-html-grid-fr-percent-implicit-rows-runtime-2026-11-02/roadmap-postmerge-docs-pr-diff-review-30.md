# C11.1 `fr`·percentage + implicit row 병합 후 문서 PR diff 검토 30개

기준은 `origin/main` 구현 PR merge SHA `732b0257146464de55ddede8a0477726bc5a380b`다. 검토 대상은 PR #1027 병합 사실을 plan, internal contract 0059, 공식 STATUS, runtime README와 public roadmap에 반영하는 문서 diff다. 제품 코드는 변경하지 않는다.

| # | 실패 관점 | 확인 근거와 판정 |
|---:|---|---|
| 1 | 구현 merge SHA를 잘못 기록하는가 | GitHub PR #1027 API의 `mergeCommit.oid`와 모든 업데이트된 문서의 `732b0257146464de55ddede8a0477726bc5a380b`가 일치한다. 통과. |
| 2 | 구현 PR의 병합 시각을 로컬 시간으로 표기하는가 | API `mergedAt=2026-11-02T03:48:24Z`와 STATUS·계획의 UTC 시각이 일치한다. 통과. |
| 3 | merge 방식을 squash/merge commit으로 잘못 적는가 | GitHub PR의 merge method와 저장소 정책에 따라 Rebase and merge로 표기한다. 통과. |
| 4 | 계획 PR #1026과 구현 PR #1027의 역할이 뒤바뀌는가 | 계획 문서와 STATUS가 각각 계획 #1026, 제한 구현 #1027로 구분한다. 통과. |
| 5 | 계획 PR의 merge SHA를 잘못 기록하는가 | PR #1026 API의 `3f7bfa71ae67a1f61cb8a5154f0cb74137bad9fc`와 대조했다. 통과. |
| 6 | 빈 PR check rollup을 CI green으로 확대하는가 | #1027 `statusCheckRollup`이 빈 배열이며 STATUS와 runtime README는 CI 통과를 주장하지 않는다. 통과. |
| 7 | 공식 완료 수가 제한 slice로 증가하는가 | 변경된 각 요약은 공식 `72/213`을 유지한다. 통과. |
| 8 | C11 상위 checkbox를 완료로 바꾸는가 | STATUS의 C11 checkbox는 `- [ ]` 그대로다. 통과. |
| 9 | C11.1 전체가 완료로 읽히는가 | 모든 신규 요약이 제한 구현 slice로 명명하고 C11.1/C11 미완료를 명시한다. 통과. |
| 10 | 지원 경로가 일반 CSS admission으로 확대되는가 | plan, 0059, STATUS 및 public card는 기본 parsed Root HTML 경로에 한정한다. 통과. |
| 11 | 명시 track 단위가 임의 CSS breadth로 확대되는가 | direct finite non-negative px/percentage/fr만 기록하며 intrinsic·auto-repeat·minmax 등을 계속 거부한다. 통과. |
| 12 | percentage/fr의 definite-axis 조건이 빠지는가 | 전용 계획과 contract가 각 axis의 definite root content-box basis를 유지한다. 통과. |
| 13 | implicit-row 범위가 arbitrary list로 확대되는가 | 단일 fixed-px `grid-auto-rows`만 허용하고 percentage/fr/list는 거부한다. 통과. |
| 14 | named areas, named lines 또는 implicit columns가 지원된다고 읽히는가 | 0059와 전용 계획에 이 입력들의 rejection이 남아 있다. 통과. |
| 15 | dense/column placement, authored placement 또는 nonzero order가 들어오는가 | 이 입력들은 전용 계획과 contract의 제한 사항으로 유지된다. 통과. |
| 16 | item contract가 일반 child tree로 넓어지는가 | empty fixed-size visible direct block children의 범위와 nested/text/image rejection을 유지한다. 통과. |
| 17 | visible overflow가 clamp된 것으로 바뀌는가 | runtime README에서 percentage-column item의 x=35px, width=100px, root 밖 15px를 그대로 기록한다. 통과. |
| 18 | Root HTML runtime geometry를 screenshot 근거와 혼동하는가 | Rust matrix가 exact frames를 검증하며 캡처는 시각·draw/present 자료라고 구분한다. 통과. |
| 19 | Android PixelCopy를 수행했다고 주장하는가 | 모든 관련 문서가 PixelCopy surface readback 미수행을 명시한다. 통과. |
| 20 | iOS Metal drawable readback을 수행했다고 주장하는가 | 모든 관련 문서가 Metal surface readback 미수행을 명시한다. 통과. |
| 21 | Android AVD를 실기기/hardware GPU라고 부르는가 | 플랫폼 근거는 API 37 AVD로 표기하고 hardware GPU와 구분한다. 통과. |
| 22 | iOS Simulator를 iPhone 실기기 결과로 확대하는가 | iPhone 17 Pro / iOS 26.2 Simulator 표기를 유지한다. 통과. |
| 23 | WPT를 통과했다고 주장하는가 | 변경된 문서는 WPT 미검증과 일반 conformance 미완료를 유지한다. 통과. |
| 24 | focused test 숫자가 실행 기록과 달라지는가 | layout 74/74, style projection 12/12, runtime Grid 20/20은 evidence README log 링크와 일치한다. 통과. |
| 25 | focused green을 전체 workspace green으로 확대하는가 | 문서는 scoped test/Clippy 결과만 보고하며 전체 suite 통과를 주장하지 않는다. 통과. |
| 26 | source artifact provenance가 사라지는가 | runtime README의 HTML hash, platform observation, hash manifest 링크를 유지한다. 통과. |
| 27 | internal contract 버전을 불필요하게 올리는가 | 계약 0059의 숫자 버전 `0.1.0`은 변경하지 않았다. 통과. |
| 28 | 문서만 바뀌었는데 API 구현으로 표기하는가 | 변경 파일은 plan/status/contract/evidence/public roadmap이며 public API 추가를 주장하지 않는다. 통과. |
| 29 | 문서·근거 링크가 존재하지 않는 경로를 가리키는가 | plan, evidence README, 리뷰 파일과 STATUS 링크의 저장소 경로를 확인한다. 통과. |
| 30 | 저장소 문서 갱신을 GitHub Pages 배포로 읽히게 하는가 | PR은 저장소 원본만 갱신하며 수동 Pages 배포를 수행하거나 주장하지 않는다. Tailnet 미리보기는 병합 뒤 별도로 확인한다. 통과. |

## 결과

30개 관점에서 범위·수치·merge 정보의 모순을 찾지 못했다. 공식 `72/213`과 C11.1/C11 미완료 상태는 유지한다. 이 검토는 문서 변경 검토이며 구현 동작을 재검증하지 않는다.
