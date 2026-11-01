# 양 축 negative named-line pair 구현/runtime adversarial review 20개

검토 대상은 현재 checkout의 `TemplateGridNegativeNamedLinePairAutoImplicitTracksBothAxes` candidate와 그 Rust·style·FFI·Android/iOS fixture 연결이다. PR diff review는 PR 생성 후 별도 수행한다.

1. **기존 one-axis 계약이 넓어지는가?** 별도 enum variant와 style gate를 추가했다. one-axis Android/iOS 실행은 각각 표본 8/8 exact다. [one-axis Android](../c11-3-3-negative-named-line-pair-runtime-2026-11-01/android-avd-c1133-negative-pair-android-avd-20261101T054233146Z-70fec10c/README.md) · [iOS](../c11-3-3-negative-named-line-pair-runtime-2026-11-01/ios-simulator-c1133-negative-pair-ios-simulator-20261101T054243443Z-904c5203/README.md)
2. **두 축에서 ident가 다른가?** Cross-axis gate는 row start/end, column start/end 및 두 축 ident가 모두 같아야 선택한다. 다른 row ident 검사에서 both-axes mode는 선택되지 않는다. `grid_placement_values.rs` unit test.
3. **한 축의 start/end ident가 다른가?** 각 축 사용량 판정은 기존 named-line resolver의 pair predicate를 거쳐야 하며 불일치하면 새 flag가 false다. Resolver의 missing-name와 mismatch negative tests를 그대로 사용한다.
4. **signed line이 leading implicit extent 밖을 가리키는가?** layout validator는 양 끝 line이 explicit grid line 경계보다 앞인지 축별 검사한다. row positive/mixed input 거부를 `grid_named_line_auto_tests.rs`에서 확인한다.
5. **한 축에만 implicit pair가 있는가?** style gate는 양쪽 pair flag 모두 참이어야 새 mode를 선택한다. 기존 one-axis fixture는 별도 scene ID와 mode를 유지한다.
6. **양 축 auto sizing의 기여가 서로 섞이는가?** auto rows/columns 동시 scene은 Columns `[6,6,6,0,20]`, Rows `[4,4,4,0,15]`, item `(0,0,18,12)`를 독립 기대값과 맞춘다.
7. **각 fixed pattern이 다른 축의 phase에 영향 주는가?** `5px 7px` columns와 `3px 4px` rows 결과 `[5,7,5,7,20]` 및 `[3,4,3,4,15]`를 별도 비교한다.
8. **equal endpoints가 zero span/잘못된 크기를 만드는가?** 두 축 모두 `[18,20]` columns와 `[12,15]` rows로 해석되고 item은 유효한 one-track span을 갖는다.
9. **reversed endpoint order가 축마다 정규화되는가?** both-reversed 및 columns-only/rows-only reversal scene의 final track arrays가 기준과 같다.
10. **fixed item이 track area 밖으로 칠해질 때 track이 늘어나는가?** fixed pattern의 17×10 spanned area와 18×12 item을 유지한다. `(117,10)` overflow sample은 Android/iOS에서 Chrome RGBA와 exact하다.
11. **참여 자식이 0개 또는 여러 개인가?** 양 축 mode에서 둘 다 InvalidGridPlacement로 거부한다. focused layout boundary test.
12. **item 한 축이 auto size인가?** Style gate는 width와 height 둘 다 Fixed인지 확인하고 layout validator도 각 dimension을 검사한다. width Auto negative case가 거부된다.
13. **positive, mixed, Auto, Span endpoint가 dual-axis mode로 들어오는가?** Layout shape validator는 두 축의 endpoints가 모두 leading negative Line인지 요구한다. row positive/mixed case를 거부한다. Named span과 Auto는 같은 enum pattern에서 거부된다.
14. **template areas가 함께 지정되는가?** 양 축 mode와 `GridTemplateAreasInput` 조합을 검증 전 차단한다. `grid-template-areas` property의 InvalidGridTrackList를 확인한다.
15. **implicit track list에 auto sizing function이 섞이는가?** `GridTrackSize::Auto` 목록을 UnsupportedGridAutoColumns로 거부한다. 기존 one-axis invalid-track-kind regression도 남아 있다.
16. **breadth가 음수/비유한가?** 공통 track validator가 거부한다. 양 축 mode test는 infinite row breadth를 InvalidGridTrack으로 확인한다. 기존 negative px test도 유지한다.
17. **implicit expansion이 축별 10,000개를 넘는가?** 10,001-track line request가 `GridTrackLimitExceeded`가 되며 partial success를 내지 않는다.
18. **Android fixture가 다른 scene을 허용하거나 좌표 오차를 숨기는가?** exact scene allowlist와 pixel sample allowlist를 확인했다. 잘못된 좌표는 readback 전, unknown scene은 runtime draw 전 거부된다. SwiftShader API 37 AVD surface 6/6 exact.
19. **iOS fixture가 좌표/표면 identity를 무시하는가?** iPhone 17 Pro/iOS 26.2의 Metal drawable 6/6 exact, 잘못된 좌표는 readback 전 거부된다. 이는 compositor 이후 표면을 뜻하지 않는다.
20. **fixture payload가 기본 FFI archive로 새는가?** macOS default archive에서 `negative-named-line-pair-two-axis-matrix`, `two-axis-fixed-pattern`, `two-axis-fixed-item` marker 모두 없다. opt-in positive control에는 세 marker 모두 있다. APK/IPA archive boundary는 별도 미검증이다.

검토 과정에서 발견한 검증 공백은 0/2 children, non-fixed item, valid template-areas rejection, unsupported/invalid `grid-auto-*`, 10,001-track rejection을 focused layout test에 추가해 닫았다. focused test 최종 실행은 통과했다. 전체 style suite에는 위 문서의 unrelated C07.3 float assertion 하나가 남아 있어 전체 suite green으로 기록하지 않았다.
