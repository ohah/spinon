# C10.3.4 구현 검토와 실행 근거

**검토일:** 2026-10-11 · **기준 Chrome:** 154.0.8037.98 · **내부 계약:** 0053 / `0.1.0`

## 검토 결과

초기 적대적 검토에서 `display:none` Flex subtree의 `align-content:first baseline`까지 입력 전체 검사에 걸려 실패할 수 있는 결함을 찾았다. 숨겨진 컨테이너는 baseline 검증에서 건너뛰도록 수정하고 회귀 테스트를 추가했다. 해당 수정 후 전체 Rust 테스트, Clippy, 고정 Chrome 기준 검사를 다시 실행했다.

## 실패 경로와 경계 검토

1. `align-items: baseline`의 first baseline sharing group은 빈 상자의 border-end baseline을 공유한다. Chrome 기준 `first-group` frame과 여섯 runtime profile 비교를 확인했다.
2. `align-items: last baseline`은 first group과 별도 계산하며 line의 반대쪽 edge에 놓인다. `last-group`의 세 border-end 좌표가 Chrome 기준과 일치한다.
3. first와 last 참여자가 함께 있을 때 두 sharing group이 서로 섞이지 않는다. `mixed-groups`에서 각 그룹의 baseline 좌표를 별도로 비교했다.
4. 명시적 `align-self:first baseline`은 부모의 `align-items` 값과 관계없이 first group에 들어간다. `first-explicit-and-place-self`를 확인했다.
5. `place-self:first baseline`이 생성한 longhand도 같은 first group 계산에 도달하고 computed `place-self` 문자열을 보존한다.
6. `align-self:auto`는 부모 `align-items`를 상속해 first 또는 last 그룹을 선택한다. `first-group`과 `last-group` fixture가 이 경로를 포함한다.
7. first 또는 last 그룹에 참여자가 하나뿐이면 반대쪽 fallback이 아니라 해당 baseline preference의 cross-start/cross-end fallback을 적용한다. 두 single-participant case를 대조했다.
8. last baseline 위치에서 위·아래 cross margin을 제외하지 않는다. `last-cross-margins`의 margin box 끝과 border edge 좌표를 각각 비교했다.
9. 합성 baseline은 padding이나 border 바깥이 아니라 border box의 block-end edge다. `border-padding-synthesized`를 확인했다.
10. 중첩 row Flex의 first baseline은 startmost flex line에서 가져온다. `nested-first-wrap`에서 부모 공유 baseline과 자식 baseline 좌표를 비교했다.
11. 중첩 row Flex의 last baseline은 endmost flex line에서 가져온다. `nested-last-wrap`의 끝 baseline을 확인했다.
12. `wrap-reverse`가 visual line 선택을 바꾸고 `order`가 item/line 순서를 바꿔도 원본 DOM 형제 순서는 유지한다. `nested-first-wrap-reverse-order`와 `wrapped-last-parent-wrap-reverse-order`를 확인했다.
13. 여러 줄 `align-content:stretch` 계산은 실제 `row-gap`을 공간 합계에서 한 번만 제외한다. `wrapped-last-parent-stretch`는 10 CSS px row-gap으로 Chrome frame을 비교한다.
14. 각 줄에 item 하나만 있는 경우 줄 경계 복구와 last fallback이 맞다. `wrapped-last-parent-single-item-lines`를 비교했다.
15. 주축 크기·gap이 0이고 두 zero-size item이 같은 좌표에 놓이면 exact fit으로 분류해 한 줄을 유지한다. `zero_main_size_items_that_exactly_fit_share_one_baseline_line`을 추가했다.
16. 같은 주축 좌표라도 양수 gap 때문에 다음 줄로 가는 경우를 놓치지 않는다. `positive_main_gap_can_wrap_zero_size_items_at_the_same_main_coordinate`를 추가했다.
17. 음수 main margin으로 line 경계를 프레임만으로 확정할 수 없는 겹침은 임의의 baseline 결과 대신 `UnsupportedBaseline`으로 실패한다. `ambiguous_negative_main_margin_fails_closed`를 추가했다.
18. RTL baseline 값을 LTR 방향으로 조용히 계산하지 않고 명시적으로 실패한다. `rtl_baseline_fails_closed_instead_of_reusing_ltr_geometry`를 추가했다.
19. 현재 제한한 `align-content:first baseline`에서 자동 크기 item은 성공 경로에 들어오지 않는다. `first_baseline_content_alignment_rejects_auto_sized_items`를 추가했다.
20. 숨겨진 `display:none` baseline subtree는 보이는 형제의 계산을 막지 않는다. 발견한 과잉 거부를 수정하고 `display_none_baseline_container_does_not_reject_its_hidden_auto_sized_subtree`를 추가했다.
21. `align-content:last baseline`과 해당 `place-content` 성분은 Chrome 154의 invalid declaration fallback을 보존하고 `!important` 우선순위를 흐리지 않는다. inline parser 단위 테스트와 author stylesheet 통합 테스트를 확인했다.
22. CSS 주석·문자열·함수 값과 중첩 grouping rule 안의 동명 토큰을 실제 선언처럼 잘못 바꾸지 않는다. scanner fixture 테스트에서 입력 원문과 출력 원문을 대조했다.
23. escaped `last` identifier를 invalidation할 때 문자열 길이와 이후 byte offset을 보존한다. escaped-identifier 테스트를 확인했다.
24. CSS rule nesting이 64단계를 초과하면 부분 수정하지 않고 원문 전체를 유지한다. excessive-nesting 테스트를 확인했다.
25. 새 baseline typed value가 legacy Flex profile에 새 기능으로 새어 들어오면 성공하지 않는다. `baseline_values_do_not_leak_into_the_legacy_flex_profile`을 확인했다.
26. 고정 Chromium 18 case·73 node를 DPR 1·2로 여섯 runtime Flex profile에서 computed style과 각 node frame으로 비교했다. 허용 오차는 필드당 0.5 CSS px다.

