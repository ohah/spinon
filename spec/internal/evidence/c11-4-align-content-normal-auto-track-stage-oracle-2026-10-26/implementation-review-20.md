# C11.4 `align-content: normal` §11.8 구현 검토 20개

## 범위와 판정 기준

대상은 구현 commit `31c7874a4675bd9182c83aab2bc711a93ced9c34`, 실행 근거 README commit `27bc643e`, 그리고 상태 정합성 갱신 commit `accf37ff18e2740c2b980ac84de1f3f1a611ce71`이다. Rust 테스트 코드와 기존 helper, 독립 precomparison의 입력·기대값, 커밋된 검증 요약, 공식 상태 범위를 대조했다. 이번 검토에서는 테스트, simulator 또는 browser를 실행하지 않았다. 따라서 실행 결과는 README에 기록된 보고이며 이번 검토에서 재실행해 확인한 결과가 아니다.

## 20개 실패 관점

1. **변경 범위 누수 — PASS.** 구현 commit은 `crates/spinon-layout/src/grid_stretch_auto_tracks_oracle_tests.rs`에 테스트 하나만 추가한다. production algorithm, style projection, FFI, app 및 API 변경은 없다. 계획과 README의 범위 설명과 실제 diff가 일치한다.

2. **제품 빌드에 test 코드 포함 — PASS.** 새 테스트 모듈은 `lib.rs`에서 `#[cfg(test)]`로만 선언되어 있고, fixture feature나 제품 모듈 경로로 노출되지 않는다. 순수 Rust 단위 테스트라는 분류와 일치한다.

3. **입력 크기·기여값 불일치 — PASS.** 기존 helper의 root 크기 `20×100px`와 child 높이 `[10,20,5]px`가 precomparison 및 expected JSON의 값과 같다. child width도 각 `20px`로 고정되어 frame 기대값을 독립적으로 산출할 수 있다.

4. **definite sizing 조건 상실 — PASS.** root width/height를 fixed로 두고 viewport도 `20×100px`, `RootSizingPolicy::Match`를 사용한다. 따라서 row axis의 definite `100px` 조건이 구현 입력에 실제로 반영된다.

5. **track sizing 함수·축 오선택 — PASS.** 입력은 한 개의 `20px` column, rows `[auto, auto, 5px]`, `GridPlacementMode::TrackSizing`이다. 늘릴 대상으로 설정한 두 auto row와 고정 세 번째 row가 expected JSON의 대상 인덱스와 대응한다.

6. **item placement와 row 순서 오류 — PASS.** root의 direct child ID 순서와 `[10,20,5]px` 높이 순서가 같고, row auto-flow 및 세 개의 명시 row로 각 child가 해당 행에 놓인다. frame 검사는 이 배치 결과도 확인한다.

7. **gutter 수 또는 축 단위 오류 — PASS.** helper는 row gap `10px`, column gap `0px`를 설정한다. trace assertion은 두 내부 row gutter를 각각 `10px`, 양 끝 sentinel을 `0px`로 검사해 gap을 track base로 잘못 취급하는 오류를 잡는다.

8. **free-space 산술 오류 — PASS.** 독립 기대값은 `100 − (10+20+5) − (10+10) = 45px`로 계산한다. 두 auto max track에 각각 `22.5px`을 배분해 `[32.5,42.5,5]px`를 기대한다. 테스트의 상수와 expected JSON이 일치한다.

9. **`normal` 매핑 누락 — PASS.** 새 테스트는 `input(LayoutAlignContent::Normal)`을 직접 전달하고, 기존 `to_taffy_align_content`는 `Normal`을 `AlignContent::STRETCH`로 매핑한다. Grid §11.8의 `normal`/`stretch` 분기와 이 내부 입력 경로가 맞는다.

10. **다른 grid trace를 우연히 검사 — PASS.** `rows_pass`는 trace가 정확히 하나이며 그 소유 노드가 root ID 1인지 확인한다. child 또는 별도 grid의 trace가 기대 stage를 만족하는 경우를 배제한다.

11. **잘못된 sizing pass 선택 — PASS.** trace에서 `Rows` pass가 정확히 하나인지 확인한다. 목표 행 축의 trace가 없거나 중복된 경우 테스트가 실패한다.

