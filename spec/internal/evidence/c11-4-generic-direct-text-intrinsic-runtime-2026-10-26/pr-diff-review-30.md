# C11.4 generic direct-text PR diff hostile review (30 views)

**역사적 판정:** 이 보고서는 원래 PR HEAD `bbd6fe42`만 검토했다. FAIL #3은 `f45596b8`에서 수정됐고, 별도 fix recheck는 [후속 보고서](./pr-diff-review-30-fix-recheck.md)에 기록한다. 이 파일은 첫 finding의 출처를 보존한다.

## 기준과 범위

- 비교 기준: origin/main 12741e4546c855d29967044a62025a57c74caf54 → PR #645 HEAD bbd6fe42f898da1a3597e7fad0af2de34c087f53 (708bee0 구현 + bbd6fe42 증거/문서).
- 소스 판정은 공유 worktree의 현재 파일이 아니라 git show bbd6fe42:<path>로 읽은 원래 PR 커밋을 기준으로 했다. 리뷰 중 worktree에 수정된 Rust 파일이 보였지만 그 변경은 이 판정에 섞지 않았다.
- Rust/runtime/FFI 구현, 내부 계약, 테스트와 커밋된 Android/iOS 실행 근거를 읽었다. 리뷰 중 테스트·빌드는 실행하지 않았다.
- 짧은 Rust source 경로는 crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_text/ 기준이다. 예외: fixtures/c11_4_text.rs는 runtime_layout/fixtures/c11_4_text.rs, runtime_layout.rs는 session/ua_cascade/runtime_layout.rs, c11_4_image.rs는 runtime_layout/fixtures/c11_4_image.rs를 가리킨다. FFI, Android, iOS와 문서 경로는 표에 적힌 repo-relative path다.

**결과: PASS 21 / PARTIAL 8 / FAIL 1.** 유일한 FAIL은 generic collector가 Grid 직속 이미지 sibling을 text-only 장면으로 수집하는 계약 경계다. 런타임 전체가 잘못된 layout을 반환한다고 확대해서 말하지 않는다. 아래에서 설명하듯 별도 image fixture handler가 그 입력을 별도 오류로 거부한다.

## 30개 관점

