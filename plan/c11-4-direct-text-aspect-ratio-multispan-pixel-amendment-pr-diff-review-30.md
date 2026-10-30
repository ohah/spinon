# C11.4 direct-text ratio span pixel 기준 보정 PR diff 검토 30개

**검토 대상:** simulator pixel acceptance 보정 및 상위 계획 인덱스 변경. 제품 코드는 포함하지 않는다.

| # | 적대 검토 관점 | 결과 |
|---:|---|---|
| 1 | 보정이 기능 scope를 은근히 넓히거나 줄이는가 | **통과.** Nine-case layout matrix와 ratio/minmax subset은 그대로다. |
| 2 | Chrome oracle의 direct Text range가 사라지는가 | **통과.** Text range와 item frame의 독립 geometry 비교가 수용 기준에 남아 있다. |
| 3 | text overhang를 runtime geometry에서 확인하는 항목이 있는가 | **통과.** Chrome range, paragraph layout, C11.4 trace와 item frame이 유지된다. |
| 4 | Android/iOS surface receipt가 제거되는가 | **통과.** 51/51 scene per platform과 PixelCopy/Metal 표본 수용이 남아 있다. |
| 5 | PixelCopy/Metal이 지원하지 않는 glyph paint를 통과 기준으로 삼는가 | **수정됨.** Box fill/clear만 assert하고 glyph 표면 비교는 하지 않는다. |
| 6 | Box 안/밖 sample이 track/frame geometry와 관계없이 정해지는가 | **통과.** CSS coordinate는 독립 expected geometry에서 capture 전에 고정한다. |
| 7 | Ahem font metric이 text overhang 계산에서 빠지는가 | **통과.** Browser precomparison과 runtime paragraph의 Ahem 10px 기준은 변경하지 않았다. |
| 8 | visible overflow가 item box background overflow와 오인되는가 | **통과.** Text range overflow와 CSS item background bounds를 다른 assertion으로 둔다. |
| 9 | track seam을 연속 item paint의 pixel로 검증한다고 과장하는가 | **수정됨.** seam은 track sizing trace/geometry로만 본다. |
| 10 | Chrome screenshot과 simulator screenshot이 같은 종류의 증거라고 쓰이는가 | **통과.** browser는 text paint oracle, simulator는 현재 box surface route로 역할을 구별한다. |
| 11 | 기존 Android screenshot이 이번 새 candidate run처럼 보이는가 | **통과.** 문서는 #850의 기존 screenshot을 limitation evidence로 표시한다. |
| 12 | Android software-renderer 한계가 빠지는가 | **통과.** AVD를 hardware GPU 증거로 확대하지 않는 문구를 유지한다. |
| 13 | iOS drawable readback이 compositor 이후 결과로 표시되는가 | **통과.** compositor 전 sample 경계가 유지된다. |
| 14 | simulator pixel 한계가 WPT/CSS support claim으로 번지는가 | **통과.** full WPT와 general CSS admission은 계속 제외된다. |
| 15 | C11.4 전체 구현을 완료로 표시하는가 | **통과.** parent status와 공식 `72/213` unchanged가 적혀 있다. |
| 16 | 내부 계약 0059를 계획 PR에서 이미 수정하는가 | **통과.** 계약 갱신은 구현 수용 뒤다. |
| 17 | 계획/리뷰 relative links가 변경 branch에서 깨지는가 | **통과.** 변경 후 모든 새 relative Markdown links를 다시 확인한다. |
| 18 | PR screenshot이 현재 새 feature output이라고 오인되는가 | **통과.** 기존 Android example이라는 alt/caption과 문구를 넣는다. |
| 19 | 계획 보정 PR에 코드/test change가 섞이는가 | **통과.** 계획과 검토 문서만 수정하고 product source를 건드리지 않는다. |
| 20 | PR 상태를 CI/runtime 검증 성공으로 부르는가 | **통과.** PR check rollup과 계획 amendment의 검증 범위를 별도로 보고한다. |
| 21 | 초기에 작성한 20개 계획 리뷰와 PR diff 검토를 같은 관점으로 중복 집계하는가 | **통과.** 이 표는 실제 변경된 두 계획 문서와 링크만 대조하며, 계획 자체 리뷰 표와 별도다. |
| 22 | 51 scene 합계가 각 플랫폼 51개씩이라는 의미를 잃는가 | **통과.** 문구는 Android와 iOS 각각 51/51을 요구한다. |
| 23 | text-overhang 수치가 Chrome oracle에만 의존하고 runtime 결과를 생략하는가 | **통과.** 계획은 Chrome Text range와 runtime paragraph, track trace, item frame을 함께 요구한다. |
| 24 | screenshot 경로가 아직 공개되지 않은 feature 캡처를 가리키는가 | **통과.** 연결된 PNG는 #850의 기존 Android AVD evidence이며 새 nine-case run이라고 부르지 않는다. |
| 25 | 변경이 원래 9-case HTML oracle 값이나 hash-bound precomparison evidence를 바꾸는가 | **통과.** amendment에는 oracle 입력·기대 geometry·evidence manifest 변경이 없다. |
| 26 | 지원하지 않는 glyph pixel 검증을 다른 용어로 다시 요구하는가 | **통과.** surface 기준은 box fill과 clear sample로 한정하고 glyph raster proof는 범위에서 제외했다. |
| 27 | runtime summary가 text range와 item background를 하나의 경계로 축약할 수 있는가 | **통과.** 두 geometry를 독립적으로 비교하라는 구분이 보정 뒤에도 유지된다. |
| 28 | C11.4 simulator evidence의 의미가 C11.3.2 AVD 완료 예외와 혼동되는가 | **통과.** 이 amendment는 C11.4 부분 계획이며 C11.3.2의 별도 승인 범위를 변경하지 않는다. |
| 29 | 새 내부 module/fixture 분리 설계가 이번 문서 변경만으로 구현된 것처럼 보이는가 | **통과.** 변경 파일은 plan/review Markdown뿐이고 runtime 구현은 명시적으로 미착수다. |
| 30 | 기존 계획 문서와 상위 계획 인덱스에 서로 다른 pixel acceptance가 남는가 | **통과.** 둘 다 box/clear pixel과 geometry 기반 text-overhang 검증으로 일치시켰다. |

최종 검증은 `git diff --check`, 변경 Markdown 상대 링크 확인 및 Agent Browser PR 본문 이미지 로드 확인이다. 이는 계획 보정 PR 검토이며 구현/CI 완료가 아니다.