12. **stage 중복·순서 변경 — PASS.** `unique_stage`로 intrinsic, maximize, flex expansion, stretch stage를 각각 하나로 제한하고 인덱스 순서도 검사한다. 필수 단계가 빠지거나 중복되거나 순서가 바뀌는 실패 경로를 다룬다.

13. **stretch 전 기초값 변조 — PASS.** `IntrinsicSizingComplete`, `MaximizeTracks`, `ExpandFlexibleTracks`에서 각각 CSS row bases `[10,20,5]px`를 검사한다. 최종값만 우연히 맞고 중간 sizing 단계가 달라지는 회귀를 탐지한다.

14. **stretch 단계의 적용 여부 오류 — PASS.** 새 `normal` 입력에서 선택한 stage가 정확히 `StretchAutoTracks { applied: true }`인지 확인한다. 단순히 최종 frame만 비교하는 검사가 아니다.

15. **고정 track까지 늘어남 — PASS.** 최종 CSS row bases를 `[32.5,42.5,5]px`와 비교해 첫 두 auto row의 균등 증가와 세 번째 fixed `5px` 유지 여부를 확인한다.

16. **trace slot 집계·경계 gutter 오류 — PASS.** helper가 CSS track과 gutter의 alternating slot 수, gutter/collapsed flags, 네 gutter의 값과 순서를 검사한다. boundary sentinel을 CSS track 개수나 내부 gap에 섞는 오류를 배제한다.

17. **남은 공간 부호·합계 오류 — PASS.** stretch stage의 모든 slot base 합을 root block size에서 빼고 유한값·`0px` 허용 오차를 검사한다. gutter를 포함한 전체 row axis가 root를 채우는지 확인한다.

18. **frame 배치·크기·수치 비교 — PASS.** root와 세 child 각각의 x/y/width/height를 검사하며 독립 기대 y 좌표 `[0,42.5,95]px`와 고정 child 크기를 반영한다. 공통 comparator는 actual의 finite 여부를 요구하고 절대 오차를 `1e-4 CSS px`로 제한하므로 NaN/무한값이나 의미 있는 좌표 오차가 통과하지 않는다.

19. **대조군 포함 검증의 원본 출처 — PASS.** 추가 실행 근거의 `cargo test --locked -p spinon-layout grid_stretch_auto_tracks_oracle_tests` raw stdout을 읽어 `normal` 테스트와 기존 `stretch`/`start` 대조군 테스트가 모두 실행되어 `2 passed, 0 failed, 275 filtered out`인 것을 확인했다. 기존 stretch 테스트 안에서 `start` 입력의 `StretchAutoTracks { applied:false }`, row bases, child frames를 검사하므로 negative control도 해당 실행에 포함된다. README가 raw gzip 로그와 연결되고 `toolchain.txt`에 Rust/Cargo 1.96.1 및 host가 기록되어 이전 PARTIAL은 해소됐다. 이번 재확인 과정에서는 테스트를 실행하지 않았다.

20. **공식 상태와 플랫폼 검증 경계 — PASS.** `spec/STATUS.md`의 C11 상위 체크는 미완료이며 구현 branch에서 해당 상태 파일을 수정하지 않았다. 갱신된 계획·근거는 `72/213`을 유지한다. 변경이 `#[cfg(test)]` Rust 단위 테스트뿐이고 app/FFI를 건드리지 않아 Android/iOS simulator 실행은 이 slice의 검증 대상이 아니다. 문서는 일반 CSS cascade, app rendering, Chrome/WPT 또는 전체 C11.4 완료를 주장하지 않는다.

## 집계

- **PASS:** 20
- **PARTIAL:** 0
- **FAIL:** 0
- **Blocker:** 없음

검증 근거 경계 PARTIAL은 대조군 포함 실행의 raw stdout 및 toolchain 기록을 확인해 해소했다. 코드 입력, 독립 산술, trace assertion 또는 공식 상태에서 구현 결함은 발견하지 않았다. C11.4 완료나 일반 CSS/runtime 지원을 이 검토로 주장하지 않는다.
