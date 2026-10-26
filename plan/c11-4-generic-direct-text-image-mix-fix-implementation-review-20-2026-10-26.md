# C11.4 direct-text 이미지 혼합 수정 구현 리뷰 · 20개 관점

검토일: 2026-10-26  
대상 commit: `f45596b8b0b8a4d35807028bac30314a5dceaaab`  
검토 범위: `generic_direct_text.rs`, `generic_direct_text_tests/negative.rs`, 계획 및 pass03 Rust/Android/iOS 근거. 읽기 전용으로 수행했으며 빌드·테스트는 실행하지 않았다.

## 판정

**코드 blocker 0건.** 두 정상 direct-text item과 root-level HTML `<img>`가 함께 있을 때 collector가 `unsupported_c114_text_source`로 거부한다. 의도적으로 비텍스트인 빈 block sibling은 text candidate에 포함하지 않고, 기존 collector 테스트에서 계속 허용된다. pass03 앱 런타임은 새 commit에서 정상 positive 장면을 재검증했지만, image negative와 빈 sibling을 앱에서 실행하지는 않는다. 계획은 이 rejection controls를 Rust test 범위로 한정하므로 범위 위반은 아니다.

**문서 상태:** 검토 중 canonical runtime README가 pass03로 동기화됐다. 현재 source-head, Android/iOS artifact 및 screenshot hashes, Rust test/Clippy/fmt logs를 새 run에 연결하고 image-mix fix의 범위도 명시한다.

## 적대적 관점 20개

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | collector가 legacy special adapter보다 앞서 실행되어 기존 scene을 가로채는가 | **PASS** — `prepare_c11_4_text`에서 special adapter 뒤, legacy text scan 전에 호출된다 (`c11_4_text.rs:75-132`). |
| 2 | 비-Grid root가 새 image scan으로 잘못 거부되는가 | **PASS** — root computed `display`가 `grid`가 아니면 `Ok(None)`로 반환한다 (`generic_direct_text.rs:27-31`). |
| 3 | root의 직접 자식이 아닌 text item을 direct candidate로 잘못 세는가 | **PASS** — candidate 생성은 root의 직접 자식을 순회하고 각 자식 subtree에서 의미 있는 text 유무만 확인한다 (`:33-55`). |
| 4 | 접을 수 있는 HTML whitespace가 빈 block을 text item으로 바꾸는가 | **PASS** — collapsible whitespace만 가진 subtree는 candidate가 아니다 (`:281-305`). |
| 5 | 의미 있는 텍스트가 없는 빈 block이 image로 오인되는가 | **PASS** — image predicate는 HTML namespace와 local name `img`만 확인한다 (`:307-329`). |
| 6 | candidate가 하나뿐인 입력이 새 generic profile에 부분 수용되는가 | **PASS** — one-item shape는 기존 fallback으로 넘긴다 (`:58-60`). |
| 7 | candidate가 세 개 이상일 때 image guard가 정확한 candidate-count 오류를 덮는가 | **PASS** — 2개 초과를 먼저 명시적으로 거부한다 (`:62-66`). |
| 8 | 두 valid item 뒤 root-level `<img>`를 놓치는가 | **PASS** — 정확히 두 candidate 다음 root 전체를 검사하고 image를 source owner로 거부한다 (`:68-73`). |
| 9 | root 아래 더 깊은 descendant에 있는 HTML `<img>`를 놓치는가 | **PASS** — `first_html_image`가 root부터 자식 subtree 전체를 stack traversal 한다 (`:307-327`). |
| 10 | SVG 등 다른 namespace의 이름 `img`를 HTML image로 오검출하는가 | **PASS** — `HTML_NAMESPACE` 일치 조건이 있다 (`:19`, `:316-319`). |
| 11 | HTML namespace의 다른 element를 image로 오검출하는가 | **PASS** — `local_name() == "img"`를 함께 요구한다 (`:316-319`). |
| 12 | 여러 image가 있을 때 offending image가 오류에서 누락되는가 | **PASS** — 첫 발견 handle의 `NodeId`를 `source_failure`에 전달한다 (`:68-73`). 어떤 이미지가 먼저 선택되는지는 지원 계약의 결과를 바꾸지 않는다. |
| 13 | 기존 item 내부 image 혼합 rejection이 새 root-level case 때문에 사라지는가 | **PASS** — 기존 malformed control은 그대로 남고, 새 control은 별도 case로 추가됐다 (`negative.rs:45-67`). |
| 14 | 새 회귀 입력이 실제로 두 direct-text item과 root-level image를 구성하는가 | **PASS** — `a/A`, `b/B`, root child `img` 세 자식을 지정한다 (`negative.rs:60-66`). |
| 15 | negative test가 단순 실패만 보고 올바른 거부 경계를 놓치는가 | **PASS** — 공통 assertion이 error code `unsupported_c114_text_source`와 `node_id.is_some()`을 확인한다 (`negative.rs:94-103`). |
| 16 | 빈 block sibling이 세 번째 text candidate로 추가되는가 | **PASS** — empty `<div class="item">`은 의미 있는 text가 없어 candidate가 아니다 (`negative.rs:296-306`, `generic_direct_text.rs:49-52`). |
| 17 | image scan이 정상 빈 block을 무조건 거부하는가 | **PASS** — 별도 collector test가 빈 block 형제와 함께 input 반환 및 `paragraph_styles.len() == 2`를 확인한다 (`negative.rs:295-312`). |
| 18 | 새 negative control이 실제 computed style/HostDocument cascade 밖에서 수동 구성되어 거짓 양성을 내는가 | **PASS** — test는 `custom_fixture`와 `cascade`를 거쳐 `prepare`를 호출한다 (`negative.rs:94-100`). 다만 이 negative는 app FFI/Simulator 입력이 아니다. |
| 19 | pass03 runtime이 수정 commit보다 앞서거나 다른 source revision을 대상으로 하는가 | **PASS** — run의 `source-head.txt`는 정확히 `f45596b8…`; Android/iOS build/runtime 로그 timestamp도 commit 시각 이후다. 두 플랫폼 summary는 `status=0`, `layout=ready`, `boxes=3`, draw `presented boxes=3`를 기록한다. |
| 20 | 스크린샷/앱 런타임을 이미지 negative 또는 글리프 paint 증명으로 과장하는가 | **PASS** — pass03은 정상 `generic-root-normal`만 실행하며 계획대로 rejection controls는 Rust tests에서 확인한다. Android/iOS 모두 `runtime_text_count=0`, `glyph_batch_draw_count=0`이고, 현재 [runtime README](../spec/internal/evidence/c11-4-generic-direct-text-intrinsic-runtime-2026-10-26/README.md)는 screenshot을 layout/surface evidence로 한정하고 pass03 source-head·artifact·screenshot hashes를 연결한다. |

