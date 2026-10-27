# C11.4 상태 대장 PR diff 검토 30개 · 2026-10-28

## 범위

검토 대상은 `spec/STATUS.md`의 두 C11.4 기록 교체다. 이미 병합된 PR #683·#691의 상태·merge SHA·지원 경계를 계획서, 공개 roadmap, 원본 실행 evidence와 대조했다. 이 문서 PR은 앱 코드나 runtime 결과를 새로 검증하지 않는다. C11.4 완료 표시와 공식 `72/213`을 바꾸지 않는다.

| # | 실패 가설 | 판정과 근거 |
|---:|---|---|
| 1 | 제한 slice 결과를 C11.4 전체 완료로 올릴 수 있다. | **PASS** — 상위 `C11.4 intrinsic track sizing` checkbox는 미완료로 남아 있다. |
| 2 | 두 PR을 완료 수에 더해 `72/213`이 달라질 수 있다. | **PASS** — 두 상태 문구 모두 `72/213` 유지와 상위 미완료를 명시한다. |
| 3 | #683 기록 날짜가 plan/runtime 기록과 다를 수 있다. | **PASS** — status, merged plan, public roadmap이 2026-10-27을 쓴다. |
| 4 | #683 링크가 다른 구현 PR을 가리킬 수 있다. | **PASS** — GitHub PR #683 제목은 auto-repeat intrinsic 재계산 fixture 구현이다. |
| 5 | #683 병합 시간이 틀릴 수 있다. | **PASS** — GitHub mergedAt과 plan이 `2026-10-27T05:54:04Z`로 일치한다. |
| 6 | #683 merge SHA가 원래 branch SHA일 수 있다. | **PASS** — status, GitHub mergeCommit, plan이 `83df2c59dd63d83b53ff68f8a4717e0fae5b841e`로 일치한다. |
| 7 | #683 WPT 이름이 auto-repeat 항목과 혼동될 수 있다. | **PASS** — `grid-change-intrinsic-size-with-auto-repeat-tracks-001.html`을 정확히 쓴다. |
| 8 | #683 집중 계획 경로가 깨질 수 있다. | **PASS** — `plan/c11-4-auto-repeat-intrinsic-recompute.md`가 존재한다. |
| 9 | #683 사전 비교가 구현 결과와 섞일 수 있다. | **PASS** — Chrome/WPT 및 독립 기대값 README를 별도 링크로 둔다. |
| 10 | #683 runtime evidence 링크가 빠질 수 있다. | **PASS** — simulator·Rust sequence 근거 README 경로가 존재한다. |
| 11 | #683의 동일 문서 재계산을 앱 scene으로 오해할 수 있다. | **PASS** — 같은 HostDocument/node handle의 Rust sequence와 독립 app scenes를 분리해 적는다. |
| 12 | #683 revision이 실제로 진행했는지 알 수 없다. | **PASS** — document/style revision 및 invalidation 확인을 범위에 적는다. |
| 13 | #683이 최종 used columns를 확인하지 않았는데 주장할 수 있다. | **PASS** — 기존 runtime evidence가 8개 순차 상태의 used columns를 연결한다. |
| 14 | #683 absolute probe 결과가 빠져 재계산 범위를 과장할 수 있다. | **PASS** — item/probe geometry 재계산으로 범위를 제한한다. |
| 15 | #683 Android 결과가 AVD 아닌 실기기 결과로 읽힐 수 있다. | **PASS** — Android API 37 ARM64 AVD라고 명시한다. |
| 16 | #683 iOS 결과의 OS/runtime 정보가 누락될 수 있다. | **PASS** — iPhone 17 Pro / iOS 26.2 Simulator라고 명시한다. |
| 17 | #683의 8/8을 전체 WPT 8개 test pass로 확대할 수 있다. | **PASS** — 앱 scene과 같은 문서의 mutation sequence를 구분하고 전체 WPT를 주장하지 않는다. |
| 18 | #683 check rollup이 비어 있는데 CI 통과라 쓸 수 있다. | **PASS** — merge 당시 check rollup이 비어 있었으며 CI 통과로 기록하지 않는다고 적는다. |
| 19 | #683 결과를 일반 author CSS admission으로 확대할 수 있다. | **PASS** — 제한 fixture이며 일반 CSS admission이 아니라고 명시한다. |
| 20 | #683 evidence 20/30 리뷰 경로가 존재하지 않을 수 있다. | **PASS** — 구현/runtime review 20개와 PR diff review 30개 모두 저장소에 있다. |
| 21 | #691이 미병합이거나 다른 PR일 수 있다. | **PASS** — GitHub PR #691은 제한 auto-repeat intrinsic runtime 구현으로 merged 상태다. |
| 22 | #691 병합 시각이 public roadmap과 어긋날 수 있다. | **PASS** — GitHub, plan, public roadmap이 `2026-10-27T10:30:12Z`로 일치한다. |
| 23 | #691 merge SHA가 plan과 다를 수 있다. | **PASS** — `8993e3f5f28f13ab707018dda52f8f619d2b8bb4`가 status, plan, public roadmap에서 일치한다. |
| 24 | #691 WPT 파일명이 #683 재계산 test와 뒤바뀔 수 있다. | **PASS** — `grid-auto-repeat-intrinsic-001.html`을 별도 명시한다. |
| 25 | #691의 pinned input 수와 교차 조합이 부정확할 수 있다. | **PASS** — auto-fill/auto-fit, trailing track shape, max/min-content의 8 scene이라고 쓴다. |
| 26 | #691 Android 8/8이 동일 PR의 근거가 아닐 수 있다. | **PASS** — 계획 및 runtime README의 Android API 37 ARM64 AVD 행렬과 연결한다. |
| 27 | #691 iOS 8/8이 simulator 대신 실기기로 오해될 수 있다. | **PASS** — iPhone 17 Pro / iOS 26.2 Simulator로 제한한다. |
| 28 | min-content overflow 장면의 표본 검증을 전체 화면 픽셀 검증으로 확대할 수 있다. | **PASS** — 두 장면의 border·outside item·outside control 표본만 각 3/3으로 한정한다. |
| 29 | PixelCopy/Metal 표본을 동일한 platform stage로 묶을 수 있다. | **PASS** — Android PixelCopy와 iOS compositor 제출 전 Metal drawable을 구분한다. |
| 30 | #691을 전체 WPT, 일반 CSS, hardware GPU 또는 iOS compositor 결과로 읽을 수 있다. | **PASS** — Android ANGLE/SwiftShader, 빈 CI rollup, 미검증 platform 범위를 status 문구에 남긴다. |

## 결론

두 항목을 이미 병합된 제한 구현으로 정정했으며 supporting code/evidence나 공개 roadmap과 모순되지 않는다. 이 PR은 C11.4 완료를 표시하거나 공식 집계를 바꾸지 않는다.
