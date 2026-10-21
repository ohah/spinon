# C11.4 cyclic percentage track runtime PR diff 검토 30개

검토 대상은 이번 PR의 staged code·runner·plan/status·simulator evidence 전체다. 구현 단계에 별도로 남긴 [20관점 검토](./implementation-review-20.md)와 독립적으로 PR diff를 다시 읽었다. 각 항목은 서로 다른 실패 지점을 확인한다.

1. **규범 입력 고정:** cyclic intrinsic sizing의 기대값이 CSS Grid Layout Module Level 1 2025-03-26 CRD 및 사전 HTML oracle에서 왔는가? plan/evidence가 기준 문서와 고정 입력을 연결한다. PASS.
2. **Chrome 버전 혼합:** 기존 Chrome 155 pinned baseline과 추가 Chrome 156 관찰이 합쳐져 기대값 출처가 불명확해지는가? README가 두 결과를 분리한다. PASS.
3. **기본 제품 artifact 누출:** 새 runtime fixture가 기본 FFI release library에 포함되는가? default `.rlib`의 ID/module marker 부재와 fixture feature positive control의 포함을 artifact hash와 함께 확인했다. PASS.
4. **scene lookup 허용 범위:** 임의 문자열로 런타임 HTML을 생성하는가? adapter는 세 고정 ID만 lookup하고 미등록 ID는 거부한다. PASS.
5. **HTML tree shape:** 테스트 scene에 의도치 않은 요소나 빠진 renderer node가 생기는가? 각 source는 root/item 두 `div`, 두 `data-node`, 두 ID 반환만 포함하며 bounded-size 검사가 있다. PASS.
6. **column cyclic root basis:** max-content root의 auto 취급 단계가 24px intrinsic width를 만들지 못해 50% final column이 12px에서 벗어나는가? Android/iOS 두 축 trace가 Columns `[12]`을 기록했다. PASS.
7. **row cyclic root basis:** 24px inline width와 max-content block sizing에서 50% final row가 8px가 아닌가? 두 simulator trace가 Rows `[8]`이다. PASS.
8. **definite control basis:** definite 40px width의 50% column control이 `[20]`이 아닌가? 양 플랫폼 trace가 `[20]`이다. PASS.
9. **교차 축 누락:** 대상 축만 맞고 반대 축 intrinsic size가 틀려도 통과하는가? runner는 세 scene 모두 Columns와 Rows의 used size를 검사한다. PASS.
10. **column track 경계:** 12px track boundary를 11px와 12px 표면에서 구분하지 못하는가? PixelCopy 및 Metal drawable에서 x=11/12를 고정 sample로 검사했다. PASS.
11. **row track 경계:** 8px row boundary를 7px와 8px 표면에서 구분하지 못하는가? 두 표면 검증기가 y=7/8을 각각 검사했다. PASS.
12. **definite track 경계:** 20px definite track control을 19px와 20px 표면에서 구분하지 못하는가? 두 표면 검증기가 x=19/20을 검사했다. PASS.
13. **root 크기 잘못 인정:** overflowing item 크기를 root에 반영해 24×16 대신 확장된 Grid를 허용하는가? cyclic column/row root는 24×16, definite root는 40×16을 플랫폼 assertion과 runner가 고정한다. PASS.
14. **item frame 회귀:** track overflow 사례에서 item frame이 24×16이 아닌데 sample color만으로 통과하는가? 모든 scene의 item frame이 독립 예상값과 비교된다. PASS.
15. **node 순서·개수:** 예상 node 외 추가 node나 잘못된 tree 순서를 허용하는가? 두 node의 전체 ordered frame trace와 `boxes=2`를 확인한다. PASS.
16. **Android scene admission:** Android runtime의 C11.4 allowlist에서 신규 ID가 누락되거나 엉뚱한 dispatch로 가는가? 세 ID가 기존 grid track sizing 목록에 추가됐고 AVD 세 launch가 요청 ID로 완료됐다. PASS.
17. **iOS scene admission:** iOS runtime allowlist에서 신규 ID가 누락되거나 다른 scene으로 대체되는가? 세 ID가 C11.4 set에 추가됐고 Simulator 세 launch가 같은 scene ID를 반환했다. PASS.
18. **Android root expectation dispatch:** host가 신규 scene을 기존 일반 expectation으로 처리하는가? Android root-frame 함수가 24×16 cyclic과 40×16 control을 각각 분기한다. PASS.
19. **iOS root expectation dispatch:** iOS가 definite control을 cyclic scene geometry로 비교하는가? iOS root-frame 함수에 동일한 두 결과를 별도로 명시했다. PASS.
20. **Android node frame dispatch:** root/item 개별 좌표나 크기가 틀려도 summary만 통과하는가? Android ordered node-frame 기대값이 세 scene에 추가됐다. PASS.
21. **iOS node frame dispatch:** Metal 실행 경로에서 node geometry 검증이 빠지는가? iOS ordered node-frame 기대값과 전체 summary assertion이 연결됐다. PASS.
22. **Android box count:** 두 node 외 box가 생겨도 통과하는가? Android `c114ExpectedBoxCount`는 신규 세 scene에서 정확히 2를 요구한다. PASS.
23. **Android pixel request admission:** 신규 sample request가 C11.4 pixel request allowlist에서 거부되거나 다른 장면에 붙는가? request scene match 분기에 세 ID가 추가됐다. PASS.
24. **Android 표본 누락/중복:** 한 경계 표본만 오거나 같은 표본이 두 번 와도 통과하는가? validator는 정확히 두 sample을 요구하고 known ID별 한 건의 존재를 확인한다. PASS.
25. **Android sample node 오인:** 기대 색만 맞는 주변/root node를 item 표본으로 오인하는가? 각 sample은 고정 item node ID와 비교된다. PASS.
26. **표본 좌표 drift:** renderer 좌표 변환이 어긋나도 유사한 화면색으로 통과하는가? 각 scene의 두 고정 CSS 좌표를 exact 비교한다. PASS.
27. **표면 RGB fallback:** 신규 sample ID의 expected color가 누락되어 default 색으로 판정되는가? Android RGB map과 iOS RGBA tuples에 여섯 ID 모두 정확한 `[229,72,77]` 계열 값을 명시하고 6/6을 관찰했다. PASS.
28. **runner target 자동 분류:** 세 scene 묶음이 이전 실행 그룹에 들어가 evidence 경로 또는 기본 run kind가 혼동되는가? 전 요청 ID가 신규 set에 속할 때만 cyclic group으로 분류하고 explicit evidence group 우선순위를 보존한다. PASS.
29. **runner inventory integrity:** 세 scene 추가로 전체 C11.4 ID 수나 중복 검사가 빗나가는가? count는 205로 조정되고 모든 ID의 unique 검사와 세 scene filter 실행이 유지된다. PASS.
30. **증거의 주장 경계:** Simulator drawable capture를 compositor 이후 또는 실기기·일반 CSS 지원으로 과장하는가? plan/README/status가 compositor 이전, fixed fixture, 미검증 범위를 명시하며 C11.4 checkbox 및 `72/213`을 유지한다. PASS.

판정: 30개 PR diff 실패 지점을 통과했다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각 3/3 scene을 확인했고, PixelCopy와 Metal drawable 경계 표본은 각각 6/6 exact다. 첫 Android 실행의 미지원 `justify-items:start` projection은 oracle에 불필요한 선언 제거 후 최종 simulator 행렬을 다시 실행해 해결했다. 이 판정은 fixed opt-in fixture 범위에 한정한다.