| # | 상태 | 관점과 근거 |
|---:|---|---|
| 1 | PASS | 기존 C11.4 전용 adapter를 먼저 실행하고 generic collector를 기존 root-wide text fallback 바로 앞에 둔다. Ok(None)과 오류의 경계가 보존된다. crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_text.rs:71-152, plan/c11-4-generic-direct-text-intrinsic-input.md:11-13,132-137. |
| 2 | PASS | collector는 scene/root/item ID를 dispatch key로 쓰지 않는다. 변경된 ID·content 장면과 최종 child-order 장면이 여섯 positive에 포함된다. generic_direct_text.rs:33-100, generic_direct_text_tests/cases.rs:76-293. |
| 3 | **FAIL (P2)** | Grid 직속 img는 의미 있는 Text descendant가 없어 candidate도 error도 되지 않는다. direct text item 두 개와 함께 img sibling을 두면 collector가 candidates.len()==2를 통과해 Some(C11_4TextInput)을 반환한다. validate_grid_root는 root track/box style만 검사하고 child cardinality/type은 검사하지 않는다. generic_direct_text.rs:33-67,148-220,273-297. 원래 negative test는 item 내부의 image만 검사하며 root sibling 사례가 없다: generic_direct_text_tests/negative.rs:45-58. end-to-end caller는 이후 prepare_c11_4_image도 호출하고, 알 수 없는 root ID를 unsupported_c114_image_fixture_root로 거부하므로 이 리뷰는 잘못된 layout이 최종 반환된다고 주장하지 않는다. 다만 collector의 허용/거부 계약이 어긋나며 이후 이미지 handler 변경 시 보호 테스트가 없다. 비교 계약: plan/c11-4-generic-direct-text-intrinsic-input.md:11,136; 별도 image handler: c11_4_image.rs:53-78; caller: runtime_layout.rs:460-480. |
| 4 | PASS | Grid root 바로 아래 의미 있는 anonymous Text는 item candidate로 추정하지 않고 명시 오류로 닫는다. generic_direct_text.rs:43-48, generic_direct_text_tests/negative.rs:99-115. |
| 5 | PASS | text owner 내부의 nested element/replaced child는 direct-source 검사에서 거부된다. generic_direct_text.rs:254-281, generic_direct_text_tests/negative.rs:26-58. |
| 6 | PASS | 하나의 item이 둘 이상의 의미 있는 direct Text node를 소유하면 정확히 하나 조건에서 거부된다. generic_direct_text.rs:254-281, generic_direct_text_tests/negative.rs:18-24. |
| 7 | PASS | text item 후보가 세 개 이상이면 두 번째 이후 후보를 고르지 않고 오류를 반환한다. generic_direct_text.rs:61-69, generic_direct_text_tests/negative.rs:74-97. |
| 8 | PASS | SVG 등 foreign namespace element를 text item으로 통과시키지 않는다. generic_direct_text.rs:233-252, generic_direct_text_tests/negative.rs:60-73. |
| 9 | PASS | 수집 text를 ASCII 대문자와 U+0020으로 제한해 이 fixture의 oracle 밖 문자 shaping을 주장하지 않는다. generic_direct_text.rs:78-96; 여섯 장면 기대값은 generic_direct_text_tests/cases.rs:76-293. |
| 10 | PARTIAL | 구현은 style profile와 HostDocument generation/document/render-tree revision을 함께 대조한다. 다만 negative test는 다른 snapshot generation 한 가지만 바꾸며 document/render-tree revision별 불일치는 따로 만들지 않는다. 구현 generic_direct_text.rs:114-145, 시험 generic_direct_text_tests/negative.rs:269-284. |
| 11 | PASS | computed-style owner 누락과 중복을 허용하지 않고, 해당 node의 단일 ComputedElementStyle을 요구한다. generic_direct_text.rs:398-420. |
| 12 | PASS | root가 HTML element이고 computed display:grid여야 진행한다. generic_direct_text.rs:28-31,148-168. |
| 13 | PARTIAL | root에 정확한 [min-content,max-content] columns, auto row, row auto-flow, no auto-repeat/template areas/named lines를 요구한다. 구현은 닫혀 있으나 negative matrix는 다른 explicit tracks만 변형하고 repeat/areas/line-name 각각의 control은 확인되지 않는다. generic_direct_text.rs:171-220, generic_direct_text_tests/negative.rs:187-219. |
| 14 | PASS | root alignment, direction과 writing-mode/orientation을 명시 검사하고 변형 시 오류를 확인한다. generic_direct_text.rs:156-179, generic_direct_text_tests/negative.rs:200-220. |
| 15 | PASS | root width/height/min/max/aspect-ratio/position, gap, margin/padding/border 경계를 typed computed style로 제한한다. generic_direct_text.rs:199-228. |
| 16 | PASS | text item은 block/static/order 0/align-self auto/justify-self auto를 요구한다. generic_direct_text.rs:303-309; negative.rs:151-185는 일부 비허용 값을 거부한다. |
| 17 | PASS | item width/height/min/max, aspect ratio, margin/padding/border를 지원 profile로 제한한다. generic_direct_text.rs:350-368, generic_direct_text_tests/negative.rs:160-185. |
| 18 | PASS | item Grid placement의 네 축을 모두 auto로 제한한다. generic_direct_text.rs:369-379; non-auto column placement negative는 negative.rs:163-165에 있다. |
| 19 | PASS | font family/size/weight/style/line-height를 exact profile 값으로 확인한 뒤 ParagraphStyle을 생성한다. generic_direct_text.rs:311-326,381-405; positive 값 검사는 positive.rs:69-95. |
| 20 | PASS | text-wrap-mode를 normal/nowrap으로, white-space-collapse는 collapse로 검사한다. nowrap과 whitespace 장면이 따로 있다. generic_direct_text.rs:316-343, cases.rs:113-147,221-255. |
| 21 | PASS | word-break normal/break-all만 매핑하고 overflow-wrap 및 text-transform 등 영향 속성을 제한한다. generic_direct_text.rs:318-325,345-349; break-all positive는 cases.rs:149-220,257-292. |
| 22 | PASS | text item의 직접 Text node ID와 원문을 보존하고 raw whitespace를 임의 사전 정규화하지 않는다. generic_direct_text.rs:233-281; 최종 TextLayoutSnapshot owner/source/text 검사는 positive.rs:259-277, " A  BC " 기대값은 cases.rs:221-255. |
| 23 | PASS | 시스템 font 없는 engine에 저장소 Ahem을 등록하고 raw intrinsic size와 contribution의 font-registry identity를 비교한다. generic_direct_text.rs:99-122, positive.rs:137-225, 독립 engine의 숫자 revision 충돌 control은 negative.rs:305-337. |
| 24 | PASS | 여섯 Rust positive가 computed profile, raw/contribution intrinsic metrics, frames, 최종 C15 text layout과 line geometry를 검사한다. positive.rs:16-123,124-290; 여섯 고정 기대값은 cases.rs:76-293. |
| 25 | PARTIAL | Rust six-scene positive는 모든 node의 모든 축 probe 집합을 완전 고정하지 않는다. item당 measurement 존재/동일 revision을 검사하고 첫 item의 width MinContent 또는 둘째의 width MaxContent 선택 probe만 확인한다. canonical app matcher가 더 넓은 축 집합을 확인하지만 나머지 다섯 장면에는 이 검사가 없다. positive.rs:227-257, 계획의 분리 경계 plan/c11-4-generic-direct-text-intrinsic-input.md:13,135. |
| 26 | PARTIAL | font-variant-ligatures는 computed property collector 목록에 없고 ParagraphStyle.ligatures는 Normal로 고정된다. 거부 control은 inline declaration preflight만 확인한다. 계획도 제한 stylesheet declaration에 대한 거부를 주장하지 않으므로 현재 명시 범위 밖의 한계이며 blocker로 확대하지 않는다. crates/spinon-style/src/stylo_dom/cascade/runtime_grid.rs:155-177,296-325, negative.rs:223-247, 계획 plan/c11-4-generic-direct-text-intrinsic-input.md:136. |
| 27 | PARTIAL | one-item/non-grid 경계에서 collector가 None을 반환하는지는 시험하지만, 기존 special adapter 이후 root-wide TEXT_CASES fallback까지 포함해 13개 legacy scene의 전체 route 성공을 재검증하지 않는다. 계획과 evidence도 13개 runtime fallback을 입증하지 않는다고 명시한다. negative.rs:117-149, fixtures/c11_4_text.rs:128-155, plan/c11-4-generic-direct-text-intrinsic-input.md:137. |
| 28 | PARTIAL | FFI에는 canonical generic-root-normal scene 하나만 등록되고 Android/iOS feature allowlist도 같은 장면으로 제한된다. FFI unit test는 실행 대신 source substring/count를 검사한다. Android/iOS pass02 runtime evidence가 실제 이 positive를 보완하지만 나머지 다섯 positive 및 negatives의 앱 경로는 범위 밖이다. crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/generic_direct_text.rs:1-57, Java C0410RuntimeGpuDemo.java:130,641-645, Swift C0410RuntimeGpuDemo.swift:84-88, evidence README:5-7. |
| 29 | PARTIAL | Android/iOS matcher는 intrinsic source/metric과 node별 probe set을 검사하지만 availableSpace를 Debug 문자열 substring으로 분류하고 measured width/height는 finite·nonnegative만 확인한다. 따라서 축 probe의 typed 구조 및 callback numeric size는 정확히 고정되지 않는다. Android C0410RuntimeGpuDemo.java:2417-2463, iOS C0410RuntimeGpuDemo.swift:3096-3131. Android JSONTokener도 trailing 입력 완료 여부를 검사하지 않고, iOS parser는 첫 balanced object 뒤 payload를 무시한다: Java 2370-2375, Swift 2623-2659. |
| 30 | PARTIAL | trace matcher는 실제 columns/rows contribution과 track 결과를 비교한다. 그러나 Android는 마지막 stage track만 검사하고 sequence/phase/autoRepeatCount/percentageBasis를 고정하지 않는다. iOS는 모든 stage 이름/track 크기를 더 확인하지만 두 matcher 모두 infinitelyGrowable가 Boolean인지에만 의존해 예상 false를 요구하지 않는다. Android C0410RuntimeGpuDemo.java:2503-2572; iOS C0410RuntimeGpuDemo.swift:3166-3242. |

