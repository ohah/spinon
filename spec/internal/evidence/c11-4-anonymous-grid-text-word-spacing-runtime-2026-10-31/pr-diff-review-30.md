# C11.4 anonymous Grid text computed `word-spacing` PR diff 30 관점 검토

## 검토 범위

PR [#924](https://github.com/ohah/spinon/pull/924), head `4f459328c170a8bd7c2d3e06f79fc06a78c1fb9c`, base `ca0627b89c2a76b82347627dceccbc6e0bd84872`의 실제 92-file diff와 PR 본문을 검토했다. 아래 항목은 앞선 구현/runtime 20개와 구분해 변경 묶음·통합 경계·제출 근거를 확인한다.

## 독립 검토 항목

1. **변경 범위의 귀속** — tracked 변경은 C11.4 word-spacing 연결, 0px compatibility initializer, fixture-host 검증과 해당 증거다. 별도 untracked 작업 자료는 PR diff에 포함되지 않았다.
2. **`ParagraphStyle` caller 호환** — 새 public Rust field를 추가한 뒤 직접 struct initializer를 검색·갱신했다. 기존 경로는 모두 0px를 보존한다.
3. **외부 ABI 혼입** — 새 public C/Swift/Java user API나 FFI ABI entry point는 추가되지 않았다. 기존 opt-in fixture gate만 사용한다.
4. **finite 값 계약** — `ParagraphStyle` non-finite guard와 computed CSS scene validator가 역할별로 적용되고 잘못된 계산값이 layout publication을 통과하지 않는다.
5. **legacy anonymous profile 기본값** — 새 property가 없는 기존 anonymous-text 장면은 `0px`을 기대한다. 이전 `validate_text_profile_properties`의 새 책임 오검사는 전체 runtime 검사에서 찾아 새 word-spacing validator negative test로 이동했다.
6. **Parley API 사용** — pinned Parley `0.12.0`의 `StyleProperty::WordSpacing`을 사용하며 독자적인 glyph advance 보정을 추가하지 않는다.
7. **text layout 축 분리** — letter spacing의 기존 경계 보정 코드를 word spacing으로 재사용·변경하지 않고 별도 style property를 전달한다.
8. **whitespace normalization** — 반복·후행 공백 raw source와 collapsed paragraph source를 각각 검증해 spacing 적용 횟수가 source whitespace 개수와 혼동되지 않는다.
9. **min-content 축소** — track의 `min-content` 기여는 longest word이며 separator spacing에 따라 늘지 않는다. Rust와 양 platform trace가 모두 확인한다.
10. **max-content 확장** — separator 한 개/두 개에만 `10px+d`가 적용되고 −1/0/+1px 결과가 independent Ahem 계산과 일치한다.
11. **separator 없는 control** — 한 단어 scene은 spacing 값이 달라도 min/max width가 일정하다.
12. **computed value distinction** — authored `normal`과 `0px`는 서로 다른 고정 scene이지만 computed value와 used geometry는 둘 다 0px다.
13. **serialization strictness** — 값은 exact scene serialization에 묶이며 `-0px`, `0.0px`, `+1px`, `1.00px`, unit mismatch와 unknown scene mutation은 거부된다.
14. **text input source ownership** — anonymous source node가 item node와 같은 HostDocument node라는 점을 trace 검증에 반영했고 raw text/node/inline ancestor를 확인한다.
15. **document revision isolation** — runtime expected report에서 같은 HostDocument source revision과 finite nonnegative revision fields를 요구한다.
16. **intrinsic record 과소 검증** — min/max size뿐 아니라 사용된 text layout, line count/advance, item frame, 최종 Columns/Rows track과 contribution kind/value도 확인한다.
17. **row auto-sizing 영향** — min-content 여러 줄과 max-content 한 줄의 row height가 root/item frame 및 Rows track에 이어지는지 확인한다.
18. **trace report compatibility** — `wordSpacing`은 기존 `paragraphStyle` JSON object에 additive field로 추가되고 fixed expected scene 검사에서 값도 비교한다.
19. **FFI scene catalog containment** — generator가 고정 24 ID만 만들고 unlisted ID는 runtime source를 만들지 않는다. fixture crate module boundary test에 연결했다.
20. **fixture feature gating** — Rust scene/validator 경로는 `c04-runtime-gpu-fixture-test` 아래이며 product `c04-runtime-gpu` 기본 archive에 candidate symbols/payload가 없다.
21. **Android build define** — Java expectation route는 fixture `BuildConfig` 조건을 확인하고 Android API 37 AVD에서 route가 실제 실행됐다.
22. **Android diagnostic checks** — source identity, exact computed style, contributions, final tracks, frame, status와 trace completeness를 검증한다.
23. **iOS compile define** — Swift helper는 simulator fixture build define 아래에 있고 재빌드가 성공했다.
24. **iOS diagnostic parity** — Swift validator가 Android와 같은 style/trace/geometry contract를 검사하며 최종 새 executable에서 24/24를 다시 실행했다.
25. **scene expectation duplication** — native host formula와 Chrome-derived independent expectation JSON을 capture runner에서 교차 비교하므로 host 자체 expected frame만으로 통과하지 않는다.
26. **capture timeout/partial output** — summary 누락, error marker, trace chunk 누락/중복과 잘못된 JSON은 실패 처리된다. 최종 matrix에는 각 플랫폼 24개 log row가 있다.
27. **installed binary identity** — Android APK 및 최종 iOS executable의 build hash와 installed hash가 일치한다. iOS 재빌드 후 이전 실행값을 재사용하지 않고 24-case matrix를 새로 실행했다.
28. **default/positive artifact boundary** — Homebrew `llvm-nm` 22.1.8 및 archive bytes 검사에서 제품 archive 0 symbol·0/24 payload, fixture positive control 2 symbols·24/24 payload다. 보고서가 정적 라이브러리 검사를 app bundle로 확대하지 않는다.
29. **화면 증거와 이미지 URL** — simulator 캡처는 검은 renderer를 glyph proof로 쓰지 않는다. PR raw image URL의 최초 SHA 오타를 Agent Browser에서 발견해 실제 PR head SHA로 고친 뒤 두 이미지의 `naturalWidth`를 Android 1080·iOS 1206으로 확인했다.
30. **공식 진행 및 병합 경계** — `spec/STATUS.md`는 후보 증거만 추가하며 C11.4/C11 및 공식 `72/213`을 유지한다. PR check은 보고되지 않았고 main에는 required status checks가 없음을 PR metadata/API에서 확인했다. 이 자체는 CI pass가 아니다.

## 판정

30개 diff 관점 중 구현 결함은 남지 않았다. 리뷰 중 발견한 PR screenshot URL SHA 오류는 수정해 브라우저에서 재검증했다. 나머지 한계—검은 renderer, simulator 전용, 제한 24 scene, 일반 CSS admission/WPT 부재—는 본문과 증거에 명시했다. 병합은 `Rebase and merge`만 허용한다.
