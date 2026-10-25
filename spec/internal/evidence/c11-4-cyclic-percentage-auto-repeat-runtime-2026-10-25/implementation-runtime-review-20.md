# C11.4 cyclic × auto-repeat implementation/runtime hostile review

- 검토 대상 source revision: `c5b8ef75804e39d36a37aa61b82b1f028115fefd`
- 방식: 별도 reviewer의 read-only source 및 evidence 검토
- 초기 결과: 19 PASS / 1 PARTIAL / 0 FAIL.
- Follow-up 이후 최종 결과: **20 PASS / 0 PARTIAL / 0 FAIL**.
- 검토 중 빌드, 테스트, simulator, browser 실행: 없음
- PARTIAL은 동작 문제가 아닌 evidence provenance/persistence 문제 (#20)다. Review response와 후속 증거 고정 내용은 아래 follow-up에 연결한다.

| # | 독립 실패 관점 | 초기 판정 | 관찰 근거 |
|---:|---|---|---|
| 1 | Default product admission이 바뀌지 않는가 | PASS | Fixture exception 및 Taffy provenance fields는 `c11-4-grid-track-sizing-fixture-test` cfg로 보호 (`crates/spinon-layout/Cargo.toml:7-8`, `grid.rs:826-838`, `vendor/taffy/src/compute/grid/mod.rs:255-260`). Default archive에는 marker가 없음. |
| 2 | Root cyclic exception이 전체 style equality로 제한되는가 | PASS | root `LayoutStyle` 전체 equality 검사 (`crates/spinon-layout/src/grid.rs:871-896`); height/aspect/direction 변형 거부. |
| 3 | Item style도 전체 equality로 제한되는가 | PASS | 24×16 item을 포함한 전체 `LayoutStyle` 검사 (`grid.rs:886-893,920-921`). |
| 4 | Auto-repeat grammar/profile이 target에 고정되는가 | PASS | 정확히 한 `auto-fill` fragment와 `minmax(0px,50%)`, 허용 track 범위 및 line names 검사 (`grid.rs:895-912`). |
| 5 | Placement 전제가 고정되는가 | PASS | root-only 2-node tree, child 하나, auto rows, 기본 implicit tracks, areas/placement overrides 제한 (`grid.rs:857-927`). |
| 6 | 인접 track 입력이 exception을 재사용하지 않는가 | PASS | `40%`, `minmax(10%,50%)`, `auto-fit` 변형을 `invalid_grid_track_list`로 거부 (`runtime_layout_tests/c11_grid_tests/cyclic_percentage_auto_repeat.rs:252-265`). |
| 7 | 추가 in-flow item이 통과하지 않는가 | PASS | extra child는 거부; vendor 주석도 정확 scene 경계 설명 (`cyclic_percentage_auto_repeat.rs:257-265`, `vendor/taffy/src/compute/grid/mod.rs:681-695`). |
| 8 | Root style 경계 mutation이 차단되는가 | PASS | height/aspect/direction은 `invalid_grid_track_list`, overflow는 cascade `unsupported_inline_property` (`cyclic_percentage_auto_repeat.rs:267-282`). |
| 9 | Feature-off 일반 경로가 여전히 target을 거부하는가 | PASS | feature-off test가 `invalid_grid_track_list` 확인 (`cyclic_percentage_auto_repeat.rs:285-292`, `runtime-feature-off-control.log`). |
| 10 | 첫 sizing pass의 phase/count/basis가 보이는가 | PASS | 실제 `track_sizing_algorithm` call에 repeat count/context를 전달 (`vendor/taffy/src/compute/grid/mod.rs:478-504`); runtime trace는 `IntrinsicMaxContent`, count 1, basis null을 기록. |
| 11 | 첫 pass item contribution 24px이 실제 값인가 | PASS | target trace의 `MaxContent=24.0`, intrinsic stage에서 track 24 기록. |
| 12 | Final cyclic percentage sizing의 count/basis/tracks가 맞는가 | PASS | count 2, basis24, used columns 12+12; host assertions와 Android/iOS logs가 일치. |
| 13 | Definite control이 regression 기준을 제공하는가 | PASS | count2, basis40, tracks20+20; intrinsic re-expansion 없음. |
| 14 | Trace가 실제 Taffy callsite에서 관찰되는가 | PASS | 실제 pass context가 basis/count 기록 (`vendor/taffy/src/compute/grid/track_sizing.rs:361-365`, `compute/grid/mod.rs:1542-1571`), Spinon diagnostics/report까지 보존 (`calc_tree.rs:579-604`, `trace_report.rs:221-229`). |
| 15 | Phase ambiguity/trace allocation failure가 fail-closed인가 | PASS | phase 제거 host test는 `MissingSemanticPhase`; allocation/limit failure는 layout error로 전파되어 중단. |
| 16 | Trace가 최신 runtime revision에 묶이는가 | PASS | FFI reporter가 completed key 및 source/style/environment/intrinsic revisions를 확인 (`trace_report.rs:18-45`); simulator logs도 document revision/generation을 남김. |
| 17 | Focused validations가 있는가 | PASS | runtime grid 25/25, feature-off 1/1, FFI C11.4 72/72, runtime/FFI clippy 통과 로그. |
| 18 | Android adapter가 phase/count/basis/tracks와 frame을 fail-closed 검증하는가 | PASS | adapter matcher/acceptance assertions 및 target/control frame/trace log 확인 (`C0410RuntimeGpuDemo.java:815-899`, Android logs). |
| 19 | iOS adapter가 동일 acceptance를 적용하는가 | PASS | Swift matcher/assertion 및 target/control frame/trace log 확인 (`C0410RuntimeGpuDemo.swift:2505-2568`, iOS logs). |
| 20 | Simulator pixel 증거와 source SHA가 durable하게 연결되는가 | PARTIAL | 네 target/control run의 Android PixelCopy/iOS Metal 각각 3/3 match 및 screenshot은 확인. 당시 evidence 폴더가 untracked이고 README가 없어 캡처/build hash와 full source SHA의 durable link가 없었음. |

## Follow-up: #20 evidence provenance

초기 review에서 #20은 README/캡처/log/build hash를 고정된 source revision에 영구적으로 연결하는 근거가 미흡하다고 판정했다. Evidence commit `ea06d8ff86729d582b4821554cf87d22170aefc0`에 README, 4장 simulator screenshot, 원본 log/sample 자료와 APK/iOS executable/static archive SHA를 추가했다. 독립 reviewer가 해당 commit을 재검토해 #20을 PASS로 변경했다.

- 후속 판정: **PASS — #20 해소**.
- reviewer는 evidence commit의 parent가 정확한 source commit `c5b8ef75804e39d36a37aa61b82b1f028115fefd`임을 확인했고, README의 full SHA 및 Android/iOS executable SHA-256이 빌드 출력 재계산값과 일치함을 확인했다. 31개 evidence 파일이 commit tree에 포함되고 worktree clean이었으며, four run IDs, logs, pixel samples, archive markers/hash/size도 README와 맞았다. Reviewer는 read-only로 확인했으며 추가 테스트·빌드·시뮬레이터 실행은 하지 않았다.
