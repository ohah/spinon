# C11.4 fixed-repeat + auto-repeat 구현/runtime 적대적 검토 20개

계획 검토 20개와 별개로, 최종 adapter·fixture·시뮬레이터 동작을 대상으로 실패 관점을 새로 확인했다.

1. **Repeat count 변환:** signed computed count를 usize로 변환하고 0보다 커야 한다. 범위 밖/0 count가 track 확장에 도달하지 않는다.
2. **Integer repeat 개수:** 한 list의 fixed-repeat은 하나만 허용한다. 두 개를 섞은 computed AST는 node/property 오류로 거부하고 cascade 회귀 test가 확인한다.
3. **Auto-repeat 개수:** auto-fill/auto-fit 중 정확히 하나를 요구한다. 둘 이상이거나 누락되면 auto-repeat adapter가 layout을 만들지 않는다.
4. **Pattern 길이:** fixed-repeat pattern은 1–2 track만 허용한다. 세 track pattern과 empty pattern은 거부한다.
5. **Track type:** fixed pattern의 각 track은 finite nonnegative LengthPx만 통과한다. percentage, unresolved percentage calc, flex, intrinsic, minmax는 결과에 섞이지 않는다.
6. **Computed calc 경계:** Stylo가 calc(20px)를 LengthPx로 정규화한 사실을 인정하고 같은 computed track 결과를 검사한다. authored spelling을 runtime에서 복구한다고 주장하지 않는다.
7. **Absolute unit 환산:** 0.25in이 computed 24px로 들어오며 두 반복 위치 모두 동일한 typed 값이다.
8. **Prefix order:** fixed pattern의 모든 확장 track이 auto-repeat보다 앞에 남고 auto-repeat start/end는 확장 뒤 index를 가리킨다.
9. **Suffix order:** suffix fixed-repeat의 track 순서가 보존되고 auto-repeat range는 suffix를 포함하지 않는다.
10. **Rows 독립성:** 같은 확장과 index 조정이 rows에도 적용되며 columns auto-repeat metadata가 잘못 채워지지 않는다.
11. **Cross-axis scope:** 혼합 list가 다른 축 repeat와 함께 쓰이면 다른 property에 귀속된 오류로 거부한다.
12. **Template areas 경계:** 혼합 fixed/auto list와 areas는 거부되지만 기존 auto-repeat + areas fixture는 계속 받아들인다.
13. **Track descriptor overflow:** checked multiply/add가 확장 전 최대 10,000을 적용한다. 10,000은 받아들이고 10,001은 거부하며 부분 descriptor를 내보내지 않는다.
14. **Allocation overflow 및 복구:** track reservation 실패는 node/property를 포함한 GridAllocationFailure가 되고 다음 정상 cascade는 성공한다.
15. **Line group reservation:** fixed-repeat metadata는 이름 없는 expanded boundary로만 생성한다. 강제 reservation failure가 부분 cascade 없이 실패하고 다음 호출에서 복구한다.
16. **Named line ambiguity:** 외부·fixed·auto repeat line name을 profile에서 거부해 반복 seam의 merge 의미를 추정하지 않는다.
17. **기존 auto-repeat 회귀:** fixed-repeat이 없는 auto-repeat의 line groups와 fixed-minimum gate 경로를 그대로 복사·검증한다. 기존 template-area test도 통과한다.
18. **auto-fit empty collapse:** empty scene은 반복된 auto-fit track 둘을 0px로 collapse하고 fixed prefix/suffix와 gutter 순서를 유지한다. Android/iOS에서 tracks와 collapse flag를 비교했다.
19. **occupied collapse 및 gutter:** item이 첫 반복 track을 점유할 때 84px track, 다음 empty track collapse, item offset 92px과 표면 sample을 확인했다. 두 platform frame, trace, draw/present 및 sample이 통과했다.
20. **Fixture/runtime 격리:** 9개 고정 source scene, app-side sample validation, 기본 APK/iOS bundle marker 부재와 opt-in positive control marker 존재를 각각 확인했다. AVD와 iOS Simulator는 별개 run으로 검증했다.

**검토 결과:** 이 20개 관점에서 차단 결함은 남지 않았다. bring-up 중 Android whitelist/expected frame/RGB mapping과 Swift conditional declaration 문제를 실제 실패에서 찾아 고쳤고, 최종 build/capture를 다시 실행했다. 전체 CSS Grid 적합성은 이 검토 범위에 포함하지 않는다.
