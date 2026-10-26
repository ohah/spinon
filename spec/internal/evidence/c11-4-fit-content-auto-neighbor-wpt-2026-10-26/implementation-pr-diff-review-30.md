# C11.4 fit-content 옆 auto row 구현 PR diff 실패 관점 검토 · 30개

- 검토 PR: [#662](https://github.com/ohah/spinon/pull/662)
- 검토 base: `4f6b38a4e6528667a0079df88a69bbfc909b86e8`
- 검토 head: `f0dd896ab7c670f8caf7e837b447c16742fd03c0`
- 최종 판정: **30 PASS / 0 PARTIAL / 0 FAIL**
- 범위: 당시 전체 Git diff, PR 본문·링크·라벨 및 검증 주장. 코드 변경·PR 댓글·테스트 재실행은 하지 않았다.

| # | 실패 관점 | 판정 및 근거 |
|---:|---|---|
| 1 | 변경 범위가 계획한 단일 WPT slice를 넘는가 | **PASS** — `fit-content(40px) auto` implicit row assertion 하나에 한정된다. |
| 2 | 계획 PR과 구현 PR의 기준 커밋이 잘못 연결됐는가 | **PASS** — #661은 `4f6b38a4`로 병합됐고 #662 base도 같은 커밋이다. |
| 3 | 작은 테스트를 C11.4 완료로 확대 표시하는가 | **PASS** — PR 본문과 계획은 C11.4 미완료 및 공식 `72/213` 유지로 명시하고 `spec/STATUS.md` diff가 없다. |
| 4 | 제품 알고리즘·런타임 동작을 바꾸는가 | **PASS** — Rust 변경은 신규 테스트 파일과 모듈 등록뿐이다. |
| 5 | 테스트 코드가 기본 제품 빌드에 들어가는가 | **PASS** — `lib.rs` 등록은 `#[cfg(test)]`에 한정된다. |
| 6 | 테스트 파일이 단일 책임 또는 프로젝트 크기 경계를 벗어나는가 | **PASS** — 하나의 oracle 책임이며 305줄이다. |
| 7 | 검증 입력 의존성이 이동했는가 | **PASS** — `--locked` 실행 기록과 base Cargo.lock blob을 고정했고 lockfile 변경이 없다. |
| 8 | WPT 원본이 다른 revision일 수 있는가 | **PASS** — revision과 파일 SHA-256이 `source-pins.json`에 있다. |
| 9 | 고정된 assertion이 다른 조건을 가리키는가 | **PASS** — pinned source 130–132행은 `autoRowAndFitContentRow`와 `10px 290px`을 확인한다. |
| 10 | 규범 산술의 기준 명세가 불명확한가 | **PASS** — CSS Grid Level 2 2025 CRD §6.6, §12.2, §12.4–§12.8을 참조한다. |
| 11 | 기대값이 Spinon/Taffy 결과에서 유도됐는가 | **PASS** — `expected.json`은 독립 유도값을 분리해 기록한다. |
| 12 | WPT와 Rust 단위 테스트 root 폭을 혼동하는가 | **PASS** — `spinon_unit_test_root_width_px`를 쓰고 계획에서 WPT의 containing-block 폭과 구분한다. |
| 13 | 10px minimum contribution의 근거가 누락됐는가 | **PASS** — visible overflow, auto minimum track, nested 10px content를 명세와 연결한다. |
| 14 | fit-content row가 auto row처럼 stretch되는가 | **PASS** — fit-content max function과 auto stretch 자격을 구분하고 두 번째 row만 적격으로 둔다. |
| 15 | 남은 공간 산술이 잘못됐는가 | **PASS** — `300 - 10 - 10 = 280px`, 최종 `[10,290]px`가 계획·JSON·테스트에서 일치한다. |
| 16 | explicit row를 implicit row로 오인하는가 | **PASS** — 입력 `rows`는 비었고 `auto_rows`는 `[FitContent(40px), Auto]`다. |
| 17 | pattern 순서나 item placement가 바뀌어도 테스트가 통과하는가 | **PASS** — authored order, 두 item, 결과 frame을 함께 검사한다. |
| 18 | 다른 grid의 trace를 검사하는가 | **PASS** — 유일한 trace가 root `id(1)`에 속한다. |
| 19 | 다른 축의 pass를 row 결과로 오해하는가 | **PASS** — Rows pass가 정확히 하나인지 확인한다. |
| 20 | sizing stage가 빠지거나 순서가 바뀌는가 | **PASS** — 네 stage의 존재·유일성·순서를 검사한다. |
| 21 | intrinsic 단계에서 track base가 달라지는가 | **PASS** — `IntrinsicSizingComplete`에서 `[10,10]`을 확인한다. |
| 22 | maximize/flexible 단계가 예상 밖으로 track을 바꾸는가 | **PASS** — 각 단계 `[10,10]`을 확인하고 stretch 이전임을 검사한다. |
| 23 | 남는 공간이 잘못된 track에 배정되는가 | **PASS** — 단 한 번의 `StretchAutoTracks { applied: true }` 뒤 `[10,290]`을 요구한다. |
| 24 | gutter/collapsed sentinel을 CSS track으로 세는가 | **PASS** — 두 CSS row와 세 gutter를 분리하고 양 끝만 collapsed인지, 크기가 0인지 검사한다. |
| 25 | trace는 맞지만 실제 frame은 틀리는가 | **PASS** — root·wrapper 2개·nested child 2개의 다섯 frame을 `1e-4` 허용치로 비교한다. |
| 26 | 테스트 로그가 다른 source revision에서 나왔는가 | **PASS** — 테스트와 `lib.rs` source hashes가 구현 커밋과 맞고 review 기록은 소스 commit `9c3ee690`을 식별한다. |
| 27 | formatter 성공 주장에 결과물이 없는가 | **PASS** — fmt exit 0과 무출력 파일이 기록되고 manifest에 포함된다. |
| 28 | implementation 20-view가 plan review와 혼합됐는가 | **PASS** — 별도 `implementation-review-20.md`가 구현 commit과 20 관점을 기록한다. |
| 29 | evidence 수정으로 manifest가 낡거나 빠졌는가 | **PASS** — 검토 head의 manifest 19개 항목이 모두 통과했고 count 문서도 19로 맞춰졌다. |
| 30 | PR 본문 링크·라벨·검증 범위가 근거와 다른가 | **PASS** — 현재 head의 plan/evidence 링크, `chore` 및 area labels, 제한 범위 설명이 일치한다. #662 본문 이미지는 Agent Browser에서 1280×1000으로 로드됐고 raw URL은 HTTP 200이다. 이전 `pr-body-image-url-check.json`은 #661 기록이라 #662 근거로 사용하지 않았다. GitHub checks는 보고되지 않았으며 본문은 CI 통과를 주장하지 않는다. |

`git diff --check`는 `implementation-test.raw.txt`에 보존한 Cargo stdout의 끝 개행만 표시한다. 이는 실제 raw output이며 source/doc whitespace 오류는 없었다. 본문 이미지·hash 검증 세부 사항은 [implementation-pr-body-image-check.json](./implementation-pr-body-image-check.json)에 있다.
