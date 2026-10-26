# C11.4 generic direct-text test split targeted recheck

**결과: PASS.** 직전 targeted plan review의 test split PARTIAL이 현재 child plan 개정으로 해소됐다. 이 report는 그 항목만 대조했으며 authored plan/source를 수정하지 않았고, tests·browser·simulator를 실행하지 않았다.

| 기존 단일 파일 구간 | 계획된 module | 판정 근거 |
| --- | --- | --- |
| L1–313, 약 313줄 | `generic_direct_text_tests/cases.rs` | scene expectations와 immutable case data만 분리한다. |
| L314–563, 약 250줄 | `generic_direct_text_tests/positive.rs` | six-scene positive collector/output assertions를 소유한다. |
| L564–864, 약 301줄 | `generic_direct_text_tests/negative.rs` | source/style/parser/fallback controls를 소유한다. |
| L866–1111, 약 246줄 | `generic_direct_text_tests/test_fixture.rs` | HostDocument/Stylo builder, `NodeSpec`, fixture builder와 assertion helpers를 소유한다. |

`generic_direct_text_tests.rs`는 네 module declaration만 가진 얇은 root로 두고 `mod.rs`는 만들지 않는다고 계획에 명시돼 있다([child plan §test split](./c11-4-generic-direct-text-intrinsic-input.md#L120)). 네 책임을 모두 별도 module에 두므로 expectations와 builder가 결합된 500줄 초과 module 문제가 해소되며, 각 계획 크기도 500줄 이하다([기존 test 경계](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_text/generic_direct_text_tests.rs#L1-L1111), [프로젝트 규칙](../docs/project-rules.md#L30)).

**잔여 모호성:** 이 test split 범위에는 없음. 이전 targeted report의 12 PASS / 1 PARTIAL은 이 수정을 반영하기 전의 역사적 판정이며, 이번 recheck는 새 독립 증거로 해당 PARTIAL이 해소됐음을 기록한다. 구현 및 회귀 tests는 split 이후 재실행 gate로 남는다.
