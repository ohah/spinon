# PR #675 공개 roadmap/status 동기화 diff 검토 30회

**대상:** PR #675 병합 기록을 추가한 `spec/STATUS.md`, `spec/public/roadmap.html`, 로컬 Agent Browser 미리보기 근거
**판정:** 아래의 서로 다른 상태·출처·렌더링·범위 경계를 검토했다.

| # | 실패 관점 | 확인한 조건과 판정 |
| ---: | --- | --- |
| 1 | C11 상위 상태가 완료로 바뀜 | 상위 C11 체크는 계속 미완료다. 통과 |
| 2 | C11.4 하위 상태가 완료로 바뀜 | C11.4 체크를 유지한다. 통과 |
| 3 | 제한 fixture 때문에 공식 완료 수가 증가 | `72/213`을 그대로 표시한다. 통과 |
| 4 | STATUS와 공개 roadmap의 완료 수가 어긋남 | 두 페이지 모두 `72/213`을 표시한다. 통과 |
| 5 | 일부 구현이 전체 C11.4로 확대 표시 | 카드에 제한된 app-runtime fixture라고 적었다. 통과 |
| 6 | 병합되지 않은 PR을 병합으로 기록 | PR #675 상태는 GitHub에서 `MERGED`다. 통과 |
| 7 | 병합 SHA가 원본 브랜치 commit으로 잘못 기록 | 실제 merge commit `96a392d32f69fae6453aecd897aa6d77c8b4aced`를 쓴다. 통과 |
| 8 | 병합 시각이 현지 시각과 UTC를 혼동 | GitHub `mergedAt`의 UTC `2026-10-27T02:16:18Z`를 기록한다. 통과 |
| 9 | merge 방식이 저장소 규칙과 다름 | PR은 Rebase and merge로 병합됐다. 통과 |
| 10 | PR 링크가 다른 구현 PR을 가리킴 | 링크는 `/pull/675`를 가리킨다. 통과 |
| 11 | 비어 있는 check rollup을 CI 통과로 표현 | rollup이 비었다고 기록하며 CI 통과 주장을 하지 않는다. 통과 |
| 12 | branch protection의 required checks 상태 누락 | main required status checks가 설정되지 않았음을 기록한다. 통과 |
| 13 | Android 기기 종류를 실기기로 확대 | API 37 ARM64 16 KB AVD라고 명시한다. 통과 |
| 14 | iOS 기기·OS를 시뮬레이터와 혼동 | iPhone 17 Pro / iOS 26.2 Simulator라고 명시한다. 통과 |
| 15 | 두 플랫폼의 scene 수가 섞임 | 각 플랫폼 네 fixed scene의 4/4 결과를 기록한다. 통과 |
| 16 | negative route가 제품 입력 지원으로 오해 | unknown scene와 profile conflict 거부만 기록한다. 통과 |
| 17 | frame 검사만으로 stage 동작을 주장 | Rows stage snapshot과 내부 trace 검사를 구분해 적었다. 통과 |
| 18 | trace만으로 전체 geometry를 주장 | 전체 node frame 검사도 별도 명시한다. 통과 |
| 19 | source-order 결과가 누락 | source-order contribution을 명시한다. 통과 |
| 20 | fixed Columns control이 Rows 결과와 혼합 | fixed Columns 대조군을 별도로 표시한다. 통과 |
| 21 | surface 제출이 최종 화면 픽셀과 동일시됨 | draw/present와 screenshot 한계를 구분한다. 통과 |
| 22 | 실패 재현 수정이 기록에서 빠짐 | implicit column 입력 결함과 `grid-column:1 / 2` 수정이 둘 다 있다. 통과 |
| 23 | Rust test 결과가 simulator matrix와 혼합 | focused Rust test 1/1을 별도 기록한다. 통과 |
| 24 | formatting 확인을 build/CI 결과로 과장 | `cargo fmt --all -- --check`만 format 근거로 적었다. 통과 |
| 25 | archive marker가 최종 APK/IPA 배제를 뜻함 | 정적 FFI archive 한계와 APK/IPA 제외 미검증을 적었다. 통과 |
| 26 | 일반 CSS admission을 지원한다고 오해 | 일반 author CSS admission은 남은 범위로 명시한다. 통과 |
| 27 | WPT 검증 범위가 존재한다고 오해 | WPT 미검증을 명시한다. 통과 |
| 28 | AVD 결과가 hardware GPU나 실기기 결과로 오해 | 실기기/hardware GPU 검증이 아님을 명시한다. 통과 |
| 29 | 캡처가 Tailnet live 배포 증거로 재사용됨 | README/capture JSON은 로컬 미리보기이며 Tailnet 증거가 아님을 명시한다. 통과 |
| 30 | roadmap 상대 링크가 새 파일 위치와 불일치 | 계획·실행 근거·검토·화면 링크는 merge SHA에 고정된 GitHub 경로다. Agent Browser에서 카드 텍스트와 링크를 확인했다. 통과 |