## 실행 검증

- `mise exec -- cargo fmt --all -- --check` · `git diff --check`
- `mise exec -- cargo test --locked --workspace --all-features`
- `mise exec -- cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
- `mise exec -- bun run test:css-reference` · 결과 129 passed, 0 failed
- Android SM-S731N 실기기(Android 16 / API 36): 최신 debug APK를 다시 설치·실행했다. 실제 V8 보고서에 12 node와 `layout=ready boxes=12`가 있고, 재생성된 surface generation 2에서 Samsung Xclipse 940 / Vulkan renderer가 `status=0 presented boxes=12`를 보고했다. 시작 중 이전 generation 요청 하나가 `-12`로 거부됐고 이후 현재 generation은 성공했다.
- iPhone 17 Pro / iOS 26.2 Simulator: 최신 앱을 다시 설치·실행했다. 실제 V8 frame 12개, `status=0 layout=ready boxes=12`, `SPINON_C0410_DRAW generation=1 status=0 presented boxes=12`를 확인했다.
- 실행 로그와 실제 화면 캡처는 이 문서와 같은 디렉터리의 `c10-3-4-baseline-runtime-2026-10-11/`에 둔다. 캡처 화면 양쪽 모두 12-box scene과 성공 상태를 보여준다.
- Android 빌드에는 compile SDK 37.2와 현재 Android Gradle Plugin 8.13.2 조합 경고가 있었다. 빌드는 성공했고 이 경고는 이번 C10.3.4 baseline 결과의 실패가 아니다. iOS build는 성공했으며 V8 archive debug-map 중복 경고가 남았다.

## 남은 검증 경계

- WPT 원본 suite는 실행하지 않았다. inventory의 mapped subset은 입력별 Chromium 기준으로 비교했다.
- 텍스트·replaced 요소의 실제 shaping baseline과 line box, intrinsic sizing, pseudo-element, inline formatting은 C15/C14 이후다.
- RTL·다른 writing mode는 지원하지 않으며 현재 baseline 경로에서 fail-closed한다.
- `align-content:first baseline`은 빈 definite-size fixture에서의 관찰값만 비교했다. 일반 baseline content-alignment와 텍스트 baseline 분배의 완성된 지원을 뜻하지 않는다.
- 모바일은 같은 12-node smoke를 양쪽 플랫폼에서 확인했다. 18-case 전체 모바일 행렬, Android/iOS pixel equality, 성능·제품 수준 호환성은 검증하지 않았다.
