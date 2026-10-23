# C11.4 definite percentage auto-repeat maximum 구현/runtime 실패 관점 검토 20개

검토 대상은 현재 작업 브랜치의 cascade, layout validation, vendored Taffy 연결, fixture route와 Android/iOS simulator 결과다. 계획 검토를 재사용하지 않았다. 이 문서는 PR diff 적대 검토와도 분리한다.

## 검토 결과

1. **CSS cascade가 미지원 maximum을 통과시키는 경로** — `auto_repeat_maximum_is_supported`는 유한한 px/percentage/fraction과 제한 calc만 보존하고 auto/min-content/max-content는 거부한다. `min(20px,10%)`, `calc(20px - 10%)`는 cascade 실패 테스트로 확인했다. 결과: 통과.
2. **음수·NaN·무한대 percentage maximum** — cascade breadth validation과 layout track validation 모두 유한성 및 0 이상을 요구한다. layout maximum 경로도 같은 검사를 한 번 더 한다. 결과: 통과(경계는 코드와 기존 validation으로 확인).
3. **percentage maximum을 indefinite 축에서 해석하는 경로** — layout은 definite-axis bitmap과 양수 content-axis basis를 모두 요구한다. indefinite 폭 fixture는 Taffy sizing 전에 `InvalidGridTrackList`를 반환한다. 결과: 통과.
4. **0 또는 음수 content-axis 크기** — 양수 fixed dimension 및 최종 content size `> 0`을 요구한다. 0 폭 fixture와 border-box padding이 content 영역을 소진하는 fixture가 오류를 확인한다. 결과: 통과.
5. **min/max constraint가 percentage basis를 바꾸는 경우** — 이 slice는 min-size 0, max-size auto만 받는다. `max-width` constraint fixture는 거부되고, 다른 축 조건은 공통 helper의 axis field 선택을 확인했다. 결과: 통과; min-size가 0이 아닌 경우와 max-height 제한은 이 제한 profile 밖이다.
6. **columns/rows basis 혼동** — columns는 width와 좌우 inset, rows는 height와 상하 inset을 선택하고 calc의 GridTemplateColumns/Rows property binding도 일치시킨다. 양 축 단독 및 동시 scene이 simulator에서 실행됐다. 결과: 통과.
7. **border-box의 outer size를 percentage basis로 잘못 쓰는 경우** — border-box dimension에서 고정 padding과 border를 빼 content size를 계산한다. columns percentage와 rows calc inset 회귀가 각각 44px/37.6px track을 기대하는 layout test로 통과했다. 결과: 통과.
8. **percentage 곱셈의 f32 overflow** — direct percentage maximum은 `value * basis`가 finite인지 layout gate에서 검사한다. `f32::MAX` percentage와 184px basis가 Taffy에 전달되지 않는 회귀 테스트를 추가해 확인했다. 결과: 통과.
9. **maximum이 minimum보다 작은 경우** — CSS Grid/Taffy의 definite max를 min으로 floor하는 경로를 고정 px control과 비교했다. 두 축과 auto-fill/auto-fit에서 3회 반복과 46px track이 일치했다. 결과: 통과.
10. **0% maximum** — zero maximum이 minimum보다 작으면 minimum으로 floor되는지 두 축·두 반복 방식의 고정 control과 비교했다. 결과: 통과.
11. **calc AST가 허용 범위를 벗어나는 경우** — maximum은 정확히 두 항의 Sum, length와 percentage 한 개씩만 허용한다. `max(20px,10%)`, 음수 length/percentage, 세 항 Sum을 추가 layout 테스트로 fail-closed 확인했다. 결과: 통과.
12. **calc operand 순서와 항 값** — layout과 cascade 양쪽에서 length+percentage 및 percentage+length 순서를 테스트했다. length `> 0`, percentage `>= 0`, 두 값 finite 규칙은 unlisted tree 및 비유효값 검사로 확인했다. 결과: 통과.
13. **calc id가 다른 node의 식을 가리키는 경우** — 전체 css-math binding validation이 모든 참조를 node/property에 묶고, auto-repeat gate도 해당 node를 다시 확인한다. 결과: 통과(공통 validation 경계와 auto-repeat property mismatch 검사).
14. **calc id가 다른 CSS property의 식을 가리키는 경우** — grid track math reference validation과 새 auto-repeat 검사에서 axis property를 비교한다. columns가 rows calc id를 받는 경우 `CssMathBindingMismatch`가 발생하는 layout test가 있다. 결과: 통과.
15. **중복·누락 calc id 또는 오래된 calc handle** — 입력 단계에서 duplicate id와 missing reference를 거부하고, `CalcOwner`의 Arc 소유권이 Taffy 계산 종료까지 유지된다. 최대값은 기존 typed calc handle 변환을 재사용한다. 결과: 통과(입력 validator 및 handle 수명 경로 확인).
16. **repeat count에서 최대값 대신 최소값을 사용하거나 gap을 누락하는 경우** — vendored Taffy의 explicit-grid 계산은 definite maximum을 먼저 택하고 minimum으로 floor한 뒤 gap을 더한다. 184px axis, 8px gap 결과는 percentage/calc와 fixed controls에서 동일한 3/4 repeat count를 냈다. 결과: 통과.
17. **auto-fill과 auto-fit collapse 시점 혼동** — fill은 빈 반복 트랙을 유지하고 fit은 item placement 후 빈 트랙과 gutter를 collapse한다. 양 축 fixture/layout tests에서 repeat 수와 collapse bitmap을 분리해 확인했다. 결과: 통과.
18. **양 축 동시 calc에서 node/property id가 섞이는 경우** — both-axis percentage/calc scene은 각 축의 independent template property와 fixed controls를 둔다. Android/iOS의 both-axis runtime trace 및 node frames가 모두 통과했다. 결과: 통과.
19. **simulator sample이 track 경계나 다른 node를 잘못 검증하는 경우** — 고정 sample 검증은 scene allowlist, target node id, 예상 item interior 좌표, 색상과 sample 수를 모두 검사한다. Android PixelCopy와 iOS Metal drawable 로그 각각 28 scene summary 및 78/78 exact sample을 확인했다. iOS 표본은 compositor 제출 전이고 Android는 ANGLE/SwiftShader다. 결과: 해당 경계에서 통과.
20. **fixture 전용 코드를 제품 artifact나 완료 상태로 과장하는 경우** — feature/module boundary Node test와 제품·fixture FFI archive 대조에서 제품 archive의 fixture symbol/payload 부재, opt-in archive의 positive control을 확인했다. 이는 APK/IPA 전체 exclusion, 일반 CSS, WPT, 실기기, hardware GPU 또는 C11.4 완료를 증명하지 않는다. 상태 대장과 roadmap은 C11.4 checkbox 및 `72/213`을 유지한다. 결과: 범위 표기 통과.

