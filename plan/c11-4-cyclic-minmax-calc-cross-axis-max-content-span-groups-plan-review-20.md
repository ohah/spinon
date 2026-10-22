# C11.4 cyclic <code>minmax(calc())</code> max-content span-groups 계획 적대 검토 20개

구현 전 계획·입력·oracle을 서로 다른 실패 경로 20개로 검토했다. 각 발견은 계획, 사전 비교 또는 fixture 입력에 반영했다. 구현/runtime 검토와 PR diff 검토는 별도 수행한다.

1. **Max-content sizing이 기존 min-content slice의 복사인가?** max-content constraint와 span-2/3 growth-limit sizing을 추가하고 기존 40px definite control을 80px로 분리했다.
2. **Cyclic min track과 최종 resolved minimum이 한 단계로 합쳐지는가?** intrinsic root calculation과 resolved-percentage layout을 별도 trace stage와 expectation으로 구분했다.
3. **Percentage basis가 border box를 포함하는가?** 110px/80px content widths를 basis로 쓰며 border 4px은 root border box에만 더한다.
4. **Max-content root width가 C contribution을 무시하는가?** C의 110px max-content contribution과 4px total gutter에서 110px content width를 유도했다.
5. **Span-2 order가 track별 증가를 누적해 순서 의존성을 만드는가?** A/B를 span group 안에서 independently distribute하고 track별 planned increase maximum을 적용한다.
6. **Span-3 C가 span-2 group보다 먼저 계산되는가?** gate에 span length 2를 모두 처리한 뒤 3을 처리하는 trace assertion을 넣었다.
7. **A/B/C 기여 폭이 임의 텍스트 수에 의존하는가?** 10px Ahem, 저장소 font hash, 고정 문자열과 각 min/max-content 폭을 입력에 고정했다.
8. **Root width와 final track sum이 같은 값이라고 가정하는가?** 2px gaps와 2px border를 포함/제외한 content·border sizes를 분리했다.
9. **80px control이 max-content sizing과 혼동되는가?** definite 80px scene을 별도 named matrix row로 두고 track maximize 결과를 별도 계산했다.
10. **80px track equalization이 growth limits를 무시하는가?** B min-content, A max-content, C max-content 순서로 growth limits를 도출하고 maximize free space 46px를 계산했다.
11. **Cyclic row basis가 width 80px을 따라가는가?** 80px은 inline basis 전용이다. row basis는 wrapped text로 정해진 20px max-content height다.
12. **Row wrap 결과가 column layout보다 먼저 계산되는가?** A/C line widths와 row contributions는 final columns 이후에 측정하도록 gate를 둔다.
13. **Max-content 110px scene에서 A나 C가 불필요하게 wrap되는가?** 각 span/frame width와 60px/110px text width를 고정하고 한 줄 판정을 기록했다.
14. **80px에서 B width가 grid area로 stretch되는가?** B는 40×10px fixed control이고 text 한 줄을 유지하도록 명시했다.
15. **DOM reverse가 node identity도 바꾸는가?** scene 변형은 child order만 반전하며 item-keyed geometry와 identity를 유지한다.
16. **Computed CSSOM quantization을 spec 산술값으로 오인하는가?** rational expected tracks와 Chrome quantized observations를 분리하고 허용치를 1/64 CSS px로 고정했다.
17. **Diagnostic auto variant를 normative oracle로 쓰는가?** Chromium diagnostic JSON과 independent spec-derived JSON을 다른 산출물로 분리하고 한계를 적었다.
18. **스크린샷이 앱 runtime 표면 결과로 확대 해석되는가?** README와 plan은 Chrome screenshot이 브라우저 사전 관측이며 Android/iOS 앱 결과가 아님을 밝힌다.
19. **Simulator 성공이 실기기·GPU·compositor 증거가 되는가?** 실행 gate에서 Android API 37 ARM64 AVD와 iOS Simulator의 capture boundary를 각각 기록하고 실기기·hardware/compositor 이후 검증을 제외했다.
20. **계획 PR이 C11.4 완료 수나 지원 계약을 올리는가?** checkbox, 77/213, 일반 CSS admission, WPT와 상위 completion을 그대로 두도록 모든 문서와 gate에 명시했다.

**판정:** 20개 관점마다 구별되는 불변 조건과 대응 산출물을 확인했다. 계획 단계 gate를 통과한다. 구현 결과나 PR diff 검토를 대신하지 않는다.
