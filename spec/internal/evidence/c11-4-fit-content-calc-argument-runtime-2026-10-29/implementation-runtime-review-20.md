# C11.4 definite calc fit-content runtime · implementation/runtime failure review 20

대상은 [`fit-content-calc-percentage-row-wpt`](./README.md)의 제한 opt-in slice다. 각 관점은 실패할 때 예상되는 거부·검증 실패 경로와 근거를 따로 확인했다. Rust/FFI/fixture tests와 시뮬레이터 전체 결과는 README의 실행 근거를 따른다. 전체 체크박스나 일반 CSS 지원을 승격하지 않는다.

최종 검토 때 computed style·typed GridInput·CSS math registry 검증 함수를 610줄 fixture 파일에서 private `style_validation.rs` 모듈로 분리한 뒤 20개 관점을 다시 확인했다. 추출된 함수의 비교 조건·오류 경로와 parent의 호출 인자를 대조했고, module boundary 검사에 새 파일 경로와 핵심 조건을 추가했다. 런타임 4/4, FFI 1/1, boundary 2/2, runtime/FFI Clippy, 양쪽 앱 빌드와 시뮬레이터 실행을 최종 tree에서 재검증했다.

1. **Pinned input drift:** WPT revision/source SHA가 달라지면 오래된 oracle을 쓰지 않도록 plan 및 precomparison source hash를 대조했다. 현재 캡처는 pinned `c999c583…`를 기준으로 한다.
2. **Unknown scene alias:** 비등록 root ID는 이 adapter에서 `None`이고 다른 ID를 calc scene으로 오인하지 않는다. FFI source allowlist 테스트는 두 고정 scene 외 노출을 막는다.
3. **Conflicting runtime profile:** `RuntimeBlockFlexGridTrackSizingV1` 외 profile은 준비 단계에서 거부한다. Runtime exact-route test 통과.
4. **Wrong root element:** non-HTML namespace, non-`div`, nested root는 element/parent gate에서 거부한다. Code-path review.
5. **Extra direct item:** root direct child 수가 하나가 아니면 거부한다. `prepare` child-count gate 검토.
6. **Wrong grid item identity/parent:** item ID가 다른 경우나 root 직계 자식이 아닌 경우 거부한다. Code-path review.
7. **Missing or reordered child:** item 안 child 수/부모/ID가 바뀌면 거부한다. Code-path review.
8. **Style node set drift:** style snapshot에 root/item/child 외 node가 있으면 거부한다. Exact style-node set 검증 확인.
9. **Wrong percentage basis:** root height 또는 typed height가 300px이 아니면 computed-style 검증에서 거부한다. Runtime fixture는 definite-height only.
10. **Wrong CSS argument:** percentage `50%`, `calc(10% + 4px)`, 항 순서/구조 변경은 정확한 computed property와 typed `ComputedCssMath` 비교에서 거부한다. Runtime test는 calc/percentage positive control 및 잘못된 trace를 다룬다.
11. **Wrong projected track:** LayoutInput의 column/explicit row/auto-flow/placement/track shape 변경을 거부한다. `fit_content_calc...35px_trace` focused test 통과.
12. **Missing calc handle:** `FitContent(Calc(id))`에서 ID를 뽑지 못하면 거부한다. Runtime positive control에서 ID 7 결합을 확인한다.
13. **Stale/foreign calc ID:** registry id와 track handle이 다르면 거부한다. `binds_calc_handle_to_the_exact_row_owner_property_and_expression`의 stale-ID negative control 통과.
14. **Wrong registry owner:** registry node가 grid root가 아니면 거부한다. 같은 unit test의 wrong-owner control 통과.
15. **Wrong property/expression:** `GridTemplateColumns` 또는 4px term/추가 registry entry는 거부한다. Same unit test property, expression, extra-entry negative controls 통과.
16. **Unexpected percentage registry:** percentage scene에 math entry가 있으면 거부한다. Runtime validator와 native capture matchers는 registry 비어 있음을 검사한다.
17. **Incorrect min contribution:** item `min-height:0`/SpecifiedMinimum 0이 빠지면 plan's 35px bound assumption이 유효하지 않으며 trace validation must fail. Exact item computed style와 contribution trace validation을 검토했다.
18. **Incorrect row sizing:** initialize/intrinsic stages `[0,0,0]`, maximize 이후 `[35,35,35]` 및 contribution MinContent/MaxContent 400을 바꾸면 row trace validator가 거부한다. Runtime positive/wrong-final-base tests 통과.
19. **Frame or overflow clipping:** expected root/item/child geometry는 `60×300 / 60×35 / 30×400px`; Android AVD와 iOS Simulator 모두 일치했다. Viewport/root 바깥 child 표본까지 양 플랫폼 4/4 exact라 row가 400px content를 clip하지 않았다.
20. **Fixture leaks or weak platform report:** default FFI archive에서 marker를 검색해 부재, opt-in artifact에서 positive control을 확인했다. Android와 iOS strict matchers는 scene/style/frame/trace/registry 검사 후 통과했다. AVD software renderer 및 compositor 전 Metal이라는 한계는 유지했다.

## Review result

최종 opt-in source, Rust/FFI focused tests, runtime/FFI Clippy, fixture-module boundary, Android API 37 AVD와 iOS 26.2 Simulator evidence가 일치한다. 초기 검증 중 발견된 call-site 인자 연결 오류와 Android calc-ID suffix parsing 오류는 수정 뒤 재검증했다. 이 review는 전체 WPT, Android 실기기/hardware GPU, iOS compositor 후 surface 및 일반 CSS admission을 증명하지 않는다.