## pass03에서 확인한 실행 근거

- Rust 로그: generic direct-text `9/9`, C11.4 runtime `34/34`, C11.4 focused runtime `19/19` 통과로 기록되어 있고, saved targeted Clippy log에는 오류/경고가 없으며 fmt log도 함께 보존됐다. 리뷰어는 이를 다시 실행하지 않았다.
- Android API 37 ARM64 AVD `spinon_api37_1_compat16k`: build 성공, scene summary `status=0 layout=ready boxes=3`, frame trace root `(0,0,100,20)`, item `(0,0,20,20)` 및 `(30,0,40,10)`, draw `presented boxes=3`.
- iPhone 17 Pro / iOS 26.2 Simulator: build 성공, 동일 frame/summary와 draw/present.
- 양쪽 trace는 `layoutInputRevision.intrinsicSize=0`, collapsed outer gutters, 두 text intrinsic inputs 및 contribution trace를 남긴다. Draw log에서 `runtime_image_count=0`, `image_draw_count=0`, `runtime_text_count=0`, `glyph_batch_draw_count=0`이므로 text/image paint는 입증하지 않는다.
- 새 screenshot SHA-256: Android `9b74719ac4872fa9068b9cc02f1fc4a3f8a89c83c4fadb613ad7cf62642e5a73`; iOS `a7b38f7a12f89040962c0bc64fa731d4693baa31e534f041d962a7d3764ea721`. pass03 artifacts는 [run directory](../spec/internal/evidence/c11-4-generic-direct-text-intrinsic-runtime-2026-10-26/runs/c114-generic-direct-text-20261026T034304Z-f45596b8-pass03/)에 있다.
- 코드 SHA-256: `generic_direct_text.rs` `76755b714a92b8abe15b14e160b6ddbb3507e25186eb6652344b57f222bd8fc0`; `negative.rs` `2538f9e53b75862aeed69947cf4fd367c4969e3b38ca9a3c1910343bbcc5492c`.

## 남은 범위

새 `<img>` rejection은 Rust HostDocument collector test에서 확인됐으며 앱 경로에서 별도 negative scene을 실행하지 않았다. 빈-block control도 collector 차원에서만 확인되어 extra sibling을 포함한 end-to-end layout geometry를 주장하지 않는다. 이는 계획의 Rust-only negative 범위와 일치한다. canonical runtime README는 검토 중 pass03 run, artifact/screenshot hashes 및 source revision으로 갱신됐다. 공식 C11.4 완료나 `72/213` 상태는 이 리뷰에서 변경하지 않는다.
