# C11.3.3 음수 named-line pair 구현·runtime 실패 관점 검토 20개

대상은 `feat/c11-3-3-negative-named-line-auto-tracks`의 변경 코드와 고정 Chrome fixture다. 기존 positive named-line pair 및 signed fixed-px profile의 실패 경계를 함께 대조했다. 이는 계획 검토나 병합 전 PR diff 검토를 대신하지 않는다.

1. **이름이 다른 두 endpoint가 auto profile에 들어가는가?** — `-4 start-ghost / -1 end-ghost`가 같은 숫자 line으로 풀리는 경우를 확인했다. 원래 custom-ident 동등성 가드를 projection 전에 보존하고, 통합 fixture에서 `grid-auto-columns:auto`를 `UnsupportedGridAutoColumns`로 거부한다. 회귀 test 통과.
2. **authored occurrence가 leading fallback보다 우선하는가?** — authored `[ghost]`와 음수 occurrence 대조군에서 implicit auto profile flag가 선택되지 않음을 확인했다. named-line projection test와 Chrome-control 장면에 포함한다.
3. **explicit grid 경계에서 첫 leading implicit line을 off-by-one 없이 구분하는가?** — 3개 explicit track에서 `-4`는 leading edge이며 `-8/-5` internal pair는 네 leading track을 만든다. CSS projection test와 최종 17-frame runtime 결과가 일치한다.
4. **line 계산이 허용 범위를 넘을 때 panic 또는 wrap하는가?** — track count 변환, `+1`, 음수 변환을 checked arithmetic으로 계산한다. 10,001 total track 입력은 `GridTrackLimitExceeded`로 종료하며 layout test 통과.
5. **역순 endpoint가 올바른 start/end track으로 정규화되는가?** — `-8/-5`와 `-5/-8` 입력이 동일한 leading track 순서와 item frame을 만든다. layout test와 Chrome 고정 비교가 통과했다.
6. **equal endpoint가 0-track span을 만들거나 큰 영역을 만드는가?** — 동일 line의 end를 제거하고 기본 1-track span으로 처리한다. equal-end layout test와 `columns-negative-equal` runtime frame이 통과했다.
7. **요청 span과 시작 endpoint가 달라도 auto 기여분을 같은 범위에 나누는가?** — 18px item의 3-track span은 `6,6,6px`, span 밖의 네 번째 leading auto track은 `0px`이다. Row/Column layout tests와 Chrome fixture가 이 값을 고정한다.
8. **앞쪽 fixed auto-track pattern이 CSS start-to-end 순서로 확장되는가?** — `[5px,7px]` pattern을 leading 방향에 맞춰 역상 적용한다. span·reverse·offset 대조군에서 최종 순서가 layout tests와 일치한다.
9. **Rows/Columns 구현이 한 축에만 맞춰져 있지 않은가?** — 동일한 negative pair 규칙과 auto contribution을 두 축에서 확인했다. layout, projection, Android·iOS fixture에 row/column case가 각각 있다.
10. **반대 축의 implicit line 또는 auto placement가 profile을 우회하는가?** — 반대 축은 explicit definite placement만 허용한다. 양 축 implicit 입력은 `UnsupportedGridAutoRows` 또는 `InvalidGridPlacement`로 거부하는 projection/layout tests가 통과했다.
11. **item의 width/height 중 하나가 auto여도 고정 크기 profile로 들어가는가?** — projection의 두 축 fixed gate와 layout의 별도 dimension 검사로 닫는다. `width:auto` control은 `UnsupportedGridAutoColumns`로 거부한다.
12. **추가 participating child가 검사 순서에 따라 통과하는가?** — mode 선택 후 모든 direct layout-participating child 수를 검사해 1개만 허용한다. 두 번째 child fixture는 frame 반환 전에 실패한다.
13. **숨겨진 child가 in-flow child 수에 포함되는가?** — validator가 `hidden` map으로 숨겨진 직접 child를 제외한다. 이 동작은 코드 경로상 맞지만, 이번 slice에는 hidden-child 전용 runtime 장면을 추가하지 않았다.
14. **grid-template-areas가 음수 no-area profile로 잘못 투영되는가?** — `template_areas.is_none()` 투영 gate와 layout validation의 명시 거부를 확인했다. 기존 area-backed typed profile은 별도 mode로 유지된다.
15. **auto-track 목록이 임의 grammar, 빈 목록 또는 잘못된 크기를 수용하는가?** — 비어 있지 않은 fixed-px 목록만 열고 Auto·음수·무한대 값은 거부한다. candidate layout tests에서 unsupported kind 및 invalid track을 확인했다.
16. **큰 leading extent가 자원 상한을 넘는가?** — signed extent의 공통 10,000-track 상한을 재사용하며 10,001개에서 실패한다. 초과 오류는 frame 계산보다 앞서 반환한다.
17. **한 번 실패한 compute 뒤에 잘못된 부분 frame이 남는가?** — engine 입력은 immutable snapshot이고 실패는 `Result::Err`로 반환되어 부분 `LayoutOutput`이 노출되지 않는다. 초과·shape-invalid 경로는 반환 오류를 단언한다. allocator/OOM 전역 복구는 이 profile의 보장 범위가 아니다.
18. **mode가 Stylo projection부터 dense-column normalization까지 끊기지 않는가?** — 전용 `GridPlacementMode`가 normalization·signed leading expansion·track validation의 각 match에 연결됐고 layout, style projection, FFI fixture test가 통과했다.
19. **fixture ABI와 새 source payload가 기본 제품 앱에 들어가는가?** — Android 기본 빌드에서 C11.3.3 fixture export와 scene 문자열이 없고, fixture-test 빌드 positive control에서는 둘 다 존재했다. 이 비교는 해당 Android app ELF에 한정한다.
20. **runtime geometry·표면 readback·negative control이 기대 입력과 같은 장면을 확인하는가?** — Android API 37 AVD의 전용 장면은 17 frame·PixelCopy 8/8 exact, iOS 26.2 Simulator는 17 frame·Metal drawable 8/8 exact다. 좌표 변조는 readback 전에, 미등록 장면은 draw 전에 거부했다. 39-scene 기존 행렬도 양 플랫폼에서 126 frame geometry 오차 0px로 통과했다.

## 경계

Android 결과는 API 37 ARM64 AVD의 SwiftShader 소프트웨어 renderer다. iOS 결과는 iPhone 17 Pro / iOS 26.2 Simulator Metal drawable이며 OS compositor 제출 전이다. 실기기/hardware GPU, Spinon WPT 적합성, 일반 CSS admission, 전체 C11.3.3·C11 완료는 입증하지 않는다. [실행 근거](./README.md)를 따른다.
