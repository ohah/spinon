# C11.4 fit-content/min-content/auto PR diff 실패 관점 검토 30개

실제 변경 diff와 앞선 Chrome·Rust·Simulator 근거를 별도로 대조했다.

1. **요청 범위가 나머지 네 순서로 한정되어 있는가?** 구현은 기존 두 순서를 유지하면서 네 order만 추가한다.
2. **Style projection의 accepted order 집합이 예상과 같은가?** 여섯 순서 모두 exact three-track pattern으로 확인했다.
3. **Layout admission도 같은 집합을 쓰는가?** layout-side `supports_min_content_auto_track_list`가 projection 집합과 맞는다.
4. **Sparse auto-placement validator와 두 allowlist가 일치하는가?** 새 pattern 네 개를 대조했고 track mode를 일반화하지 않았다.
5. **변경이 `RuntimeRootHtmlTextPaintV1` 경계를 넘어가는가?** 기존 owner/profile·root identity gate는 수정하지 않았다.
6. **Taffy sizing 알고리즘을 임의로 바꾸는가?** 새 알고리즘 없이 typed track list를 기존 intrinsic/auto sizing route로 보낸다.
7. **Direct cap 종류가 넓어졌는가?** direct px 및 percentage만 허용한다.
8. **cap의 finite/non-negative 경계가 빠졌는가?** projection과 layout에서 유지되고 negative/NaN/infinity 테스트가 있다.
9. **잘못된 list length가 통과하는가?** slice pattern이 세 항목 exact length이므로 추가 track 포함 목록은 실패한다.
10. **한 멤버의 중복 또는 다른 intrinsic 멤버가 실수로 허용되는가?** explicit negative cases와 exact order pattern이 이를 막는다.
11. **기존 두 order의 보존 의미가 바뀌는가?** 기존 test vectors 및 used rows는 유지됐다.
12. **새 순서 typed mapping이 잘못 뒤집히는가?** 네 신규 order마다 px와 percentage를 별도로 projection test한다.
13. **Computed CSS order를 정렬/정규화해 authored 의미를 잃는가?** projection 결과의 track vector를 입력 순서대로 비교한다.
14. **Negative control이 `max-content`를 허용할 수 있는가?** 신규 branch 주변에서 fail-closed error를 확인한다.
15. **Missing `fit-content()` anchor가 fallback 허용되는가?** anchor 부재 profile의 RuntimeSession 거부가 있다.
16. **Unsupported alignment가 새 목록을 통해 들어오는가?** `align-content:center` negative runtime case를 기존 exact contract에서 거부한다.
17. **Px와 %의 결과를 같은 cap 산술로 다루는가?** definite 100px basis에서 20%와 20px이 네 order별 동일 rows를 만든다.
18. **Fit-content capped contribution과 `auto` free space가 뒤바뀌는가?** 신규 layout cases는 cap row를 20px로 유지하고 `auto`에만 잔여 공간을 배분한다.
19. **Auto 한 개에서 residual 계산이 틀리는가?** 독립 기대값은 20px 또는 6px residual을 사용한다.
20. **Auto 두 개의 equal distribution이 틀리는가?** 산식은 20px→10/10 및 6px→3/3을 확인한다.
21. **Explicit row와 첫 implicit row의 인덱스가 밀리는가?** 테스트 배열의 첫 `10px` explicit row 뒤 네 implicit row가 기록된다.
22. **Track pattern wrap을 확인하지 않고 단일 주기만 검사하는가?** 네 번째 implicit row까지 item/payload frame을 비교해 다시 첫 목록 항목으로 wrap한다.
23. **Layout item frame만 맞고 nested payload overflow가 틀리는가?** Rust layout 및 RuntimeSession tests가 두 종류 frame을 별개로 확인한다.
24. **CSS fixture가 scene을 실제 auto-placement로 만들어내는가?** 고정 HTML은 authored row placement 없이 explicit 1 row와 네 direct item을 사용한다.
25. **Chrome oracle가 다른 source를 비교하는가?** fixture SHA를 JSON oracle와 Android/iOS app artifact에서 대조했다.
26. **Chrome 관측을 독립 기대값으로 오인하는가?** README에서 track base/free-space 산식과 비교 장면을 분리했다.
27. **Android 캡처가 blank pre-present PNG를 최종으로 남기는가?** 최초 이미지와 hash를 보존하고 present 이후 재촬영을 final로 표시했다.
28. **iOS/Android 캡처가 native pixel readback을 증명한다고 주장하는가?** OS screenshot visual check만 주장하며 PixelCopy/Metal readback 한계를 적었다.
29. **Simulator 결과를 real device 또는 GPU 성능 결과로 넓혀 말하는가?** Android는 API 37 ARM64 AVD·SwiftShader, iOS는 iPhone 17 Pro Simulator로 명시한다.
30. **환경 상태 변화나 결손 근거를 감추는가?** iOS `simctl install` 전후 app-data path 변경과 미확정 원인, 삭제 명령 부재를 README에 기록했다.

판정: 코드 allowlist와 typed projection은 계획의 제한 경계와 맞는다. 반복되던 placement-side pattern match를 기존 exact matcher 호출로 바꿔 두 gate의 drift 가능성을 줄였다. `grid_auto_placement.rs`는 기존 556줄에서 541줄로 감소했으며 sparse-grid normalization이라는 한 책임을 유지한다. 최초 Android screenshot 순서 문제는 캡처를 수정하고 artifact를 남겼다. iOS app-data container identity 변화는 원인을 특정하지 못했으며 Simulator evidence의 제한으로 공개했다. 공식 C11/C11.4 completion은 주장하지 않는다.
