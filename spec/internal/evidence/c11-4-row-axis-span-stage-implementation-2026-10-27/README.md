# C11.4 row-axis overlapping-span stage trace implementation

**상태 · 2026-10-27:** 이 변경은 기존 track-sizing trace를 직접 검사하는 test-only Rust slice다. 제품 알고리즘, CSS admission, 앱 runtime, FFI는 변경하지 않았다. 네 개 고정 row scene과 Columns 대조군을 확인했으며 C11.4 Stage B 전체, 공식 체크박스, `72/213`은 미완료다.

## 구현 범위

- [row_axis.rs](../../../../crates/spinon-layout/src/grid_multispan_chrome_oracle_tests/row_axis.rs)는 `Rows` pass의 `Initialize`, span-2/span-3 `IntrinsicItemBatch`, `IntrinsicSizingComplete`, `MaximizeTracks` snapshot을 검사한다.
- [기존 trace 테스트](../../../../crates/spinon-layout/src/grid_multispan_chrome_oracle_tests.rs)는 column helper가 case ID와 기대 span batch를 받도록 일반화했다. 기존 column 기대 배열과 negative control은 유지한다.
- 전체 Rows stage sequence는 `[Initialize, span batches, IntrinsicSizingComplete, MaximizeTracks, ExpandFlexibleTracks, StretchAutoTracks { applied: false }]`로 고정한다. Fixed Columns early-return snapshot의 CSS track base size도 220px임을 검사한다.
- 네 scene은 span-2 only와 span-2+3 overlapping 입력 각각의 forward/reversed source order다. 기대 row sizes는 각각 `[40,60,60,0]`과 `[160/3,220/3,220/3,0]`이며 남은 block free space는 60px/20px이다.
- fixed 220px Columns pass는 `FixedTrackEarlyReturn` 한 개와 intrinsic batch 부재를 대조군으로 확인한다. Rows contribution 종류·item·span·높이, root/item frame도 확인한다.
- 단계 기대값은 CSS Grid 2025 CRD §11.5.1/§11.6의 독립 산술에서 왔다. Chrome 156은 최종 frame 비교에만 쓰고, 내부 trace oracle로 쓰지 않는다. 사전 비교 artifact와 고정 fixture는 [precomparison README](../c11-4-row-axis-span-stage-precomparison-2026-10-27/README.md)에 있다.

## 검증

| 확인 | 결과 |
| --- | --- |
| `cargo test --locked -p spinon-layout grid_multispan_chrome_oracle_tests` | 통과, 3 tests (기존 column test·negative control·새 row-axis test) |
| `cargo fmt --all -- --check` | 통과 |
| `cargo clippy --locked -p spinon-layout --all-targets -- -D warnings` | 통과 |
| `git diff --cached --check` | 의도한 변경 9개 파일 전체 통과 |
| 구현 실패 관점 검토 | 19 PASS / 1 PARTIAL; [review](./implementation-review-20.md) |

검증 source revision은 `validation.json`에 기준 commit과 두 Rust source 파일의 SHA-256으로 고정했다. 첫 exact-sequence assertion은 기존 trace의 trailing `ExpandFlexibleTracks`와 `StretchAutoTracks { applied: false }`를 놓쳐 실패했다. 두 stage를 기대 목록에 포함한 뒤 위 focused test가 통과했다. 이는 제품 코드 변경이 아닌 assertion 보완이었다.

Chrome 156 screenshot은 [사전 비교 image](../c11-4-row-axis-span-stage-precomparison-2026-10-27/chrome156-row-span-precomparison.png)다. Chrome reference 화면이며 Spinon app/runtime screenshot이 아니다. 이 test-only 변경에서는 Android/iOS 앱이나 simulator를 실행하지 않았다. GitHub CI 상태는 PR에 올라간 뒤 별도로 기록한다.

## 범위 한계

테스트는 네 개의 고정 입력만 다룬다. 단계 snapshot은 각 span batch 뒤의 trace 상태이며 batch 내부 transient subpass를 뜻하지 않는다. `None`은 trace에서 비유한 값을 나타내며 내부 `+∞`, NaN, `-∞`를 구별하지 못한다. Chrome의 `gridTemplateRows` 문자열은 반올림되므로 규범적 유리수 기대값과 구분한다. 고정 reference JSON과 일부 테스트 상수는 서로 복제되어 있으며 기계적으로 같은 artifact에서 생성되지 않는다(구현 검토 20번 PARTIAL).

이 slice는 일반 CSS admission, Spinon WPT 적합성, Android/iOS 앱 경로, 실기기, GPU/compositor pixels 또는 C11.4 전체 완료를 주장하지 않는다.
