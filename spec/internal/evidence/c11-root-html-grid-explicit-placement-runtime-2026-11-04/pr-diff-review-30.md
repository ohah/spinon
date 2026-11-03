# 구현 PR diff 적대 검토 30개 · pre-PR · 2026-11-04

**대상:** `impl/c11-root-html-grid-explicit-placement`의 staged patch, base `1a12ca5a405067363b97027e20afcca07c870770` (`origin/main`과 동일). 이 검토는 구현/runtime 20관점 기록과 별도다. 검토를 시작할 때 구현/evidence patch는 26 files, 2,473 insertions, 12 deletions였다. review record를 포함해 현재 staged diff는 27 files다.

1. **상태 ID 경계** — 이 diff가 C11.1 하위 제한 slice인지 C11 parent 완료인지 검사했다. plan, STATUS와 계약에서 후보로 부르고 상위 C11 checkbox를 바꾸지 않는다. 통과.
2. **공식 완료 집계** — 새 구현을 `72/213`에 더했는지 확인했다. STATUS·plan·0059 모두 `72/213`을 유지한다. 통과.
3. **하위 C11 상태 전이** — 구현 후보가 C11.3/C11.3.3/C11.4 또는 C11.5 전체 완료를 의미하는지 검사했다. 해당 checklist를 변경하지 않았다. 통과.
4. **API/계약 versioning** — 새 public API나 ABI가 생겨 0059 version을 올려야 하는지 확인했다. 추가 parser/ABI/API 없이 기존 C11.2.1 normalizer에 연결하고 내부 계약 `0.1.0`을 유지한다. 통과.
5. **계획의 실행 상태** — plan이 아직 “구현 전”이라고 말하거나 merge 완료로 오인시키는지 검사했다. Chrome/Rust/Simulator 완료, 20관점 완료, PR 30관점·PR·merge·Tailnet 미완료가 분리됐다. 통과.
6. **computed value provenance** — candidate selector가 inline/source declaration 문자열로 배치를 파싱하는지 검사했다. `ComputedStyleSnapshot`의 typed Grid layout 및 `grid_item_placement`만 사용한다. 통과.
7. **owner 경계** — 한 Grid의 placement mode가 sibling owner에게 적용되는지 검사했다. item을 owner별로 묶고 Rust sibling Grid test에서 all-auto frame을 별도로 확인한다. 통과.
8. **visible item 전체성** — 일부 child만 검사해 partial scene을 선택하는지 검사했다. owner item 전부가 네 endpoint에서 numeric `Line|Span`이어야 하며 mixed case는 scene 전체를 거부한다. 통과.
9. **flow-mode 범위** — row explicit route가 column, row dense 또는 column dense를 가로채는지 검사했다. non-row flow와 이미 소유된 dense owner를 제외한다. 통과.
10. **named-area 범위** — areas Grid가 no-area route를 통해 validator를 우회하는지 검사했다. candidate에서 area Grid를 제외하고 named-area validator와 기존 route를 유지한다. 통과.
11. **placement algorithm 중복** — 새 track/line algorithm이나 shorthand parser를 도입했는지 검사했다. `ExplicitOnly`에 기존 `project_grid_item_placement`와 C11.2.1 normalizer를 사용한다. 통과.
12. **line/span 경계** — zero line, negative line, reversed endpoints, span과 산술 범위가 누락됐는지 검사했다. zero-line runtime rejection, Rust expected frames와 C11.2.1 placement 15/15로 확인한다. 통과.
13. **mixed/partial auto 오류** — mixed placement를 성공으로 돌리거나 일부 축만 고정하는지 검사했다. both runtime and projection tests가 diagnostic/error로 scene을 거부한다. 통과.
14. **implicit track 경계** — explicit grid 밖 placement가 새 implicit track을 조용히 생성하는지 검사했다. 해당 route는 explicit range만 허용하고 runtime row overflow case가 atomic rejection을 확인한다. 통과.
15. **hidden child semantics** — `display:none` child의 out-of-range placement가 visible siblings를 막거나 cell을 점유하는지 검사했다. hidden control은 zero frame이며 visible item은 authored cell을 유지한다. 통과.
16. **anonymous text inventory** — element-only enumeration 때문에 direct text item이 누락되는지 검사했다. HostDocument child inventory가 anonymous auto item을 보존하며 mixed scene을 거부한다. 통과.
17. **nested Grid admission** — explicit parent가 direct child Grid를 empty block item으로 오인하는지 검사했다. Root HTML block-item gate가 nested `display:grid`를 거부하는 runtime negative를 둔다. 통과.
18. **replaced item admission** — `img`가 numeric placement만으로 fixed empty `div` profile을 통과하는지 검사했다. 기존 image item guard의 runtime negative를 둔다. 통과.
19. **overlap order** — 유효한 overlapping item을 거부하거나 source order를 바꾸는지 검사했다. 같은 frame 허용과 source-order render box index를 style/runtime에서 확인한다. 통과.
20. **C11.5 style carrier** — `ExplicitOnly` 설정이 alignment computed values를 덮거나 제거하는지 검사했다. placement mode만 갱신하고 item `align-self`/`justify-self` 값을 보존하는 projection과 expected frame test를 둔다. 통과.
21. **failure atomicity** — invalid scene이 partial/stale `completed`를 노출하는지 검사했다. runtime negative에서 `completed=None`을 확인하며 같은 session의 다음 유효 environment는 ready snapshot으로 회복한다. 통과.
22. **fixture 크기와 접힘** — 6개 scene이 320×240 CSS viewport에 들어가 다음 scene·overflow sample과 겹치지 않는지 검사했다. 각각 40px 높이로 240px에 맞고 first scene의 20px horizontal overflow는 의도적이다. 통과.
23. **Chrome observation provenance** — runtime JSON이 기대값을 관찰 자료로 위장하는지 확인했다. 관찰 JSON과 별도 11-scene precomparison/expected frames의 책임을 분리한다. 통과.
24. **Chrome 화면 대조** — 첫 item/parent frame과 visible overflow pixel 표본이 일치하는지 확인했다. observation은 item `[40,0,100,30]`, Grid `[0,0,120,40]`; PNG sample에서 x=125는 red, x=145는 white다. 통과.
25. **Android 설치 대상** — simulator evidence가 실기기 결과로 표시되는지 검사했다. API 37 ARM64 `emulator-5554`만 사용했고 physical USB serial `R5KYB06SMPE`는 설치/실행하지 않았다고 기록한다. 통과.
26. **Android renderer 범위** — software renderer를 hardware GPU라고 오인할 위험을 확인했다. log와 README는 ANGLE/Vulkan/SwiftShader software renderer로 한정한다. 통과.
27. **iOS platform target** — Simulator app build/draw를 실기기나 compositor readback으로 확대하는지 검사했다. iPhone 17 Pro iOS 26.2 Simulator, Metal drawable 960×720 및 `presented` log만 주장한다. 통과.
28. **화면 표본 경계** — Android/iOS screenshot RGB sample을 PixelCopy/Metal readback으로 부르는지 검사했다. JSON/README는 screen screenshot에서 읽은 값이고 renderer-surface parity가 아니라고 명시한다. 통과.
29. **asset·log·baseline 보존** — temporary app assets 복원, Xcode log 정리, baseline 실패가 숨겨지는지 검사했다. 두 asset SHA가 원본과 같고 raw Xcode log는 gzip으로 보존했으며 alignment baseline 실패 log를 포함한다. 통과.
30. **staging·문서·게이트 일치** — unrelated working data나 초기 캡처가 PR에 섞이는지 확인했다. staged list는 구현·문서·최종 evidence 26 files와 이 review record 한 개만 포함하고 이전 300px/대체 캡처 8개와 나머지 untracked evidence는 local에 남겨뒀다. `git diff --cached --check`, focused tests, rustfmt와 scoped Clippy는 통과했고 PR check/merge는 아직 없다. 통과.

## 변경 중 관찰 및 처리

- 처음 기록한 Xcode build log에는 출력 줄 끝 공백이 있어 `git diff --cached --check`에서 걸렸다. 원본을 `ios-build-raw.log.gz`로 byte-preserving 보존하고 읽기용 log의 trailing whitespace만 정리한 뒤 gate가 통과했다.
- Runtime recovery fixture의 `background` shorthand가 제한 cascade profile에서 `background-position-x` 진단을 만들었다. test input만 `background-color`로 좁혔고 production code는 이 관찰 때문에 바꾸지 않았다.
- `root_html_grid_item_alignment` broad filter의 4개 실패는 변경 전 HEAD에서도 재현됐다. baseline log가 evidence folder에 있고 이번 PR의 pass로 계산하지 않는다.
