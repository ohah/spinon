# C11.4 cyclic × auto-repeat implementation/runtime hostile review

- 검토 대상 source revision: `c5b8ef75804e39d36a37aa61b82b1f028115fefd`
- 방식: 별도 reviewer의 read-only source 및 evidence 검토
- 초기 결과: 19 PASS / 1 PARTIAL / 0 FAIL.
- Evidence provenance follow-up 이후 최종 결과: **20 PASS / 0 PARTIAL / 0 FAIL**.
- 검토 중 빌드, 테스트, simulator, browser 실행: 없음
- 초기 PARTIAL은 동작 문제가 아닌 evidence provenance/persistence 문제 (#20)였으며, 아래 follow-up에서 해소 근거를 기록한다.

| # | 독립 실패 관점 | 최종 판정 | 관찰 근거 |
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
| 20 | Simulator pixel 증거와 source SHA가 durable하게 연결되는가 | PASS | Evidence commit `ea06d8ff86729d582b4821554cf87d22170aefc0`의 parent는 target source SHA `c5b8ef75804e39d36a37aa61b82b1f028115fefd`다. README는 full SHA, 네 run ID, APK/iOS executable SHA와 archive SHA·크기·marker 결과를 함께 기록한다 (`README.md:3-10,41-59,70-106`). 후속 독립 reviewer는 당시 보유한 Android APK와 iOS executable에서 SHA를 재계산해 README 값과 일치함을 확인했다. 두 앱 바이너리 자체는 evidence commit tree에 없으므로, 현재 commit만으로 이 두 앱 SHA를 다시 계산할 수 있다고 주장하지 않는다. README, build/runtime logs, sample JSON/base64, PixelCopy·Simulator 캡처, archive logs, review files 31개는 evidence commit tree에 포함되고 worktree도 clean이었다. |

## Follow-up: #20 evidence provenance

초기 review에서 #20은 README/캡처/log/build hash를 고정된 source revision에 영구적으로 연결하는 근거가 미흡하다고 판정했다. Evidence commit `ea06d8ff86729d582b4821554cf87d22170aefc0`에 README, 4장 simulator screenshot, 원본 log/sample 자료와 APK/iOS executable/static archive SHA를 추가했다. 별도 reviewer는 해당 commit parent와 source SHA의 일치, 당시 보유했던 앱 산출물의 재계산 SHA, 저장된 simulator·archive 자료를 read-only로 확인해 #20을 PASS로 변경했다.

- 후속 판정: **PASS — #20 해소**.
- 검증한 네 run ID는 `cyclic-android-c5b8ef75`, `definite-android-c5b8ef75`, `cyclic-ios-c5b8ef75`, `definite-ios-c5b8ef75`이며, 각각 README의 full source SHA prefix와 일치하고 원본 로그에서 trace/frame/pixel acceptance를 확인할 수 있다. Android PixelCopy 및 iOS Metal target/control samples는 각 3/3 matched다.
- Android debug APK와 iOS Simulator executable의 바이트는 evidence commit에 저장되지 않았다. Commit에서 재확인할 수 있는 것은 README에 고정한 digest와 연결된 run/log/sample/screenshot이며, 앱 digest 재계산은 후속 reviewer가 당시 보유한 빌드 산출물로 수행한 역사적 확인이다. 이 기록은 바이너리 자체의 보존·재생성을 주장하지 않는다.
- Default static archive는 SHA `2369089af5a46cc7ead1009ec807bda36e9359550d6f0babccf7b4a1d4f3f30a`, 60,233,880 bytes, `fixture_markers=absent`; positive fixture archive는 SHA `479209a7250109b7bad8bb85984069981a94fb30ac6d9c44e5bf06d0d06722a2`, 78,356,464 bytes이며 cyclic target/control symbol과 payload marker가 존재한다. 둘 다 README와 committed logs로 대조했다. Static archive 결과를 release APK/IPA 제외 증거로 확대하지 않는 README 한계도 유지되어 있다.
- Review는 read-only였으며 추가 테스트·빌드·시뮬레이터·브라우저 실행은 하지 않았다.
