# C15.3 Flex intrinsic test-only 후보 · PR 준비 적대 검토 30관점 · pass01

## 검토 경계

세 명의 검토자가 C15.3 Flex intrinsic text 회귀 후보를 서로 다른 test/oracle, 범위, PR packaging 관점으로 읽었다. 파일 수정이나 테스트·빌드는 하지 않았다. 현재 pass07 Cargo run의 고정 manifest에 C15 Rust test path도 포함돼 있으므로 pass07 종료 및 사후 hash closure 전에는 이 후보를 staging하거나 commit하지 않는다. 이 기록은 실제 GitHub PR diff 검토를 대신하지 않는다.

## Test·oracle 검토 · 10관점

| # | 실패 경로·불변 조건 | 결과와 근거 |
|---:|---|---|
| 1 | 테스트가 release/product layout 동작을 바꿈 | **PASS.** 부모 module이 `#[cfg(test)]` 아래라 제품 경로에 들어가지 않는다. `crates/spinon-layout/src/lib.rs:28-30`, `c15_text_layout_tests.rs:525` |
| 2 | 시스템 폰트 fallback이 관찰값을 바꿈 | **PASS.** `new_without_system_fonts()` 뒤 저장소 Noto Sans KR만 등록한다. `c15_text_layout_tests/flex_intrinsic.rs:7-16` |
| 3 | Noto font bytes가 evidence와 다른데 이름만 같음 | **PARTIAL.** 현재 font SHA는 fixture README 값과 일치하지만 테스트가 SHA를 직접 assertion하지 않는다. C11 pass07의 external pre/post manifest가 current input을 묶어야 한다. `flex_intrinsic.rs:7`, `tests/fixtures/resources/c15/c15-3/README.md:7` |
| 4 | Chrome expected 값이 다른 관찰 파일에서 복사됨 | **PASS.** Chrome observation JSON hash가 precomparison README 기록과 일치한다. `c15-3-unicode-shaping-precomparison-2026-10-17/chrome155-observation.json` |
| 5 | Chrome 버전·DPR·viewport·font readiness가 불명확함 | **PASS.** observation은 HeadlessChrome 155, DPR 1, 고정 viewport와 loaded font 상태를 기록한다. `c15-3-unicode-shaping-precomparison-2026-10-17/README.md:9` |
| 6 | remote font가 load되지 않은 frame을 oracle로 사용 | **PASS.** observation은 fonts loaded를 기록하고 fixture는 고정 Noto font를 지정한다. `tests/fixtures/css/c15/c15-3-unicode-shaping-oracle.html:6` |
| 7 | text 또는 기본 타이포그래피가 Chrome과 달라짐 | **PASS with default-value caveat.** `한글字`, 20px, line-height 24px, LTR, nowrap이 맞는다. weight 400/normal은 default에 의존한다. `flex_intrinsic.rs:21-44`, fixture `:61-64` |
| 8 | 다른 geometry를 캔버스 advance와 혼동 | **PASS.** oracle은 `#flex-sizing` DOM rect `56.8125×24px`이다. `chrome155-observation.json:458-461` |
| 9 | Taffy frame 결과에 intrinsic measurement를 미리 주입 | **PASS.** auto width/height/basis text item을 실제 Flex max-content sizing에 전달한다. `flex_intrinsic.rs:24-48` |
| 10 | parent/item 중 하나만 비교하거나 frame 차이를 놓침 | **PASS.** 부모·item frame을 각각 Chrome 기준에서 0.05 CSS px 이내로 비교한다. `flex_intrinsic.rs:49-55` |

## Sizing·snapshot·scope 검토 · 10관점

| # | 실패 경로·불변 조건 | 결과와 근거 |
|---:|---|---|
| 11 | Chrome의 auto-height와 Rust의 max-content height를 동등 입력으로 오인 | **PARTIAL.** Chrome fixture는 flex width만 max-content, height는 auto이며 Rust parent는 height도 max-content다. 현재 한 줄 표본에서 결과가 같아도 auto-height 동작 자체는 비교하지 않는다. CSS `:62`, Rust `flex_intrinsic.rs:24` |
| 12 | HTML span item과 Paragraph layout node가 동일하다고 주장 | **PARTIAL, scope에 부합.** test는 Taffy의 text paragraph Flex consumer를 검증한다. Chrome `span` 구조 및 HTML/CSS 투영은 동일하지 않으며 README가 이를 명시한다. fixture `:109`, `c15_text_layout_tests.rs:37` |
| 13 | `inline-flex` 외부 inline formatting도 검증했다고 주장 | **PARTIAL, scope에 부합.** Rust는 Taffy Flex만 다루며 outer inline formatting은 빠진다. evidence README `:17-21` |
| 14 | CSS max-content를 주지 않은 단순 고정폭 test가 됨 | **PASS.** Taffy max-content parent와 auto item sizing을 호출한다. `flex_intrinsic.rs:24-48` |
| 15 | 넓은 tolerance가 작은 layout regression을 숨김 | **PARTIAL.** 0.05px은 1/64px 단위 약 세 개까지 허용할 수 있다. 이 값은 exact parity가 아니라 bounded proximity를 주장한다. `flex_intrinsic.rs:49-55` |
| 16 | text owner나 source/style 변경을 무시 | **PASS.** final snapshot에서 node owner, source, style와 revision을 확인한다. `flex_intrinsic.rs:57-69` |
| 17 | line count/height/available width가 달라도 frame만 통과 | **PASS.** snapshot의 한 줄, line-height와 available width를 검사한다. `flex_intrinsic.rs:57-69` |
| 18 | shaping width 또는 glyph advance가 다른데 snapshot이 통과 | **PARTIAL.** `text.layout.width`와 glyph advance를 expected oracle에 직접 대조하지 않는다. 이 test는 frame consumer에 한정된다. `flex_intrinsic.rs:57-69` |
| 19 | 어떤 intrinsic callback 경로가 frame을 냈는지 증명 | **PARTIAL.** 정확한 callback/measurement attribution을 관찰하지 않는다. 기존 implementation review의 limitation과 일치한다. `implementation-review-20.md:26` |
| 20 | C15.3 전체 Block/Flex/Grid 계열이 완료된 것으로 읽힘 | **PASS.** 현재 근거는 단일 Flex consumer test이며 HTML/CSS/runtime/paint를 명시적으로 제외한다. evidence README `:17-21`, `plan/c15-inline-text-layout.md:87` |

