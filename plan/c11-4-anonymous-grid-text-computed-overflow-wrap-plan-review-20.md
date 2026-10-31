# C11.4 anonymous Grid text computed `overflow-wrap` 계획 검토 20개

**검토 대상:** [집중 계획](./c11-4-anonymous-grid-text-computed-overflow-wrap.md), [Chrome 156 관찰과 독립 기대값](../spec/internal/evidence/c11-4-anonymous-grid-text-overflow-wrap-precomparison-2026-11-01/README.md), 고정 HTML fixture와 Ahem font table. 제품 코드와 모바일 runtime은 검토 대상이 아니다.

| # | 계획을 공격하는 실패 관점 | 판정 |
|---:|---|---|
| 1 | `anywhere`와 `break-word`가 min-content에서 같은 값이어도 될 수 있다 | **통과.** W3C CSS Text 3 §5.4 의미와 Chrome 관측은 각각 10px과 40px로 구별한다. |
| 2 | 기대값이 Chrome CSSOM에서 그대로 복사될 수 있다 | **통과.** Ahem UPEM와 `X` advance를 읽어 10px glyph 폭과 40px max-content 폭을 독립 계산한다. |
| 3 | 다른 Ahem 복사본이나 fallback font가 기준을 바꾼다 | **통과.** 저장소 canonical Ahem path와 SHA를 기록하고 browser의 `document.fonts.check`가 true인지 확인한다. |
| 4 | `XXXX`가 두 단어로 분해되어 normal min-content가 10px이 될 수 있다 | **통과.** 공백·하이픈 없는 한 단어를 고정하고 `word-break:normal`을 기록한다. |
| 5 | `anywhere`가 grapheme cluster 내부를 분리한다고 과장할 수 있다 | **통과.** Ahem의 독립 ASCII `X` grapheme만 사용하고 임의 Unicode를 범위에서 제외한다. |
| 6 | `break-word`의 emergency break를 min-content 값에 포함할 수 있다 | **통과.** 계획은 min-content pass에서 normal opportunities를 쓰고 definite-width break-word wrapping은 별도로 유지한다. |
| 7 | fixed 10px control이 실제 text wrapping을 검사하지 않고 intrinsic 값만 비교한다 | **통과.** fixed track scene에서 normal 1 line overflow와 anywhere/break-word 4 line wrapping을 분리한다. |
| 8 | fixed track의 min-size 또는 root width가 10px column을 바꿀 수 있다 | **통과.** Chrome 관측은 세 값 모두 CSSOM column 10px임을 보여주며, runtime gate도 exact track/frame을 검사한다. |
| 9 | max-content control이 `anywhere` soft break 때문에 폭이 줄어들 수 있다 | **통과.** 세 값 모두 `XXXX`의 한 줄 40px을 기대하며 Chrome이 이를 확인한다. |
| 10 | min-content auto row가 열 폭과 반대 방향으로 잘못 계산될 수 있다 | **통과.** anywhere의 10px column은 네 줄 40px row, normal/break-word는 40×10px을 기대한다. |
| 11 | line-height가 10px이 아니어서 row 높이 비교가 흔들릴 수 있다 | **통과.** HTML fixture·계획·JSON 모두 `font-size`와 absolute `line-height`를 10px으로 고정한다. |
| 12 | 다른 line-breaking property가 break opportunities를 바꿀 수 있다 | **통과.** `word-break:normal`, `white-space:normal`, `text-wrap-mode:wrap`, `line-break:auto`를 고정한다. |
| 13 | computed property가 child나 stale style revision에서 올 수 있다 | **통과.** 정확한 root owner, HostDocument/style revision, scene 및 typed 값의 동시 일치를 요구한다. |
| 14 | scene ID가 computed property 값을 대신할 수 있다 | **통과.** validator는 CSSOM serialization과 typed `OverflowWrapMode`를 모두 실제 snapshot에서 확인한다. |
| 15 | 모든 값 지원이 일반 CSS admission으로 읽힐 수 있다 | **통과.** scope를 opt-in fixture의 세 값과 아홉 고정 장면으로 제한한다. |
| 16 | fail-closed 입력이 partial track trace/frame/draw를 남길 수 있다 | **통과.** unknown scene, mismatch, stale revision과 profile conflict의 publication 전 거부를 완료 기준으로 둔다. |
| 17 | Parley 버전 변경이나 없는 enum variant를 전제로 할 수 있다 | **통과.** 고정 `parley 0.12.0` registry source에 `Normal`, `Anywhere`, `BreakWord`가 존재하고 plan은 버전을 올리지 않는다. |
| 18 | Android/iOS simulator evidence를 glyph pixels로 해석할 수 있다 | **통과.** 계획은 frame/track/draw/present만 검사하며 software AVD와 compositor 전 drawable 한계를 분리한다. |
| 19 | fixture-test code가 default archive나 APK/IPA 모두에서 제거됐다고 과장할 수 있다 | **통과.** default/product와 opt-in static archive만 비교하고 app bundle exclusion은 주장하지 않는다. |
| 20 | 작은 text fixture가 C11.4/C11 완료 수를 올릴 수 있다 | **통과.** 공식 checkboxes, 내부 계약 버전 `0.1.0`, 집계 `72/213`을 유지한다. |

**판정:** 계획 경계·독립 기대값·실행 구분 20/20 통과. Parley `BreakWord`의 min-content 동작은 구현 회귀로 직접 확인하며, 고정 10px definite layout은 실제 emergency wrapping control로 유지한다. 이 기록은 계획 PR 변경 검토 및 구현/runtime 검토와 별도다.
