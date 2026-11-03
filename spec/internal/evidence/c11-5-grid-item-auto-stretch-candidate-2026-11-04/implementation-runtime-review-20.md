# C11.5 auto-size/stretch 구현·runtime 실패 관점 검토 20개

범위: 로컬 후보 `8288072446e1847580c2804570a0d5d6f20723c4`와 수정된 working tree. 각 항목을 변경 코드, independent expected scene, focused test 또는 시뮬레이터 근거와 대조했다.

1. **새 지원이 임의 CSS 경로로 새지 않는가?** Root HTML text-paint profile과 이미 추출된 eligible grid owner set에서만 새 placement mode를 설정한다. 통과.
2. **다른 Grid runtime profile까지 새 mode를 받는가?** projection branch가 profile을 확인하고 C11.5 Root HTML alignment node에만 mode를 지정한다. 통과.
3. **지원하지 않는 auto-flow가 sparse placement로 흘러들지 않는가?** layout validation이 row flow를 요구하고, CSS profile도 row만 허용한다. 통과.
4. **implicit/auto track이 조용히 추가되는가?** explicit tracks는 유한한 nonnegative px만 받고 auto track과 반복 track을 제외한다. auto-row negative control 통과.
5. **item 수가 track capacity보다 많으면 부분 배치되는가?** style eligibility 및 layout contract가 capacity를 확인한다. 두 번째 item으로 implicit row를 만드는 negative control은 scene publication 전에 거부됐다.
6. **Grid container 크기/box model이 auto size에 영향을 주는데 제한이 빠졌는가?** definite finite size, border-box, min/max, ratio, zero edges/gap, LTR/static 조건을 검사한다. content-box, gap negative controls 통과.
7. **item owner가 다른 Grid의 정렬을 상속하는가?** 정확한 grid-item owner map을 대조하고, runtime test는 Grid/item computed values 및 scene identity가 같은 revision인지 확인한다. 통과.
8. **non-div/replaced 또는 자식이 있는 item이 받아들여지는가?** HTML eligibility는 빈 direct `div`만 선택하고 layout은 block, childless, no-content를 확인한다. text와 replaced negative controls 통과.
9. **`auto` 크기와 유효하지 않은 크기가 섞여 허용되는가?** 축별 `auto` 또는 finite nonnegative fixed만 허용한다. 음수·infinity는 projection unit test, percentage는 runtime negative control에서 거부됐다.
10. **min/max가 used size를 조용히 바꾸는가?** item과 container의 지정된 auto/none 경계만 받는다. min-width/max-height controls가 atomic rejection됐다.
11. **aspect-ratio가 stretch 축을 바꾸는가?** item/container ratio가 없어야 한다. ratio negative control 통과.
12. **margin/padding/border가 stretch area를 줄이거나 키우는가?** item/container decoration을 zero로 제한한다. item margin/padding/border 및 container padding eligibility를 확인했다.
13. **overflow 또는 positioning 상태가 범위 밖 동작을 넣는가?** item/container overflow-visible와 static positioning을 요구한다. plan의 safe/unsafe overflow 동작을 새 주장으로 추가하지 않는다.
14. **baseline/unknown self-alignment가 임의 fallback되는가?** typed allowlist는 기존 position 값과 `auto`에 `normal`/`stretch`만 추가한다. baseline/unknown은 실패한다.
15. **부모 정렬값이 item의 auto 상속을 대체하는가?** 10 scene 모두 computed `align-items`/`justify-items`와 `align-self`/`justify-self`를 독립 JSON과 대조한다. 통과.
16. **`normal`을 일반 block/flex 의미로 잘못 처리하는가?** C11.5 mode는 CSS Grid 경로이고 Taffy Grid stretch 결과를 사용한다. 10 scene의 expected frames와 Android/iOS 표본이 일치한다.
17. **한 축의 fixed 크기가 stretch로 덮이는가?** 두 축 fixed와 각 한 축 fixed scene을 포함하며, fixed-stretch 바깥쪽 pixel은 grid 배경색이다. Rust geometry 및 Android/iOS 표본 통과.
18. **잘못된 입력에서 다른 장면 일부가 publication되는가?** 16개 negative variant마다 layout Ready와 render snapshot 부재를 검증한다. 통과.
19. **빈 문단 measure가 empty auto item과 충돌하거나 텍스트 회귀를 만드는가?** source 없는 paragraph measure는 만들지 않고, auto-size runtime scene과 기존 Grid text stretch tests 2/2를 확인했다. 통과.
20. **기본 artifact 또는 OS surface 경계를 과장하는가?** 기본 Android DEX/iOS executable에서 fixture route가 빠지고 opt-in에서만 포함된다. PixelCopy 11/11, Metal drawable 11/11, 별도 OS screenshots를 남겼다. Android emulator는 hardware GPU가 아니며 iOS readback은 compositor 전이다. C11.5/C11 및 `72/213`을 완료 처리하지 않았다. 통과.

검증 입력과 raw logs는 같은 디렉터리의 README 및 `simulator-build-metadata.json`에 연결했다. PR 변경분 검토와 CI/merge 상태는 이 검토 범위에 포함하지 않는다.

구현 중 발견·수정한 artifact 경계 결함: opt-in scene label 비교가 Swift 공통 status expression에 있어 iOS default executable에 `root-html-grid-item-auto-stretch` 문자열이 남았다. 비교를 `SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST_1` 조건으로 옮긴 뒤 opt-in/default를 다시 빌드해 문자열 포함·제외를 확인했다. Android DEX 검증은 adapter가 든 `classes2.dex`를 포함해 전체 DEX에서 positive/negative 대조했다.
