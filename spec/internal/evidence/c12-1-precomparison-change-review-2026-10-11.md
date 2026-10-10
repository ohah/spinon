# C12.1 사전 비교 변경 검토

고정 Chromium 사전 비교 artifact의 입력·수집·비교·문서 경계를 검토한 기록이다. 이 검토는 C12.1 런타임 구현 검증을 대신하지 않는다.

| 검토 관점 | 실패 가능성 | 확인 또는 반영 |
| --- | --- | --- |
| CSS 표준 provenance | mutable latest URL만 남아 나중에 의미가 바뀜 | 날짜 고정 W3C Working Draft URL을 inventory와 reference에 보존했다. |
| Chromium 바이너리 판별 | 다른 Chrome이 같은 결과를 생성했다고 오인 | 제품 버전, DevTools revision, 실행 파일 SHA-256을 각각 검사·기록한다. |
| 브라우저 임시 설정 | 개발자 프로필이나 네트워크 상태가 결과에 영향 | 임시 user-data-dir와 headless 고정 실행 flag만 사용한다. |
| locale/time zone | 시스템 기본값 차이가 fixture 동작에 섞임 | `en-US`·UTC를 설정하고 페이지에서 읽어 재검증한다. |
| 미디어 환경 | dark/forced-colors/pointer/hover 분기 누락 | 네 가지 media feature를 지정하고 관측값을 검증한다. |
| viewport/DPR | 고해상도 화면에서 CSS px가 물리 px와 혼동 | 360×800 CSS px, DPR 1·2를 각각 관측하고 양쪽 geometry를 보존한다. |
| inventory identity | 중복 ID나 case가 결과 배열을 오염 | case 12개와 node 47개의 고정 수 및 ID 중복을 거부한다. |
| case membership | 존재하지 않는 case나 빠진 node 연결 | 각 case의 node ID와 각 node의 case·parent 연결을 양방향 확인한다. |
| DOM parent/child | 화면 순서를 맞추려고 트리 순서를 바꿈 | 실제 fixture parent와 직접 자식 source order를 NodeId별 저장한다. |
| computed value | CSS 선언 원문을 computed value로 잘못 대체 | `display`, `position`, 네 물리 inset을 Chromium CSSOM에서 수집한다. |
| static 의미 | inset 계산값은 있는데 좌표에 잘못 적용 | static target의 computed inset과 원래 frame을 별도로 보존하고 기준 좌표를 단언한다. |
| relative 흐름 | 시각 offset을 layout/sibling 좌표에 반영 | flowRect와 최종 rect를 분리하고 뒤 형제의 불변을 검사한다. |
| 반대 방향 inset | right/bottom을 positive offset으로 해석 | right/bottom 단독 지정과 음수 offset을 각각 포함하고 부호를 검사한다. |
| 양쪽 inset 초과 제약 | 축·direction 조건을 섞어 처리 | LTR에서 left/right 및 top/bottom 양쪽 값의 사용 결과를 기록한다. RTL은 제외로 명시한다. |
| percentage basis | x/y 축 또는 content/padding edge 혼용 | 고정 크기 containing block의 computed inset과 target rect를 함께 보존한다. indefinite 입력은 제외한다. |
| calc/var | CSS math를 임의 숫자 파싱으로 치환 | author CSS의 `calc()`·custom property·fallback을 그대로 fixture에 둔다. |
| shorthand/cascade | shorthand 확장 또는 longhand 우선순위 누락 | inset 1–4값, longhand, layers, `!important`의 computed 물리 side를 검사한다. |
| owner ancestry | 자기 자신을 owner로 지정하거나 static 중간 노드가 가로챔 | 중첩 positioned ancestor와 static 자손을 포함하고 가장 가까운 positioned ancestor를 대조한다. |
| display:none | 숨김 subtree에 0 크기 box를 생성하고 성공 처리 | hidden subtree는 box 부재를 별도 검사하고 다음 sibling은 계속 관찰한다. |
| mutation/revision | 위치 변경 후 이전 owner 또는 stale frame 재사용 | target 단독, ancestor, nested ancestor 변경 상태를 별도 capture하고 흐름·owner·visual frame을 대조한다. |
| DPR 불변 | device pixel과 CSS geometry를 혼합 | 모든 state의 computed value, frame, flow frame을 DPR 1·2 사이 비교한다. |
| WPT 범위 표기 | fixture 일부 대응을 WPT suite 통과로 확대 | WPT revision/path는 subset/excluded로 기록하고 execution은 `not-run`으로 고정한다. |
| 기존 결과 보호 | 반복 capture가 검토된 reference를 조용히 덮음 | 기본 capture는 기존 파일 존재 시 중단하고 명시적 교체 flag를 요구한다. |
| 부분 생성 방지 | capture 중 오류가 나도 완료 JSON을 게시 | 전체 관측이 끝난 뒤에만 reference 파일을 쓴다. |
| 플랫폼 완료 과장 | fixture baseline을 제품 runtime 검증으로 표시 | 상태 대장과 evidence에 Android/iOS runtime 미실행 및 C12.1 미구현을 명시했다. |

## 반영한 보완

- 최초 검토에서 CSS Position 원본의 날짜 고정 URL, node-to-case 양방향 연결 검증, 한국어 fixture title이 빠져 있어 추가했다.
- reference 검사에서 WPT 실행을 한 것으로 오인하지 않도록 고정 `not-run` 판정과 excluded path를 유지했다.
- 현재 남은 제품 작업은 내부 API 계약, Stylo cascade, Taffy 위치 DTO, flow/visual frame 전달, cache invalidation, Android 실기기·iOS Simulator 실행이다.
