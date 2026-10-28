# C11.4 mixed image negative-control follow-up review

## 범위

PR #729 이후 generic mixed intrinsic/flexible/fixed adapter의 이미지 검증 공백을 좁게 보완한다. `prepare()`의 호출 계약과 정상 PNG는 그대로 두고, private helper를 통해 source URL·decode·natural-size 거부 경로를 Columns와 Rows에서 직접 실행한다. 공개 API, FFI route, 제품 CSS admission, 앱 장면과 공식 상태는 변경하지 않는다.

검토 대상은 `generic_intrinsic_flex_fixed.rs`와 전용 `tests.rs`다. 기준 동작과 허용 profile은 [C11.4 혼합 intrinsic/flexible/fixed 계획](../../../../plan/c11-4-mixed-intrinsic-flex-fixed.md)에 고정되어 있다.

## 확인 결과

다음 테스트를 실행했다.

```sh
mise exec -- cargo test --locked -p spinon-runtime --lib \
  --features c11-4-grid-track-sizing-fixture-test \
  generic_intrinsic_flex_fixed::tests
```

결과는 5 passed, 0 failed다. 신규 세 제어는 Columns와 Rows에서 모두 실행된다. 잘못된 크기 제어 PNG는 별도로 디코드하고 크기가 `2×2`임을 확인하므로 decode 실패로 오인하지 않는다. 기존 8-scene positive matrix도 같은 실행에서 통과했다.

`cargo fmt --all -- --check`, `git diff --check`가 통과했다. 엄격한 package Clippy는 수정하지 않은 `c11_4_grid_layout_auto_tracks/validation.rs:329`의 `type_complexity`와 `generic_direct_text_image/fixture.rs:311`의 `map_identity`에서 실패했다. 이 두 lint만 제외한 feature-enabled package Clippy는 통과했다. Android/iOS app matrix와 simulator는 실행하지 않았다. 이 변경은 Rust adapter negative controls이며 app launcher나 scene payload를 바꾸지 않는다.

## 구현/runtime 실패 관점 20개

| # | 독립 실패 가설 | 확인 결과 |
| --- | --- | --- |
| 1 | 정상 fixture가 빈 PNG 입력을 사용하게 된다 | `prepare()`가 계속 `super::PNG`를 helper에 전달한다. 기존 positive matrix 통과. |
| 2 | helper가 외부에서 임의 PNG를 받는 공개 입력 경로가 된다 | helper는 private이고 FFI/export 변경이 없다. |
| 3 | 잘못된 source가 `Ok(None)`으로 빠져 다른 adapter가 처리한다 | 잘못된 source test는 `Err`만 통과시키며 `Ok(None)`도 실패 처리한다. |
| 4 | 올바른 source 조건이 제거되거나 느슨해진다 | 기존 exact URL 비교를 유지한다. 잘못된 URL 제어가 양 축에서 거부됐다. |
| 5 | 다른 source와 정상 PNG가 결합돼 허용된다 | source 오류 장면은 canonical PNG bytes를 주입해도 거부됐다. |
| 6 | 올바른 URL에서 손상된 PNG가 성공 처리된다 | `not a PNG` 제어가 양 축에서 동일 fixture 오류로 거부됐다. |
| 7 | 손상된 PNG가 panic을 일으킨다 | 실제 `ImageRequest::decode` 경로의 결과를 검사하며 panic 없이 `Err`를 반환했다. |
| 8 | decoder가 실패해도 부분 intrinsic input이 반환된다 | helper 결과가 `Err`인지 검사한다. `C11_4MixedInput`은 반환되지 않는다. |
| 9 | natural-size 제어가 사실은 decode 실패만 검사한다 | 대조 PNG를 먼저 디코드하고 2×2 크기를 확인한 뒤 adapter에 전달한다. |
| 10 | width 또는 height 검사가 생략된다 | 기존 adapter는 두 dimension을 기대값과 각각 비교한다. 2×2 대조가 둘 모두 불일치한다. |
| 11 | CSS 지정 크기가 natural size를 대신한다 | fixture CSS는 width/height `auto`; 검사는 decoded resource metadata에 적용된다. |
| 12 | image가 아닌 node가 오류 주체로 기록된다 | 세 제어 모두 error node ID가 image item ID인지 단언한다. |
| 13 | 거부가 다른 fixture의 오류를 우연히 통과시킨다 | 각 제어는 동일 정상 구조에서 source 또는 PNG bytes 하나만 바꾼다. |
| 14 | row-axis에서만 image gate가 우회된다 | source, decode, size 제어 모두 Columns와 Rows로 반복한다. |
| 15 | decode 재구성으로 snapshot revision이 달라진다 | 기존 `ImageRequest::from_document`와 `IntrinsicSizeRevision` 순서를 변경하지 않았다. |
| 16 | 이미지 두 개 또는 placement 변조가 이미지 제어를 가린다 | 변조 fixture는 기존 3-item 구성과 배치를 유지한다. 해당 baseline positive도 통과한다. |
| 17 | valid 24×16 PNG도 거부된다 | helper의 default positive 경로가 기존 8 scene 모두에서 통과했다. |
| 18 | 잘못된 크기 PNG가 compile-time path에서 누락된다 | 공유 tracked C18.4 asset을 `CARGO_MANIFEST_DIR` 기준으로 include하고 실제 decode한다. |
| 19 | fixture code가 test feature 밖으로 제품 동작을 바꾼다 | 변경은 fixture-only module에 있고 runtime product API·feature 선언은 건드리지 않았다. |
| 20 | 한 실패가 다른 실패를 복구 불가능하게 남긴다 | 세 입력은 별도 document fixture로 실행한다. worker나 전역 상태를 공유하지 않는다. |

