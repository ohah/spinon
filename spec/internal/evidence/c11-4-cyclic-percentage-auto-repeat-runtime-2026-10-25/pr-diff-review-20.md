# C11.4 cyclic × auto-repeat PR diff review

- 대상 revision: `c5b8ef75804e39d36a37aa61b82b1f028115fefd`
- 검토 방식: 독립 reviewer의 read-only 변경 diff 및 관련 경로 대조
- 결과: **20 PASS / 0 PARTIAL / 0 FAIL**
- 추가 테스트 실행: 없음 (구현 담당자의 테스트와 시뮬레이터 근거는 runtime evidence 별도 파일 참조)

| # | 검토 관점 | 판정 | 증거 / 결론 |
|---:|---|---|---|
| 1 | 기본 빌드에서 fixture phase/count/basis 계측이 빠지는가 | PASS | 변경된 trace·pass 경로는 `c11-4-grid-track-sizing-fixture-test` opt-in으로 제한됨. |
| 2 | Cargo feature 전파가 FFI→runtime→layout→Taffy 방향으로 일치하는가 | PASS | 각 Cargo.toml의 opt-in feature 관계가 연결되어 있음. |
| 3 | `c04-runtime-gpu` 기본 feature에 cyclic fixture 예외가 섞이는가 | PASS | fixture-test feature 없는 runtime 경로는 예외를 활성화하지 않음. |
| 4 | FFI scene selector가 두 고정 ID 외 값을 허용하는가 | PASS | `cyclic-max-content-auto-fill`, `definite-40px-auto-fill-control`만 dispatch; 오타/미등록 ID는 오류. |
| 5 | target CSS가 명시된 max-content 및 auto-fill track 패턴과 일치하는가 | PASS | target은 `width:max-content`, `repeat(auto-fill,minmax(0px,50%))`, `24×16` child 한 개. |
| 6 | definite control이 target 외 입력을 함께 바꾸는가 | PASS | control은 width 40px만 달라짐. |
| 7 | exact scene tree 및 item placement가 고정됐는가 | PASS | root와 item 식별자·순서·단일 direct child·placement가 검증됨. |
| 8 | root `LayoutStyle` 전체가 exact allowlist 되는가 | PASS | c5b8ef7 보정 후 허용된 fixture style 전체와 비교. |
| 9 | item `LayoutStyle` 전체가 exact allowlist 되는가 | PASS | c5b8ef7 보정 후 크기·정렬 등 전체 style 비교. |
| 10 | track list, auto-fill, rows, areas 등 범위 밖 구조가 걸러지는가 | PASS | 허용된 pattern 외 track/profile은 거부됨. |
| 11 | style mutation controls가 거부되는가 | PASS | height·overflow·aspect-ratio·direction 등 추가 변형은 소유 계층의 오류로 실패. |
| 12 | 인접 width/profile 변형이 실수로 허용되는가 | PASS | 주변 폭, auto-fit, 추가 item 등의 변형은 gate에서 거부됨. |
| 13 | feature-off에서 cyclic scene을 거부하는가 | PASS | feature-off control test 통과. |
| 14 | phase provenance가 실제 Taffy sizing callsite에서 오는가 | PASS | fixture opt-in trace hook이 실제 sizing 호출에서 semantic phase를 붙임. |
| 15 | intrinsic pass가 count 1, basis 없음, item max-content 24px을 기록하는가 | PASS | runtime trace에 각 값이 별도로 남음. |
| 16 | final pass가 count 2, basis 24px, tracks `[12,12]`를 기록하는가 | PASS | target runtime trace가 규범 기대와 일치. |
| 17 | auto-repeat count/resolver metadata가 최종 pass에서 갱신되는가 | PASS | final pass metadata가 재계산된 2-track 결과를 기록. |
| 18 | final pass가 track initialization 및 sizing을 실제 재실행하는가 | PASS | Taffy code path 및 trace stages에 재실행이 보임. |
| 19 | missing/stale/phase-ambiguous diagnostics가 성공 report로 통과하는가 | PASS | report/fixture gate가 누락·revision mismatch·semantic phase 부재를 fail-closed 처리. |
| 20 | Android/iOS trace·frame·pixel evidence가 final source와 연결되는가 | PASS | 4 simulator run 모두 c5b8ef7에 귀속; target/control의 frame, trace, pixel sample 통과 로그는 README에 연결. |

### 발견 후 보정

첫 diff review에서 exact-style 검사 범위가 좁다는 PARTIAL 1건을 발견했다. c5b8ef7에서 root와 item의 전체 `LayoutStyle`을 fixture의 기대 style과 비교하도록 보정한 뒤 독립 reviewer가 해당 부분을 재검토했고, 나머지 19 관점 포함 **20 PASS / 0 PARTIAL / 0 FAIL**을 확인했다. 수정 후 Android/iOS target/control 장면을 같은 revision에서 다시 빌드하고 실행했다.

