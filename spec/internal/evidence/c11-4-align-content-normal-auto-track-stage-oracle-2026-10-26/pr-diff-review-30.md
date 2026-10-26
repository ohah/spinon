# PR #656 구현 diff 적대 검토 30개

**대상:** [PR #656](https://github.com/ohah/spinon/pull/656)
**Base:** `main` / `fb1704f950ce29097c482f16d7ec4401b0009493`
**검토 head:** `dcd8e83024c0380bd4e63046d2d4b183070641cb`
**기준:** `docs/project-rules.md`의 PR diff 적대 검증 규칙, CSS Grid Layout Module Level 1 2025 CRD §11.8, 정확한 head의 전체 diff·본문·라벨·상태 및 커밋 evidence

검토 결과는 **PASS 30 / PARTIAL 0 / FAIL 0, 차단 0개**다. 초기 검토에서 raw 실행 출력이 없던 증거 경계는 마지막 `dcd8e830`에서 원본 stdout과 exit-code/hash manifest가 추가되어 보완됐다. 코드 검토는 앞선 head에서 수행한 결과를 유지하고, 추가된 증거 관점은 이 head에서 재검토했다.

## 30개 관점

1. **검토 head 고정 — PASS.** 체크아웃 `HEAD`와 GitHub PR head가 `dcd8e83024c0380bd4e63046d2d4b183070641cb`로 일치한다. 이 보고서는 해당 head에 포함되지 않은 새 산출물이다.
2. **기준 브랜치 — PASS.** PR base `main`, `origin/main`, GitHub `main` ref가 모두 `fb1704f950ce29097c482f16d7ec4401b0009493`이다.
3. **변경 파일 경계 — PASS.** 12개 파일 변경은 기존 단위 테스트 하나, 계획 상태 갱신, 실행 evidence와 사전 비교 상태 갱신으로 한정된다. 제품 알고리즘·스타일 projection·FFI·앱 소스는 변경되지 않는다.
4. **테스트 모듈 격리 — PASS.** 기존 `grid_stretch_auto_tracks_oracle_tests` 선언은 `#[cfg(test)]` 안에 있어 새 테스트가 일반 제품 artifact 경로에 들어가지 않는다.
5. **PR 본문과 실제 범위 — PASS.** 본문은 test-only Rust 단위 테스트 하나와 문서·evidence 갱신을 명시하며 public API나 런타임 변경을 주장하지 않는다.
6. **규범 출처 — PASS.** 계획·evidence·PR 본문은 CSS Grid 2025 CRD §11.8 공식 anchor를 일관되게 가리킨다: [§11.8 Stretch auto Tracks](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-stretch).
7. **`normal`의 규범 조건 — PASS.** 계획은 content-distribution이 `normal` 또는 `stretch`이고 남은 공간이 양수·definite일 때만 §11.8 늘림을 기대한다.
8. **대상 트랙의 조건 — PASS.** 균등 분배 대상으로 `auto` max track sizing function을 가진 트랙만 지정한다.
9. **행 축 속성 — PASS.** `align-content: normal`을 row-axis content distribution 입력으로 사용하며 inline-axis `justify-content`를 §11.8 조건으로 혼동하지 않는다.
10. **내부 enum projection — PASS.** 실제 코드의 `LayoutAlignContent::Normal → AlignContent::STRETCH` 경로와 기대 `StretchAutoTracks { applied: true }`가 맞는다.
11. **trace flag의 의미 — PASS.** 테스트는 `applied:true`뿐 아니라 최종 row bases와 남은 공간도 검사하므로 단계 gate만 열리고 실제 track 확장은 안 된 회귀도 잡는다.
12. **definite block-size — PASS.** root 높이는 고정 `100px`이며 viewport도 `100px`로 고정돼 indefinite sizing 경로와 구별된다.
13. **root sizing policy — PASS.** 기존 입력 helper의 `RootSizingPolicy::Match` 및 동일한 root/viewport 크기가 계획·precomparison 기록과 일치한다.
14. **content-box basis — PASS.** root border와 padding이 0이어서 사용 가능한 row 크기 `100px`을 content-box로 그대로 계산할 수 있다.
15. **grid topology — PASS.** 명시 row는 `[auto, auto, 5px]`, column은 `20px` 하나라 세 자식의 예상 행과 fixed 세 번째 트랙이 분명하다.
16. **자식 순서·위치 — PASS.** direct child 순서와 row indices가 일대일이고 각 자식이 한 행에 놓인다.
17. **아이템 크기 입력 — PASS.** 고정 높이 `[10,20,5]px`와 너비 `20px`가 기존 helper와 독립 입력 JSON에 일치하게 기록됐다.
18. **기초 트랙값 — PASS.** `IntrinsicSizingComplete`, `MaximizeTracks`, `ExpandFlexibleTracks`에서 CSS row bases `[10,20,5]px`를 각각 검사해 중간 단계 회귀를 탐지한다.
19. **gutter 수와 축 — PASS.** row-gap `10px` 두 개, column-gap `0px` 입력이어서 row axis의 내부 gutter 수와 값을 고정한다.
20. **free-space 산술 — PASS.** `100 − (10+20+5) − (10+10) = 45px` 계산이 track bases와 두 gutter를 각각 한 번 포함한다.
21. **eligible/fixed 집합 — PASS.** auto 최대 함수 index `[0,1]`만 자격이 있고 index `2`의 fixed `5px` 행은 대상에서 제외된다.
22. **균등 증가량 — PASS.** 남은 `45px`을 두 auto 행에 `22.5px`씩 더해 `[32.5,42.5,5]px`를 기대한다.
23. **최종 행 합계 — PASS.** 최종 트랙과 두 gutter 합이 `100px`이고 fixed 행은 `5px`로 유지되며 남은 공간은 `0px`이다.
24. **trace 소유자 — PASS.** `rows_pass`는 trace 하나와 root node ID를 요구해 다른 grid의 stage가 통과하는 경우를 차단한다.
25. **row pass와 stage 순서 — PASS.** 행 pass 하나를 선택하고 네 stage가 각각 한 번이며 intrinsic → maximize → flex expansion → auto stretch 순서인지 검사한다.
26. **raw trace 슬롯 — PASS.** CSS tracks와 alternating gutter entries를 분리하고 양 끝 collapsed sentinel 및 두 내부 gutter 값을 확인한다.
27. **frame geometry — PASS.** root `(0,0,20,100)` 및 자식 `(0,0,20,10)`, `(0,42.5,20,20)`, `(0,95,20,5)`를 독립 row-start 계산과 대조한다.
28. **대조군·오라클 독립성 — PASS.** 새 테스트는 `normal` 결과를 상수 기대값에 직접 대조하며 기존 stretch 출력에서 oracle을 역산하지 않는다. 기존 stretch suite에는 `start`의 `applied:false`, 고정 tracks, child frames도 포함된다.
29. **수치 허용치 — PASS.** 비교 helper는 실제값의 finite 여부와 `1e-4 CSS px` 절대 오차를 요구한다. 기대 반쪽 픽셀 값은 이 테스트에서 정확히 표현 가능하다.
30. **검증 artifact와 상태 경계 — PASS.** README의 `validation-results.json`은 focused test 1/1, combined test 2/2, fmt, workspace Clippy, diff-check exit code를 기록하고 압축 stdout SHA-256을 연결한다. 검토자가 네 압축 로그의 SHA를 manifest와 대조했고 stdout 요약도 맞았다. toolchain은 Rust/Cargo 1.96.1과 `aarch64-apple-darwin`으로 기록됐다. `statusCheckRollup`은 비어 있으므로 CI pass로 보지 않으며 C11.4와 `72/213`은 미완료로 유지한다.

## 최종 판정

**PASS 30 / PARTIAL 0 / FAIL 0. 차단 항목은 없다.** 앞선 부분 판정은 focused test·fmt·Clippy stdout 및 종료 상태·hash manifest가 추가된 뒤 해소됐다. 이 리뷰에서는 테스트, formatter, Clippy, 시뮬레이터, UI 브라우저를 실행하지 않았다. PR의 기록된 로컬 검증 결과와 GitHub 상태 rollup은 별개이며, rollup은 비어 있다.

## 상태 전용 delta 재확인

2026-10-26 현재 PR head `a183e2a77b0f10f2b509cd46f6721bc5bedccb3a`의 상태 갱신을 확인했다. 이 commit의 delta는 계획 문서, C11.4 계획 색인, 구현 evidence README 세 곳의 상태 문구만 바꾸며 테스트 코드나 독립 기대값은 수정하지 않는다. 원래 30개 diff review의 대상은 계속 `dcd8e83024c0380bd4e63046d2d4b183070641cb`이고 집계도 **PASS 30 / PARTIAL 0 / FAIL 0**으로 유지한다.

세 문서는 동일하게 그 원래 head의 PASS 30/30과 merge 대기를 기록한다. C11.4 공식 완료 상태는 미완료, 집계는 `72/213`이며 `spec/STATUS.md`는 이 delta에서 변경되지 않았다. 계획·evidence의 PR diff report 링크는 저장소 안의 [동일 보고서](./pr-diff-review-30.md)를 가리키고, PR 본문도 원래 검토 head와 `a183e2a7` 상태 delta를 명시한다.

이 재확인 시점의 GitHub `statusCheckRollup`은 비어 있고 merge state는 `CLEAN`이다. 문서와 PR 본문은 GitHub checks 미보고 및 merge 대기와 일치한다. 테스트·브라우저·시뮬레이터는 실행하지 않았다.