## Evidence·staging·PR packaging · 10관점

| # | 실패 경로·불변 조건 | 결과와 근거 |
|---:|---|---|
| 21 | focused test를 실행하지 않은 채 통과로 기록 | **PASS, 기록 시점 한정.** 기존 로그는 1/1이며 SHA-256 `960efc0b…b03a`; 이 검토는 재실행을 주장하지 않는다. evidence README `:11-15` |
| 22 | test source 변경이 실행 로그와 연결되지 않음 | **PARTIAL.** README에 test-source SHA가 없다. 현재 SHA `3d050943…43843fa`를 pass07 post-hash closure 뒤 evidence에 기록해야 한다. `flex_intrinsic.rs`, pass07 source manifest |
| 23 | hash closure 전에 Rust source를 바꿔 동시 run을 오염 | **PARTIAL gate.** C15 test Rust paths가 pass07 고정 입력에 있으므로 현재 PR stage/commit은 보류한다. |
| 24 | C11 matrix 전용 `sha2` dependency가 C15 PR에 유입 | **FAIL for current mixed staging.** C15 test가 쓰지 않으므로 `Cargo.lock`의 추가분은 C11.4 PR에만 속한다. `Cargo.lock:1795-1805` |
| 25 | Rust 1.100.0 toolchain bump를 C15 테스트 전용 변경으로 오인 | **PARTIAL.** 사용자 요청 작업이므로 별도 toolchain PR 또는 명시적인 결합 근거가 필요하다. `mise.toml:1-3`, `plan/c15-inline-text-layout.md:26` |
| 26 | C11과 C15의 STATUS 변경이 같은 파일이라 전체 문단을 섞어 넣음 | **PASS with selective staging.** C15.3은 분리된 status entry라 해당 hunk만 취할 수 있다. `spec/STATUS.md:665` |
| 27 | roadmap execution object의 C11.4 진행 상태가 C15 PR에 따라 바뀜 | **PARTIAL.** `detailInProgress`가 두 항목을 한 줄에 둔다. 분리 PR은 C11.4 객체를 보존하고 C15.3 항목만 분리해야 한다. `spec/public/roadmap.html:531` |
| 28 | PR body에 빠진 작업 유형·영역·범위·검증 한계 | **PARTIAL until authored.** 저장소 PR template에 맞춰 labels, test-only 의미, 남은 CSS/runtime 범위를 적어야 한다. `.github/pull_request_template.md` |
| 29 | test-only 후보에 앱 동작 화면을 제품 evidence처럼 첨부 | **PASS if scoped.** app screenshot은 없다. Chrome screenshot을 붙인다면 oracle 이미지라고 표시해야 한다. 현재 evidence와 PR readiness review가 범위를 명시한다. |
| 30 | 열린 PR 또는 검토 완료를 실제보다 과장 | **PASS.** GitHub 검색에서 C15.3 열린 PR은 없었다. 실제 PR diff 적대 검토 30관점은 PR 생성 후 별도 수행해야 한다. |

## 결론

test-only Flex consumer 범위에는 기능 코드 blocker가 없다. PARTIAL들은 auto-height/HTML 구조/inline formatting/measurement attribution 범위를 넘지 않는다는 문서 경계와 외부 source manifest로 한정한다. 후보는 현재 혼합 작업 트리에서 바로 PR로 낼 준비가 되지 않았다. pass07 실행과 1,920-input 사후 hash 폐쇄 후 C15 test source SHA를 evidence에 추가하고, `Cargo.lock`의 `sha2`, `mise.toml` toolchain update, roadmap의 C11.4 내용과 분리해야 한다. 이 문서는 PR-prep review이며 실제 PR diff 30관점 검토를 대체하지 않는다. C15.3/C15는 미완료, 공식 `78/214` 유지다. 현재 C15 README의 fixture link는 `tests/fixtures/resources/c15/c15-3/README.md`로 해석되어 존재하므로, 앞서 제기된 missing-link 판정은 철회한다.
