# C11.4 direct-text finite-positive aspect-ratio PR diff 검토 — 30 checks

**검토 대상:** 현재 branch의 implementation candidate diff, 관련 계획·내부 계약·상태 대장·public roadmap, source-bound simulator outputs와 review/evidence records. 구현 commit/PR은 아직 생성 전이므로 이 문서는 GitHub PR/check 상태를 미리 판정하지 않는다.

| # | diff 실패 관점 | 근거와 판정 |
| --- | --- | --- |
| 1 | 구현 후보가 일반 CSS 지원이나 전체 C11.4 완료로 표현되는가 | 계획·0059·STATUS·roadmap은 모두 opt-in fixture candidate로 표시하고 C11.4/C11 checkbox와 `72/213`을 유지한다. **PASS** |
| 2 | 기존 #834의 2/1 구현 기록이 이번 finite-positive 확장 결과로 덮이는가 | 기존 4-scene/#834 note는 병합 이력으로 남고 새 16-scene 후보 note가 별도로 추가됐다. **PASS** |
| 3 | 계획·명세·상태 대장·public roadmap이 서로 다른 ratio 범위를 선언하는가 | 각 문서는 finite-positive typed ratio와 computed serialization 일치 조건, four-ratio matrix와 non-completion boundary를 공유한다. **PASS** |
| 4 | spec 문서가 CSS Grid의 replaced-item transferred-size 규칙을 non-replaced text에 잘못 적용하는가 | 계획 및 내부 계약은 §6.2/§11.5 contribution을 다루고 §6.6 replaced shortcut을 명시적으로 제외한다. **PASS** |
| 5 | Chrome 관측값과 독립 규범 계산을 같은 증거로 취급하는가 | precomparison은 두 결과를 나누고 `3/2` Chrome quantization 및 1/64px tolerance를 명시한다. **PASS** |
| 6 | `auto <ratio>`가 계획에서는 제외되지만 code/test에서는 일반 기능처럼 연결되는가 | Collector는 plain numeric pair만 받고 plan/README는 `auto <ratio>` cascade rejection을 잔여 범위에 기록한다. **PASS** |
| 7 | computed ratio serialization이 실제 Stylo value가 아닌 scene label/source text를 복사하는가 | Serialization helper는 cloned computed `PreferredRatio`에서 pair/prefix를 구성한다. Source hash와 style regression test를 연결했다. **PASS** |
| 8 | `None` typed ratio controls가 degenerate numeric serialization을 `auto`로 오인하는가 | Runtime requires serialized `auto` whenever typed ratio is absent; internal contract distinguishes numeric pair from sizing effect. **PASS** |
| 9 | ratio-derived invalid dimension이 Taffy/renderer까지 전파되는가 | Plan and collector document the pre-layout finite-positive f32 checks for auto and fixed-row derived axes, with extreme-ratio adversarial cases. **PASS** |
| 10 | fixed-row allowlist와 simulator matrix의 row heights가 다르게 설정되는가 | Collector, plan, evidence and app scene list agree on 20/30/40/45/60px fixed rows; values outside the subset fail closed. **PASS** |
| 11 | fixture tree/typography/style restrictions silently broaden? | Direct-child, text owner, Ahem, line-height, writing mode, wrap, edge, gap, placement and alignment limits remain explicit in the plan and collector. **PASS** |
| 12 | ratio source can move to a different text item or multiply across items? | Runtime count/owner checks bind one ratio to `AAAA`; test matrix and FFI catalog use fixed IDs and assert unsupported owner/count rejection. **PASS** |
| 13 | intrinsic column floor fix contradicts CSS Grid model or widens to replaced/explicit-size items? | Taffy helper is limited to non-replaced ratio items with auto width/height/min/max; contract states its ContentSize basis and exclusions. **PASS** |
| 14 | Taffy contribution patch drops the opposite known dimension or selects wrong min/max measurement mode? | Helper clears only the queried axis, preserves the other known dimension and routes the matching min-content/max-content available-space mode through existing adjustments. **PASS** |
| 15 | numeric item frame and direct-text line box are conflated? | Plan/evidence distinguish the item box from 40×10 text line; `column-1-2` fixes 40×80 item independently from 40px intrinsic track. **PASS** |
| 16 | scene list omits historical regressions or one axis/control combination? | Receipt and docs enumerate four #834 scenes, four row ratios, four column ratios and four column no-ratio controls: 16 total. **PASS** |
| 17 | expected values are derived from Spinon output after implementation? | Precomparison source has independent CRD calculations and Agent Browser Chrome 156 observations; runtime tests/app expectations use those fixed numbers. **PASS** |
| 18 | FFI scene route accepts arbitrary caller IDs/payload or is available without fixture flag? | Catalog is fixed and feature-gated; static archive negative/positive controls and FFI route test evidence remain linked. **PASS** |
| 19 | Android scene definitions drift from Rust registry on item IDs, ratio target or row axis? | Java’s fixed 16 IDs, roots, text, row template and ratio placement match the FFI/source-bound scene matrix. **PASS** |
| 20 | iOS scene definitions drift from Rust/Android on the same fields? | Swift’s 16-scene definitions use the same IDs and ratio/control layout; final iOS run receipt binds the executable and source hashes. **PASS** |
| 21 | Android verifier allows extra/missing Grid stages, duplicate contributions or incorrect track basis? | Ratio branch checks exact stage order/cardinality and track/contribution fields; the final AVD run passed all 16 scenes. **PASS** |
| 22 | iOS verifier applies weaker structural matching or boolean checks than Android? | Swift uses strict stage/cardinality comparisons; the prior verifier defect and correction remain documented in historical evidence, and the final run is 16/16. **PASS** |
| 23 | app and Node capture runner disagree about Chrome float tolerance? | Java, Swift and Node numeric matchers use 1/64px tolerance; `3/2` row is the fractional adversarial case. **PASS** |
| 24 | overflow coverage stops at root bounds even though screenshot/summary claims outside painting? | `column-1-2` explicitly samples y=70 below its 60px root and records red exact pixels in both final run directories. **PASS** |
| 25 | surface colors or pixel counts are rounded/misreported as geometry agreement? | Receipts separate numeric frame tolerance from exact RGBA samples; totals are 49/49 per platform. **PASS** |
| 26 | captured output is from a different simulator or stale installed binary? | `simulator.json` identifies AVD/iPhone Simulator and reports local/installed APK or executable hashes as equal. **PASS** |
| 27 | source receipt omits changed Rust/platform/fixture inputs or points at another tree? | Both result receipts record source HEAD `e56dd00264e9ed2ad05539a70f9e143e6d9ac1bf`, dirty paths and SHA-256 entries including runtime, style, Taffy, FFI, app, fixture and runner. **PASS** |
| 28 | failed harness runs disappear or the final run is selected by convenience rather than receipt? | Earlier evidence directories remain untouched; README records four corrected verifier/expectation failures and links the named final receipts. **PASS** |
| 29 | workspace test limitation or archive inspection is generalized beyond evidence? | README distinguishes the unavailable standalone `cargo test -p taffy` target, passing library Clippy and downstream tests, and static archives from full APK/IPA exclusion. **PASS** |
| 30 | 캡처·문서 링크가 stale screenshot을 가리키거나 public roadmap이 확인되지 않은 배포를 보고하는가 | 새 evidence links는 지정한 final candidate PNG를 가리키고 roadmap note는 source 상태만 기록한다. Tailnet sync와 hosted render 확인을 선취하지 않는다. **PASS** |

**결과:** 30개의 별도 diff 관점에서 범위·source/cascade/layout 경계·플랫폼 행렬·evidence·문서 상태를 검토했다. 이 기록은 merge, CI success 또는 Tailnet page delivery의 증거로 사용하지 않는다.
