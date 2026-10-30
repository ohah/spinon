# C11.4 anonymous Grid text `white-space:pre-wrap` PR diff 실패 관점 30개

계획 검토 20개와 구현/runtime 검토 20개를 재사용하지 않고, 구현 후보의 변경 diff·플랫폼 fixture·문서·배포 경계를 PR 관점에서 확인했다. 기준은 plan PR #869가 병합된 base `47ffeb1216a1085cb40c505f312108dea66c600c`와 현재 구현 후보 diff다.

1. **추가 enum variant의 exhaustive match 누락** — workspace Clippy 및 변경 crate opt-in compile이 완료되어 `WhiteSpaceMode::PreWrap`을 소비하는 exhaustive match 누락이 없다. 통과.
2. **normalization의 오류 반환 누락** — `normalize`가 `Result`로 바뀌고 `TextLayoutEngine`이 `?`로 tab profile 오류를 전달한다. 호출 경로에서 `unwrap`이나 panic을 추가하지 않았다. 통과.
3. **source UTF-16과 paragraph UTF-8 offset 교환** — `SourceCharacter`는 `char::len_utf16()` 범위를 유지하고 output은 UTF-8 byte range를 별도 기록한다. CRLF source/output mapping 테스트와 tab expanded range 테스트가 확인됐다. 비-BMP 새 profile은 별도 검증하지 않았다고 한정한다. 통과.
4. **여러 Text node에 걸친 CRLF 중복·절단** — source flatten 이후 CRLF를 감지하고 두 source range를 한 LF output range로 연결한다. 같은 node와 두 node 분할 입력이 각각 시험됐다. 통과.
5. **Form Feed segment break 누락** — PreWrap branch는 LF·CR·Form Feed를 LF line break로 넘긴다. Form Feed 두 줄 테스트가 있다. 통과.
6. **반복·앞·뒤 U+0020의 조기 collapse** — PreWrap branch가 Normal의 pending-space collapse 로직에 들어가지 않는다. `XX  XX`, `  XX`, `XX  `의 text/intrinsic 결과를 검사한다. 통과.
7. **공백 전용 paragraph가 익명 item을 생성** — whitespace-only 앞 텍스트와 block marker를 분리한 fixture에서 marker가 첫 cell `(0,0)`에 놓인다. Layout track trace와 양 플랫폼 runtime가 같은 값을 반환했다. 통과.
8. **CSS Text `pre` 또는 `break-spaces`까지 잘못 허용** — `WhiteSpaceMode`에는 `PreWrap`만 추가하고 Grid gate의 다른 기존 허용 목록은 유지한다. 범위 밖 white-space mode는 계속 거부한다. 통과.
9. **PreWrap의 wrapping 값이 NoWrap으로 투영** — text engine은 `TextWrapMode::Wrap`으로 map하고 `WhiteSpaceCollapse::Preserve`를 지정한다. runtime validator가 두 computed longhand를 검사한다. 통과.
10. **기존 Ahem fast path가 잘못 적용** — Normal/NoWrap 전용 분기가 PreWrap을 포함하지 않는다. opt-in intrinsic/layout tests에서 preserve behavior를 확인했다. 통과.
11. **tab stop을 문자 개수가 아니라 CSS column으로 오해** — tab 처리는 Ahem ASCII `XX\tXX`만 허용하며 column 2에서 8-column stop까지 6 spaces로 확장한다. 이 고정 profile max-content 100px와 source tab `2..8` range를 단위 테스트한다. 통과.
12. **tab profile에서 inline style이 폭을 바꾸는데 허용** — `inline_styles`, `inline_run_styles`, 각 source의 `inline_ancestors`가 비어 있지 않으면 typed unsupported error로 닫는다. nested source rejection 테스트가 있다. 통과.
13. **runtime이 authored `tab-size`를 검사한 것으로 과장** — projection에 `tab-size`가 없는 사실과 앱 scene에 tab이 없는 점을 plan·0059·README에 명시했다. 이 구현은 일반 authored tab layout으로 소개하지 않는다. 통과.
14. **layout soft-wrap line의 hanging advance를 track에 더함** — text test는 visible 20px/hanging 10px를 분리하고 layout width를 20px로 확인한다. Layout test는 final contribution의 20px 값을 확인한다. 통과.
15. **min-content 또는 max-content 계산만 검증하고 실제 frame 누락** — repeated-space max/min, soft-wrap, forced-break 각 scene에서 intrinsic sizes와 root/item frames 및 양 축 track 결과를 Rust/layout/runtime 층별로 확인했다. 통과.
16. **comment split을 실제 comment node로 검증했다고 오인** — 앱 fixture source는 두 direct Text node이며 Comment node는 없다. Chrome oracle 대응 경계라고만 기술하고 README·0059에 차이를 표시했다. 통과.
17. **source split 시 한 source provenance만 남음** — 두 direct Text source의 `TextSourceRange`가 각각 node/source offset을 유지하는 layout test와 runtime report가 있다. 통과.
18. **C11.4 외부 경로에서 PreWrap 허용** — 변경은 C11.4 anonymous paragraph gate와 기존 opt-in runtime fixture route에 한정된다. 일반 author CSS admission 또는 root HTML mapping 변경은 없다. 통과.
19. **`order != 0` fail-closed 삭제** — collector의 `layout_style.order != 0` 조건을 유지했고 PreWrap reorder test가 `UnsupportedTextParagraph`를 확인한다. 통과.
20. **빈 explicit 비텍스트 Grid가 text ratio adapter로 들어감** — 전체 opt-in runtime tests에서 발견한 false routing을 수정하고 `empty_explicit_grid_item_does_not_enter_the_text_only_span_adapter`를 추가했다. runtime suite 수정 후 327 passed, 0 failed, 3 ignored다. 영향 경계 통과.
21. **기존 explicit text ratio/minmax scene가 새 guard에서 차단** — guard는 explicit placement가 있더라도 meaningful item/anonymous text가 모두 없는 경우만 조기 반환한다. 기존 ratio/multispan tests와 전체 opt-in runtime suite를 다시 통과했다. 통과.
22. **fixture scene ID의 Android/FFI 불일치** — Rust FFI, Android, iOS, capture runner의 16개 ID allowlist를 대조했다. 신규 7개가 각 플랫폼 기대값 목록에도 들어 있고 FFI test는 중복/unknown ID를 거부한다. 통과.
23. **scene별 text input과 computed style의 mismatch** — 각 PreWrap scene은 `white-space-collapse:preserve`와 `text-wrap-mode:wrap`을 사용하고 LF, repeated spaces, split source, before-block input이 해당 ID로 고정된다. FFI source matrix test와 simulator logs가 일치한다. 통과.
24. **Android expected frame과 track만 추가하고 draw completion 누락** — API 37 ARM64 AVD에서 최종 16/16 장면이 layout-ready, frame/track, draw/present 및 zero superseded retries를 통과했고 설치 APK hash가 빌드 APK와 일치한다. 통과.
25. **iOS Simulator expected frame과 track만 추가하고 surface 제출 시퀀스 누락** — iPhone 17 Pro/iOS 26.2에서 최종 16/16 장면이 capture runner 검사를 통과했고 설치 app hash가 빌드 app과 일치한다. 증거는 compositor 이후 pixels나 glyph ink equality라고 확대하지 않는다. 통과.
26. **blank harness PNG를 동작 픽셀 증거로 제출** — 두 최종 PNG는 앱의 dark harness field와 status만 보이고 text glyph가 보이지 않는다. 이를 PR의 앱 동작 screenshot으로 embed하지 않고 runtime README에서 pixel oracle가 아니라고 한정한다. 통과.
27. **capture script와 fixture boundary test의 신규 scene stale hardcode** — fixed matrix expectation·scene names·source classifier를 함께 갱신했고 Node fixture module boundary tests 24/24가 통과했다. 전체 JS boundary 범위를 초과하는 주장 없음. 통과.
28. **기본 제품 artifact로 fixture ABI/payload 유입** — macOS ARM64 Rust 1.96.1 static archive product/positive-control scan에서 marker 0/7·export 0개 대 marker 7/7·export 2개를 확인한다. APK/IPA 전체 제외로 확대하지 않는다. 통과.
29. **미완료 후보를 공식 완료 수나 내부 계약 버전으로 승격** — `spec/STATUS.md`, public roadmap, internal contract는 C11/C11.4 미완료와 `72/213`을 유지하고 내부 계약 숫자 버전 `0.1.0`도 유지한다. 통과.
30. **실패한 전체 workspace test를 숨기거나 patch 원인으로 단정** — `cargo test --workspace`의 `spinon-style-to-layout` C07.3 pinned-Chromium format assertion은 기대 `1.77778 / 1`, actual `1.7777778 / 1`로 기록했다. 해당 crate는 후보 diff에서 수정하지 않았으며 base에서도 실패했는지는 별도 증거가 없어 주장하지 않는다. PR/CI 결과와 함께 공개해야 한다. 통과.

**판정:** 이 diff에서 새로 수정할 blocker를 찾지 못했다. 이 검토는 PR 열기 전 로컬 diff 검토이며, PR 생성 후 final head와 check 상태를 확인하고 문서에서 PR 상태를 갱신한다. 최종 PR을 병합하려면 이 30관점 검토를 final diff에 연결하고 required checks 결과와 merge state를 별도로 확인해야 한다.
