# C11.4 intrinsic measure callback 구현 적대 검토 20개

**날짜:** 2026-10-19
**범위:** `LayoutOutput.intrinsic_measurements` callback trace 후보와 관련 Rust 입력 경계
**판정:** 이 조각은 측정 callback의 출처를 보존한다. CSS contribution 계산, track sizing 또는 C11.4 완료로 승격하지 않는다.

각 관점은 서로 다른 실패 경로를 확인했다. “통과”는 이 조각의 명시된 범위 안에서만 사용한다.

| # | 적대 관점 | 확인 결과와 조치 |
|---:|---|---|
| 1 | 계산 출력이 image resource 교체 전 revision으로 찍히는가 | 출력 revision에 `LayoutIntrinsicSizeSnapshot` revision을 먼저 반영한 뒤 trace에 stamp한다. image regression이 trace와 output의 두 revision을 모두 비교한다. 통과. |
| 2 | text measure가 등록 뒤 바뀐 font registry revision을 잃는가 | definite measure는 `TextLayoutSnapshot`, intrinsic measure는 `TextIntrinsicInlineSizes`의 font revision을 전달한다. Ahem 테스트에서 revision `1`을 확인한다. 통과. |
| 3 | text/replaced source가 서로 혼동되거나 자연 크기를 잃는가 | source variant를 분리하고 replaced 입력의 natural size와 item `LayoutStyle`을 기록한다. image test가 `24×16 CSS px` source와 output revision을 확인한다. 통과. |
| 4 | available space의 가로·세로 축이 뒤바뀌거나 한쪽이 유실되는가 | Taffy `Size<AvailableSpace>`의 두 축을 각각 저장한다. text fixture에서 width `100px`, height `10px` definite 값을 확인한다. 통과. |
| 5 | callback의 min/max 제약이 definite 값으로 뭉개지는가 | `MinContent`, `MaxContent`, `Definite(px)`를 별도 variant로 저장한다. 테스트에서 definite callback을 기대값으로 고정한다. 통과. |
| 6 | known dimensions와 parent content-box 크기를 측정 결과에서 역산하는가 | callback 입력값을 그대로 복사한다. text fixture가 known dimensions와 parent `100×10px`를 별도 assertion한다. 통과. |
| 7 | content box와 border box가 같은 필드로 오해되는가 | measure callback이 돌려준 content size만 `content_size`에 저장한다. replaced path는 `resolve_used_size`의 content width/height를 사용하며 문서에 box 경계를 명시했다. 통과. |
| 8 | text item의 padding/border/box-sizing 정보를 contribution 후처리 전에 잃는가 | 모든 trace가 직접 item의 `item_style`을 포함한다. text와 image 테스트가 입력 style과 전체 equality를 확인한다. 외곽 contribution 계산은 아직 하지 않는다. |
| 9 | non-Grid Block/Flex measure가 Grid trace로 섞이는가 | 계산 tree의 direct in-flow Grid child membership만 기록 대상이다. 기존 non-Grid text regression에서 trace가 비어 있음을 확인한다. 통과. |
| 10 | absolute/fixed child가 reparent 뒤 Grid item으로 오인되는가 | Grid membership을 visual tree의 reparent 이후 만들고 Absolute/Fixed를 제외한다. 현재 positioned Grid item은 기존 validator가 fail closed하므로 해당 지원을 주장하지 않는다. |
| 11 | `display:none` subtree가 zero-size callback 기록으로 남는가 | hidden layout은 measure callback보다 먼저 종료되고, trace는 callback에서만 생성된다. 숨김 item 지원으로 확대 해석하지 않는다. 통과 범위 제한. |
| 12 | static-position용 flow-only 두 번째 계산이 trace를 중복 기록하거나 불필요한 OOM을 일으키는가 | `record_intrinsic_measurements = !flow_only`로 보조 tree의 기록과 예약을 끈다. 실제 layout output은 visual tree의 trace만 사용한다. 통과. |
| 13 | Taffy cache가 callback을 생략했는데 trace가 완전한 contribution 목록으로 오해되는가 | cache에 의해 callback이 없을 수 있음을 계약에 적었다. trace를 callback 관찰값으로만 부르고 canonical contribution set으로 취급하지 않는다. 통과. |
| 14 | 같은 node가 여러 sizing stage에서 측정될 때 뒤 호출이 앞 호출을 덮어쓰는가 | map으로 축약하지 않고 callback마다 Vec 끝에 추가한다. 중복과 호출 순서를 유지한다. 통과. |
| 15 | Grid container 안 anonymous text가 외부 child `NodeId`처럼 잘못 매핑되는가 | 현재 membership model은 anonymous Grid text item을 기록하지 않는다. 이를 0059, C11.4 plan, status와 conformance의 미완료 범위에 명시했다. 후속 단계에서 synthetic item identity가 필요하다. 미완료. |
| 16 | nested non-leaf Grid item의 descendant text callback이 item aggregate 결과인 것처럼 잘못 기록되는가 | 현재 조각은 direct item leaf callback만 기록한다. nested non-leaf aggregate와 descendant provenance를 지원한다고 주장하지 않으며 계획과 상태에 미완료로 남겼다. 미완료. |
| 17 | callback trace `Vec` 예약 실패가 panic 또는 기본 크기 대체로 숨겨지는가 | `try_reserve(1)` 실패를 node가 포함된 `IntrinsicMeasurementAllocationFailure`로 저장한다. allocator failpoint가 실제 예약 실패를 관찰했다. 통과. |
| 18 | trace 예약 실패 뒤 부분 LayoutOutput 또는 재사용 불가 tree가 남는가 | 실패 뒤 `compute_tree`가 calc error를 반환해 output을 게시하지 않는다. 같은 engine의 다음 계산이 trace를 포함해 정상 복구하는 테스트를 추가했다. 최종 revision은 기존 trace Vec에서 제자리 갱신해 두 번째 할당도 피한다. 통과. |
| 19 | callback 수가 item 수를 넘을 때 메모리 사용량 상한이 있는가 | callback마다 fallible reserve를 하지만 별도 trace 항목 수 cap은 없다. 현재 앱 runtime 경로에는 연결하지 않았고, 이 trace를 sizing algorithm 입력으로 승격하기 전 callback 상한 또는 별도 budget을 정해야 한다. 후속 위험으로 기록. |
| 20 | 새 `LayoutOutput` 필드와 C15 text style 전달 변경이 앱 공개 API로 잘못 읽히는가 | `spinon-layout` Rust 내부 계약 0059와 C15 계약 0070을 갱신했다. `@spinon` API, FFI, Android/iOS runtime에 연결되지 않았고 workspace 호출자 전체가 compile/test된다. 외부 Rust semver 안정성을 보장하지 않는 내부 0.1.0 경계다. 통과 범위 제한. |