## 현재 재실행한 검증

- `mise exec -- cargo test --locked --workspace`: 통과(2026-10-23).
- `mise exec -- cargo fmt --all -- --check`: 통과.
- `mise exec -- cargo clippy --locked --workspace --all-targets -- -D warnings`: 통과.
- `node --test tools/css-reference/fixture-module-boundary.test.mjs`: 7/7 통과.
- `git diff --check`: 통과.
- Android API 37 ARM64 AVD: 28/28 scene, 78/78 PixelCopy sample exact; ANGLE/SwiftShader software renderer.
- iPhone 17 Pro / iOS 26.2 Simulator: 28/28 scene, 78/78 Metal drawable sample exact; compositor 제출 전.

런타임 구현 결함은 발견하지 못했다. 검토에서 드러난 calc 항 순서와 percentage 곱셈 overflow의 테스트 공백은 회귀 테스트로 보완하고 다시 실행했다. 위 결과는 opt-in fixture slice에 한정한다. 구현 변경과 공개 범위를 대상으로 한 별도 PR diff 실패 관점 30개는 [검토 기록](./pr-diff-review-30.md)에 남겼다. PR #568은 로컬·GitHub diff 일치와 본문 화면 로딩을 확인한 뒤 `c436a38388fe8a191fd374495c516c88d1e3cd54`로 병합됐으며, check rollup은 비어 있었다.
