# C11.4 generic direct-text app matrix 계획 독립 검토 20관점

- 대상 계획: `plan/c11-4-generic-direct-text-runtime-matrix.md`
- 기준 상태: `origin/main` `8b5a3f9a0cbb664b479bf70aa78dc398baa71de8`
- 검토 방식: 기존 direct-text precomparison, Rust scene expectations, merged STATUS 및 앱 FFI source를 서로 대조했다. 새 browser 관찰·build·test는 실행하지 않았다.
- 결과: **PASS 20 / PARTIAL 0 / FAIL 0**. 기존 계획에 정의된 비교 경계만 재사용하며 구현 완료나 검증 통과를 주장하지 않는다.

| # | 독립 실패 관점 | 대조 및 판정 |
| --- | --- | --- |
| 1 | 대상 범위가 실제 미검증 장면과 일치하는가 | 기존 #645 앱 route는 `generic-root-normal` 하나, Rust positive는 6개다. 나머지 5개를 범위로 잡아 PASS. |
| 2 | 기존 구현/기준 브랜치가 최신인가 | 계획 기준은 병합된 `origin/main` `8b5a3f9a`이며 최신 #675 runtime과 #677 preview sync가 포함돼 PASS. |
| 3 | oracle provenance가 구현과 분리되어 있는가 | Chrome 156/Ahem README와 기존 JSON을 그대로 참조하며 신규 browser 수집을 금지해 PASS. |
| 4 | 장면 수와 IDs가 중복·누락 없이 정해졌는가 | 정상 대조군 1개와 신규 5개를 구체 route ID로 열거해 PASS. |
| 5 | item별 intrinsic 기대값이 정확한가 | `cases.rs`와 대조해 각 item's width/height min/max를 표에 분리했다. 특히 reorder의 `item-z` min width 20을 `item-a`의 10과 구별해 PASS. |
| 6 | 최종 item placement의 순서가 보존되는가 | content variation의 `item-zeta → item-3`, reorder의 최종 `item-a → item-z`를 명시해 PASS. |
| 7 | style owner 교차 적용을 탐지하는가 | reorder 장면에서 `item-a`만 break-all, `item-z`는 normal임을 별도로 확인하도록 해 PASS. |
| 8 | whitespace 원문과 computed collapse가 구분되는가 | `grid-ws` raw `" A  BC "` 보존과 intrinsic 계산 후 collapse를 별도 검사하도록 해 PASS. |
| 9 | nowrap/break-all 입력의 contribution을 구분하는가 | 두 장면의 min/max width와 text height 기대값이 각각 지정돼 PASS. |
| 10 | root ID와 scene route ID를 혼동하지 않는가 | content variation의 `grid-9x`, whitespace의 fixture mapping을 별도 기술하고 scene 선택을 harness route로 한정해 PASS. |
| 11 | frame coordinate 비교에 페이지 offset이 섞이지 않는가 | Chrome document y를 root-relative로 바꾸고 runtime mount root `(0,0)`을 요구해 PASS. |
| 12 | columns 값만 맞고 rows/text layout이 틀린 경우를 거르는가 | 두 축 contribution, final text layout, root/item frames와 draw/present를 함께 요구해 PASS. |
| 13 | intrinsic input contract의 소유권/typed values가 빠지지 않는가 | source order, font revision, source kind, preferred/min/max size, zero edges, style/layout identity를 matcher 대상에 포함해 PASS. |
| 14 | partial output 또는 legacy fallback이 성공처럼 보일 수 있는가 | 알 수 없는 scene과 revision/style mismatch가 성공 frame/draw 전 거부되어야 하며 generic collector ID-dispatch를 회귀 검사하도록 해 PASS. |
| 15 | simulator matrix가 고정 source와 기존 control을 포함하는가 | 같은 commit의 Android AVD와 iOS Simulator 각각 6/6, 정상 대조군 포함을 조건으로 해 PASS. |
| 16 | 실행 환경/화면의 한계를 넘겨 주장할 수 있는가 | glyph paint, pixel equality, compositor 이후 값 및 실기기/hardware GPU를 제외해 PASS. |
| 17 | fixture 전용 코드가 제품 artifact로 새지 않는가 | default/product archive와 fixture-test positive control을 비교하되 APK/IPA 제외라고 확대하지 않아 PASS. |
| 18 | 화면 evidence의 역할이 정확한가 | 대표 scene screenshot은 route/layout evidence로 한정하고 text paint로 해석하지 않도록 해 PASS. |
| 19 | 검증·리뷰·merge gate가 구체적인가 | focused test/fmt/Clippy, 새 implementation 20-view, 별도 PR diff 20-view, required check 상태와 rebase merge를 정의해 PASS. |
| 20 | 상태 대장·로드맵이 성급하게 승격될 위험이 있는가 | post-merge 동기화를 요구하되 C11.4/C11 및 `72/213`을 유지해 PASS. |

## 발견과 조치

초기 계획 표는 reorder scene의 모든 item에 동일한 min-content width를 기대할 수 있는 것처럼 축약돼 있었다. Rust expectation을 다시 대조해 `item-a`는 width min/max `10/40`, `item-z`는 `20/40`으로 구분했고 나머지 item별 block-size 입력도 표에 함께 고정했다. 구현 전 oracle 오해 경로는 닫혔다.

## 범위·기대값 수정 후 재확인

line-advance 기대값과 `Rows` node별 기여 조건을 `cases.rs` 및 `positive.rs` assertion과 다시 대조했다. PASS 5, 9, 12, 13 관점은 수정된 계획 기준으로 모두 통과하며, 나머지 검토 항목의 입력·판정·범위는 바뀌지 않았다.