## 실행 근거와 미해결 게이트

- `mise exec -- cargo test --locked --workspace`: 1,096 passed, 4 ignored, 0 failed. `spinon-layout` 158개 포함.
- `mise exec -- cargo clippy --locked -p spinon-layout -p spinon-style-to-layout --no-deps --all-targets -- -D warnings`: 통과.
- `mise exec -- cargo clippy --locked --workspace --all-targets -- -D warnings`: 미통과. 변경하지 않은 `crates/spinon-style/src/stylo_dom/cascade/incremental/compute.rs:43`의 기존 `clippy::too_many_arguments` 경고가 `-D warnings`에서 오류가 된다.
- `mise exec -- bun run test:css-reference`: 559 passed, 4 failed. C04 runtime viewport, C11.3.2 Android/iOS scene entry, C13.3 iOS tap route의 기존 source/test mismatch다. 이 변경은 해당 파일을 수정하지 않았다. 전체 `bun run test`도 이 단계에서 중단됐고 Rust workspace 전체는 위 별도 명령으로 실행했다.
- Android/iOS Simulator scene, surface, artifact 경계는 이 Rust 내부 trace 변경이 app path에 연결되지 않아 실행하지 않았다. Agent Browser와 Terminal Browser를 사용하지 않았다.

이 검토는 단계 A의 관찰 trace만 검토했다. automatic minimum, item span, track stage, auto-repeat, WPT 및 모바일 runtime 관문은 계속 남아 있다.
