# C19.1 computed color incremental reuse · 2026-10-26

## 변경

`C19ComputedColorV1`에 대해 full cascade와 subtree incremental cascade가 같은 두 Longhand(`background-color`, `color`)를 직렬화하도록 고정했다. incremental 경로는 dirty subtree를 다시 계산하고 상위 context를 계산해 상속을 갱신한 뒤, 영향 없는 root/sibling output을 재사용한다. typed color/background carrier 전체는 같은 현재 문서의 full cascade와 정확히 일치한다.

legacy `background_color` 및 `background_paint`는 계속 `None`이다. author stylesheet가 있는 호출, profile/viewport가 바뀐 호출, 허용 범위를 벗어난 style revision 및 이전 문서 revision 재호출은 기존처럼 `None` fallback으로 남는다. CSS color 계산, gamut mapping, paint, hit-test와 GPU 경로는 바꾸지 않았다.

## 고정 입력과 변경 식별

- 기준 source: `origin/main` `29449e4f794e53255909b58ed8fb2fd4995dd5d2`.
- 계획: [C19.1 incremental reuse 계획](../../../../plan/c19-1-incremental-color-reuse.md), blob SHA `97d31e561670a3a3de128bdf84c9a5c0c86ee071`.
- 계획 실패 관점 검토: [20개 계획 검토](./plan-review-20.md).
- 검토한 Rust source patch SHA-256: `3bef4bc6237df85228456722183e23b69c373e85bf310266feda93fcd876e1ce`.
- 구현 실패 관점 검토: [20개 구현 검토](./implementation-review-20.md).
- C19 공식 체크 상태와 집계는 바꾸지 않았다. 이 결과는 C19.1 incremental 재사용 조각만 설명한다.

## 검증

- `mise exec -- cargo test -p spinon-style c19_`: **4 passed**. [로그](./logs/c19-tests.log)
- `mise exec -- cargo test -p spinon-style`: **244 passed**, doc tests 0. [로그](./logs/spinon-style-tests.log)
- `mise exec -- cargo clippy --locked -p spinon-style --all-targets -- -D warnings`: 통과. [로그](./logs/clippy.log)
- `mise exec -- cargo fmt --all -- --check`: 통과. [로그](./logs/format-check.log)

검증 입력은 고정 CSS fragment tree다. 변경 전후 full cascade와 incremental snapshot의 ordered element, color property map, typed `computed_color`/`computed_background_color`, legacy paint None 경계, diagnostics를 비교했다. dirty parent context 및 영향 없는 sibling의 재사용 통계·값과 author stylesheet/profile/viewport/style revision/document revision 거부 조건도 확인했다.

Browser, Android/iOS simulator, 실기기, GPU surface는 이번 순수 Rust cache-equivalence 범위에서 실행하지 않았다. 이 결과는 화면 출력, 색상 oracle parity, C19.1 전체 완료 또는 C19 완료를 증명하지 않는다.