## 실행 근거와 남은 경계

- 커밋된 pass02는 source-head.txt에 구현 commit 708bee0cae9b16872ab3a4d5da9d2f67dc8e29bd를 기록하고 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 build, status=0, layout=ready, 3 boxes, trace 및 draw/present 성공을 보고한다. bbd6fe42는 해당 구현 뒤에 실행 근거/문서를 더한 커밋이다. README에는 APK/app/screenshot와 Java/Swift matcher hash가 기록되어 있다: evidence README:11-29.
- 이 화면은 text glyph 표시나 pixel equality를 입증하지 않는다. 두 runtime log 모두 runtime_text_count=0, glyph_batch_draw_count=0이다. C11.4 전체/Stage A, 13개 legacy runtime fallback, Release APK/IPA에서 fixture 코드 배제, 제품 CSS admission도 이 PR evidence가 입증하지 않는다. README:5-8,31-42,53-55.
- Rust 9/9와 FFI 1/1로 기록된 pass01 로그의 source-head.txt는 base 12741e45다. dirty-source fingerprint가 없어 해당 실행을 현재 PR source commit에 결속된 증거로 세지 않았다. pass02 app build/runtime은 source commit을 기록한다. pass01 로그 경계는 runs/c114-generic-direct-text-20261026T023012Z-12741e45-pass01/source-head.txt 및 README:44-53.
- 공식 상태는 C11.4 미완료, 72/213으로 유지되어 있으며 이 PR만으로 상위 완료를 주장할 수 없다. plan/c11-4-generic-direct-text-intrinsic-input.md:5,146-148, spec/internal/0059-c11-grid-layout.md:552-562.

