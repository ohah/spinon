# C11.1 named-area public roadmap 변경 실패 관점 검토 30개

**범위:** 구현 PR #1006 병합 뒤 공개 `spec/public/roadmap.html`의 named-area 카드를 현재 제한 runtime 상태와 일치시키는 문서 변경을 검토했다.

| # | 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | 카드가 구현 전 상태로 남아 있다. | “구현/runtime 시작 전” 문구를 제거하고 현재 제한 candidate로 갱신했다. 통과. |
| 2 | 계획 PR과 구현 PR을 하나로 혼동한다. | 구현 PR #1006과 merge SHA를 별도로 쓴다. 통과. |
| 3 | 잘못된 구현 PR URL을 연결한다. | 링크가 `ohah/spinon/pull/1006`이다. 통과. |
| 4 | merge 시각이 GitHub metadata와 다르다. | `2026-11-01T22:11:00Z`를 사용한다. 통과. |
| 5 | merge SHA가 current origin/main과 다르다. | `f4bcce22566c147277bf51ab1f3eae37f12caf4f`를 사용한다. 통과. |
| 6 | `grid-template-areas`가 일반 CSS 지원으로 과장된다. | 고정 2×2 runtime candidate라고 제한한다. 통과. |
| 7 | named-area 작업이 기본 app 경로가 아닌 fixture 전용이라고 오해된다. | 기본 parsed Root HTML route를 명시한다. 통과. |
| 8 | 세 semantic item 수를 잘못 쓴다. | fixture와 evidence의 세 item 수에 맞춘다. 통과. |
| 9 | focused Rust/Clippy 상태가 테스트 성공과 뒤섞인다. | “집중 Rust 검증과 Clippy” 수준으로만 쓴다. 통과. |
| 10 | Android hardware device 검증으로 오해된다. | Android API 37 ARM64 AVD라고 명시한다. 통과. |
| 11 | iOS physical device 검증으로 오해된다. | iPhone 17 Pro/iOS 26.2 Simulator라고 명시한다. 통과. |
| 12 | draw/present를 pixel readback으로 확대한다. | visual capture와 PixelCopy·Metal readback 미수행을 구분한다. 통과. |
| 13 | post-compositor 결과를 주장한다. | compositor 이후 pixel 주장이 없다. 통과. |
| 14 | hardware GPU 결과를 주장한다. | hardware GPU 주장이 없다. 통과. |
| 15 | 전체 style-to-layout suite를 녹색으로 표시한다. | 278/279 및 남은 기존 C07 실패를 밝힌다. 통과. |
| 16 | C07 실패의 값이 근거와 다르다. | 세부 값 생략으로 오기 없이 범위를 요약한다. 통과. |
| 17 | suite 실패를 named-area 회귀라고 잘못 귀속한다. | 카드가 기존 C07 serialization 비교로 특정한다. 통과. |
| 18 | PR checks 없음이 CI 통과로 표시된다. | check run과 required status checks가 없다고 적고 성공을 주장하지 않는다. 통과. |
| 19 | C11.1이 완료 처리된다. | C11.1 미완료로 유지한다. 통과. |
| 20 | C11.3.3이 이 변경으로 완료 처리된다. | C11.3.3 미완료로 유지한다. 통과. |
| 21 | C11 상위 항목이 완료 처리된다. | C11 미완료로 유지한다. 통과. |
| 22 | 공식 완료 집계가 올라간다. | `72/213`을 그대로 표시한다. 통과. |
| 23 | 계획 링크가 잘못된 route를 가리킨다. | `/spinon/plan/c11-root-html-grid-template-areas.html`이다. 통과. |
| 24 | 실행 README 링크가 잘못된 route를 가리킨다. | `.../c11-root-html-grid-template-areas-runtime-2026-11-02/README.html`이다. 통과. |
| 25 | Android screenshot 링크가 빠지거나 다른 실행을 가리킨다. | named-area AVD PNG를 연결한다. 통과. |
| 26 | iOS screenshot 링크가 빠지거나 다른 실행을 가리킨다. | named-area iOS Simulator PNG를 연결한다. 통과. |
| 27 | 적대 검토 자료 링크가 잘못된다. | 해당 evidence의 `pr-diff-review-30.html`을 연결한다. 통과. |
| 28 | 계획 PR #1004를 구현 병합으로 다시 표시한다. | 새 카드 제목과 본문은 구현 PR #1006을 기준으로 한다. 통과. |
| 29 | HTML 태그가 중첩되거나 닫히지 않는다. | 카드 paragraph 경계와 nested link/code markup을 대조했다. 통과. |
| 30 | 카드가 이전 #1004 precomparison만 대표하고 최신 runtime evidence를 빠뜨린다. | 실행 근거와 두 플랫폼 화면을 함께 연결한다. 통과. |

## 판정

카드는 PR #1006의 제한 구현·Simulator 범위·미완료 회귀를 표현한다. 공식 완료 상태 `72/213`은 변하지 않는다.
