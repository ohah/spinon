# C11.4 image-mix fix: focused PR review recheck

## 기준

- Rust 구현과 regression test는 immutable commit f45596b8b0b8a4d35807028bac30314a5dceaaab에서 읽었다. 이전 bbd6fe42 report는 원본 PR의 역사 기록으로 유지한다.
- 계약/상태 문서는 수정 commit 이후 현재 worktree에서 읽은 최신 문구다. 해당 docs 변경이 f455 commit에 포함되지 않은 경우 source commit에 소급 귀속하지 않는다.
- 테스트와 빌드는 실행하지 않았다.

**요약: PASS 4 / PARTIAL 3 / FAIL 0.** 원래 FAIL인 두 direct text item + root HTML img 수집은 수정됐다. 남은 PARTIAL은 namespace/tag 분기와 offending node를 고정하는 직접 test가 충분하지 않고, one-item fallback은 collector의 None 경계만 확인한다는 점이다.

## 집중 재검토

| # | 상태 | 관점과 결과 |
|---:|---|---|
| 1 | PASS | 원래 root-sibling repro가 이제 닫힌다. 두 text item 후보 뒤에 first_html_image가 root subtree를 검사하고 HTML img handle을 source_failure에 넘겨 unsupported_c114_text_source를 반환한다. Regression 입력이 두 direct text item + root img이고 expected code가 같다. crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_text/generic_direct_text.rs:33-75,307-329; crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_text/generic_direct_text_tests/negative.rs:59-67. 최신 내부 계약도 두 후보 이상에서 subtree의 HTML img를 거부한다고 명시한다: spec/internal/0059-c11-grid-layout.md:558. |
| 2 | PASS | image scan은 root에서 시작해 descendant element/text subtree 전체를 순회하므로 item 내부 또는 중첩 descendant에 있는 HTML img도 잡는다. 기존 negative case는 direct Text와 같은 item 안에 img를 둔다. img가 아닌 nested element도 direct_text_source에서 독립적으로 거부된다. generic_direct_text.rs:251-279,307-329; negative.rs:25-58. |
| 3 | PARTIAL | matcher는 namespace == http://www.w3.org/1999/xhtml 및 local_name == img 두 조건을 모두 요구한다. 최신 계약의 HTML img와 일치하며, SVG namespace의 이름 같은 element는 이 조건에 맞지 않는다. 다만 test는 HTML img rejection만 추가했고 SVG namespace의 img 이름 및 SVG image tag 대조는 없다. 코드/계약 일치는 확인됐지만 namespace/tag false-positive 회귀는 직접 고정하지 않았다. generic_direct_text.rs:19,307-329; negative.rs:59-81; spec/internal/0059-c11-grid-layout.md:558. |
| 4 | PARTIAL | HTML img 발견 시 error node는 발견한 image NodeId이고 error code는 unsupported_c114_text_source다. source_failure가 전달된 NodeId를 그대로 결과에 넣는다. test는 expected code와 node_id가 존재하는지만 확인하며, offending img의 구체 NodeId와 동일한지는 assert하지 않는다. generic_direct_text.rs:68-72,493-499; negative.rs:94-105. |
| 5 | PASS | cardinality 순서가 유지된다: 후보가 0/1개면 image scan 전에 Ok(None), 3개 이상이면 third candidate error, 정확히 2개일 때 root subtree image check가 실행된다. 따라서 기존 one-item/non-grid collector fallback boundary는 바뀌지 않았고, root image regression은 정확히 generic 2-item slice에서 실패한다. generic_direct_text.rs:28-31,58-75; negative.rs:126-158. |
| 6 | PASS | 빈 HTML block sibling은 meaningful text candidate가 아니며 HTML img도 아니어서 계속 허용된다. 이를 명시한 regression은 두 valid text item과 empty div에 대해 Some 입력과 paragraph style 2개를 확인한다. generic_direct_text.rs:37-56,68-75,307-329; negative.rs:295-312; 최신 계약 spec/internal/0059-c11-grid-layout.md:558. |
| 7 | PARTIAL | non-grid root와 one-item legacy-shaped root는 각각 collector가 None을 반환하는 것을 확인한다. 다만 검사는 generic_direct_text::prepare에서 끝나며 기존 전체 dispatch를 따라 13개 TEXT_CASES legacy fallback이 성공하는지 실행하지 않는다. one-text + img 조합도 이 경계 test에는 없고, 두 후보 미만이면 generic image rejection 대신 fallback에 넘긴다. 최신 plan은 fewer-than-two fallback을 보존한다고 명시하고, 13개 fallback의 runtime 검증은 주장하지 않는다. generic_direct_text.rs:28-31,58-60; negative.rs:126-158; crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_text.rs:128-155; plan/c11-4-generic-direct-text-intrinsic-input.md:11,132,137; spec/internal/0059-c11-grid-layout.md:558. |

## 범위 참고

최신 runtime evidence는 pass03이 f45596b8 source commit을 기록하고 Android/iOS matrix를 통과했다고 보고한다. 해당 앱 장면은 canonical positive 하나다. 이 focused review는 그 로그를 source 실행의 재현으로 간주하지 않았고, malformed mixed-image runtime scene도 추가 검증하지 않았다. See spec/internal/evidence/c11-4-generic-direct-text-intrinsic-runtime-2026-10-26/README.md:5-7,11-18,40-52.
