# PR #624 C11.4 상태·roadmap 동기화 변경 검토 30개

- 검토 대상: `origin/main...065ddf9ade798ab7ead66b872ee3e00996b277de`의 정확한 PR diff
- GitHub PR: [#624](https://github.com/ohah/spinon/pull/624), base `main`, head `docs/c11-4-cyclic-auto-repeat-merge-status`
- 변경 범위: 5개 문서/evidence 파일, 21 insertions / 12 deletions
- 구현 PR 대조: #623은 `MERGED`, merge SHA `56daefe96e8143959bc99a89c7a8c80e7f1cc4ba`, merged at `2026-10-25T04:35:52Z`; source SHA `c5b8ef75804e39d36a37aa61b82b1f028115fefd`는 PR #623 head `5c6e24fc2ec5d1303418cd15f00fc6e4cb51947c`의 ancestor
- 현재 PR metadata: labels `documentation`, `area: docs`; merge state `CLEAN`; GitHub check rollup은 비어 있고 main required status checks도 설정되지 않음
- 판정: **27 PASS / 3 PARTIAL / 0 FAIL**
- 검토는 문서, Git/GitHub metadata 및 기존 evidence 읽기에 한정했다. 테스트·빌드·브라우저·시뮬레이터를 실행하지 않았다.

| # | 실패 관점 | 판정 | 심각도 | 근거와 결과 |
|---:|---|---|---|---|
| 1 | PR이 C11.4 상태 동기화 밖의 소스 변경을 섞는가 | PASS | — | `git diff origin/main...065ddf9`는 계획 두 개, `spec/STATUS.md`, cyclic runtime README, 공개 roadmap HTML의 5개 문서 파일만 바꾼다. 제품 코드나 simulator 결과 파일은 수정하지 않았다. |
| 2 | 검토 기준이 PR의 실제 head와 다른가 | PASS | — | GitHub PR #624 `headRefOid`와 대상 commit은 모두 `065ddf9ade798ab7ead66b872ee3e00996b277de`다. |
| 3 | `origin/main...HEAD` 검토 범위가 비어 있거나 로컬 미커밋 diff만 반영하는가 | PASS | — | PR head는 `origin/main`의 `56daefe96e8143959bc99a89c7a8c80e7f1cc4ba` 뒤에 있고, 정확한 PR diff는 5개 파일·21 insertions·12 deletions다. `git diff --check origin/main...065ddf9`도 오류가 없다. |
| 4 | 문서가 구현 PR #623의 병합 여부를 잘못 말하는가 | PASS | — | GitHub metadata의 #623 상태는 `MERGED`; grid plan, cyclic plan, STATUS와 roadmap은 이를 병합 결과로 기록한다. |
| 5 | merge SHA나 병합 시각이 잘못 기록되는가 | PASS | — | GitHub `mergeCommit.oid`는 `56daefe96e8143959bc99a89c7a8c80e7f1cc4ba`, `mergedAt`은 `2026-10-25T04:35:52Z`; 계획·STATUS·roadmap 표기와 같다. |
| 6 | 검증 source SHA가 실제 PR #623 구현 계보 밖에 있는가 | PASS | — | `c5b8ef75804e39d36a37aa61b82b1f028115fefd`는 Git object이고 #623 head `5c6e24fc2ec5d1303418cd15f00fc6e4cb51947c`의 ancestor다. README도 이 값을 구현 revision으로 지정한다. |
| 7 | 구현 source SHA와 evidence/merge commit SHA가 서로 혼동되는가 | PASS | — | 계획 상태는 source SHA와 merge SHA를 별도 이름으로 표시하고, STATUS/roadmap은 merge SHA를 표시한다. |
| 8 | C11.4 공식 상위 체크가 제한 scene 병합으로 완료 처리되는가 | PASS | — | `spec/STATUS.md:273`의 C11.4는 계속 `[ ]`; 새 이력은 제한 runtime slice라고 분리한다. |
| 9 | 공식 완료 수가 잘못 올라가는가 | PASS | — | 변경 문서의 C11.4 공식 수는 `72/213`으로 유지된다. |
| 10 | 현재 C11.4 다음 계획 후보 목록이 cyclic을 미완료로 다시 남기는가 | PASS | — | `spec/STATUS.md:276`은 orthogonal row만 다음 후보로 두고 #623 cyclic 결과는 아래 기록과 연결한다. 공개 roadmap의 최신 항목도 같은 상태다. |
| 11 | target intrinsic pass와 final pass의 결과가 서로 뒤바뀌는가 | PASS | — | cyclic plan `:48,57`과 runtime README의 target row는 intrinsic count 1/basis 없음/24px 뒤 final basis 24/count 2/`[12,12]px`를 같은 순서로 기록한다. |
| 12 | definite 40px control 값이 target 값으로 오염되는가 | PASS | — | 계획과 README의 control은 basis 40px, count 2, tracks `[20,20]px`, root 40×16px로 별도 기록한다. |
| 13 | phase provenance를 sequence 순서나 최종 CSSOM에서 추측했다고 서술하는가 | PASS | — | 계획 `:42–48`은 semantic phase/count/basis를 실제 Taffy callsite trace에서 보존했고 sequence만으로 phase를 추정하지 않았다고 쓴다. Browser CSSOM과 Spinon trace도 구분한다. |
| 14 | 계획 문서가 여전히 첫 feasibility gate에 들어가기 전 상태라고 말하는가 | PASS | — | `plan/c11-4-cyclic-percentage-auto-repeat.md:42–59`는 gate를 통과한 결과와 계측을 과거형으로 바꾸고, 기존 acceptance와 구현 결과를 나눠 기록한다. |
| 15 | two-scene fixture 결과가 일반 scene admission으로 넓혀질 수 있는가 | PASS | — | cyclic plan `:9,52–63,94`는 exact 두 scene, root/child identity, 한 in-flow child, unsupported neighbor/auto-fit 변형 거부를 고정한다. |
| 16 | focused test 개수가 workspace 전체 테스트 수처럼 표현되는가 | PASS | — | runtime README는 runtime grid 25/25, feature-off 1/1, FFI 72/72를 focused 대상으로 명시하고 전체 workspace 통계로 부르지 않는다. STATUS도 같은 focused 명칭을 사용한다. |
| 17 | Clippy 결과 범위가 확대되는가 | PASS | — | evidence는 runtime/FFI focused target의 `-D warnings` Clippy라고 구분하며 workspace Clippy 통과로 확대하지 않는다. |
| 18 | Android 검증이 실기기·hardware GPU로 오해될 수 있는가 | PASS | — | STATUS·roadmap·계획·README는 Android API 37 AVD와 software renderer를 밝힌다. 물리 기기/hardware GPU 결과라고 쓰지 않는다. |
| 19 | iOS 검증이 실기기 또는 compositor 뒤 pixel 관측으로 오해될 수 있는가 | PASS | — | 문서는 iPhone 17/iOS 26.2 Simulator를 명시한다. README의 Metal sample은 drawable에서 compositor 제출 전에 읽었다고 덧붙인다. |
| 20 | PixelCopy와 Metal 표본을 동등한 capture 경로로 혼동하는가 | PASS | — | Android 표면은 PixelCopy, iOS 표면은 제출 전 Metal drawable로 명확히 분리되어 있으며 AVD/software 및 compositor 경계가 공개 기록에 있다. |
| 21 | 전체 Simulator screenshot이 pixel acceptance를 대신했다고 주장하는가 | PASS | — | runtime README는 screenshot을 전체 화면 보조 자료로 설명하고 frame/pixel 판정은 trace와 native readback log를 사용한다고 분리한다. |
| 22 | static archive의 fixture marker 부재를 release APK/IPA 전체 제외로 확대하는가 | PASS | — | README와 모든 상태 요약은 `.a` archive 경계만 검사했으며 release APK/IPA exclusion은 미검증이라고 적는다. |
| 23 | 제한 slice를 public CSS, WPT 또는 전체 C11.4 지원으로 승격하는가 | PASS | — | 각 계획·상태·roadmap은 일반 CSS admission, WPT 및 전체 C11.4를 남은 범위로 둔다. |
| 24 | orthogonal `[50]/[80]px` 기대값이 Spinon runtime 결과로 쓰이는가 | PASS | — | grid plan `:420` 부근, STATUS `:276`, roadmap 최신 항목은 값을 Chrome precomparison으로만 기술한다. Runtime은 선행 작업까지 보류라고 명시한다. |
| 25 | orthogonal blocker가 현재 구현 경계와 맞지 않거나 원인을 과장하는가 | PASS | — | orthogonal plan은 Stylo 문자열 보존 뒤 `LayoutStyle`·`ParagraphStyle` typed writing-mode와 Taffy 수평 전제의 경계를 구분한다. 새로운 문서는 이를 지원 완료로 말하지 않는다. |
| 26 | PR #623 runtime evidence나 orthogonal precomparison 링크가 존재하지 않는가 | PASS | — | 링크 대상 runtime README, implementation/runtime review, PR diff review, cyclic precomparison, orthogonal plan 및 precomparison README가 저장소에 있다. |
| 27 | 공식 상태와 공개 roadmap이 서로 다른 결과를 표시하는가 | PASS | — | 둘 다 #623 merged, `[12,12]` target, AVD/Metal sample 한계, orthogonal blocker, C11.4 미완료·72/213을 적는다. |
| 28 | 전체 docs build 실패가 이 PR의 변경 성공으로 숨겨지거나 baseline으로 단정되는가 | PARTIAL | P2 | PR 본문은 전체 `docs:build`가 dead-link 검사로 실패했다고 밝히며 원인을 PR 변경만으로 돌리지 않는다. 다만 이번 diff에서 수정된 roadmap/evidence 페이지의 선택 빌드나 실제 route 렌더링은 아직 확인되지 않았다. 병합 전 선택 페이지 빌드 또는 Tailnet/Agent Browser 화면 증거를 추가할 수 있다. |
| 29 | GitHub CI/필수 검사 상태가 통과라고 과장되거나 숨겨지는가 | PARTIAL | P2 | `gh pr checks 624`는 “no checks reported”, main required checks 조회는 404 “Required status checks not enabled”다. PR 본문은 CI 통과라고 주장하지 않지만 이 상태도 따로 적지 않았다. merge 근거에 이 경계를 덧붙이면 확인 상태가 완전해진다. |
| 30 | 변경된 공개 roadmap 화면이 가능한데도 PR 본문에 화면 증거가 빠지는가 | PARTIAL | P2 | PR #624는 `spec/public/roadmap.html` 내용을 바꾸지만 PR 본문에는 screenshot이 없다. roadmap 페이지는 Agent Browser로 화면 캡처할 수 있으므로 갱신 페이지 캡처를 PR에 연결하는 후속이 남는다. |

## 조치 전 판정

제품 문서 범위, #623 결과, C11.4 미완료 상태 및 근거 한계는 일치한다. PR #624의 문서 diff 자체에서 FAIL은 찾지 못했다. 세 PARTIAL은 선택 페이지 렌더 확인, GitHub check 상태 기재, 공개 roadmap screenshot 첨부와 관련된다.
