# C13.1 compact surface matrix 계획 실패 관점 검토 · 2026-10-14

이 검토는 compact Android/iOS runtime fixture의 기준 장면, oracle, pixel sample 판정, simulator/feature 경계만 공격한다. Chrome capture와 구현 코드를 만들기 전에 수행한다.

| # | 공격 관점 | 판정 기준 |
|---:|---|---|
| 1 | 13개 case가 300×100 surface에 실제로 들어가지 않아 행끼리 가려질 수 있다 | 각 tile/target/escape frame을 Chrome에서 고정하고 viewport 경계 안에 드는지 검사한다. 겹침은 같은 좌표 hit identity로 판정한다. |
| 2 | 3열×5행 좌표 계산에 13행만 넣고 빈 slot을 case로 센다 | 고정 ID 13개와 별도 빈 2개를 유지하며 node/sample count는 13개 기준이다. |
| 3 | DPR이 달라 Chrome CSS 좌표와 PixelCopy surface pixel이 섞인다 | Chrome DPR1 CSS point와 Android runtime density/devicePixelRatio mapping을 각 artifact에 저장한다. PixelCopy helper의 CSS→physical 좌표를 그대로 사용한다. |
| 4 | screenshot viewport와 app root size가 다르다 | Chrome/root를 300×100으로 고정하고 Android AVD runtime environment 및 PixelCopy dimensions를 기록한다. iOS 환경 차이는 report에 분리한다. |
| 5 | browser scroll이 0이 아니어서 sample 좌표가 밀린다 | `scrollTo(0,0)` 뒤 `scrollX/Y==0`을 강제한다. |
| 6 | Chrome version/fixture 변경이 옛 sample coordinates에 연결된다 | version·revision·UA·fixture SHA와 sample observation hash를 pin한다. |
| 7 | target가 완전히 clip되어 target color 표본이 없는 사례가 생긴다 | 각 case의 내부 paint 표본이 Chrome target/inner node identity를 반환하는지 oracle 단계에서 검사한다. |
| 8 | x/y clip sample이 축별 경계 밖이 아닌 target 안에 위치한다 | capture script에서 Chrome frame/clip geometry와 sample CSS coordinate를 검증하고 실제 hit identity를 저장한다. |
| 9 | overflow-clip-margin reference box 차이를 표본이 구분하지 못한다 | padding/content/border 각 case에서 clip edge 안/밖 sample을 두고 Chrome RGBA/hit identity가 분리되는지 확인한다. |
| 10 | `hidden`이 clip margin 확장을 적용해도 같은 색 sample을 통과한다 | hidden margin-ignore case의 확장 edge 좌표를 Chrome oracle이 parent/root identity로 고정한다. |
| 11 | visible-axis/hidden-axis normalization이 x/y sample에서 섞인다 | 두 mixed-axis case의 X/Y sample identity와 RGBA를 별개 sample ID로 둔다. |
| 12 | positioned-contained child와 escape child의 containing block을 바꾼다 | 두 target frame과 outside-parent sample을 Chrome geometry 및 identity에 맞춘다. |
| 13 | nested clip inner background가 target/outer node sample로 혼동된다 | nested inner, target, outer box에 각각 분리 가능한 sample color와 node identity를 둔다. |
| 14 | sample 개수만 맞고 같은 좌표를 중복 사용한다 | 52 sample ID/좌표를 중복 검사하며 case당 4개 ID 역할을 고정한다. |
| 15 | pixel sample이 Android full screenshot crop 또는 density를 잘못 적용해 우연히 색이 같다 | PixelCopy dimensions, density, generation, surface size를 고정하고 원본 PNG hash와 좌표를 기록한다. |
| 16 | PixelCopy 재시도/Surface generation 변경이 이전 scene 픽셀을 섞는다 | request 전후 renderer·generation·size gate를 유지하고 결과에서 동일 generation을 검증한다. |
| 17 | iOS screenshot을 drawable readback이나 최종 display로 과장한다 | plan과 결과에서 Metal 제출·Simulator 화면 캡처만 주장하고 readback/compositor를 제외한다. |
| 18 | 앱 fixture route가 기본 runtime ABI/payload에 들어간다 | C13 fixture feature-off/on archive에서 symbol/source 부재·존재를 각각 검증한다. |
| 19 | browser oracle 성공만으로 Android/iOS runtime 결과를 성공 처리한다 | 각 platform은 독립 app runtime report 및 surface capture가 모두 있어야 해당 결과를 통과로 기록한다. |
| 20 | sample matrix를 전체 300×100 surface·event/WPT·C13.1 완료로 과장한다 | 52 point sample과 PNG 범위를 분리하고 DOM event/WPT/C13.1 gate를 명시적으로 미완료로 둔다. |

계획 검토는 oracle capture/구현 전에 마쳤다. 표본 좌표의 실재성·tile 겹침·Chrome 범위는 다음 단계에서 capture로 확인하며, 발견된 mismatch가 있으면 구현 전에 이 계획과 review 근거를 갱신한다.
