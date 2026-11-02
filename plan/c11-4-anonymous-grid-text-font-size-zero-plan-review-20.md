# C11.4 anonymous Grid text `font-size: 0` 계획 실패 관점 검토 20개

**검토 대상:** [계획](./c11-4-anonymous-grid-text-font-size-zero.md), [Chrome 156 사전 비교](../spec/internal/evidence/c11-4-anonymous-grid-text-font-size-zero-precomparison-2026-11-02/README.md), pinned Ahem metrics, `ParagraphStyle` validation.

각 항목은 이 계획에 고유한 별도 실패 경로 또는 불변 조건을 검토한다. 구현 검토와 PR diff 검토에서는 재사용하지 않는다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | CSS Fonts의 non-negative 범위를 0 초과로 오독 | **통과.** CSS Fonts Level 4 2026-09-06 WD의 `[0,∞]` length-percentage 범위를 인용하고 0을 유효 input으로 고정한다. |
| 2 | 유효하지 않은 `-1px` 선언이 zero-size positive로 혼동 | **통과.** Chrome `CSS.supports` negative와 inherited 10px computed fallback을 별도 control로 둔다. |
| 3 | CSS의 signed zero 동작을 Rust `f32` 부호와 혼동 | **통과.** Chrome `-0px`와 `0px`가 모두 computed `0px`임을 기록하고 `-0.0`은 computed zero에만 일치하도록 한다. |
| 4 | NaN 또는 ±infinity가 `font-size:0` allowlist를 우회 | **통과.** finite check를 acceptance 조건에 명시하고 직접 typed negative matrix에 포함한다. |
| 5 | `line-height:10px`이 font-size 0에 따라 사라짐 | **통과.** 명시 absolute line-height를 고정하고 computed line-height, auto row를 각각 10px로 판정한다. |
| 6 | `line-height:normal` 결과를 zero-size 이행으로 섞음 | **통과.** `normal`은 현재 plan에서 제외해 font-metric 정책과 분리한다. |
| 7 | Range 0×0을 anonymous item box가 없는 증거로 해석 | **통과.** Range를 text fragment로 한정하고 item 존재는 다음 block의 second-row 배치로 관찰한다. |
| 8 | min-content의 0px 기여만 확인하고 max-content는 누락 | **통과.** 두 sizing function을 독립 scene으로 고정한다. |
| 9 | max-content CSSOM track 값을 독립 기대값으로 복사 | **통과.** Ahem `unitsPerEm`와 `hmtx` advance에서 zero contribution을 계산한다. |
| 10 | space를 content width로 더해 max-content가 0보다 커짐 | **통과.** Ahem space advance가 1em이며 font-size 0에서 0px임을 계산에 포함한다. |
| 11 | whitespace-only run을 zero-font non-whitespace item과 동일 취급 | **통과.** no-item control은 `columns:none`, row 0, child 없음으로 별도 고정한다. |
| 12 | zero-size text가 auto-placement cell을 소비하지 않고 다음 block이 첫 행에 놓임 | **통과.** text 후 5×3px block의 y=10px과 `10px 3px` rows를 필수로 둔다. |
| 13 | fixed following box가 zero text intrinsic contribution을 가려 min/max 결과를 잘못 판정 | **통과.** placement scene과 min/max-only scenes를 분리한다. |
| 14 | fallback font가 Ahem과 달라 독립 산식이 바뀜 | **통과.** Ahem asset SHA와 `document.fonts.check('10px Ahem')=true`를 보존한다. |
| 15 | positive font-size 회귀가 zero-only 변경에 의해 깨짐 | **통과.** 기존 5, 7.5, 10, 12.5, 20px 행렬을 구현 시 계속 실행한다. |
| 16 | 다른 `ParagraphStyle` caller가 검증 변경으로 의도치 않게 zero 입력을 얻음 | **부분 통제.** 모델 validator는 finite zero를 표현하되 runtime admission은 C11.4 exact fixture gate로 한정하고 다른 route의 범위를 PR에 적는다. |
| 17 | Root HTML renderer의 positive-only glyph rasterizer가 zero-size layout 때문에 `drawPresented` 전에 실패 | **통과.** exact fixture의 all-zero run은 renderer가 rasterizer를 호출하지 않고 no-paint로 건너뛴다. `InlineRunStyle`과 C15 rasterizer는 기존 양수 전용 계약으로 유지한다. |
| 18 | text layout builder가 zero advance에서 NaN 또는 음수 intrinsic을 게시 | **통과.** implementation gate에 zero-font Parley 결과, finite output, 0 contribution, line-height를 요구한다. |
| 19 | style-owner revision과 typed font-size 값이 다른 시점에서 조합 | **통과.** same node, root, style revision, serialization과 typed `0.0`의 동일성을 요구한다. |
| 20 | 이 bounded slice만으로 C11.4 또는 전체 roadmap 완료를 표시 | **통과.** C11.4/C11 check와 공식 `72/213`은 변경하지 않는다. |

**결론:** 계획은 구현 전 oracle·독립 계산·허용 입력·negative 경계·플랫폼 및 보고 한계를 고정했다. Root HTML renderer/rasterizer 경계는 소스 확인에서 발견해 fixture no-paint 규칙으로 제한했다. 발견된 부분 통제는 implementation 20 및 final PR diff review에서 runtime admission과 다른 `ParagraphStyle` caller를 다시 확인한다.
