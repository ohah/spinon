# C11.4 anonymous Grid `text-indent` 구현/runtime 실패 관점 20개

계획 검토와 분리해 `ParagraphStyle`/Parley intrinsic sizing, Stylo→runtime fixture 경계, FFI trace, Android·iOS adapter, 설치 artifact와 simulator 실행을 다시 확인했다. 각 항목은 별도의 실패 경로나 계약 경계를 다룬다.

1. **일반 CSS profile로의 우발적 admission** — 새 동작은 C11.4 anonymous direct-text opt-in fixture의 고정 scene 목록으로만 진입한다. 일반 cascade/author stylesheet admission을 추가하지 않았다. 미등록 scene은 `prepare`와 FFI source에서 닫힌다. 통과.
2. **기존 `ParagraphStyle` 호출자의 기본값 회귀** — 새 필드는 기본 0 CSS px이며 기존 직접 생성 지점에 명시적 0을 넣었다. `paragraph_style_default_keeps_the_legacy_horizontal_text_profile` 및 `spinon-text-layout` 76 tests가 통과했다. 통과.
3. **NaN 또는 무한 indent 전파** — `ParagraphStyle::validate`가 유한값을 요구한다. `paragraph_style_rejects_invalid_family_and_negative_or_non_finite_metrics`에서 NaN 입력을 거부한다. 통과.
4. **첫 줄 다음에도 indent 반복 적용** — Parley는 first-line indent를 배치하고 intrinsic 계산은 각 soft line을 순회하며 첫 줄에서만 indent를 더한다. `X XXXXX` 및 `XXXXX X` 불균등 단어 검사가 각각 50px·55px min-content를 확인한다. 통과.
5. **긴 후속 단어보다 첫 단어 indent를 무조건 선택** — `X XXXXX`에서 `10+5 < 50`인 경우 min-content가 50px인 것을 새 unit test로 검증했다. 통과.
6. **첫 단어가 가장 길 때 indent 기여 누락** — `XXXXX X`에서 첫 줄의 `50+5`가 min-content 55px가 되는 것을 같은 test로 검증했다. 통과.
7. **max-content에 indent 중복 합산** — 두 단어 Ahem 입력의 max-content는 글리프/분리자 advance에 첫 줄 indent를 한 번 더한 55px다. unit 및 18-scene 독립 기대값과 양 플랫폼 trace가 일치한다. 통과.
8. **line advance와 indent 위치를 하나의 폭으로 합침** — text layout의 line advance는 unindented glyph advance이고 available width/track은 indent를 포함한다. 첫 glyph x와 line advance를 따로 검사한다. Android/iOS 진단과 Rust tests에서 일치한다. 통과.
9. **공백 정규화로 min/max 식 변경** — `white-space:normal`에서 반복·끝 공백이 collapse된 `XX   XX XX  ` source를 사용한다. source text와 normalized layout text, 80px unindented max advance 및 5/10px indent 차이를 18개 행렬에서 확인한다. 통과.
10. **indent가 줄 수·행 높이를 바꾸는 오류** — 1·2·3 단어 min-content 행은 각각 10·20·30px, max-content 행은 10px로 유지된다. Runtime의 Rows 최종 track 및 두 simulator trace가 모두 맞는다. 통과.
11. **음수 값을 고정 fixture 지원으로 오인** — CSS상 유효한 negative value는 이 18-scene profile 밖이다. Runtime validator와 사전 layout 거부 test가 `-5px`를 거부하며 CSS 전반의 무효값이라고 문서화하지 않는다. 통과.
12. **percentage/keyword 확장** — `5%`, `hanging`, `each-line`은 독립 CSS 문법상 무효라고 부르지 않고 이 px-only fixture parser에서 거부한다. validator test에서 확인했다. 통과.
13. **scene과 computed style serialization 불일치** — scene별 정확한 `0px`/`5px`/`10px`을 같은 computed style snapshot에서 대조한다. 다른 고정값·`5.0px`·미등록 scene과의 mismatch는 publication 전에 실패한다. 통과.
14. **기존 scene에 예상치 못한 nonzero indent 누출** — 등록되지 않은 모든 anonymous text scene은 computed `text-indent:0px`을 요구한다. 새 5px scene이 기존 zero profile 검사를 느슨하게 만들지 않았다. 통과.
15. **잘못된 direct child topology 또는 source 혼합** — 고정 scene별 단일 direct Text source, source 문자열 및 root style을 검사한다. 추가 child나 다른 source는 anonymous text validator에서 거부된다. 통과.
16. **vertical intrinsic 경로로 profile 확대** — nonzero `text_indent`는 현재 fixed vertical intrinsic profile의 지원 조건을 벗어난다. 새 rejection assertion과 text-layout tests가 통과했다. 통과.
17. **stale HostDocument/style revision에서 값 재사용** — runtime 검증은 같은 HostDocument snapshot의 computed property와 문서 revision을 연결한다. 통합 test와 Android/iOS trace에서 source identity와 nonnegative revision tuple을 확인했다. 통과.
18. **track adapter가 intrinsic 값을 잘못 소비** — source min/max, item frame, Columns/Rows의 최종 track과 track contribution을 각각 확인한다. Runtime 356 passed 및 FFI 177 passed, 두 simulator matrix가 18/18 통과했다. 통과.
19. **iOS scene 진입·text width 기대값 불일치** — 첫 실행 검토에서 iOS의 scene gate 누락과 positive indent에서 text width를 track width로 기대하던 오류를 발견해 수정했다. Swift 재빌드 후 iPhone 17 Pro/iOS 26.2 Simulator 18/18이 통과했다. Android 기대값도 같은 규칙으로 맞춰 재빌드하고 API 37 ARM64 AVD 18/18을 재확인했다. 통과.
20. **fixture 코드나 payload가 기본 제품 archive에 포함** — 기본 release FFI archive에서 text-indent scene ID 0개, adapter/validator symbol 0개이며 opt-in positive control에서 18 scene ID와 두 symbol이 있다. 이 판정은 macOS static archive에 한정한다. APK/IPA 전체 제외로 확대하지 않는다. 통과.

**판정:** 발견된 iOS scene gate와 양 플랫폼 harness의 text width 기대값 오류를 수정하고 영향받은 iOS build/runtime 및 Android build/runtime를 다시 실행했다. Rust unit/integration, runtime, FFI, fixture boundary, 관련 Clippy와 archive positive/negative controls가 통과했다. `spinon-style-to-layout` 전체 테스트는 변경과 무관한 C07.3 pinned aspect-ratio 문자열 기대값 차이 1건(`1.77778 / 1` 기대, `1.7777778 / 1` 실제)으로 267/268이며 [별도 실행 로그](style-to-layout-pass01.log)에 남겼다. Simulator PNG는 C11.4 앱 상태 화면과 검은 renderer 영역을 보여주며 glyph paint·PixelCopy/Metal drawable·compositor 이후 pixel equality의 증거가 아니다. C11/C11.4는 미완료이고 공식 집계 `72/213`을 유지한다.
