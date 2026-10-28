# C11.4 mixed 입력 거부 후속 PR 변경 검토 20개

## 검토 범위

PR #732의 `origin/main` 대비 전체 변경을 독립 실패 관점으로 재검토했다. 코드는 `spinon-runtime`의 `#[cfg(test)]` 전용 negative-control module에 한정되고, fixture 계획·공식 상태·검토 evidence를 함께 확인했다. 제품 layout 알고리즘, FFI, app route, dependency 또는 public contract 변경은 없다.

## 변경 실패 관점 20개

| # | 독립 diff 공격 가설 | 확인 결과 |
|---|---|---|
| 1 | 테스트 변경이 제품 실행 경로를 바꾼다 | 새 Rust module은 기존 `#[cfg(test)]` module 아래에서만 선언된다. |
| 2 | 테스트 전용 변경이 Cargo feature나 dependency를 늘린다 | manifest와 lockfile은 diff에 없다. |
| 3 | malformed 결과가 `Ok(None)`으로 빠져 다음 adapter가 처리한다 | dispatcher 검증 helper는 `Err`만 허용하고 `None`/`Some` 모두 실패 처리한다. |
| 4 | 텍스트를 바꾼 뒤 이전 snapshot이나 cascade 결과를 재사용한다 | text mutation helper가 document를 commit하고 snapshot/view를 갱신한 뒤 cascade를 새로 만든다. |
| 5 | direct-child 누락 test가 실제 Grid child를 제거하지 않는다 | `RemoveChild`를 새 revision으로 commit하고 새 snapshot에서 dispatcher를 호출한다. |
| 6 | extra-child test가 detached node만 만들어 count를 속인다 | `append_spec`가 새 node를 Grid root의 direct child로 삽입한 뒤 commit한다. |
| 7 | 순서 검증이 placement mutation만 다루고 DOM 순서는 바꾸지 않는다 | 세 child를 실제로 detach하고 `[a,c,b]` 순서로 재삽입한다. |
| 8 | 한 축 negative가 다른 축의 고정 입력으로 가려진다 | 모든 구조·스타일·track mutation matrix가 Columns와 Rows를 각각 순회한다. |
| 9 | text negative가 signature 자체를 깨뜨려 검증 경계를 건너뛴다 | 변경 text는 여전히 direct text + image signature를 유지한 상태에서 거부된다. |
| 10 | wrong-root-size control이 target sizing axis와 무관한 값을 바꾼다 | Columns 폭 299px, Rows 폭 119px로 각 축의 고정 root 폭만 바꾼다. |
| 11 | gap test가 cross-axis gap을 바꿔 target-axis gate를 놓친다 | Columns에서는 column gap, Rows에서는 row gap을 9px로 바꾼다. |
| 12 | computed property와 typed field 불일치를 한 경로로만 검증한다 | display/alignment property, position/dimension/font typed fields를 분리해 공격한다. |
| 13 | flex-factor test가 기대 track을 복사해 positive case를 그대로 둔다 | target track의 2fr을 3fr과 infinity로 각각 교체한다. |
| 14 | fixed neighbor test가 target axis가 아닌 track을 바꾼다 | target-axis index 1의 40px track만 41px로 바꾼다. |
| 15 | wrong-axis test가 같은 축 입력을 다시 제출한다 | Columns/Rows track list를 서로 바꿔 root contract gate까지 확인한다. |
| 16 | stale revision test의 baseline이 우연히 revision zero다 | 현재 document revision이 nonzero인지 먼저 단언한 뒤 style revision을 default로 변경한다. |
| 17 | profile test가 unsupported style 값이 아니라 동일 profile을 재사용한다 | `RuntimeBlockFlexGridTrackSizingV1`에서 `FlexLayoutV1`로 바꾼다. |
| 18 | typed writing-mode test가 CSS property 문자열만 바꾼다 | `writing_styles`의 해당 text node entry를 반대 mode로 바꾼다. |
| 19 | 계획·상태 문서가 테스트 결과를 C11.4 구현 완료로 승격한다 | plan/status 모두 app 미실행·일반 CSS/WPT 미검증과 C11.4 미완료·72/213 유지 범위를 기록한다. |
| 20 | 근거 파일이 실제 코드·PR 또는 도구 결과와 불일치한다 | implementation review, 새 module 경로, PR #731 merge SHA, 최종 test/Clippy 결과와 문서 링크를 대조했다. |

## 검증 경계

최종 fixture-feature test suite는 unit 284 passed / 3 ignored, integration 3 passed와 16 passed다. `cargo fmt --all -- --check`, 두 기존 Clippy lint를 제외한 package Clippy와 `git diff --check`는 통과했다. strict Clippy는 수정하지 않은 `validation.rs:329` `type_complexity`와 `generic_direct_text_image/fixture.rs:311` `map_identity`에서 실패했다. Android/iOS 앱·Simulator는 실행하지 않았다.

이 변경은 테스트 경계를 확인하며 CSS Grid 제품 지원을 넓히지 않는다. C11.4 공식 체크와 `72/213`은 그대로 둔다.
