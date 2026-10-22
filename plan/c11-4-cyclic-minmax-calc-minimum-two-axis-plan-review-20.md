# C11.4 양축 cyclic minmax calc minimum 계획 적대 검토 20개

구현 전에 계획·입력·oracle을 서로 다른 실패 방향에서 검토했다. 아래 수정은 계획과 fixture에 반영했다. 구현/runtime 검토와 PR diff 검토는 별도 단계로 수행한다.

1. **Percentage basis가 border box로 잘못 잡히는가?** 2px border를 제외한 content basis를 규범과 표에 명시하고, `50px` basis에서 104px을 기대값으로 고정했다.
2. **양축 결과가 하나의 공통 basis를 공유하는가?** inline 50px과 block 10px을 다르게 만들어 `[104]`/`[22]`를 동시에 검사한다.
3. **intrinsic phase와 final resolve가 섞이는가?** cyclic 값을 intrinsic root 산정에서는 auto처럼 취급하고 이후 root size로 재평가한다고 명시했다.
4. **column 결과가 row basis를 오염시키는가?** 두 axes 동시 장면과 columns-only/rows-only 대조를 둔다.
5. **row-only 기대 높이가 텍스트 wrapping을 무시하는가?** 10px column에서 `aa aa`가 두 줄·20px이 되는 전제를 명시하고 Chrome 대조를 추가했다.
6. **root 크기를 최종 overflow track 합과 혼동하는가?** root content/border box와 item frame을 별도 관측치로 표에 고정했다.
7. **minmax maximum flooring이 검사되지 않는가?** definite control의 `[24]/[22]`와 `max-content` maximum을 대조한다.
8. **max-content 텍스트가 최소 track을 가리지 않는가?** 양축 cyclic 기대값에서 final minimum 104/22와 max-content basis 50/10을 분리해 기록한다.
9. **font load 실패가 layout 결함처럼 보이는가?** Ahem 준비 상태와 font digest를 사전 비교 evidence에 기록하고, 로드 실패 시 결과를 무효 처리한다.
10. **공백·줄바꿈 차이가 browser/platform마다 달라지는가?** 원문 `aa aa`, `10px/10px`, 고정 폭 controls를 입력으로 저장한다.
11. **계산식 파싱이 다른 profile을 열어 주는가?** exact scene ID와 exact source expression만 허용하고 일반 CSS admission을 범위 밖으로 둔다.
12. **calc handle이 다른 node에서 재사용되는가?** owner·axis 일치와 잘못된 handle 거부를 runtime adapter gate에 넣었다.
13. **Rows/Columns trace 순서가 frame 성공으로 오인되는가?** track values와 intrinsic contribution, frame, stage trace, draw/present를 별도 판정한다.
14. **시각적 overflow가 다음 scene을 덮어 oracle을 오염시키는가?** fixture scene 간 36px margin을 두고 full-page screenshot에서 scene 위치를 기록한다.
15. **border CSS color가 PNG pixel color와 같다고 오인하는가?** screenshot 표본은 공간·paint-order 비교로만 사용하고 앱의 exact CSS sRGB oracle로 재사용하지 않는다.
16. **border stroke가 화면 표본에서 빠졌는데 확인했다고 주장하는가?** runtime scene에 실제 CSS border가 있을 때만 border control을 판정하고, 표본 누락은 명시한다.
17. **root 바깥 item 표본이 track sizing 증거를 대신하는가?** geometry/trace와 PixelCopy/Metal 표본을 별도 gate로 둔다.
18. **FFI allowlist만 추가하고 runtime text-scene registry를 빠뜨리는가?** 두 registry를 따로 갱신하고 runtime registry의 exact 16-scene set assertion을 갱신한다.
19. **fixture 코드가 기본 제품 artifact에 유입되는가?** feature-off archive negative와 fixture-test positive control을 각각 검사한다.
20. **검증 실패 뒤 partial layout이 남는가?** 변형 입력과 owner/axis 불일치는 전체 요청 reject, 결과 미게시를 검사한다.
21. **제한 fixture를 C11.4 완료나 실기기 검증으로 확대하는가?** C11.4 `- [ ]`, `72/213`, AVD·Simulator 전용 한계를 유지하고 WPT·physical·hardware/compositor 범위를 명시했다.

**판정:** 21개 질문 각각에 fixture, 계산, 격리, 산출물 경계 또는 보고 한계를 붙였다. 구현 시작 전 계획 gate를 통과했다. 이 기록은 실제 구현 검토를 대신하지 않는다.
