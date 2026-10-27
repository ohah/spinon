# C11.4 row-axis span-stage implementation PR diff review · 30 viewpoints

**검토 대상:** test-only row-axis trace 추가와 기존 helper 일반화, C11.4 plan/STATUS/evidence 변경. 독립 reviewer는 코드를 수정하거나 테스트·브라우저·시뮬레이터를 실행하지 않았다. 최초 검토는 base `ba19d61a`에서 수행했고, 지적된 두 assertion을 보완한 뒤 해당 관점과 column regression을 별도 재검토했다. 실행 source hash와 최종 명령 결과는 [validation.json](./validation.json)에 기록한다.

## 최종 집계

**27 PASS / 3 PARTIAL / 0 FAIL.** 초기 검토의 fixed Columns 값과 전체 stage sequence 누락은 수정 후 PASS다. 이후 추가한 validation manifest, staged whitespace 검사, 이 보고서 링크도 확인했다. PARTIAL은 현재 oracle correctness를 막지 않는 trace/API 표현 한계와 JSON/test constant duplication이다.

| # | 실패 관점 | 판정 | 근거 |
|---:|---|---|---|
| 1 | 변경 범위가 product/runtime이 아닌가 | PASS | [README](./README.md)는 알고리즘·CSS admission·app runtime·FFI가 변경되지 않았음을 한정한다. |
| 2 | Rust 모듈이 test-only 경계 안에 있는가 | PASS | 새 row-axis module은 기존 `#[cfg(test)]` 부모 아래에 선언된다. |
| 3 | private builder/helper 재사용이 적절한가 | PASS | 새 module은 `multi_span_document` 및 shared snapshot assertion을 재사용한다. |
| 4 | fixed HTML과 HostDocument item 구조가 맞는가 | PASS | fixture와 builder 모두 빈 div item을 사용하고 text/image 기여를 섞지 않는다. |
| 5 | root/viewport definite size가 고정됐는가 | PASS | 둘 다 220×220 CSS px다. |
| 6 | CSS axis와 trace axis가 맞는가 | PASS | 220px fixed column, 네 intrinsic rows, 독립 Rows/Columns pass 선택을 사용한다. |
| 7 | root trace 중복을 거부하는가 | PASS | root trace가 정확히 하나여야 한다. |
| 8 | 각 axis pass 수를 제한하는가 | PASS | Columns와 Rows pass가 각각 정확히 하나인지 검사한다. |
| 9 | fixed Columns 대조군의 실제 값이 맞는가 | PASS | 재검토에서 early-return snapshot의 단일 CSS track이 220px임을 1e-4px 이내로 확인했다. |
| 10 | row sizing 입력이 고정됐는가 | PASS | 네 `minmax(0,max-content)` row와 fixed column이 계획과 일치한다. |
| 11 | row line placement 및 span이 맞는가 | PASS | A/B는 span-2, C는 span-3 위치를 사용한다. |
| 12 | source-order controls가 유효한가 | PASS | 각 forward/reversed pair는 placement와 contribution을 보존한다. |
| 13 | span batch 목록/순서를 확인하는가 | PASS | 전체 batch sequence `[2]` 또는 `[2,3]`을 대조한다. |
| 14 | 전체 stage order를 고정하는가 | PASS | 재검토에서 Initialize, batch들, complete, maximize, flexible expansion, `StretchAutoTracks(false)` 전체 목록 비교를 확인했다. |
| 15 | 중간/최종 stage 산술이 맞는가 | PASS | 규범 기대값, growth limits, final rows 및 남는 공간을 고정한다. |
| 16 | contribution item/count 귀속이 맞는가 | PASS | 각 item/span/kind 조합의 수와 identity를 비교한다. |
| 17 | contribution 종류/높이/non-flex selector가 맞는가 | PASS | Minimum, MinContent, MaxContent, 높이와 non-flex 상태를 검사한다. |
| 18 | source order 간 결과 독립성을 확인하는가 | PASS | snapshots, track shape 및 semantic item frames를 pairwise 비교한다. |
| 19 | `None`을 실제 +∞로 과대 해석하지 않는가 | PARTIAL | trace API가 모든 non-finite 값을 `None`으로 축약한다. 계획과 README는 실제 +∞를 증명하지 않는 한계를 명시한다. |
| 20 | track metadata의 규범 절대값을 확인하는가 | PARTIAL | gutter/collapse 구조와 source-order 간 metadata 동일성은 검사하지만 metadata 절대값을 독립 규범 oracle과 대조하지 않는다. 현재 plan의 geometry/stage 범위에는 필수 합격 조건으로 두지 않는다. |
| 21 | Chrome DOMRect와 item/root frame을 대조하는가 | PASS | x/y/width/height를 root-relative geometry와 비교한다. |
| 22 | CSSOM 반올림과 frame tolerance를 분리하는가 | PASS | Chrome CSSOM quantization, 1/64px frame tolerance, 1e-4px trace tolerance를 구분한다. |
| 23 | 기존 column test/negative control regression을 보존하는가 | PASS | 기존 callsite와 기대 batch 배열은 유지되고 focused suite에서 3 tests가 통과했다. |
| 24 | test 상수와 checked-in JSON이 자동 동기화되는가 | PARTIAL | 현재 값과 artifact는 일치하지만 상수가 JSON에서 생성되거나 test가 JSON/hash를 기계적으로 확인하지 않는다. 구현 review에도 기록했다. |
| 25 | 사전 비교 source/artifact가 고정됐는가 | PASS | Chrome observation, independent expected JSON, fixture, derivation script, screenshot의 기존 SHA256SUMS가 일치한다. |
| 26 | 규범 산술과 Chrome 역할이 분리됐는가 | PASS | §11.5.1/§11.6 계산이 stage expected source이고 Chrome은 final geometry control이다. |
| 27 | simulator 미실행 범위가 정당한가 | PASS | 순수 Rust trace test만 추가해 앱/runtime/simulator gate가 없음을 plan/evidence가 밝힌다. |
| 28 | test 실행이 정확한 source revision과 연결되는가 | PASS | base commit과 두 Rust source SHA-256, command 결과가 validation manifest에 기록됐다. |
| 29 | whitespace 검사에 신규 파일도 포함됐는가 | PASS | 최종 의도된 변경을 stage한 뒤 `git diff --cached --check`로 전체 staged diff를 검사했다. |
| 30 | report 링크 및 blocker 상태가 일치하는가 | PASS | plan의 구현 PR diff review 링크가 이 파일로 해소됐고 최종 결과는 FAIL 0이다. |

## 수정 후 재검토 기록

- **관점 9 PASS:** 고정 Columns early-return snapshot에서 CSS track 하나의 base size가 220px임을 assert한다.
- **관점 14 PASS:** 전체 Rows stage sequence를 exact list로 비교한다. 기존 Columns 테스트도 shared helper 변경 후 focused test에서 통과했다.
- **관점 28 PASS:** 실행 source hashes와 command summary는 `validation.json`에서 확인했다.
- **관점 29 PASS:** 모든 의도된 변경 및 이 보고서를 stage한 후 staged diff whitespace check를 실행했다.
- **관점 30 PASS:** 보고서 파일과 plan의 relative link가 함께 존재한다.

이 보고서는 독립 reviewer의 PR diff 결과를 고정한다. 구현 20관점 결과는 [별도 보고서](./implementation-review-20.md)에 있으며, precomparison source hash는 [기존 사전 비교 자료](../c11-4-row-axis-span-stage-precomparison-2026-10-27/SHA256SUMS)에 있다.
