# C11.4 cyclic percentage track runtime 계획 — 실패 관점 20개

**상태:** 계획 전 검토 완료. 구현 결과는 이 문서에 포함하지 않는다.

1. **Chrome 버전 혼합:** 기존 26-case pinned oracle는 Chrome 155이고 신규 row/control은 Chrome 156이다. JSON과 README에 각각 버전을 적고 신규 관찰을 pinned baseline으로 대체하지 않는다. 통과.
2. **column과 row의 basis 혼동:** column percentage는 inner inline size, row percentage는 inner block size다. 두 case를 별도 root sizing으로 두고 독립 기대값 파일에 축별 basis를 기록한다. 통과.
3. **intrinsic/final phase 혼동:** track `50%`를 intrinsic contribution에 먼저 적용하면 cyclic 의존이 생긴다. acceptance는 intrinsic size 후 final resolution 순서를 별도 기술한다. 통과.
4. **max-content가 definite로 잘못 판정됨:** width/height `max-content` case가 실제 cyclic 경로를 만들어야 한다. definite control과 root computed style/trace를 함께 확인한다. 통과.
5. **row case의 indefinite block-size 누락:** width만 고정한 row case에서 height를 `max-content`로 명시해 block axis가 indefinite에서 출발함을 고정한다. 통과.
6. **definite control이 cyclic으로 오인됨:** definite 40px column은 normal percentage basis 회귀 control이며 cyclic 결과 합산에 쓰지 않는다. 통과.
7. **border-box 기준 혼선:** fixture에는 border와 padding을 두지 않아 inner basis가 예상과 같게 고정된다. edge-box 일반성은 범위 밖으로 분리한다. 통과.
8. **gap 중복 또는 gutter 영향:** row/column gap을 0으로 고정해 track percentage 외의 gutter가 intrinsic basis를 바꾸지 않는다. 통과.
9. **item intrinsic 크기 drift:** item을 24×16px fixed block으로 고정하고 margin/edge를 0으로 둔다. 폰트와 image decode dependency가 없다. 통과.
10. **implicit track/auto-placement 영향:** scene마다 child 하나와 explicit track 하나만 둔다. auto-repeat 및 implicit track 경로를 호출하지 않는다. 통과.
11. **alignment가 frame을 이동:** justify/align-content start를 지정하고 item offset을 0으로 고정한다. 통과.
12. **scroll metric을 overflow 판정으로 사용:** CSSOM scrollWidth만으로 track overflow를 주장하지 않는다. runtime used track, item frame, boundary sample을 함께 요구한다. 통과.
13. **pixel sample 좌표가 track과 불일치:** expected CSS track boundary와 runtime root frame을 합성해 경계 직전·직후를 샘플링하고 DPR 변환을 기록한다. 통과.
14. **색 sample이 box frame assertion을 대체:** PixelCopy/Metal sample과 geometry assertions를 별도 acceptance로 둔다. 통과.
15. **partial trace/output 누락:** 모든 scene에서 final Columns/Rows pass와 revision identity를 검증하고 하나라도 누락되면 실패로 판정한다. 통과.
16. **fixture가 product path에 포함됨:** opt-in feature positive control과 default/product negative control을 둘 다 요구한다. 폴더 경로만으로 격리를 주장하지 않는다. 통과.
17. **fixture test entry가 일반 ABI를 오염:** 전용 feature, native define, header guard 경로를 기존 c11.4 fixture seam 안에서만 사용하도록 계획한다. 통과.
18. **calc·minmax 의미를 순수 percentage에 일반화:** 이 값들은 observed probe에서 제외하고 범위 밖으로 명시한다. 통과.
19. **platform 증거 확대 해석:** Android는 API 37 AVD, iOS는 iOS 26.2 Simulator로 부르고 물리기기·compositor-after 근거를 주장하지 않는다. 통과.
20. **부분 slice로 C11.4 완료 처리:** status checkbox와 공식 완료 수를 바꾸지 않고 parent plan의 남은 범위를 유지한다. 통과.

**결론:** 계획을 구현 단계로 넘긴다. Chrome 156 신규 관찰은 supplementary only이고, pinned Chrome 155 oracle와 표준에서 독립 도출한 기대값을 함께 보존한다.
