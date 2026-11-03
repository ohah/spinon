# C11.5 Root HTML Grid item auto-size/stretch PR diff 실패 관점 검토 30개

검토 대상은 PR #1132의 실제 변경과 최종 후보 source commit `5969be0aca200a5507126bbf479aa45f7cf155c1`에 대응하는 working tree다. 각 관점은 서로 다른 입력 경계, 런타임 실패 경로 또는 산출물 계약을 다룬다.

1. **일반 CSS profile로 새 mode가 전파되는가?** 새 placement mode는 `RuntimeRootHtmlTextPaintV1` 및 eligible Root HTML Grid node에만 지정된다. 통과.
2. **다른 Grid의 정렬이 child에 연결되는가?** projection owner map과 exact container identity로 item set을 제한한다. owner projection 회귀 3/3 및 runtime snapshot 대조 통과.
3. **중첩 Grid에서 부모/자식 owner가 섞이는가?** 각 child owner는 직접 부모 Grid node ID와 일치해야 한다. 다른 owner는 specialized eligibility에서 제외된다. 통과.
4. **빈 item에 text, nested child, replaced element가 들어오는가?** HTML node/namespace/direct-child 검사와 layout child/content 검사로 거부한다. text·`img` atomic negatives 통과.
5. **whitespace-only Grid child를 실제 item으로 세는가?** 허용되는 직접 text node는 HTML ASCII whitespace뿐이고 item 수에는 포함하지 않는다. static admission 검토 통과.
6. **auto/implicit track이 조용히 추가되는가?** specialized mode는 고정 explicit px track만 허용하고 implicit capacity 초과를 거부한다. auto row와 추가 item negatives 통과.
7. **track 반복·named line·area가 이 slice로 새어 들어오는가?** auto-repeat, line name, template area는 style/layout eligibility에서 제외한다. source 및 typed layout contract 대조 통과.
8. **track count 곱셈이 overflow하거나 item capacity보다 작게 평가되는가?** capacity는 checked multiplication으로 계산하고 child/item count와 비교한다. static boundary 검토 통과.
9. **sparse row order가 바뀌거나 입력 order가 달라지는가?** 전용 mode는 row flow와 auto placement를 요구하며 normalization은 sparse row algorithm을 쓴다. 통과.
10. **명시 placement 또는 nonzero `order`가 자동 배치를 덮는가?** computed placement allowlist와 runtime `order == 0` contract가 이를 차단한다. span/order negatives 통과.
11. **finite 아닌 track/크기 값이 들어오는가?** fixed values는 finite·nonnegative 조건을 거친다. 음수/infinity projection controls 및 percentage runtime control 통과.
12. **shared `box-sizing:border-box` 제한이 기존 fixed-size profile을 퇴행시키는가?** diff 검토에서 공통 gate가 기존에 허용하던 content-box 입력을 막는 점을 발견했다. zero-edge 범위에서 gate를 제거하고 content-box container, 양 box-sizing item fixture로 검증했다. 현재 fixed-size simulator route도 Android/iOS 각각 7/7 exact다. 수정 후 통과.
13. **padding/border/margin이 cell available size를 바꾸는데 무시되는가?** item edges와 Grid padding/border는 zero 조건이고 nonzero item edges/container border negatives를 거부한다. 통과.
14. **min/max 제약이 stretch 결과를 잘못 자르는가?** item/container min-size auto, max-size none만 허용한다. min/max negatives가 partial scene 없이 거부된다. 통과.
15. **aspect ratio가 다른 축 크기를 전달하는가?** container와 item 모두 ratio가 없어야 하며 ratio negative가 거부된다. 통과.
16. **text measure가 empty item 크기를 다시 덮는가?** source-free paragraph를 text measure map에 넣지 않는다. auto-stretch runtime matrix와 기존 Grid text stretch 2/2가 통과한다.
17. **`display:none`, non-block 또는 replaced child가 빈 block item처럼 취급되는가?** computed block, direct empty `div` 및 layout node 조건을 확인한다. `img` negative 통과.
18. **unsupported self-alignment가 기본값으로 몰래 바뀌는가?** typed allowlist는 `auto`, `normal`, `stretch`와 기존 위치 정렬만 허용한다. baseline negative가 atomic rejection된다.
19. **`align-self:auto`/`justify-self:auto`가 정확한 parent computed value를 따르는가?** 10 scenes에서 parent `align-items`/`justify-items`와 child computed values 및 expected frames를 대조한다. 통과.
20. **CSS Grid `normal`을 다른 layout mode 의미로 적용하는가?** specialized mode는 Grid에만 연결되고 no-ratio non-replaced subset에서 normal/stretch sizing을 비교한다. expected frame matrix 통과.
21. **fixed dimension이 stretch로 덮이는가?** 양 축 fixed 및 한 축 fixed scenes가 있고 cell 외부 표본은 Grid 배경색이다. Android/iOS auto-stretch 표본 11/11 및 fixed-size route 7/7 통과.
22. **한 축 stretch가 다른 축의 positional alignment를 바꾸는가?** align-only/justify-only cases가 독립 expected JSON과 frame에 포함된다. 통과.
23. **zero-area item이 paint scene이나 sample count를 오염시키는가?** runtime test에서 scene/paint box 수를 따로 세고 zero-area item 4개는 paint count에 포함하지 않는다. 통과.
24. **invalid scene이 일부 frame/render snapshot을 공개하는가?** 16 negative controls 각각 layout Ready와 render snapshot 부재를 검사한다. 통과.
25. **runtime test가 서로 다른 source/style revision을 잘못 결합하는가?** layout/cascade completed key, computed owner node와 frame node ID를 맞춘다. 통과.
26. **Android default APK에 adapter, scene ID 또는 intent route가 들어가는가?** `classes.dex`와 `classes2.dex` 양쪽을 검사해 adapter/auto-stretch 문자열이 제외되고 opt-in APK에는 포함되는 것을 확인했다.
27. **Android에서 fixed-size와 auto-stretch intent를 함께 주면 어떤 route가 선택되는가?** 양 route를 동시에 요청하면 명시적으로 거부하도록 고쳤다. 단일 auto-stretch 및 fixed-size 요청은 각각 현재 APK에서 실행했다.
28. **Android reflection method signature와 adapter가 어긋나는가?** 런타임 PixelCopy가 opt-in adapter를 호출해 11개 auto-stretch sample을 정확히 맞췄고, 기존 route는 7/7이다.
29. **iOS argument parser가 두 scene을 동시에 받거나 기본 executable에 opt-in 비교 문자열을 남기는가?** 여러 Root HTML run argument는 거부하고 compile-time flag가 켜진 executable에서만 auto-stretch 인수/scene이 나타난다. default/opt-in 실행 파일 문자열 대조 통과.
30. **PR 설명·roadmap·완료 판정이 증거 경계를 넘는가?** 화면 이미지와 surface readback을 분리하고 Android SwiftShader, iOS compositor 전 경계를 기록한다. CI check rollup은 비어 있어 통과로 주장하지 않으며 C11.5/C11 체크와 `72/213`을 유지한다.

## 수정 및 남은 경계

- 수정 발견: shared style gate의 신규 border-box 제한은 이전 fixed-size C11.5 경로에서 동등한 content-box 입력을 탈락시켰다. 제한을 제거하고 content-box/border-box mixed fixture, runtime expected frames, 두 플랫폼 fixed-size route 회귀로 확인했다.
- 내용상 유효한 일반 Grid container margin을 atomic rejection으로 취급한 시험 변형은 Root HTML 기본 경로에서 layout Ready가 되어 negative control에서 제거했다. 이 후보의 실행 근거는 zero-edge fixture에만 한정한다.
- GitHub PR #1132의 check rollup은 비어 있다. 이 문서는 로컬 diff 검토와 검증 근거이며 CI 통과나 전체 C11 완료를 뜻하지 않는다.
