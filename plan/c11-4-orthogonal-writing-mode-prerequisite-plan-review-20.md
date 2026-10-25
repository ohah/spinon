# C11.4 orthogonal writing-mode 선행 계획 실패 관점 검토 20개

- 대상: `plan/c11-4-orthogonal-writing-mode-prerequisite.md` 초안 및 보완본.
- 초기 검토 방법: 이전 초안, 당시 소스, orthogonal 사전 비교를 읽었다. 빌드·테스트·브라우저·시뮬레이터는 실행하지 않았다.
- 최종 재검토 기준: PR #626, base `f197239b0168e2356f4d0ece03f5d2e4c40278fe`, 계획 원문 검토 head `c19af8e81f4e569cbffd42a925b6153eaccfbbc4`. 계획 파일 blob은 `f0615bb452f4b2d6cbfd3f4e330f3ef55011cdd6`이다.
- 최종 재검토 방법: 위 계획 blob을 현재 소스, W3C CSS spec, fixture evidence 및 `spinon-runtime`의 실제 literal 목록과 대조했다. runtime에서는 22개 literal/21개 파일(생산 경로 2, C11.4 fixture 20)을 확인했다. 빌드·테스트·브라우저·시뮬레이터는 실행하지 않았다.
- 초기 판정 표의 실제 합계는 **12 PASS / 8 PARTIAL / 0 FAIL**이다. 당시 초안의 `13/7` 요약은 합산 오류였다. 표는 초기 초안에 대한 역사 기록이며 현재 계획의 최종 판정으로 사용하지 않는다.
- 최종 독립 재검토: **20 PASS / 0 PARTIAL / 0 FAIL**. 이 결과는 계획 준비도만 판정하며 코드 구현·Taffy feasibility·C11.4 완료를 의미하지 않는다.

## 초기 초안의 역사적 검토 — 12 PASS / 8 PARTIAL

아래 행별 verdict와 계획 줄 인용은 보완 전 초안 검토 당시의 기록이다. 이전 줄 인용은 현재 계획의 근거가 아니며, 최종 재검토 근거는 뒤의 현재 head 기준 20행 표를 따른다.

