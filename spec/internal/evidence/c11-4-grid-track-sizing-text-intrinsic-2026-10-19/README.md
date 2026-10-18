# C11.4 text intrinsic measure 구현 slice · 2026-10-19

## 변경 범위

이 구현 slice는 C15 Parley paragraph에서 min-content/max-content inline advance와 각 결과의 block height를 계산해 `TaffyLayoutEngine`의 intrinsic measure callback에 전달한다. 결과는 font revision 숫자와 registry identity를 함께 보존하므로 별도 engine의 같은 숫자 revision을 같은 입력으로 오인하지 않는다. definite-width `TextLayoutSnapshot` cache는 그대로 분리하고, intrinsic 결과는 한 `CalcLayoutTree` 수명 안에서 text node별로만 재사용한다. C11.4 Grid track 함수·단계별 sizing·item automatic minimum·auto-repeat·Android/iOS runtime은 구현하지 않았다. C11.4와 C11 상위는 미완료다.

## 기준 입력과 oracle

- Ahem `10px/10px` fixture는 normal `XX XXXXXXX` min/max `70/100px`, `break-all` 8글자 `10/80px`, nowrap `XX XXXXXXX` `100/100px`를 고정한다.
- Chrome 155 Agent Browser 보조 oracle은 inline span의 좌우 `4px` padding + `1px` border를 포함한다. normal `75×20px`, nowrap `110×10px`, break-all `15×80px`, pre-line `45×20px`다.
- 빈 `pre-line` 줄의 높이는 보존된 줄바꿈과 `10px` line-height로부터 예상한 값 `30px`를 독립 입력 통제로 검사한다. 이 추가 값은 보조 Chrome oracle matrix의 일부가 아니다.
- Oracle 환경·fixture·raw 관찰·세션 종료 증거는 [inline edge oracle](../c11-4-grid-track-sizing-text-inline-edge-2026-10-19/README.md)와 그 JSON에 기록한다. Chrome 관찰은 Spinon 결과가 아니다.

## 실행 결과

| 검사 | 결과 |
| --- | --- |
| `mise exec -- cargo test -p spinon-layout -p spinon-text-layout` | `spinon-layout` 155 passed, `spinon-text-layout` 40 passed, 0 failed |
| `mise exec -- cargo fmt --all -- --check` | 통과 |
| `mise exec -- cargo clippy -p spinon-text-layout -p spinon-layout --all-targets -- -D warnings -A clippy::too_many_arguments` | 통과. 예외는 변경하지 않은 `spinon-layout/src/host_document.rs`의 기존 `too_many_arguments` 진단만 억제한다. |
| `mise exec -- cargo clippy -p spinon-text-layout --all-targets -- -D warnings` | 예외 없이 통과 |
| `git diff --check` | 통과 |

원본 표준 출력은 `cargo-tests.log`, `rustfmt.log`, `clippy.log`, `clippy-text-strict.log`, `diff-check.log`에 둔다. 전체 workspace strict Clippy 성공으로 확대하지 않는다.

## 검토

- [새 구현 코드 적대 검토 20개](./implementation-review-20.md)
- `pr-delta-review-30.md`는 실제 PR diff와 PR 표시 상태를 확인한 뒤 추가한다.

## 미검증 경계

- Taffy track sizing 단계의 CSS Grid 알고리즘 적합성, typed track projection, spanning contribution, cyclic percentage, auto-repeat은 미구현이다.
- Android API 37 AVD/iOS Simulator app route와 surface sample은 이 순수 Rust text measurement 변경으로 달라지지 않아 수행하지 않았다. 실제 UI/Grid 화면을 검증했다고 주장하지 않는다.
- Spinon WPT runner와 실제 브라우저의 full CSS text compatibility는 미검증이다.
- Chrome 155 숫자는 pinned oracle subset이며 W3C 전체 적합성 기준이 아니다.
