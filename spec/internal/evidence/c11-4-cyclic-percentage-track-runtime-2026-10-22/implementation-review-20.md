# C11.4 cyclic percentage track runtime 구현 실패 관점 검토 20개

검토 대상은 고정 CSS scene, C11.4 opt-in runtime adapter, Android/iOS scene·표면 validator, simulator runner와 실제 두 simulator 결과다. 계획 review와 구분되는 새 구현 검토다.

1. **기준 버전 혼합:** Chrome 155 pinned `indefinite-percent-track`와 Chrome 156 보조 관찰을 같은 oracle로 합치지 않는가? 구분된 JSON과 계획 설명을 확인했다. PASS.
2. **cyclic column basis:** max-content inline size 24px에서 intrinsic 단계의 auto 취급 후 최종 50% column이 12px인지? Android/iOS trace가 `[12]`을 냈다. PASS.
3. **cyclic row basis:** 24px inline / 16px max-content block basis에서 50% row가 8px인지? 두 simulator가 Rows `[8]`을 냈다. PASS.
4. **definite control:** definite 40px column의 50%가 20px인지? 두 simulator trace가 `[20]`이다. PASS.
5. **다른 축의 회귀:** 한 축만 확인하고 나머지 축의 auto sizing 오차를 놓치지 않는가? 모든 scene이 두 axis trace를 검증하며 예상값과 일치했다. PASS.
6. **Grid/item frame 혼동:** overconstrained item overflow가 root frame을 늘리지 않거나 잘못된 item frame에 숨지 않는가? column/row root는 `24×16`, item은 `24×16`; definite root는 `40×16`, item은 `24×16`으로 각 호스트가 비교했다. PASS.
7. **숨은 box 허용:** 예기치 않은 node가 있어도 trace만 맞으면 통과하는가? 호스트 assertion과 runner가 각 scene `boxes=2` 및 전체 node frame 2개를 확인한다. PASS.
8. **fixture code 제품 누출:** Rust 고정 payload가 기본 FFI artifact로 들어가는가? 기본 release `.rlib`에서 scene ID/module marker가 없어 실패를 막고, fixture positive artifact에서 같은 값을 확인했다. PASS.
9. **Android runtime admission:** 새 ID가 앱 scene dispatch allowlist에서 빠져 argument failure가 되는가? Android C11.4 scene allowlist에 세 ID가 있고 모든 AVD launch가 통과했다. PASS.
10. **iOS runtime admission:** 신규 scene만 Metal pixel run에서 거부되거나 기본 scene으로 대체되는가? iOS C11.4 allowlist와 pixel parser allowlist를 함께 갱신했고 세 launch가 요청 ID로 완료됐다. PASS.
11. **runner scene selection:** `--scene-id` 선택 과정에서 다른 202개 scene까지 실행하거나 누락을 무시하는가? 205개 ID 중 정확한 세 scene만 각 simulator에서 실행됐다. PASS.
12. **runner 중복 ID:** 요청에 같은 scene ID를 반복해도 결과가 정상처럼 보이는가? runner의 중복 요청 검증이 유지되고 신규 ID 세 개는 matrix 전체에서 unique하다. PASS.
13. **Android sample target:** 좌표만 맞고 다른 node의 color를 읽는 표본이 되는가? PixelCopy sample validator가 item node ID와 고정 좌표를 모두 검사한다. PASS.
14. **Android track 경계:** track 끝 양쪽에서 overflow 여부가 구별되는가? column x=11/12, row y=7/8, definite column x=19/20에서 PixelCopy가 각각 2/2 exact다. PASS.
15. **Android color fallback:** 미등록 sample ID가 조용히 검정/기본색으로 처리되는가? 새 ID 여섯 개의 기대 RGB를 명시하고 6/6 exact를 확인했다. PASS.
16. **iOS sample identity와 좌표:** Metal parser에 다른 sample ID·node·좌표를 넣어도 fixed set을 통과하는가? `hasFixedC114TrackSizingSamples`가 ID·node·CSS 좌표·RGBA tuple 전부를 일치시킨다. PASS.
17. **iOS drawable 해석:** Metal 표본을 OS compositor 이후 픽셀로 과장하는가? evidence와 plan은 compositor 전 drawable임을 명시하며 6/6 표본만 주장한다. PASS.
18. **환경 혼동:** 실기기나 다른 simulator를 결과에 섞는가? Android `emulator-5558` API 37 ARM64 AVD와 지정 iPhone 17 Pro/iOS 26.2 Simulator ID를 metadata에서 확인했다. PASS.
19. **CSS profile 불일치:** oracle에 필요하지 않은 미지원 computed property가 전체 장면을 거부하는가? 최초 실행에서 `justify-items:start` 실패를 재현해 해당 불필요 선언을 제거하고 HTML oracle의 alignment만 남겼다. 최종 두 simulator 행렬 3/3으로 재검증했다. PASS.
20. **범위 과장:** 세 고정 fixture를 일반 cyclic percentage 또는 C11.4 완료로 취급하는가? `calc()`, percentage `minmax`/`fit-content`, 여러 track, 동시 양축, nested Grid, WPT와 C11.4 전체를 미검증으로 남기고 공식 `72/213`을 보존했다. PASS.

판정: 수정 후 위 20개 실패 경로를 재확인했다. AVD 및 iOS Simulator scene은 각 3/3, 표면 표본은 각 6/6 exact다. 이는 fixture 한정 결과다.