| # | 실패 관점 | 판정 | 심각도 | 근거와 필요한 보완 |
|---:|---|---|---|---|
| 1 | 제안 범위가 기존 C11.4 orthogonal 후보의 정확한 차이를 재정의하거나 넓히는가 | PASS | — | 기존 plan은 horizontal parent 안 `vertical-rl` direct text와 natural PNG 네 장면에서 row `[50]`/`[80]px`를 구별한다. 새 초안은 같은 scene을 유일한 최종 입력으로 지정하고 다른 writing mode·formatting context를 제외한다. `plan/c11-4-orthogonal-row-intrinsic-contribution.md:13–25`; 신규 plan `:22–36,132`. |
| 2 | blocker의 코드 위치가 실제 Stylo→text/layout 경계와 맞는가 | PASS | — | Stylo C11.4 property 목록은 `writing-mode`를 수집하지만 `LayoutStyle`과 `ParagraphStyle`에는 해당 필드가 없다. Intrinsic input은 현재 text size의 물리 width/height를 각각 기여 축으로 사용한다. `runtime_grid.rs:147–168`, `spinon-layout/src/style.rs:334–371`, `spinon-text-layout/src/model.rs:126–138`, `spinon-layout/src/intrinsic_measurement.rs:193–241`. |
| 3 | CSS 전용 공통 enum을 `spinon-core`에 소유시키는 이유와 경계가 결정됐는가 | PARTIAL | P1 | 초안은 두 소비 crate가 `spinon-core`에 의존한다는 점만으로 `spinon-core/src/writing.rs`를 제안하지만, 현재 core는 DOM·revision·ID 중심의 모듈이다(`spinon-core/src/lib.rs:1–17`). CSS 값이 core 책임에 들어가는 근거가 아직 없다. 소유자를 plan PR 전에 선택하고, core를 택하면 의존성 방향·책임 근거를 적는다. |
| 4 | `WritingMode`/`TextOrientation`의 allowed variant와 silent default 방지가 구체적인가 | PARTIAL | P1 | `HorizontalTb`, `VerticalRl`, `Mixed`만 표현하고 나머지를 거부한다고 하지만 type conversion error의 소유자·오류 식별자와 `Default` 구현 여부는 정하지 않았다. `LayoutStyle`/`ParagraphStyle` 호출자가 누락값을 horizontal로 채우지 않도록 no-default 또는 명시 constructor 정책을 계획에 추가한다. `plan/c11-4-orthogonal-writing-mode-prerequisite.md:38–52,57–67`. |
| 5 | `writing-mode`와 `text-orientation`의 cascade·inheritance 입력이 명확한가 | PARTIAL | P1 | Root `horizontal-tb`, text `vertical-rl`, image inherited `horizontal-tb`, `Mixed`라는 scene 값은 적혀 있으나, `text-orientation`의 inherited computed value와 각 node에서 missing/unknown을 판정하는 source는 불분명하다. Stylo computed longhand를 어느 node snapshot에서 읽는지와 absent-vs-default 구분을 적는다. 신규 plan `:62–77`; existing `runtime_grid.rs:147–168`. |
| 6 | Stylo typed lane이 CSS source 문자열을 그대로 layout 값으로 오인할 수 있는가 | PASS | — | 1A는 Stylo computed 값에서 공통 enum으로 변환하고 typed snapshot에 넣도록 제한한다. Unknown/missing 값은 horizontal로 치환하지 않고 오류로 닫는다. 기존 `ComputedElementStyle`이 문자열 map 및 typed property를 별도로 소유하는 구조와 맞는다. `snapshot.rs:501–540`; 신규 plan `:62–80`. |
| 7 | 1A와 1B를 병렬 편집할 때 실제 소스 파일 충돌이 예정돼 있는가 | PASS | — | common enum을 별도 선행 commit으로 고정한 뒤 1A는 `spinon-style`, 1B는 `spinon-text-layout`만 수정하도록 구분했고, 두 lane에서 core type을 재정의하지 않는다. 충돌이 있으면 중지하도록 했다. 신규 plan `:54–105,145–153`; 두 crate 모두 `spinon-core`에 의존한다(`Cargo.toml`). |
| 8 | `LayoutStyle` 및 layout projection이 writing mode를 소비할 위치가 계획에 드러나는가 | PARTIAL | P1 | 현재 `LayoutStyle`에는 `writing_mode`가 없고 step 2는 `spinon-style-to-layout`/`spinon-layout`을 언급하지만, typed mode가 `LayoutStyle` field인지 Grid item intrinsic input인지 명시하지 않는다. 공통 logical axis representation과 parent/item owner를 결정해야 Taffy callsite에서 fallback을 막을 수 있다. `spinon-layout/src/style.rs:334–371`; 신규 plan `:107–121`. |
| 9 | `ParagraphStyle` 변경이 기존 생성·기본값 경로를 깨뜨리거나 조용히 수평 처리할 수 있는가 | PARTIAL | P2 | 1B는 `ParagraphStyle`에 두 typed field를 추가한다고 하지만 기존 `Default`와 모든 struct literal의 갱신 정책을 적지 않았다. 기존 horizontal caller는 명시 `HorizontalTb`/`Mixed`를 보존하고 fixture 호출만 vertical을 받을 수 있게 constructor/migration gate를 기술한다. `spinon-text-layout/src/model.rs:126–173`; 신규 plan `:82–91`. |
| 10 | Text lane이 기대값 50/80을 주입해 실제 text intrinsic sizing을 우회할 수 있는가 | PARTIAL | P1 | 초안은 `XX XXXXX`의 logical inline 값과 logical/physical 혼동 금지를 적지만, metric이 실제 shaping/line-break 결과에서 생성됐음을 어떤 intermediate output으로 확인할지 정하지 않는다. Ahem advance와 break opportunities에서 산출된 logical min/max inline metric을 trace하고 fixture 상수 50/80 직접 주입을 거부하도록 acceptance를 구체화한다. 기존 orthogonal plan `:19–26,54–65`; 신규 plan `:82–91,125–129`. |
| 11 | Logical inline contribution을 Taffy row의 물리 height로 바꾸는 단계를 누락할 수 있는가 | PARTIAL | P1 | 1B는 logical metric을 준비하고 step 2는 row/child axis 관계를 연결하지만, `min/max inline → physical vertical row contribution` conversion contract와 동시에 보존할 logical block contribution은 지정하지 않았다. 변환 입력·출력 및 `50×50`/`50×80` item frame을 같은 actual measure callsite에서 대조하도록 적는다. `intrinsic_measurement.rs:193–241`; 신규 plan `:82–121,125–129`. |
| 12 | Taffy의 horizontal-only 제한을 `LayoutInput::transpose()`로 우회했다고 과장할 수 있는가 | PASS | — | 초안은 `transpose()`를 per-item writing-mode 지원으로 보지 않고, 실제 measure/track-sizing callsite에서 fail-closed feasibility gate를 요구한다. Taffy source도 CSS writing mode를 horizontal-tb로 제한한다고 밝힌다. `vendor/taffy/src/compute/grid/alignment.rs:146–150`, `vendor/taffy/src/style/alignment.rs:291–300`; 신규 plan `:29–36,107–121`. |
| 13 | 실제 동작이 지원되지 않아도 수평 fallback이나 예상값으로 성공할 수 있는가 | PASS | — | 세 계획 단계에서 unsupported orthogonal flow, missing style/revision mismatch 및 failure output을 거부하고 partial frame·수평 fallback·expected-value injection을 금지한다. 신규 plan `:49–52,107–121,123–129`. |
| 14 | Taffy generic 변경이 필요할 때 계획 경계가 불명확한가 | PASS | — | Generic Grid algorithm/global axis change가 필요하면 중단하고 더 넓은 상위 설계와 별도 20관점 계획 검토를 요구한다. 이는 임의 vendor patch로 제품 동작을 넓히는 경로를 막는다. 신규 plan `:113–121`. |
| 15 | 기존 horizontal C11.4 row/column 결과를 바꿔도 발견하지 못하는가 | PARTIAL | P2 | 기존 horizontal 결과 보존을 약속하지만 1A/1B lane acceptance에 기존 #611/#620 regression controls가 명시되지 않았다. 2단계에 기존 8개 row/column scene의 frame·track·trace regression을 고정된 gate로 넣는다. 신규 plan `:78–91,107–121`; 기존 orthogonal plan `:73–75`. |
| 16 | style snapshot·text measure·Grid layout·trace의 서로 다른 revision을 합칠 수 있는가 | PASS | — | 기존 orthogonal plan의 `source_revision`, `computed_style_revision`, `layout_revision`, `trace_revision`, asset/decode 조건을 유지하고 final gate에서 동 epoch를 요구한다. 신규 plan `:125–129`; 기존 plan `:56–65`. |
| 17 | 신규 typed value가 다른 CSS profile 또는 public CSS로 새어 나갈 수 있는가 | PASS | — | 초안은 fixture opt-in profile 외 `vertical-rl`/추가 orientation을 허용하지 않고 API·internal contract 숫자 변경·general admission을 배제한다. 신규 plan `:38–52,123–135`. |
| 18 | fixture ABI·payload 경계와 제품 artifact 주장이 혼동될 수 있는가 | PASS | — | 기본/product에는 fixture ABI·payload가 없어야 하고 opt-in positive control만 포함하도록 하며 archive 결과를 APK/IPA exclusion으로 확대하지 않는다. 기존 project rule의 fixture boundary와 일치한다. 신규 plan `:127–135`; `docs/project-rules.md` Rust fixture 경계 절. |
| 19 | Android/iOS runtime 행렬을 순차 선행 작업의 단위 검사 통과와 혼동하는가 | PASS | — | 1A/1B 단위 결과를 최종 runtime proof와 분리한다. Feasibility gate 뒤에만 양 simulator 4 scene을 각각 실행하고 renderer/compositor 한계를 기록한다. 신규 plan `:82–91,107–135`. |
| 20 | 이 제한 계획의 성공이 C11.4 완료 수나 내부 계약 버전을 올리는가 | PASS | — | C11.4 checkbox 미완료, `72/213`, 계약 0059 `0.1.0`을 명시적으로 유지한다. 신규 plan `:48–52,133–135`. |

