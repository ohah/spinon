# C11.4 zoomed `fit-content()` 계획 PR diff 실패 관점 검토 20개

검토 대상은 zoomed fit-content 계획 PR의 전체 staged 변경이다. 계획 검토 20개와 구분해 문서 변경, 근거 연결, 범위 및 공개 상태를 다시 살폈다. 이 검토는 Spinon 구현이나 Android/iOS runtime 검증으로 간주하지 않는다.

| # | 실패 관점 | 확인·조치 | 판정 |
|---:|---|---|---|
| 1 | 변경이 여러 fit-content assertion을 한 번에 구현 대상으로 섞을 수 있음 | 계획 범위는 pinned 파일의 `zoomedGrid` assertion 하나로 고정했다. 상위 계획도 zoomed assertion을 별도 계획이라고 표시한다. | PASS |
| 2 | source 파일이 바뀌었는데 근거가 예전 내용을 가리킬 수 있음 | WPT commit, 경로, source SHA 및 live response SHA를 pins JSON에 기록했다. 두 SHA는 같다. | PASS |
| 3 | 정확한 WPT assertion을 오인할 수 있음 | Agent Browser observation에 `zoomedGrid`의 assertion name과 expected `gridTemplateRows:40px`가 기록돼 있다. | PASS |
| 4 | WPT 전체 pass를 Spinon runtime pass로 읽을 수 있음 | 계획·STATUS·evidence는 HeadlessChrome harness 13/13을 upstream 결과로만 분류하고 Spinon 구현/runtime은 미실행으로 썼다. | PASS |
| 5 | Chrome 버전과 화면이 재현 불가능할 수 있음 | Agent Browser 0.31.1, HeadlessChrome 156, viewport, DPR, full harness screenshot과 SHA-256을 저장했다. | PASS |
| 6 | fit-content 공식을 잘못 적용할 수 있음 | 독립 계산은 CSS Grid 2025 CRD §11.2 공식 링크와 고정 입력으로 `max(0,min(40,400))=40`을 도출한다. | PASS |
| 7 | CSS zoom used values와 normalized fixture 값을 동일시할 수 있음 | 문서가 CSS Viewport Editor's Draft §4/4.1의 used-length scaling과 unscaled computed-style 경계를 기록한다. `40px`은 normalized fixture result로, `80px`은 visual integration expectation으로 구분했다. | PASS |
| 8 | WPT CSSOM assertion을 actual compositor geometry라 오인할 수 있음 | `gridTemplateRows:40px`은 WPT assertion이고 root/item/child visual heights는 별도 integration 기준이라고 반복해 경계 지었다. | PASS |
| 9 | width normalization이 block-axis expectation을 바꿀 수 있음 | original root와 inline-width 60px probe의 rows는 모두 40px이고 item/child visual heights는 각각 80/800px로 evidence에 남겼다. | PASS |
| 10 | normalized clone의 다른 dimensions를 빠뜨릴 수 있음 | Probe에는 original/normalized grid width·height·rect와 item/child rect를 각각 기록했다. Root zoom과 block geometry는 유지된다. | PASS |
| 11 | `min-height:0`을 빼서 automatic minimum이 결과를 바꿀 수 있음 | 계획과 expected input에서 direct item의 `min-height:0`을 고정하고 minimum contribution 0을 계산에 사용한다. | PASS |
| 12 | child intrinsic contribution을 과소 계산할 수 있음 | 고정 child height 400px을 max-content contribution으로 명시하고, 이 값을 cap 40px과 별도 유지한다. | PASS |
| 13 | app fixture가 computed zoom 없이 source string만 읽을 수 있음 | Stylo computed zoom을 같은 revision으로 snapshot/matcher에 전달하도록 하고 inline-only bypass는 금지했다. | PASS |
| 14 | 이 제한 fixture가 일반 CSS zoom 지원으로 홍보될 수 있음 | root 단일 zoom owner, exact scene/style/tree input만 허용하고 일반·nested/ancestor zoom은 명시적으로 제외했다. | PASS |
| 15 | Taffy coordinates와 화면 frames가 혼동될 수 있음 | normalized track result와 zoomed visual geometry를 다른 contract 값으로 정의했다. | PASS |
| 16 | Chrome 기준으로 규범 기대를 정할 수 있음 | CRD 계산, WPT CSSOM expected, Chrome observation 및 app integration expectation을 구분했다. | PASS |
| 17 | screenshot만으로 outside-row child overflow를 증명할 수 있음 | 향후 구현 acceptance가 Android PixelCopy와 iOS Metal fixed samples를 요구한다. Screenshot 단독 판정을 금지했다. | PASS |
| 18 | fixture code가 기본 product archive에 포함될 수 있음 | Default/product negative control과 fixture-test positive control을 acceptance criteria에 각각 요구한다. | PASS |
| 19 | 한 계획 항목이 공식 완료 수를 늘릴 수 있음 | STATUS와 public roadmap 모두 계획 단계·미실행 runtime·C11.4 미완료 및 `72/213` 유지를 명시한다. | PASS |
| 20 | PR 화면 링크를 열 수 없거나 로컬 링크가 깨질 수 있음 | PNG는 PR branch의 raw GitHub URL로 본문에 삽입 가능하며, 새 계획/evidence 상대 링크를 확인했다. 계획 자료 외 구현 screenshot은 아직 없다. | PASS |

**결과:** 계획 PR diff 20/20 관점에서 차단 결함 없음. 계획 자체 검토, 향후 구현/runtime 검토, 구현 PR diff 검토는 각각 별도다.
