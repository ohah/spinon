# PR diff 적대적 검토 · 20개 관점

검토 대상은 누락된 runtime source 파일과 그 누락·검증 경계를 기록하는 근거 문서다. 구현 내부의 runtime 관점 검토는 [별도 기록](implementation-runtime-review-20.md)에 두었다.

| # | 실패 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | 실제 부모의 `#[path]`와 추가할 파일 경로가 다르다 | tracked parent의 상대 경로와 추가 경로가 `generic_intrinsic_flex_fixed/visibility_hidden.rs`로 일치한다. |
| 2 | PR #738에 이미 파일이 포함되어 중복 복구가 된다 | GitHub #738 files 응답에 FFI adapter와 runtime parent/tests는 있으나 이 runtime source는 없다. |
| 3 | HEAD에서 해당 파일이 이미 추적된 것으로 기록된다 | `git cat-file -e HEAD:<path>`가 “not in HEAD”로 실패했고 local `git status`는 `??`였다. |
| 4 | 회귀 테스트는 누락 모듈을 실제로 부르지 않는다 | feature-gated test가 `prepare`를 호출하며 column/row × hidden/visible-control 네 scene을 반복한다. |
| 5 | 부모 라우터와 helper visibility가 맞지 않아 compile 오류가 남는다 | targeted `cargo test`에서 runtime crate와 테스트 target이 compile되고 선택 테스트가 통과했다. |
| 6 | fixture 코드가 기본 제품 artifact에 무조건 들어간다 | C11.4 fixture module은 기존 `c11-4-grid-track-sizing-fixture-test` feature 아래 로드된다. 이번 diff는 gate를 변경하지 않는다. |
| 7 | 공개 API나 제품 feature가 의도치 않게 바뀐다 | 코드 diff는 기존 private fixture module의 source 하나만 복구한다. public API와 Cargo feature 정의는 바뀌지 않는다. |
| 8 | 다른 runtime fixture 소스가 같이 stage된다 | stage 대상은 해당 source와 이 PR에 필요한 전용 evidence 문서뿐이다. `git add -A`는 사용하지 않는다. |
| 9 | source bytes가 검토 중 임의 편집된다 | stage 전 blob ID를 `bf94b8f953af3a6cc8b13d781db4e637e8c4042d`와 비교한다. |
| 10 | 새 의존성 또는 lockfile 변경이 생긴다 | Cargo manifest/lockfile을 수정하지 않는다. |
| 11 | Android/iOS adapter가 추가로 바뀌는데 설명에 빠진다 | 플랫폼 코드 변경은 없다. |
| 12 | roadmap 공식 C11.4 항목을 이 누락 수정만으로 완료 처리한다 | `spec/STATUS.md`와 공식 완료 수를 수정하지 않는다. |
| 13 | 기존 시뮬레이터 캡처를 새 실행 결과처럼 소개한다 | README와 PR 본문에서 #738 merge SHA에 고정된 기존 캡처라고 밝힌다. |
| 14 | 이미지 URL이 PR 화면에서 끊긴다 | 두 raw GitHub URL을 Agent Browser에서 열어 실제 이미지 로드와 natural dimensions를 확인했다. |
| 15 | 이미지가 다른 scene 또는 다른 revision의 캡처다 | URL 파일명은 Android/iOS `row-visibility-hidden`이며 SHA는 #738 merge commit이다. |
| 16 | Android 캡처가 실기기 실행으로 오인된다 | source evidence 및 PR 표기는 Android API 37 AVD다. |
| 17 | iOS 캡처가 실기기 실행으로 오인된다 | source evidence 및 PR 표기는 iPhone 17 Pro / iOS 26.2 Simulator다. |
| 18 | formatter와 targeted test 결과가 기록과 다르다 | `cargo fmt --all -- --check`와 선택 C11.4 test의 실제 exit/result를 README에 기록했다. |
| 19 | 전체 workspace Clippy failure를 통과로 보고한다 | 실패 위치 두 곳과 lint를 README에 그대로 적고, package library Clippy 결과를 분리한다. |
| 20 | CI·merge·외부 독립 reviewer 승인을 미리 주장한다 | 이 diff는 로컬 검증 근거만 남긴다. CI, merge, 외부 reviewer 결과는 실제 발생 전 주장하지 않는다. |

**검토 결과:** 저장소에서 재현 가능한 module path hole을 source tracking으로 닫는 좁은 PR이다. 전체 workspace Clippy 문제는 별도 후속 작업이며 이 diff에 합치지 않는다.
