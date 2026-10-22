# C11.4 cyclic `minmax(calc())` replaced image multi-span 계획 검토 20개

계획 구현에 앞서 입력, 규범 계산, 고정 scene과 검증 경계를 서로 다른 실패 경로 20개로 검토했다.

1. **계획이 기존 text multi-span slice와 같은 동작만 반복하는가?** replaced image contribution과 cyclic `calc()` track을 한 grid로 결합하는 미진행 교차 범위로 제한했다.
2. **계획이 기존 image multi-span slice와 같은 cyclic-free 입력을 쓰는가?** 모든 column/row track에 `minmax(calc(10% + 2px), max-content)`를 고정했다.
3. **HTML 이미지의 natural dimension이 브라우저마다 임의 파일로 달라지는가?** 저장소 fixture PNG를 고정하고 24×16 natural size와 SHA-256을 기록했다.
4. **HTML `width` attribute나 density descriptor가 CSS sizing과 섞이는가?** `width:auto`와 CSS `height` 값만 사용하고 natural aspect ratio 3:2를 보존한다.
5. **이미지 decode 대기 전에 레이아웃을 재는가?** fixture는 각 이미지를 decode한 뒤 관측 JSON을 생성하고 image `complete`와 natural size를 기록한다.
6. **기여 폭과 높이가 같은 값으로 잘못 계산되는가?** A/B/C의 inline 12/18/24px, block 8/12/16px을 분리해 expected JSON에 둔다.
7. **Item placement가 span 길이별 알고리즘을 실제로 자극하는가?** A/B는 겹치는 columns 1–2 및 2–3 span-2, C는 columns 1–3 span-3으로 고정했다.
8. **A와 B의 증가를 누적 적용해 DOM order 의존 결과를 만드는가?** §11.5.1 per-track planned increase maximum으로 `[5,8,8]`을 유도했다.
9. **span-3 item C가 span-2 group 앞에서 처리되는가?** 더 짧은 spans를 처리한 다음 C의 span-3 contribution을 평가한다.
10. **C의 contribution이 span-2 growth 뒤 추가로 tracks를 늘리는가?** 기존 25px track+gutter 영역에 24px image가 들어와 추가 증가가 없다.
11. **Row contribution이 Columns 결과와 무관하게 고정된다고 잘못 가정하는가?** replaced image는 지정 height가 고정되며 모든 item을 두 row에 span하고 `align-self:start`로 둔다.
12. **Rows에서 span-2 planned increases를 누적하는가?** A/B/C contribution당 track별 1/3/5px를 제안하고 maximum 5px로 `[7,7]`을 만든다.
13. **percentage basis가 root border box 또는 padding을 포함하는가?** padding 0, content-box 80px definite root, 2px border를 분리하고 expected sizes에 border를 마지막에 더한다.
14. **Definite control의 percentage minimum이 정수 10px이 아닌가?** `10% × 80px + 2px = 10px`을 두 축에서 독립 계산한다.
15. **Definite control에서 image intrinsic contribution이 track을 불필요하게 키우는가?** 모든 item span 내 available area가 이미지 크기보다 커서 definite track은 minimum에 남는다.
16. **Root box와 track sum에서 gap이나 border를 중복 더하는가?** columns 21+4 gap+4 border =29px, rows 14+2 gap+4 border =20px로 검산했다.
17. **Start alignment와 stretch가 item frame을 바꾸는가?** image는 `justify-self/align-self:start`, `width:auto`, 고정 CSS height로 natural aspect ratio 크기를 유지한다.
18. **DOM reverse에서 placement나 source identity까지 변하는가?** 각 variant는 정의된 A/B/C item 배열만 뒤집고 style·key·placement는 공유한다.
19. **Chrome 관측을 규범 기대값으로 역산하는가?** Chrome observation JSON과 W3C §7.2.1/§11.5.1 독립 산술 JSON을 별도로 둔다.
20. **사전 비교가 simulator/runtime, artifact 격리 또는 C11.4 완료로 확대되는가?** 계획은 미래 관문을 나열하되 runtime 미시작, simulator 범위, artifact positive/negative control과 `77/213` 유지 상태를 분명히 구별한다.

**판정:** 20개 관점은 각각 독립된 입력, 불변 조건 또는 범위 경계를 대상으로 한다. 고정 입력 및 사전 기대값 기록을 마쳐 계획 단계 관문을 통과했다. 구현/runtime 검토와 계획 PR diff 검토를 대신하지 않는다.
