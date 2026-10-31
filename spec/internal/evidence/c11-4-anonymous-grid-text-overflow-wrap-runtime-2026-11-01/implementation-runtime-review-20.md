# C11.4 anonymous Grid text `overflow-wrap` 구현/runtime 실패 관점 20개

2026-11-01에 구현 diff, 고정 Ahem `XXXX` oracle, Rust focused tests, Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator trace를 대조했다. 이 검토는 계획 PR의 20개 관점 검토와 별개다. PR diff의 30관점 검토도 별도로 남아 있다.

검토 중 adapter 기대값에서 실제 입력/trace와 맞지 않은 다섯 경로를 고쳐 최신 플랫폼 실행으로 재확인했다: `lang`이 없는 HTML scene의 nullable locale, fixed `10px` track에서 40px로 넘치는 `normal` line의 text width, fixed track에 intrinsic column measure가 없는 동작, iOS `FixedTrackEarlyReturn` trace 단계, boundary test의 scene allowlist 탐색 위치. 실패 원본은 각 simulator 디렉터리의 `*.failed-attempt-*.log`와 `attempts*.tsv`에 보존했다.

| # | 실패 관점 | 확인과 판정 |
|---:|---|---|
| 1 | 새 mode가 기존 paragraph의 computed 기본값을 바꾸는가 | `OverflowWrapMode::Normal`을 enum 및 `ParagraphStyle::default()`에 지정했다. 기존 text-layout, layout, runtime fixture 회귀에서 기본값을 사용하고 전체 C11.4 scene 수를 유지한다. 통과. |
| 2 | snapshot이 `overflow-wrap`을 잃어 이후 단계가 잘못된 style을 읽는가 | definite-width 및 empty `TextLayoutSnapshot` 모두 typed `overflow_wrap`을 소유하고 equality에도 포함한다. 세 mode와 빈 paragraph focused test 통과. |
| 3 | `normal`이 새 emergency break를 min-content에 잘못 추가하는가 | Ahem `XXXX`는 40px min-content, 40px max-content, 10px line height를 유지하며 fixed 10px 폭에서 한 줄 40px advance를 넘긴다. focused test와 두 플랫폼 trace 통과. |
| 4 | `anywhere`의 break opportunity가 min-content 계산에 반영되지 않는가 | `anywhere`는 10px min-content와 네 줄·40px auto row를 만든다. text-layout 단위 및 Android/iOS 각 3개 관련 scene 통과. |
| 5 | `break-word`가 `anywhere`처럼 min-content를 줄이는가 | min-content 한정 pass에서 `Normal` 기회를 다시 사용하고 결과가 40px이다. Chrome/Ahem 기대값 및 양 플랫폼 trace와 일치. |
| 6 | `break-word`의 definite-width emergency wrapping까지 제거되는가 | 최종 paragraph layout에는 computed `BreakWord`를 보존한다. 10px fixed column scene에서 10px line advance 네 줄을 만들고 양 플랫폼 trace 통과. |
| 7 | max-content 폭이나 높이가 mode 때문에 달라지는가 | 세 mode 모두 40px max-content와 10px block height다. nine-scene matrix의 세 max-content 행과 독립 Ahem 기대값 일치. |
| 8 | BreakWord min-content 재생성이 다른 text를 측정하는가 | 두 Parley layout의 normalized paragraph text가 다르면 layout 오류로 닫는다. 고정 입력에서 normalized text 일치 및 `XXXX` source map 확인. |
| 9 | computed mode가 source owner·revision 밖에서 추론되는가 | validator가 root computed CSS property, direct anonymous text, typed paragraph style 및 HostDocument/style revision을 함께 확인한다. Android/iOS trace의 scene revision 일치. |
| 10 | `overflow-wrap` 속성이 누락되거나 지원하지 않는 keyword가 기본값으로 통과하는가 | runtime validator에서 누락, 다른 keyword 및 allowlist와 다른 값은 오류다. focused runtime negative tests 통과. |
| 11 | 유효한 keyword여도 scene ID와 다르면 통과하는가 | `normal` scene에 `anywhere`/`break-word`를 넣은 테스트가 거부된다. FFI/runtime의 값·scene allowlist가 모두 exact. |
| 12 | 임의 scene ID가 generic anonymous-text 경로에 들어오는가 | FFI `runtime_source()` 및 runtime `find_scene()`는 아홉 ID만 허용한다. 미등록 ID test와 fixture-module boundary test 통과. |
| 13 | direct-text가 아닌 추가/누락 child가 같은 고정 기대값으로 검증되는가 | scene별 expected child 수와 `XXXX` text를 검사한다. topology mismatch는 layout/frame/trace publish 전에 닫는 기존 anonymous-text validator 사용. |
| 14 | `<html lang>` 없는 입력이 앱 예상값에서 가짜 locale을 갖는가 | fixture HTML에는 `lang`이 없고 computed locale은 nullable이다. Android와 iOS 기대값을 `null`로 맞춘 뒤 각 9/9 통과. 이전 mismatch log를 보존. |
| 15 | fixed `10px` track에서도 intrinsic column contribution이 실행된다고 잘못 가정하는가 | fixed track 경로는 Columns에서 intrinsic measure를 생략한다. Android/iOS validator와 trace가 measure 없음 및 fixed final track을 검사한다. 이전 기대값 오류를 수정했다. |
| 16 | fixed column의 available width와 overflowing text layout width를 혼동하는가 | 두 값은 별도 검증한다. `normal`은 available 10px, text width 40px이며 `anywhere`/`break-word`는 available/text width 10px다. 두 simulator trace 통과. |
| 17 | iOS가 auto/intrinsic track stage를 fixed track에도 요구하는가 | iOS trace validator는 fixed Columns의 `FixedTrackEarlyReturn`을 인정하고 Rows auto track은 따로 확인한다. 이전 잘못된 stage 가정을 고쳐 simulator 9/9 통과. |
| 18 | Android/iOS adapter가 서로 다른 computed property나 line metrics를 승인하는가 | 두 adapter가 style의 `overflowWrap`, nullable locale, text/intrinsic dimensions, line count/advance, item frame, final track을 검사한다. 플랫폼별 matrix 각각 9/9. |
| 19 | trace report에서 computed mode가 빠지거나 다른 값으로 직렬화되는가 | runtime trace JSON의 `overflowWrap`을 세 keyword별로 비교한다. 9개 scene의 raw logs 및 matrix에서 모든 assertion 통과. |
| 20 | 신규 harness가 기본 제품 FFI archive까지 fixture를 노출하는가 | 별도 기본/fixture release archive 검사를 [artifact boundary evidence](./artifact-boundary/README.md)에 기록한다. 기본 archive는 scene marker와 fixture host export를 포함하지 않아야 하고 opt-in fixture archive는 9개 marker와 host export를 포함해야 한다. archive만으로 APK/IPA 전체 bundle을 판정하지 않는다. |

## 검토 경계

9/9는 각 simulator adapter가 확인하는 이 고정 runtime scene matrix의 geometry·computed style·trace 결과다. Glyph ink/pixel, Android 실기기/hardware GPU, iOS compositor 제출 후 표면, 일반 author CSS admission, WPT 및 C11.4/C11 전체는 확인하지 않는다. [실행 logs와 screenshots](./README.md) · [사전 비교 oracle](../c11-4-anonymous-grid-text-overflow-wrap-precomparison-2026-11-01/README.md).
