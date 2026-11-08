# PR #1286 병합 후 문서 동기화 · 변경 실패 관점 30개

**대상:** `spec/STATUS.md`, `spec/internal/0059-c11-grid-layout.md`, `plan/c11-4-root-html-repeated-intrinsic-tail-permutations.md`
**기준:** PR #1286 merge commit `08da644ed876709798e118f0be9cefe79edfcd76`
**변경 성격:** 구현 코드는 바꾸지 않고 병합 전 상태 문구를 실제 merge 결과와 맞춘다.

| # | 관점 | 확인할 실패 경로 | 결과 |
|---:|---|---|---|
| 1 | PR identity | 다른 C11.4 PR의 merge 상태를 기록하는가? | #1286 링크와 제목이 대상 구현 slice와 일치한다. |
| 2 | merge timestamp | UTC와 로컬 날짜를 혼동하는가? | GitHub `mergedAt`의 `2026-11-08T09:10:52Z`를 기록했다. |
| 3 | commit identity | PR head SHA를 새 merge SHA로 잘못 표기하는가? | main과 PR `mergeCommit`에서 확인한 `08da644e…`를 기록했다. |
| 4 | merge method | Squash 또는 merge commit이라고 잘못 쓰는가? | 실제 `Rebase and merge`로 표기했다. |
| 5 | stale wording | 문서에 “아직 PR 병합 전”이 남는가? | 0059의 해당 문장을 병합 사실과 제한 범위 설명으로 교체했다. |
| 6 | plan state | 구현 계획이 계속 “구현 진행 중”으로 보이는가? | 계획의 상태와 현재 단계에 PR #1286 병합을 기록했다. |
| 7 | official checklist | bounded slice 병합만으로 C11.4/C11을 완료 처리하는가? | 두 공식 체크 상태는 그대로 미완료다. |
| 8 | completion count | 상위/세부 완료 수를 임의로 올리는가? | 공식 `78/214`를 유지했다. |
| 9 | product support | fixture profile을 일반 제품 CSS 지원으로 확대하는가? | 일반 CSS admission과 전체 C11.4/C11 완료는 범위 밖이라고 명시했다. |
| 10 | versioning | 문서 변경을 이유로 내부 계약 숫자 버전을 올리는가? | 내부 계약 `0.1.0`을 유지한다. |
| 11 | route boundary | 다른 runtime route에도 구현이 적용된다고 읽히는가? | 기본 `RuntimeRootHtmlTextPaintV1` route로 한정한다. |
| 12 | profile boundary | `RootHtmlFitContentAutoRowsV1` 외 profile을 포함하는가? | 기존 제한 profile로 범위를 고정했다. |
| 13 | list length | 임의 길이의 track list를 허용한다고 읽히는가? | 여섯-member list만 기록했다. |
| 14 | first member | fit-content 위치 제한이 빠지는가? | 첫 member에만 finite non-negative px 또는 direct percentage cap을 허용한다. |
| 15 | percentage basis | 무조건적인 percentage 지원으로 확대하는가? | direct percentage만 표현하고 기존 definite-basis 제한을 유지한다. |
| 16 | tail multiplicity | intrinsic track의 개수 경계가 모호한가? | min-content 2개, max-content 2개, auto 1개를 정확히 명시한다. |
| 17 | permutation count | 중복 순서를 별도 순서로 세는가? | 후행 multiset의 30개 고유 순서라고 기록한다. |
| 18 | predecessor compatibility | PR #1284의 기존 순서를 빠뜨리는가? | 기존 순서를 포함한다고 기록한다. |
| 19 | style projection | 120 typed cases를 전체 CSS 적합성으로 확대하는가? | 해당 제한 projection 행렬로만 기록한다. |
| 20 | layout matrix | 120 scene이 일반 Grid 전체를 검증한다고 쓰는가? | 지정 fixture의 Layout 행렬로 제한한다. |
| 21 | runtime matrix | RuntimeSession 결과가 모든 세션 동작을 보장한다고 쓰는가? | 지정 120-scene 행렬과 회귀 경계로 한정한다. |
| 22 | rejection atomicity | unsupported order rejection을 빠뜨리는가? | atomic rejection과 same-session recovery를 함께 기록한다. |
| 23 | Chrome oracle | 고정 비교 표본을 브라우저 호환성 전반으로 일반화하는가? | Chrome 157 사전 비교와 120 scene으로 한정한다. |
| 24 | comparison checks | 9,480 checks를 픽셀 동등성으로 오인하는가? | track/item/payload 비교와 최대 0 CSS px 차이로 설명한다. |
| 25 | Android identity | Android 시뮬레이터 결과를 실기기로 표현하는가? | API 37 ARM64 AVD로 표기한다. |
| 26 | iOS identity | iOS 시뮬레이터 결과를 실기기로 표현하는가? | iPhone 17 Pro/iOS 26.2 Simulator로 표기한다. |
| 27 | fixture identity | 두 플랫폼의 서로 다른 입력을 동일 fixture로 오인하는가? | 동일 fixture SHA를 근거 문서와 연결한다. |
| 28 | presentation evidence | draw-presented를 surface readback으로 확대하는가? | Android draw-presented만 기록하고 PixelCopy/Metal readback은 아니라고 명시한다. |
| 29 | CI status | 로컬 검사 결과를 CI 통과로 표현하는가? | GitHub check이 보고되지 않아 CI 통과 주장을 하지 않는다. |
| 30 | roadmap and links | 큰 C11.4가 완료처럼 보이거나 근거 링크가 끊기는가? | 공개 로드맵은 C11.4를 계속 진행 중으로 두고 STATUS·0059·계획 링크를 교차 확인한다. |

## 결론

문서의 stale pre-merge 상태만 갱신한다. 병합된 bounded slice와 전체 C11.4 제품 지원 완료를 구분하고, 공식 체크와 `78/214`는 변경하지 않는다.
