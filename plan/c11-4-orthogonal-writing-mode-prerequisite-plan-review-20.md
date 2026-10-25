# C11.4 orthogonal writing-mode 선행 계획 실패 관점 검토 20개

- 대상: `plan/c11-4-orthogonal-writing-mode-prerequisite.md` 초안
- 검토 방식: 계획·현재 소스·연결된 orthogonal 사전 비교만 읽었다. 빌드·테스트·브라우저·시뮬레이터는 실행하지 않았다.
- 최초 판정: 표의 행을 다시 합산하면 **12 PASS / 8 PARTIAL / 0 FAIL**이다. 초안에 기록했던 `13/7` 요약은 합산 오류였다.
- 최종 독립 재검토: **20 PASS / 0 PARTIAL / 0 FAIL**. 초안 보완 후 공통 타입 owner, paragraph 기본값/호출부 migration, layout transport type, 축 변환 owner와 회귀 gate를 다시 확인했다.
- 해석: 최초 검토 blocker는 계획 변경으로 모두 닫혔다. 최종 재검토는 설계 계획의 준비도만 판정하며 코드 구현·Taffy feasibility·C11.4 완료를 의미하지 않는다.

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

## 최초 검토에서 발견되어 반영한 점

1. `spinon-core`는 style과 text-layout이 공유하는 가장 낮은 의존성으로 지정했고 CSS parser/cascade 책임은 두지 않았다. 공통 enum에는 `Default`를 추가하지 않으며 missing computed value는 오류다.
2. `ParagraphStyle::default()`와 기존 literal 호출부 정책을 적었다. 기존 horizontal 호출부는 `HorizontalTb`/`Mixed`를 명시하고 C11.4 computed-style 경로는 Stylo typed 값을 필수로 받는다.
3. `LayoutGridItemIntrinsicSource::OrthogonalText`의 필드와 보존 경로를 고정했다. `LayoutGridItemIntrinsicInput::contribution_input()`만 logical-to-physical 축 변환을 맡고 `LayoutStyle`에는 전역 writing-mode 필드를 추가하지 않는다.
4. Ahem의 실제 shaping/line-break 측정, 50/80 비주입, trace provenance와 Taffy stop gate를 유지했다. #611/#620 기존 scene은 12-scene acceptance의 별도 회귀 그룹으로 고정했다.

## 최종 재검토 결과

수정된 계획의 두 번째 독립 20관점 검토에서 **20 PASS / 0 PARTIAL / 0 FAIL**을 확인했다. 최종 검토는 #8의 typed source/field owner와 축 변환 단일 소유자, #9의 `ParagraphStyle::default()` 및 전체 literal migration, 1A/1B 파일 경계, Ahem 실측과 회귀 gate를 확인했다. 남은 blocker는 없으며, 다음 구현 전제는 이 계획 PR의 검토·병합과 0단계 `spinon-core` 공통 타입 계약이다. 이 판정은 아직 실행하지 않은 Taffy feasibility gate를 통과했다는 뜻은 아니다.
