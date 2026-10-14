# C13.2 visible overflow 표면 픽셀 계획 검토 20개

- **대상:** C13.2 visible parent border 밖 descendant의 Chrome 기준과 Android PixelCopy/iOS Metal 표본 계획.
- **검토 시점:** 구현 시작 전.
- **근거:** Chrome 155 agent-browser capture, 고정 runtime JS source, C13.2 plan/contract, 기존 C13.1 PixelCopy/Metal capture 경계.
- **판정:** 좌표·색상·플랫폼 sample source·mutation 기준을 고정했다. 제품 CSS 지원이나 최종 display scanout을 주장하지 않는다.

| # | 계획을 깨뜨릴 실패 관점 | 확인 및 판정 |
|---|---|---|
| 1 | Chrome 버전이 바뀌어 색상/레이아웃 기준이 다른데 예전 버전으로 보고한다. | `navigator.userAgentData`에서 `155.0.8059.40`을 확인했고 C13.2 고정 oracle의 revision `cfaadc5a132d78e1828635aa8405a499f3e14864`와 일치시켰다. **PASS** |
| 2 | 브라우저 viewport 또는 DPR이 달라 표본의 CSS cell이 surface 좌표와 어긋난다. | agent-browser viewport `400×240`, DPR 1, 캡처 PNG `400×240`을 observation에 고정했다. **PASS** |
| 3 | oracle HTML이 runtime fixture와 다른 선언을 복사해 실제 input을 대표하지 않는다. | oracle page가 `runtime-scroll-container-c13-2.js`를 직접 script로 로드한다. 파일 bytes SHA를 모두 기록했다. **PASS** |
| 4 | 화면에 유사한 빨간 사각형이 여러 개 있어 잘못된 child를 선택한다. | `(150,30)`에서 `elementFromPoint`가 `c132-visible-escape`인지 확인하고 해당 node rect도 저장한다. **PASS** |
| 5 | 표본 좌표가 CSS cell index인지 pixel center인지 혼합되어 경계선에서 다른 값을 읽는다. | observation은 `css_cell`과 `(x+0.5,y+0.5)` center를 분리한다. surface mapper는 center를 사용한다. **PASS** |
| 6 | red 표본이 시각상 부모 밖처럼 보여도 실제 parent border 안에 있다. | parent border box right는 `130`, sample center는 `150.5`, child left는 `143`이다. 좌표상 parent 밖임이 명확하다. **PASS** |
| 7 | clipping 상태에서도 outside coordinate가 parent background여서 positive pass한다. | escaped child red `(227,84,78,255)`와 별도로 parent 안 gray sample을 둔다. mutation은 child sample을 parent 쪽으로 옮겨 거부하게 한다. **PASS** |
| 8 | 실제 escaped paint가 아닌 빈 배경 색을 성공으로 오인한다. | `(136,30)` gap sample을 독립 negative color control로 고정했고 escaped child와 겹치지 않음을 기록했다. **PASS** |
| 9 | 표면 offset/scale 오차가 있어도 escaped red 우연 일치만으로 통과한다. | parent interior와 gold border strip sample을 같이 검사해 좌표 변환을 교차 확인한다. **PASS** |
| 10 | 다른 장면이 덮어써도 root 배경 sample 하나로 전체 scene을 오인한다. | root background, parent background, border, escape, gap을 서로 다른 위치/색상으로 분리한다. **PASS** |
| 11 | CSS computed color만 보고 GPU 표면 RGBA를 얻었다고 주장한다. | Chrome 기대값은 agent-browser 캡처 PNG의 실제 byte를 repository RGBA PNG decoder로 읽었다. **PASS** |
| 12 | 캡처 파일 또는 oracle source가 교체됐지만 같은 JSON을 재사용한다. | screenshot, HTML fixture, runtime source SHA를 각각 기록하고 plan acceptance에서 exact hash gate를 요구한다. **PASS** |
| 13 | Android PixelCopy가 실제 renderer SurfaceView 대신 앱 window screenshot을 읽는다. | 계획은 SurfaceView PixelCopy API와 PNG bytes/size/hash를 기록하게 구분한다. **PASS** |
| 14 | Android density를 고정 상수로 써 다른 surface pixel 위치를 읽는다. | viewport mapping은 고정 density가 아니라 PixelCopy 실제 width/height와 400×240 viewport로 계산한다. **PASS** |
| 15 | iOS drawable이 400×240이 아닌데 CSS 좌표를 그대로 drawable index로 쓴다. | Metal sample mapping은 실제 drawable size/scale 및 sample bounds를 검사하도록 요구한다. **PASS** |
| 16 | Metal 표본은 이전 frame/revision에서 읽고 현재 C13.2 scene에 귀속한다. | process/run ID, surface·layout generation, document/render-tree/environment revision을 correlation key로 묶는다. **PASS** |
| 17 | 실패 주입 및 lifecycle 실행 중 앞 scene을 PixelCopy해 matrix sample이라고 한다. | app route는 C13.2 assertions 뒤 최종 26-box matrix draw에서 sample을 요청하며 draw/report revision을 연결한다. **PASS** |
| 18 | C13.2 default 제품 artifact에 pixel-test ABI나 fixture payload가 들어간다. | 계획은 모든 새 FFI 요청을 C13.2 opt-in feature와 Android/iOS fixture flags에 한정하고 default/opt-in archive를 재검사한다. **PASS** |
| 19 | 표본 누락/중복 또는 PNG hash drift를 일부 success sample만으로 통과시킨다. | exact sample count와 IDs, image dimensions/hash, 누락/중복 mutation rejection을 독립 조건으로 둔다. **PASS** |
| 20 | simulator 결과를 실기기, hardware GPU, compositor/display scanout 또는 C13.2 전체 지원으로 확대한다. | Android API 37 ARM64 AVD PixelCopy와 iOS 26.2 Simulator의 pre-compositor Metal 표본으로 범위를 제한하고 남은 항목을 명시한다. **PASS — 범위 한정** |