## PR diff 실패 관점 30개

| # | 독립 diff 공격 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | `prepare()`의 인자·반환 계약 변경 | signature가 유지된다. |
| 2 | 정상 경로가 fixture PNG 대신 빈 값 전달 | wrapper가 `super::PNG`를 전달한다. |
| 3 | image decoder 호출을 제거 | `ImageRequest::decode` 경로가 유지된다. |
| 4 | decode error를 무시하거나 대체 자원 생성 | 기존 `map_err(fail)` 경로가 그대로다. |
| 5 | dimensions 검사를 삭제 | 24×16 exact 비교가 그대로다. |
| 6 | helper가 source/profile 검증보다 먼저 decode | helper는 기존 본문을 그대로 감싸 검증 순서를 보존한다. |
| 7 | `has_mixed_signature` fallback 변화 | 기존 선행 gate가 이동·수정되지 않았다. |
| 8 | document/style revision 검증 변화 | 기존 revision 검증 호출이 그대로다. |
| 9 | root CSS profile 변경 | diff에 root validation 변경이 없다. |
| 10 | item placement gate 변경 | diff에 placement 함수 변경이 없다. |
| 11 | text contribution 기준 변경 | diff에 text validation 변경이 없다. |
| 12 | generic dispatcher 변경 | 수정된 파일은 전용 adapter와 그 test뿐이다. |
| 13 | source 변조 test가 computed style을 재사용해 stale 상태가 된다 | 변조 snapshot으로 새 fixture와 cascade를 만든다. |
| 14 | default fixture source가 accidental override된다 | `make_fixture()`는 기존 `IMAGE_URL` wrapper를 유지한다. |
| 15 | test builder가 CSS를 바꿔 오류 원인이 모호해진다 | source helper는 `src` 값만 매개변수화한다. |
| 16 | wrong URL test가 decode 실패를 source 실패로 오인 | 올바른 PNG bytes를 제공한다. |
| 17 | malformed PNG test가 URL gate에서 끝난다 | canonical source URL을 사용한다. |
| 18 | size test의 PNG가 실제로 invalid하다 | decoder precondition이 성공과 2×2 dimensions를 확인한다. |
| 19 | size test가 한 축에서만 수행된다 | Columns와 Rows 모두에서 실행한다. |
| 20 | source test가 한 축에서만 수행된다 | Columns와 Rows 모두에서 실행한다. |
| 21 | decode test가 한 축에서만 수행된다 | Columns와 Rows 모두에서 실행한다. |
| 22 | wrong-size assertion이 예상 branch를 확인하지 않는다 | valid 2×2 bytes에서만 helper를 호출하며 failure node는 image다. |
| 23 | 모든 실패가 generic root 오류로 잘못 귀속된다 | error node ID를 image item으로 단언한다. |
| 24 | 새 test가 `Ok(None)`을 성공으로 취급한다 | helper assertion은 모든 `Ok(_)`를 panic으로 처리한다. |
| 25 | known-good scene 회귀가 빠진다 | 기존 8-scene positive test가 같은 filter에서 함께 통과한다. |
| 26 | test-only dependency가 추가된다 | Cargo manifest와 lockfile diff가 없다. |
| 27 | Android/iOS FFI route 또는 payload가 달라진다 | FFI·platform 파일 diff가 없다. |
| 28 | simulator 증거를 실행하지 않고 주장한다 | 이 기록은 simulator를 실행하지 않았다고 명시한다. |
| 29 | Clippy 결과를 새 코드 통과로 과장한다 | strict failure와 기존 두 lint, 예외 Clippy 결과를 분리해 적었다. |
| 30 | 제한 테스트가 C11.4 완료나 공식 수 증가로 승격된다 | 상태 대장, C11.4 checkbox, `72/213` 변경이 없다. |

## 남은 범위

이 보완은 direct generic mixed adapter의 고정 fixture 입력에서 세 image rejection 경로만 확인한다. Android/iOS simulator 앱, 실기기, 일반 CSS admission, WPT 및 전체 C11.4는 검증하지 않는다. 공식 C11.4와 `72/213`은 미완료다.
