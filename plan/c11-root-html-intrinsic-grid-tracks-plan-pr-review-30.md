# C11.1 intrinsic Grid 계획 PR 변경분 적대 검토 30개

검토 대상은 이 PR에 들어가는 계획, 구현 전 비교 fixture·자료, 계획 전용 20관점 검토다. 실제 구현이나 runtime 동작은 이 검토 범위에 포함하지 않는다. 각 항목은 PR 변경의 별도 계약·문서·증거 실패 경로를 확인한다.

1. **공식 상태 오승격** — 계획은 C11.1/C11/C11.4를 미완료로 명시하고 `spec/STATUS.md`를 수정하지 않는다. 통과.
2. **지원 API처럼 보이는 문구** — 기본 Root HTML 제한 경로의 후보만 다루고 public CSS 지원으로 표현하지 않는다. 통과.
3. **구현과 계획의 순서 역전** — 비교 기준과 계획 검토 뒤에 코드 구현을 두며, 현재 PR에 runtime 코드를 섞지 않는다. 통과.
4. **이미 병합된 minmax slice 중복** — `minmax(px|%, fr|px|%)` 기존 작업을 이번 범위에서 다시 구현하지 않는다. 통과.
5. **기능 목표의 입력 경계** — parsed Root HTML 및 기존 default profile을 명시하고 fixture 전용 success route를 배제한다. 통과.
6. **cascade와 projection 책임 혼동** — CSS-invalid fallback과 valid-but-unsupported computed value를 서로 다른 판정 계층으로 둔다. 통과.
7. **CSS 문법과 허용 typed value 불일치** — flex minimum을 invalid로 분류하고 max breadth에서만 허용한다. 통과.
8. **intrinsic keyword 위치 오독** — 직접 track 및 `minmax()`의 minimum/maximum breadth를 별도로 정의한다. 통과.
9. **repeat 범위 확대** — 기존 fixed integer repeat 확장만 허용하고 auto-repeat을 제외한다. 통과.
10. **area·line-name 결합 누락** — 이번 계획에서 조합을 명시적으로 fail-closed 범위로 남긴다. 통과.
11. **implicit track 경계 모호화** — `grid-auto-rows`/`grid-auto-columns` intrinsic sizing을 포함하지 않는다고 명시한다. 통과.
12. **placement와 intrinsic sizing의 혼합** — 기존 row-flow placement와 숫자 span만 비교하도록 한정한다. 통과.
13. **row 결과 과장** — precomparison에서 min/max row의 값이 같은 이유를 단일 line-height control로 설명하고 독립 분기 oracle로 세지 않는다. 통과.
14. **Chrome 기대값을 규범으로 오인** — CSS Grid 명세와 독립 계산을 normative 기준으로 두고 Chrome은 관찰 기준으로만 쓴다. 통과.
15. **명세 버전 재현성** — W3C CSS Grid Level 1 2025-03-26 CRD와 절 번호를 기록한다. 통과.
16. **fixture 재현성** — HTML 입력에 viewport metadata, font, font size, line-height, white-space와 각 장면 selector가 포함된다. 통과.
17. **fixture와 observation의 불일치** — observation은 14개 장면을 포함하고 fixture도 같은 14개 ID를 제공한다. 통과.
18. **Chrome 장면 수 문서 불일치** — 계획의 초기 10개 표현을 14개 고정 장면으로 고쳤다. 수정 후 통과.
19. **길이 기여값 의미 혼동** — `alpha beta`의 min-content는 break opportunity 사이의 가장 긴 unbreakable run으로 설명한다. 통과.
20. **플랫폼 폰트 차이를 Chrome 오차로 처리** — simulator geometry는 platform text advance를 반영해 따로 판정한다. 통과.
21. **비교 허용치 충돌** — Chrome observation 대 independent expected는 `0.02 CSS px`, simulator 플랫폼별 expected 비교는 `0.5 CSS px`, Rust 회귀는 `0.02 CSS px`로 계획과 사전 자료의 구분을 맞췄다. 통과.
22. **DPR에 따른 CSS 기하 변경 오판** — CSS px 기대값은 DPR 간 동일하다고 계획한다. 통과.
23. **spanning oracle 누락** — 두 track span 장면과 각 track 및 item frame의 독립 기대값을 제공한다. 통과.
24. **hidden child 영향 오염** — `display:none` child가 contribution과 frame에 미치는 음성 대조군을 포함한다. 통과.
25. **overflow 화면 검증 오독** — 20px container와 max-content item frame을 분리 기록해 clip과 layout size를 구별한다. 통과.
26. **range preflight 우회** — intrinsic 항목 뒤의 fixed/percentage track도 축 합계 검증을 받아야 한다고 요구한다. 통과.
27. **부분 scene 게시** — 여러 Grid 중 하나가 실패할 때 전체 scene/layout/draw publication 부재를 검증하도록 명시한다. 통과.
28. **기존 제품 profile 퇴행 범위 부족** — 이미 지원 중인 sizing, placement, alignment, aspect-ratio 및 flow 검사를 회귀 집합에 열거한다. 통과.
29. **Simulator 근거 과장** — Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator를 기본 app entry로 계획하며 readback 범위를 별도 제한한다. 통과.
30. **후속 공식화 누락** — 병합 뒤 계약, 상태 대장, C11 전용 계획과 Tailnet preview를 맞추되 상위 완료 체크는 유지하도록 순서를 정한다. 통과.

## 판정

30개 변경분 관점에서 차단 결함은 남지 않았다. 검토 중 발견한 장면 수와 비교 허용치 서술의 불일치는 이 검토 결과를 반영해 수정했다. 이 판정은 계획 PR 변경분에만 해당하며, 구현/runtime 검토와 구현 PR 검토를 대신하지 않는다.
