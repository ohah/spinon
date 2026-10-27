# C11.4 auto-repeat roadmap 동기화 · PR diff 검토

상위 Grid 계획의 auto-repeat WPT 상태를 2026-10-28 기준 GitHub 병합 기록, 각 slice 계획, 실행 근거와 대조했다. 각 행은 문서가 서로 다른 WPT 입력과 evidence 경계를 보존하는지 확인한다.

| # | 실패 가설 | 대조 근거와 판정 |
|---:|---|---|
| 1 | 상위 상태 날짜나 완료 집계를 오래된 값으로 둔다 | 최신 기록 날짜는 2026-10-28이며 공식 `72/213`은 유지했다. **통과** |
| 2 | #683의 WPT를 auto-repeat intrinsic 고정 matrix와 혼동한다 | #683은 `grid-change-intrinsic-size-with-auto-repeat-tracks-001.html`의 같은 문서 연속 변경이다. **통과** |
| 3 | #683을 미병합 상태로 기록한다 | GitHub PR state는 `MERGED`다. **통과** |
| 4 | #683의 merge 시각이나 SHA를 틀린다 | `2026-10-27T05:54:04Z`, merge SHA `83df2c59dd63d83b53ff68f8a4717e0fae5b841e`와 일치한다. **통과** |
| 5 | #683의 Rust sequence를 앱 scene 실행으로 대체해 서술한다 | runtime evidence는 하나의 `HostDocument`에서 연속 mutation을 검사한다고 명시한다. **통과** |
| 6 | #683에서 node identity 보존을 누락한다 | 여섯 node handle을 유지한 8단계 sequence로 제한해 썼다. **통과** |
| 7 | #683의 revision/invalidation 조건을 과장한다 | generation 유지, document/render-tree/style revision 진행, dirty-root invalidation을 계획과 evidence에서 대조했다. **통과** |
| 8 | #683 Android 결과를 실기기 결과로 기록한다 | Android 결과는 API 37 ARM64 AVD로 표현했다. **통과** |
| 9 | #683 iOS 앱 행렬을 문서 mutation sequence로 부른다 | 두 simulator의 고정 scene은 별도 scene evidence로, mutation 검사는 Rust sequence로 구분했다. **통과** |
| 10 | #683 simulator scene 수를 잘못 기록한다 | runtime evidence에서 Android와 iOS 각각 8/8이다. **통과** |
| 11 | #683의 빈 GitHub check rollup을 CI pass라고 주장한다 | PR check rollup은 빈 배열이며 문서가 CI 통과라고 하지 않는다. **통과** |
| 12 | #691을 #683과 같은 WPT 파일로 취급한다 | #691은 별도 파일 `grid-auto-repeat-intrinsic-001.html`을 다룬다. **통과** |
| 13 | #691을 미병합 계획으로 남긴다 | 집중 계획과 GitHub 기록 모두 #691 병합을 확인한다. **통과** |
| 14 | #691의 병합 시각을 틀린다 | `2026-10-27T10:30:12Z`로 계획과 GitHub가 일치한다. **통과** |
| 15 | #691의 merge SHA를 잘못 복사한다 | `8993e3f5f28f13ab707018dda52f8f619d2b8bb4`로 대조했다. **통과** |
| 16 | #691의 8-scene 제한을 전체 WPT 8개 PASS로 넓힌다 | parent table은 제한 runtime 8-scene 행렬로 부르고 child plan은 전체 WPT/Spinon conformance를 주장하지 않는다. **통과** |
| 17 | #695의 WPT를 auto-repeat 재계산 파일로 혼동한다 | #695는 `grid-auto-fit-columns-001.html` 35개 scene이다. **통과** |
| 18 | #695가 아직 계획 단계라고 남긴다 | child plan과 GitHub에서 실제 병합 상태를 확인했다. **통과** |
| 19 | #695의 merge time/SHA가 실제와 다르다 | `2026-10-27T14:39:47Z`, `d6dd9fd15d2897804142e6bb119149a633952a9b`와 일치한다. **통과** |
| 20 | #695 Android matrix 수를 잘못 쓴다 | child plan과 status evidence의 Android API 37 AVD 결과는 35/35다. **통과** |
| 21 | #695 iOS matrix 수를 잘못 쓴다 | child plan과 runtime evidence의 iPhone 17 Pro/iOS 26.2 Simulator 결과는 35/35다. **통과** |
| 22 | #695 pixel sample 수를 전체 scene 또는 일반 표본으로 과장한다 | 여섯 overflow scene에서 PixelCopy와 Metal 표본이 각각 24/24 exact다. **통과** |
| 23 | #695 Metal 표본을 compositor 이후 결과로 부른다 | child plan은 iOS Metal 표본이 compositor 제출 전이라고 적는다. **통과** |
| 24 | #695 빈 check rollup을 CI pass로 적는다 | child plan은 status check 미보고와 CI 미확인을 명시한다. **통과** |
| 25 | `fit-content(40px) auto` assertion을 미구현 후보로 남긴다 | parent의 PR #664 entry와 대조해 해당 제한 runtime slice가 병합됐음을 확인했다. **통과** |
| 26 | `height:auto; min-height:300px` assertion을 #664와 합친다 | 두 번째 assertion은 별도의 PR #699 slice이며 parent plan에서 분리돼 있다. **통과** |
| 27 | 두 fit-content assertion의 기대 행 크기를 뒤섞는다 | #664의 `[10px,290px]`와 #699의 단일 `10px` 행을 각각 구분했다. **통과** |
| 28 | limited fixtures를 일반 CSS admission 또는 전체 WPT 통과로 넓힌다 | 상위 계획과 각 child plan은 해당 범위를 명시적으로 미완료로 둔다. **통과** |
| 29 | `grid-layout-auto-tracks.html`을 이미 구현된 것처럼 기록한다 | 이를 남은 계획 후보로만 두고 구현 상태는 주장하지 않는다. **통과** |
| 30 | 변경 때문에 공식 상태표나 완료 수가 바뀐다 | `spec/STATUS.md` C11.4 checkbox는 unchecked이고 공식 집계 `72/213`으로 유지된다. **통과** |

이 문서 변경은 상위 계획의 기존 상태 설명만 최신 병합 기록에 맞췄다. 전체 WPT 실행이나 C11.4 완료를 주장하지 않는다.
