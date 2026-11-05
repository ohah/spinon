# C11.4 PR 변경분 적대 검토 30개

대상은 C11.4 bounded candidate의 소스, fixture, 명세와 runtime evidence 변경이다. 검토 기록 파일 자체와 저장소에 이미 남아 있던 다른 작업의 untracked evidence는 기능 변경 대상에서 제외했다. 계획 검토 30개와 구현/runtime 검토 20개를 재사용하지 않고, 서로 다른 경계와 실패 경로를 확인했다.

1. **기존 profile 누출:** typed `grid-auto-rows` 허용은 Root HTML intrinsic route에만 더해졌고 기본 `Auto` 및 다른 profile의 투영 경로는 바뀌지 않는다.
2. **속성 혼동:** cascade 변경은 auto rows에만 적용한다. auto columns의 `min-content`는 계속 typed 지원 밖이라는 focused test가 고정한다.
3. **토큰 정밀도:** singleton `MinContent`만 후보 selector를 통과한다. `MaxContent`, 혼합 목록, 다른 intrinsic track은 별도 거부 검사에 걸린다.
4. **기존 explicit intrinsic 우선순위:** explicit 축에 intrinsic sizing이 있으면 새 implicit-row selector는 꺼지고 기존 route 규칙이 적용된다.
5. **aspect-ratio route 우선순위:** direct-text aspect-ratio owner를 후보 검사 전에 제외해 해당 전용 route와 경쟁시키지 않는다.
6. **visible owner 수:** 후보 Grid 하나만 인정한다. 두 번째 visible Grid가 있을 때 아이템 유무에 관계없이 후보를 닫는다.
7. **hidden subtree:** 기존 hidden-node 집합에 포함된 직접 자식은 후보 item 수에서 제외한다. 보이지 않는 요소가 placement capacity를 과장하지 않는다.
8. **whitespace anonymous item:** 탭·LF·FF·CR·space만 있는 직접 text는 무시한다. non-whitespace 직접 text는 기존 text-item 거부 경계를 유지한다.
9. **직접 element 종류:** HTML namespace `div` 이외의 직접 element는 거부한다. SVG 및 `span` 같은 요소가 Grid item 의미로 잘못 받아들여지지 않는다.
10. **빈 item 형상:** 직접 `div`의 child node가 하나라도 있으면 거부한다. descendant text/element/comment를 fixed empty-box 모델로 추정하지 않는다.
11. **중첩 Grid:** direct item 자체가 또 다른 Grid owner이면 후보 item predicate가 거부한다.
12. **cascade snapshot 누락:** Grid owner handle이나 tree children을 읽을 수 없으면 추정값을 만들지 않고 contextual `UnsupportedComputedValue`를 반환한다.
13. **item style 누락:** item의 computed snapshot이 없을 때 `is_some_and`가 false가 되어 닫힌다.
14. **width의 유한성:** width는 finite non-negative px만 허용한다. `auto`, percentage, infinity/NaN을 intrinsic contribution 값으로 넘기지 않는다.
15. **height의 유한성:** height에도 같은 조건을 별도로 적용해 block-size contribution이 자동 계산되지 않도록 한다.
16. **min/max clamp:** 양 축 min-size는 `auto`, max-size는 `None`만 허용한다. Grid auto minimum 등 clamp가 row base를 바꾸는 조합은 거부된다.
17. **aspect ratio:** item ratio가 있으면 거부한다. 폭으로부터 높이를 유도하는 별도 sizing 경로와 혼합되지 않는다.
18. **positioning:** item은 static이어야 한다. absolute/fixed item이 implicit row 생성이나 track sizing에 포함되지 않는다.
19. **placement endpoint:** row/column start/end 네 typed longhand가 모두 `auto`인지 검사한다. 부분 지정, span, named line을 자동 배치로 정규화하지 않는다.
20. **row flow:** projection과 layout contract가 모두 row flow를 요구한다. column/dense placement로 expanded row 결과를 재사용하지 않는다.
21. **명시 축 형상:** 두 explicit axis 모두 positive finite px track 하나여야 한다. 다중 track 및 0·음수·비정수 단위는 projection에서 막힌다.
22. **repeat/area/name 경계:** auto-repeat, fixed repeat group, template area 및 non-empty line name은 기존 Grid grammar 검증에서 거부된다.
23. **implicit columns:** auto-columns 외 값은 projection과 layout contract가 각각 거부한다. row capacity가 column 확장 수를 숨기지 않는다.
24. **gap:** projection은 row/column gap 외에 layout contract도 zero gap만 통과시킨다. 한 축의 비영 gap이 다른 축 검사에 가려지지 않는다.
25. **container sizing/edges:** positive definite container dimensions, 기본 min/max와 ratio, LTR, visible overflow 및 zero edges가 intrinsic layout preflight에 남아 있다.
26. **auto-row layout gate:** layout mode에서 MinContent 예외는 정확히 단일 track, 단일 positive fixed track 두 축, zero gap, no areas로 제한된다. `validate_tracks`도 intrinsic track 허용 모드에서 실행된다.
27. **capacity arithmetic:** style capacity 예외는 단일 positive px 축과 row-flow 조건 뒤에서만 계산되며 checked multiplication 및 기존 10,000 track ceiling을 사용한다.
28. **placement 확장:** implicit expansion 결과는 이 mode에서만 전달한다. 기존 mode는 `GridImplicitTrackRequired` 거부 동작을 유지한다.
29. **publication 실패:** RuntimeSession negative matrix는 13 unsupported 조합 각각 `Failed`, contextual code/property, `completed=None`을 확인한다. 부분 frame/render scene이 남지 않는다.
30. **근거와 상태 과장:** 기대 JSON, Chrome 사전 관찰, Android/iOS OS screenshot, Rust frames, broad-filter baseline을 서로 다른 증거로 표시한다. 동일 9개 기존 C11.5 실패를 green으로 부르지 않고 공식 완료 `78/214` 및 C11 checkboxes를 올리지 않는다.

Focused cascade 1/1, projection 1/1, capacity 2/2, layout 11/11, RuntimeSession 2/2, scoped Clippy와 rustfmt 결과는 [runtime evidence](README.md)에 기록되어 있다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 결과는 OS screenshot 기반이며 PixelCopy/Metal drawable readback 또는 WPT로 확대 해석하지 않는다.
