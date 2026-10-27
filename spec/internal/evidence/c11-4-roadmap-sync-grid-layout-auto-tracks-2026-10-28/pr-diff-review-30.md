# C11.4 `grid-layout-auto-tracks` 상태 정정 · PR diff 검토

이번 문서 diff가 이미 병합된 PR #688을 정확히 반영하고, 앞선 roadmap 검토의 오판을 투명하게 바로잡는지 계획·실행 근거·GitHub 병합 기록과 대조했다.

| # | 실패 가설 | 대조 근거와 판정 |
|---:|---|---|
| 1 | 상위 계획의 현재 상태 날짜가 stale하다 | 문서 상단은 2026-10-28로 유지한다. **통과** |
| 2 | 공식 완료 집계가 임의로 변경된다 | C11.4 상위 상태와 `72/213`은 수정하지 않았다. **통과** |
| 3 | C11.4 전체 체크박스를 완료 처리한다 | `spec/STATUS.md`의 C11.4는 unchecked로 유지한다. **통과** |
| 4 | PR #688에 다른 WPT 파일을 연결한다 | 집중 계획의 source는 `grid-layout-auto-tracks.html`이다. **통과** |
| 5 | PR #688이 아직 open인데 merged라고 표기한다 | GitHub PR state는 `MERGED`다. **통과** |
| 6 | PR #688의 merge timestamp를 추정한다 | GitHub `mergedAt`과 child plan은 `2026-10-27T08:19:30Z`로 일치한다. **통과** |
| 7 | PR #688 merge SHA를 head commit으로 혼동한다 | GitHub `mergeCommit.oid`와 plan의 `0b434e16ac66d0a742057e6e1e5749151bd3e08f`를 대조했다. **통과** |
| 8 | #688의 기여를 PR title보다 넓게 적는다 | parent row는 positive implicit track 및 제한 scene 연결로 표현한다. **통과** |
| 9 | upstream CSS와 앱 입력을 같은 입력으로 주장한다 | child plan의 app 입력은 Ahem `10px/10px`로 정규화한 scene임을 명시한다. **통과** |
| 10 | WPT screenshot reftest를 testharness pass로 바꿔 말한다 | child plan은 원본의 `meta rel=match`가 testharness 검사가 아니라고 밝힌다. **통과** |
| 11 | columns 기대값을 부정확하게 설명한다 | runtime evidence는 `[100,50]px`이고 parent는 세부값 대신 제한 범위를 연결한다. **통과** |
| 12 | rows 기대값을 부정확하게 설명한다 | runtime evidence는 `[10,10]px`다. **통과** |
| 13 | root frame을 100×38px Chrome baseline으로 혼동한다 | normalized runtime expectation은 `100×20px`; Chrome 기본 font 관측은 별도라고 기록한다. **통과** |
| 14 | B와 C의 위치·기여를 합쳐 버린다 | runtime evidence는 B의 implicit column, C의 positive implicit row와 contributions를 따로 기록한다. **통과** |
| 15 | PR #688의 한 scene을 원본 전체로 표시한다 | parent table은 명시적으로 “한 normalized scene”이라고 제한한다. **통과** |
| 16 | Android 시뮬레이터를 실기기로 부른다 | child plan은 API 37 ARM64 AVD로 특정한다. **통과** |
| 17 | iOS Simulator를 iPhone 실기기로 부른다 | child plan은 iPhone 17 Pro / iOS 26.2 Simulator로 특정한다. **통과** |
| 18 | 표면 표본 결과를 frame 검사로 오인한다 | runtime evidence는 geometry와 surface samples를 별도 표로 기록한다. **통과** |
| 19 | Metal sample을 compositor 이후 픽셀로 확대한다 | child plan은 iOS readback이 compositor submission 전이라고 명시한다. **통과** |
| 20 | static archive 검사로 APK/IPA 전체 fixture exclusion을 주장한다 | runtime evidence는 해당 marker 검사가 static Rust archive에 한정됨을 말한다. **통과** |
| 21 | PR #688의 빈 check rollup을 CI 통과라고 부른다 | child plan은 check rollup이 비어 있고 CI pass가 아님을 기록한다. **통과** |
| 22 | #688의 계획·근거 링크가 끊긴다 | 집중 계획과 runtime README 경로가 저장소에 존재함을 확인했다. **통과** |
| 23 | PR #683의 동일 문서 mutation을 #688 scene과 합친다 | parent의 #683 section은 same-HostDocument 8-stage sequence로 별도 유지한다. **통과** |
| 24 | PR #691의 auto-repeat intrinsic scenes를 #688로 합친다 | parent table에서 별도 WPT 파일과 PR #691 link를 유지한다. **통과** |
| 25 | PR #695 auto-fit 35-scene 결과를 #688로 합친다 | parent table에서 별도 WPT 파일, PR #695와 35-scene 근거를 유지한다. **통과** |
| 26 | PR #664와 #699의 두 fit-content assertion을 혼합한다 | parent는 마지막 assertion #664와 min-height assertion #699를 각각 적는다. **통과** |
| 27 | 이전 roadmap 검토의 잘못된 row 29를 조용히 지운다 | 해당 row를 PR #688을 놓친 판정이라고 명시하고 사후 정정을 덧붙였다. **통과** |
| 28 | #688의 한 scene을 전체 `grid-layout-auto-tracks.html` 통과로 과장한다 | parent text는 WPT file 전체·일반 auto-track 지원과 분리한다. **통과** |
| 29 | 다른 auto-track 조합까지 완료로 승격한다 | parent는 남은 조합과 전체 C11.4가 미완료라고 남긴다. **통과** |
| 30 | 문서 정정이 runtime code나 제품 계약을 바꾼다 | 변경은 상위 계획과 검토 기록뿐이며 product code와 `spec/STATUS.md`는 건드리지 않았다. **통과** |

PR #701 review artifact의 row 29가 잘못된 미구현 판정을 포함했던 사실도 본 정정에 연결했다. 변경은 PR #688의 제한 scene 상태만 바로잡으며 전체 WPT·일반 CSS admission·C11.4 완료를 주장하지 않는다.
