# C11 Root HTML Grid 콘텐츠 정렬 계획: 실패 관점 20개

계획 구현 전에 범위와 판정 조건을 독립 관점으로 점검했다. 아래 항목은 겹치지 않는 실패 경로와 대응 조건이다.

1. **기본값을 재정의할 위험:** `normal`이 기존 화면을 바꾸지 않도록 고정 explicit tracks만 허용하고 기본 회귀를 둔다.
2. **Grid 외 요소 확장:** Grid container에서만 runtime parser를 쓰고 일반 block/flex profile은 그대로 둔다.
3. **잘못된 CSS 값 누락:** parser 오류가 node/property/value를 포함하는지 확인한다.
4. **baseline 축약:** Taffy adapter가 baseline을 flex-start로 매핑하므로 baseline content 정렬은 거부한다.
5. **safe 처리 누락:** 음수 free space의 `safe center`와 `safe end`가 start로 복귀하는지 양 축 각각 확인한다.
6. **unsafe 처리 누락:** unsafe center/end가 음수 free space를 보존하는지 확인한다.
7. **기존 gap 포함 오류:** free-space 산식에 기존 column/row gap을 한 번만 포함한다.
8. **분배 gutter 오류:** between/around/evenly에 기존 gap과 추가 distributed gap을 분리해 기대값을 계산한다.
9. **Chrome 양자화 오해:** `space-evenly`의 1/64 CSS px quantization을 명시하고 더 큰 오차를 허용하지 않는다.
10. **definite size 누락:** 두 축의 computed dimension이 finite fixed px인 profile만 허용한다.
11. **intrinsic track 오염:** auto/minmax/fit-content/fr/percentage/repeat track은 고정-px profile gate에서 제외한다.
12. **implicit track 생성:** auto placement item 수가 explicit cell 수보다 많을 때는 새 profile 허용을 거부한다.
13. **named placement 우회:** named area/line·span을 포함한 placement input은 범위 밖으로 유지한다.
14. **방향성 차이:** RTL 및 `horizontal-tb` 이외의 writing mode가 지원되지 않는다고 명시하고 거부한다.
15. **scroll safety 과장:** scroll container의 자동 safe 동작은 이번 fixture에 없고 근거로 주장하지 않는다.
16. **부분 결과 노출:** style/project/layout 오류 뒤 partial frame이나 draw item이 공개되지 않는지 확인한다.
17. **기존 special route 회귀:** dense, named-area, fit-content, C11.4 fixture modes가 동일한 rejection/geometry 계약을 유지한다.
18. **앱과 Rust 불일치:** Chrome baseline뿐 아니라 같은 source hash의 Android/iOS simulator frame을 대조한다.
19. **화면 캡처 과장:** draw/present screenshot과 PixelCopy/Metal surface readback을 분리해 기록한다.
20. **완료 상태 오승격:** slice 결과로 C11.1/C11 또는 공식 완료 수를 올리지 않는다.

모든 항목은 계획에 반영했다. 구현 후에는 이 표를 재사용하지 않고 새로운 구현/runtime 실패 관점으로 다시 검토한다.
