# C11.4 max-content 병합 상태 및 다음 작업 정정

PR [#1282](https://github.com/ohah/spinon/pull/1282)이 singleton `grid-auto-rows:max-content`를 다음 구현 후보로 표시했지만, GitHub와 공식 상태 대장을 다시 대조한 결과 그 slice는 이미 PR [#1264](https://github.com/ohah/spinon/pull/1264)로 병합돼 있었다. 이 문서 변경은 그 오류를 바로잡고 C11.4의 다음 bounded 구현이 아직 선택되지 않았음을 명시한다.

## 근거

- PR #1264는 `2026-11-07T11:56:35Z` Rebase and merge됐고 merge SHA는 `5389aae4981bb6dccc7083b554c446f055a72708`이다.
- 공식 [C11.4 상태 기록](../../../STATUS.md) line 328에는 singleton `max-content`의 Style projection, Layout admission, RuntimeSession, Rust 검증과 Android/iOS Simulator 7-box 실행이 이미 기록돼 있다.
- PR #1264의 [실행 근거](../c11-4-root-html-intrinsic-auto-rows-max-content-runtime-2026-11-07/README.md)는 layout 17/17, style-to-layout 25/25, RuntimeSession 3/3 및 기존 intrinsic-row 회귀 7/7을 기록하고, simulator 화면을 visual smoke로 한정한다.
- PR #1280은 `RootHtmlFitContentAutoRowsV1`의 four-member list를 추가한 별도 owner/profile이다. PR #1264의 singleton `RootHtmlIntrinsicTrackSizingV1` slice와 구분한다.

## 변경과 경계

- `plan/c11-4-root-html-intrinsic-auto-rows-max-content.md`를 PR #1264 병합 상태로 표시한다.
- `plan/c11-4-grid-track-sizing.md`에서 이미 병합된 max-content를 다음 작업으로 되풀이하지 않고, PR #1280 이후 다음 bounded slice는 아직 선정되지 않았다고 기록한다.
- 공식 C11.3/C11.4/C11 체크 및 완료 집계 `78/214`는 변경하지 않는다. `spec/STATUS.md`에는 PR #1264 기록이 이미 있으므로 이번 변경에서 중복 수정하지 않는다.
- 제품 코드, API, simulator evidence, Tailnet 제공본은 이 정정 PR에서 변경하지 않는다.

`git diff --check`와 새로 연결된 경로를 확인했다. 문서 변경이므로 제품 테스트를 실행하지 않았다. 서로 다른 실패 관점을 확인한 문서 PR diff 검토 30개는 [검토 기록](./pr-diff-review-30.md)에 있다.
