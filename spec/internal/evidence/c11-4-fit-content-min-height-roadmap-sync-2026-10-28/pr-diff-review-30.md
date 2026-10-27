# C11.4 `fit-content()` roadmap 상태 동기화 · PR diff 검토

2026-10-28 변경한 상위 C11.4 계획과 집중 계획을 병합된 PR 및 보존된 실행 근거에 대조했다. 이 문서는 계획의 상태·수치·범위 주장만 검토하며, 새로운 구현·테스트·플랫폼 동작을 주장하지 않는다.

| # | 실패 가설 | 대조 근거와 판정 |
|---:|---|---|
| 1 | 상위 계획의 현재 날짜가 실제 작업 날짜보다 오래됐다 | 현재 날짜와 PR 병합 후 기록을 대조해 상단 상태 날짜를 `2026-10-28`로 갱신했다. **통과** |
| 2 | 병합 시각을 한국 현지 시각으로 적어 GitHub UTC와 혼동한다 | GitHub `mergedAt` 값 `2026-10-27T16:47:03Z`를 두 계획에 UTC로 그대로 적었다. **통과** |
| 3 | 계획 PR #698을 미병합 상태로 남긴다 | GitHub에서 #698의 기존 병합 기록과 parent plan의 이전 계획 근거를 확인했다. **통과** |
| 4 | 구현 PR #699가 아직 열렸거나 Draft인데 병합된 것으로 쓴다 | GitHub PR state는 `MERGED`다. **통과** |
| 5 | merge commit SHA를 head SHA와 혼동한다 | GitHub `mergeCommit.oid` `d31201de4a46fca406791fcb5d7143ce1eb65f50`을 사용했다. **통과** |
| 6 | 병합 방식을 실제와 다르게 서술한다 | #699는 Rebase and merge로 병합됐다. **통과** |
| 7 | 제한 slice 때문에 활성 lane이 C11.4에서 바뀐다 | 상위 계획의 활성 lane을 C11.4로 유지했다. **통과** |
| 8 | 부모 C11.4 체크박스를 완료로 바꾼다 | `spec/STATUS.md` C11.4는 unchecked 상태로 남아 있다. **통과** |
| 9 | 공식 집계 `72/213`을 임의로 늘린다 | 두 계획 모두 공식 집계 `72/213`을 유지한다. **통과** |
| 10 | 과거 집계값이 최신 공식값으로 보인다 | 상단에서 날짜별 기록의 다른 집계는 해당 시점의 값이라고 구분했다. **통과** |
| 11 | parent plan의 제목이나 목적을 완료 선언으로 바꾼다 | 문서는 계속 C11.4 계획으로 남고 제한 slice만 별도 기록한다. **통과** |
| 12 | child plan이 계속 구현 후보라고 말해 현재 코드를 숨긴다 | 상태를 PR #699 병합 및 검증된 제한 구현으로 갱신했다. **통과** |
| 13 | 병합 전 작업이 끝났다는 이유로 PR을 이미 병합됐다고 앞서 쓴다 | 상태 변경은 GitHub의 실제 `MERGED` 확인 뒤 기록했다. **통과** |
| 14 | 다른 `fit-content` WPT assertion을 이번 변경의 대상으로 바꾼다 | 고정 대상은 `fit-content-track-sizing.html`의 두 번째 assertion으로 유지했다. **통과** |
| 15 | fixture의 40px limit을 다른 입력으로 잘못 기록한다 | 집중 계획의 `fit-content(40px)` 입력과 일치한다. **통과** |
| 16 | `min-height` 없이 indefinite 사례를 테스트한 것으로 쓴다 | 계획·근거는 `min-height:300px` canonical input에 연결된다. **통과** |
| 17 | `height:300px`인 별도 definite-height 사례로 설명한다 | 입력은 `height:auto; min-height:300px` 그대로다. **통과** |
| 18 | explicit row를 implicit row라고 기록한다 | 계획은 유일한 implicit row와 `grid-auto-rows` 입력을 명시한다. **통과** |
| 19 | 10px row를 40px fit-content limit 또는 300px root height와 혼동한다 | expected row는 10px, root block size는 300px로 구분해 적었다. **통과** |
| 20 | root의 고정 inline size를 놓쳐 frame 결과가 불명확하다 | 실행 근거의 root frame은 `60×300px`이며 해당 값을 계획에서 유지했다. **통과** |
| 21 | item frame을 row track size로 대체해 쓴다 | item/child frames와 row size를 각각 10px로 연결한 실행 기록이 있다. **통과** |
| 22 | Android 실기기 검증으로 과장한다 | Android 결과는 API 37 ARM64 AVD로 제한했다. **통과** |
| 23 | iOS 실기기 검증으로 과장한다 | iOS 결과는 iPhone 17 Pro / iOS 26.2 Simulator로 제한했다. **통과** |
| 24 | iOS 표본을 compositor 이후 결과로 확대한다 | linked runtime evidence는 iOS Metal 표본이 compositor 제출 전임을 밝힌다. **통과** |
| 25 | draw 호출을 presentation 증거로 혼동한다 | 기록은 draw/presentation 검사를 구분하고 runtime README를 연결한다. **통과** |
| 26 | oracle·adapter·FFI 테스트 수를 부풀리거나 섞는다 | PR 본문과 runtime evidence의 5/5, 1/1, 1/1 결과를 각각 대조했다. **통과** |
| 27 | fixture-module boundary 검사를 다른 테스트 결과로 혼동한다 | `fixture-module-boundary.test.mjs`의 15/15를 별도로 연결했다. **통과** |
| 28 | GitHub check rollup이 빈 것을 CI pass라고 쓴다 | GitHub `statusCheckRollup`은 빈 배열이며 계획은 CI 통과로 간주하지 않는다. **통과** |
| 29 | 제한 fixture를 일반 CSS admission 또는 전체 WPT 지원으로 확대한다 | 두 계획 모두 일반 CSS admission과 전체 WPT를 미완료 범위로 둔다. **통과** |
| 30 | 제한 slice를 전체 C11.4 완료로 승격한다 | C11.4 체크와 공식 `72/213`을 유지하고 전체 Grid track sizing을 미완료로 적었다. **통과** |

문서 diff에서 상태·수치·범위 오류를 찾지 못했다. PR #699의 체크 rollup이 비어 있다는 한계도 그대로 보존했다.