## 초기 검토에서 발견되어 현재 계획에 반영한 점

1. `spinon-core`는 style과 text-layout이 공유하는 가장 낮은 의존성으로 지정했고 CSS parser/cascade 책임은 두지 않았다. 공통 enum에는 `Default`를 추가하지 않으며 missing computed value는 오류다.
2. `ParagraphStyle::default()`와 기존 literal 호출부 정책을 적었다. 기존 horizontal 호출부는 `HorizontalTb`/`Mixed`를 명시하고 C11.4 computed-style 경로는 Stylo typed 값을 필수로 받는다.
3. `LayoutGridItemIntrinsicSource::OrthogonalText`의 필드와 보존 경로를 고정했다. `LayoutGridItemIntrinsicInput::contribution_input()`만 logical-to-physical 축 변환을 맡고 `LayoutStyle`에는 전역 writing-mode 필드를 추가하지 않는다.
4. Ahem의 실제 shaping/line-break 측정, 50/80 비주입, trace provenance와 Taffy stop gate를 유지했다. #611/#620 기존 scene은 12-scene acceptance의 별도 회귀 그룹으로 고정했다.

## PR #626 현재 계획 최종 재검토 — 20 PASS / 0 PARTIAL / 0 FAIL

| # | 실패 관점 | 판정 | 현재 head의 줄별 근거 |
|---:|---|---|---|
| 1 | 제안 범위가 기존 C11.4 orthogonal 후보의 scene이나 지원 범위를 바꾸는가 | PASS | 계획 23–30, 98–106행은 네 exact scene과 실패 경계, 아직 미완료인 상태를 고정한다. 사전비교 README 14–37행에는 입력과 네 scene geometry가 기록되어 있다. |
| 2 | 차단 경로 설명이 실제 Stylo→text/layout 코드와 어긋나는가 | PASS | 계획 13–17행을 `runtime_grid.rs:147–168`, `spinon-layout/src/style.rs:334–371`, `spinon-text-layout/src/model.rs:126–138`, `spinon-layout/src/intrinsic_measurement.rs:193–243`과 대조했다. |
| 3 | `spinon-core` 공통 타입 owner가 dependency DAG와 맞지 않는가 | PASS | 계획 16, 34, 42행의 shared dependency 근거는 `spinon-style/Cargo.toml:12`, `spinon-text-layout/Cargo.toml:11`, `spinon-layout/Cargo.toml:11–12`과 일치한다. |
| 4 | 지원 variant·기본값·누락 및 unsupported 의미가 모호한가 | PASS | 계획 38–40, 48, 56, 64, 98행이 허용 enum, enum `Default` 금지, 구분된 오류, paragraph 기본값과 누락 시 fail-closed를 명시한다. |
| 5 | writing-mode·text-orientation의 computed/inherited 계약이 불분명한가 | PASS | 계획 36–39, 55–56행이 root/text/image의 computed 값, inherited `mixed`, CSS initial/inheritance와 incomplete snapshot을 구분한다. W3C 명세 링크는 계획 36행에 있다. |
| 6 | Stylo authored 문자열을 typed computed style로 잘못 운반하는가 | PASS | 계획 55–57행은 Stylo computed values를 공통 enum으로 변환하고 node identity 및 같은 computed-style epoch에 묶는다. 현재 node 값은 `snapshot.rs:501–529`, 해당 요소 목록과 공유 `StyleRevision`은 `snapshot.rs:576–587`에 있다. |
| 7 | 1A/1B 병렬 file boundary가 겹치거나 shared enum을 동시에 바꾸는가 | PASS | 계획 48, 52–57, 63–74, 70, 100, 114행은 공통 타입 단독 선행, 1A style 경계, 1B의 기존 literal migration 경계(runtime 22 literal/21 files 포함), 이후 순차 단계와 owner 충돌 시 중단을 정한다. |
| 8 | layout으로 보낼 exact carrier와 field owner가 지정되지 않았는가 | PASS | 계획 78–80행은 `spinon-layout/src/intrinsic_measurement.rs`의 `LayoutGridItemIntrinsicSource::OrthogonalText` 여섯 필드, size·mode/orientation 타입, 두 intrinsic input의 `.source` 보존, `LayoutInputRevision` 결합과 `contribution_input()` 단일 축 변환 owner를 지정한다. 기존 revision type은 `intrinsic_measurement.rs:2,44–60,75–84`에 있다. |
| 9 | `ParagraphStyle::default()`와 모든 기존 literal 호출부 migration이 빠지는가 | PASS | 계획 64–70, 100, 114행은 default의 `HorizontalTb`/`Mixed`, C11.4 missing snapshot 거부, 네 consumer crate migration, runtime 22 literal/21 files, production 두 경로의 horizontal gate와 20 fixture regression을 분리해 명시한다. 현재 runtime에는 `..ParagraphStyle::default()`를 쓰는 literal도 있으므로 line 100의 22개 명시 필드 점검이 compile-only 확인을 보완한다. Production guards는 `root_html_default_render/style.rs:14–16,45–95`, `root_html_text_paint.rs:78–80`, `root_html_text_paint/tree.rs:330–369`에 있고 기존 모델/default는 `spinon-text-layout/src/model.rs:126–178`에 있다. |
| 10 | Ahem 50/80을 측정 결과 대신 입력으로 넣을 수 있는가 | PASS | 계획 67–68, 101행은 실제 shaping/break evidence와 상수 비주입을 요구한다. 사전비교 README 28–37, 51–55행은 독립 수치 산출과 자산 hash를 기록한다. |
| 11 | logical inline contribution을 parent 물리 row/column으로 바꾸는 축이 틀리는가 | PASS | 계획 80–82, 101행은 단일 매핑과 최종 frame 기대를 명시한다. 사전비교 README 26–37행은 orthogonal 축 관계와 네 scene geometry를 기록한다. |
| 12 | `LayoutInput::transpose()`만으로 per-item orthogonal support를 주장하는가 | PASS | 계획 17, 82, 86–90행은 transpose만으로 지원을 주장하지 않고 실제 feasibility 및 중단 조건을 둔다. Taffy의 horizontal-tb 전제는 `vendor/taffy/src/compute/grid/alignment.rs:148–149`에 있다. |
| 13 | unsupported/missing 입력이 fallback·예상값·부분 결과로 성공 처리되는가 | PASS | 계획 38–39, 56, 82, 98–101행은 오류 경계와 fallback, expected injection, partial output 금지를 명시한다. |
| 14 | Taffy generic algorithm 수정이 필요해도 계획 범위가 확장되는가 | PASS | 계획 84–90행은 generic Grid/global axis 변경이 필요할 때 이 계획을 중지하고 상위 설계와 별도 검토를 요구한다. |
| 15 | 기존 #611/#620 horizontal regression을 놓치는가 | PASS | 계획 30, 78, 102행은 #611 column 네 scene과 #620 horizontal row 네 scene의 전후 gate 및 별도 12-scene 보고를 고정한다. |
| 16 | style/text/layout/trace revision이 서로 다른 입력 epoch로 합쳐지는가 | PASS | 계획 57, 78, 80, 82, 94, 101행은 computed-style epoch, `LayoutInputRevision`, `FontRegistryRevision`, layout/trace provenance를 연결한다. 현재 snapshot의 `StyleRevision`은 `snapshot.rs:576–587`, layout revision의 style/source/environment/intrinsic components는 `crates/spinon-layout/src/revision.rs:25–32`에 있다. |
| 17 | typed values가 다른 profile이나 public CSS 지원으로 새어 나가는가 | PASS | 계획 38–40, 56, 99–100, 103행은 C11.4 opt-in profile에만 typed values를 허용하고 다른 profile 및 public CSS admission을 배제한다. |
| 18 | fixture ABI·payload 조건을 이미 검증한 product artifact 결과처럼 표현하는가 | PASS | 계획 103행은 기본/product artifact와 opt-in positive control의 미래 acceptance를 분리하고 archive 결과를 APK/IPA 전체 제외 근거로 확대하지 못하게 한다. project rule은 `docs/project-rules.md:35–37`이다. |
| 19 | Android/iOS 수용 행렬이나 simulator 실행이 이미 완료된 것처럼 읽히는가 | PASS | 계획 92, 100–104행은 gate 뒤의 미래 runtime matrix, 별도 1B/Step 3, 플랫폼 한계를 명시하며 unit 결과와 runtime proof를 분리한다. |
| 20 | 계획 성공이 C11.4 완료 수나 내부 숫자 버전을 올리는가 | PASS | 계획 40, 53, 106행은 공개 지원 범위를 넓히지 않고 공식 C11.4 미완료, `72/213`, 내부 계약 `0.1.0`을 유지한다고 명시한다. |

최종 판정은 **20 PASS / 0 PARTIAL / 0 FAIL**이다. 초기 12/8 표는 보완 전 초안의 역사 기록으로 남기고 최종 판정에는 포함하지 않는다. 이 재검토는 계획 준비도만 확인했으며, 구현·Taffy feasibility gate·C11.4 완료를 뜻하지 않는다. 다음 구현 전제는 이 계획 PR의 검토·병합과 0단계 `spinon-core` 공통 타입 계약이다.
