# C11.4 computed `font-size` 병합 상태 동기화 · PR diff 적대 검토 30개

**검토 대상:** 구현 PR #909의 GitHub 병합 사실을 계획, 공식 상태 대장, 공개 roadmap HTML, 내부 계약 0059와 runtime evidence에 반영한 문서 diff. 이 검토는 구현/runtime 검토 20개와 구현 PR diff 검토 30개를 대체하지 않는다.

| # | 독립 실패 경로 | 확인 및 판정 |
|---:|---|---|
| 1 | 열린 PR을 병합된 것으로 잘못 표기 | **통과.** GitHub PR #909의 `state=MERGED`를 대조했다. |
| 2 | 다른 PR의 병합 SHA를 기록 | **통과.** PR 조회의 `mergeCommit.oid`와 `6fc3c04871904c5c1a0c8a279b85785f382bb739`가 같다. |
| 3 | 병합 시각을 혼동 | **통과.** GitHub `mergedAt=2026-10-31T06:42:47Z`를 사용했다. |
| 4 | PR head SHA를 merge SHA로 취급 | **통과.** 원 구현 커밋과 Rebase and merge 결과 SHA를 구별했다. |
| 5 | `Rebase and merge`를 다른 방식으로 표기 | **통과.** GitHub 병합 방법과 저장소 정책이 일치한다. |
| 6 | 빈 check rollup을 CI 성공으로 해석 | **통과.** check rollup이 비어 있음을 명시하고 CI 성공 주장을 하지 않는다. |
| 7 | 병합 후 코드 revision을 잘못 연결 | **통과.** 로컬 `main`과 `origin/main`이 merge SHA를 가리켰던 근거와 일치한다. |
| 8 | focused runtime 필터의 결과 수 오류 | **통과.** 보존한 post-merge 로그는 5 passed, 0 failed다. |
| 9 | runtime 결과를 전체 runtime suite로 확대 | **통과.** 5/5 focused filter라고 범위를 한정했다. |
| 10 | FFI scene 필터의 결과 수 오류 | **통과.** 보존한 post-merge 로그는 1 passed, 0 failed다. |
| 11 | FFI 단위 검사를 플랫폼 앱 검증으로 확대 | **통과.** FFI 필터와 앞서 수행한 simulator 행렬을 분리해 기술한다. |
| 12 | 원본 로그가 없는 결과를 사후 근거로 제시 | **통과.** runtime·FFI raw output 두 개를 `builds/`에 연결했다. |
| 13 | C11 상위 checkbox를 완료 처리 | **통과.** `spec/STATUS.md`의 C11 상위 checkbox는 그대로 미완료다. |
| 14 | C11.4 checkbox를 제한 fixture 때문에 완료 처리 | **통과.** C11.4 checkbox는 그대로 미완료다. |
| 15 | 공식 집계 수를 증가 | **통과.** 문서의 공식 완료 수 `72/213`을 유지한다. |
| 16 | 체크 수가 근거 없는 하위 완료 개수를 암시 | **통과.** 새 문구는 독립적인 공식 항목 완료 수를 만들지 않는다. |
| 17 | fixture 구현을 제품 CSS 지원으로 표현 | **통과.** 범위를 opt-in fixture로 계속 제한한다. |
| 18 | 10개 scene matrix 범위를 변경 | **통과.** 다섯 값과 min/max-content 조합으로 고정한다. |
| 19 | computed font-size 허용값을 확대 | **통과.** 5, 7.5, 10, 12.5, 20px 이외의 입력을 지원한다고 쓰지 않는다. |
| 20 | 텍스트 또는 font 조건을 누락·변경 | **통과.** pinned Ahem `XX XX` 및 400 weight 조건이 유지된다. |
| 21 | 독립 intrinsic 산식과 행 높이를 변경 | **통과.** `2em`/`5em`, 고정 10px line-height에서 20px/10px 행 높이를 유지한다. |
| 22 | 시뮬레이터 행렬 결과를 잘못 요약 | **통과.** Android와 iOS 각각 10/10 결과를 보존한다. |
| 23 | Android AVD 결과를 실기기 GPU 증거로 확대 | **통과.** AVD임을 명시하고 실기기/hardware GPU는 제외한다. |
| 24 | iOS Simulator를 실기기 또는 compositor 이후 픽셀로 확대 | **통과.** Simulator 제한을 보존한다. |
| 25 | 검은 renderer screenshot을 glyph 출력 증거로 취급 | **통과.** glyph paint/pixel 증거가 아니라고 유지한다. |
| 26 | static archive scan을 전체 APK/IPA 제외로 확대 | **통과.** 정적 archive 경계로 한정한다. |
| 27 | Chrome oracle를 Spinon WPT 적합성으로 확대 | **통과.** WPT 미검증 범위를 유지한다. |
| 28 | 내부 계약 버전을 새로 올림 | **통과.** 내부 계약 숫자 버전 `0.1.0`은 그대로다. |
| 29 | 공개 roadmap HTML이 공식 Markdown 상태와 어긋남 | **통과.** static roadmap에 PR #909, merge SHA, 10/10 simulator 결과, 미완료 한계와 `72/213` 유지 내용을 추가했다. |
| 30 | 계획에 남은 PR 절차 또는 잘못된 문서 링크 | **통과.** pending PR 문구를 제거했고 계획·STATUS·0059·README·roadmap의 새 링크가 저장소 내 실제 파일을 가리킨다. |

**결과:** 30개 검토 항목 모두 통과했다. 문서 갱신은 공식 C11/C11.4 완료 상태나 `72/213`을 바꾸지 않는다.
