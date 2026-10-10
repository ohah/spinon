# C12.1 구현 실패 경로 검토

검토 대상은 C12.1 변경 코드, Chromium fixture, Rust layout·style-to-layout 검사, Android 실기기와 iOS Simulator 로그다. 아래 항목은 같은 질문을 반복하지 않고 입력 의미, 좌표, 소유자, 오류, 런타임·플랫폼 경계를 각각 확인한다.

| # | 실패 관점 | 확인 결과와 근거 |
| --- | --- | --- |
| 1 | fixture나 Chromium reference가 수정된 뒤에도 옛 기준을 통과시킬 수 있는가 | inventory·HTML·runtime source·capture tool digest와 Chromium 154 revision을 고정했다. 명시적 교체 없이 reference를 덮어쓰지 않는 검사도 통과했다. |
| 2 | 47개 fixture 노드의 부모·source order가 플랫폼 로그 매핑과 다를 수 있는가 | inventory, reference, runtime DOM preorder를 양방향 대조한다. node 누락·중복·부모 불일치 검사를 통과했다. |
| 3 | `position:static`의 계산된 inset이 used geometry에 잘못 적용되는가 | `left:12px;top:6px`를 computed style에 보존하면서 `rect == flowRect`인 것을 Chrome fixture와 style-to-layout 테스트에서 확인했다. |
| 4 | relative inset이 자기 normal-flow 위치를 바꾸는가 | 별도 `flow_frames`와 visual `frames`를 검사했다. 상대 대상의 흐름 좌표는 그대로이고 시각 좌표만 이동했다. |
| 5 | 이동한 Flex 자식 때문에 다음 sibling이 밀리는가 | Rust `relative_flex_item_moves_visually_without_moving_its_sibling`과 Chromium fixture에서 sibling x 좌표가 유지됐다. |
| 6 | 중첩 relative offset이 자손에게 두 번 더해지는가 | 중첩 부모·자식 inset을 각각 적용한 grandchild 좌표가 합계 offset과 일치했고 flow 좌표는 움직이지 않았다. |
| 7 | `right`만 있는 경우 방향 또는 부호가 뒤집히는가 | LTR `right:8px`의 시각 x 이동이 −8 CSS px인지 reference에서 확인했다. |
| 8 | `bottom`만 있는 경우 수직 축이나 부호가 뒤집히는가 | LTR `bottom:4px`의 시각 y 이동이 +4 CSS px인지 reference에서 확인했다. |
| 9 | 양쪽 inset이 모두 설정된 과잉 제약에서 반대쪽이 선택되는가 | LTR에서 가로 `left`, 세로 `top`이 사용되는 reference geometry와 computed 값을 확인했다. |
| 10 | 네 inset이 `auto`일 때 이전 위치나 0이 아닌 offset이 남는가 | `auto` inset의 visual frame과 flow frame이 같은지 Chromium fixture에서 확인했다. |
| 11 | percentage가 width 대신 height 또는 반대로 계산되는가 | 서로 다른 containing-block 폭·높이로 Rust와 Chromium에서 가로·세로 basis를 각각 비교했다. |
| 12 | `calc()`·custom property·음수 계산값을 길이로 잘못 제한하는가 | `calc(10% + var(...))`, 음수 calc 및 Typed math 투영 결과가 CSS px geometry와 일치했다. |
| 13 | 1–4값 `inset` shorthand나 cascade layer·`!important`가 longhand 값을 덮어쓰는가 | 네 shorthand 확장과 layer·important 승자를 computed property 및 geometry로 검사했다. |
| 14 | positioned containing-block owner가 자기 자신 또는 먼 ancestor로 잘못 기록되는가 | root는 viewport owner, nested child는 가장 가까운 relative ancestor, 중첩 relative는 자기 자손의 owner로 확인했다. |
| 15 | `display:none` ancestor의 자손이 보이는 box나 owner를 남기는가 | hidden parent·child가 모두 box와 owner를 만들지 않고, 뒤 sibling은 flow에 남았다. |
| 16 | layout root 자체의 relative percentage가 definite viewport basis를 받지 못하거나 Taffy root에서 무시되는가 | 초기 실패를 테스트로 재현했다. relative root일 때만 synthetic viewport containing box를 추가하고, root·자손의 10%/25% 좌표 회귀 테스트를 통과시켰다. |
| 17 | DPR이 바뀌면 computed CSS 값이나 CSS geometry가 바뀌는가 | DPR 1·2의 전체 fixture property, flow rect, visual rect가 동일한지 확인했다. |
| 18 | runtime inline style 변경 뒤 stale flow frame 또는 owner cache가 남는가 | 같은 V8 문서에서 initial→target-relative→ancestor-relative을 적용해 흐름 보존, visual offset 및 owner 갱신을 대조했다. |
| 19 | 이번 단계에서 제외한 `absolute`·`fixed`·`sticky`가 성공 기본값으로 조용히 치환되는가 | 세 값을 style-to-layout 통합 테스트에서 모두 `position` 속성이 명시된 오류로 거부한다. 기존 C09 오류 기대값도 이 진단 계약에 맞췄다. |
| 20 | 잘못된 입력이 partial frame을 내거나 FFI fixture state가 임의 script를 실행할 수 있는가 | 알 수 없는 NodeId, NaN·무한대 inset, RTL relative, 범위 밖 state 번호를 거부한다. 상태 변경 script는 고정 inventory에서 파생되는지 검사했다. |

## 플랫폼 재현

[실행 근거 폴더](./c12-1-static-relative/README.md)의 Android 실기기와 iOS Simulator 로그에서 세 상태 모두 layout-ready·48 frames·WGPU 제출을 확인했다. 체크인 로그를 읽는 CSS reference 검사에서 Android와 iOS의 각 47 fixture element를 Chrome에 대조했고 모든 비교 필드의 최대 차이는 0 CSS px였다. 모바일 root의 viewport width·height는 화면 크기가 달라 비교에서 제외했으며, 실기기 성능 일반화나 전체 CSS 동작은 주장하지 않는다.

## 검토 중 발견해 고친 문제

- `position:absolute`가 이제 구체적인 `unsupported_computed_value`로 거부되므로 C09 테스트의 오래된 일반 오류 기대값을 갱신했다.
- inset 공용 parser가 인자 8개를 받아 Clippy에서 실패했다. inset `auto` 처리를 호출자에 두어 공용 parser를 단순화했다.
- runtime state 실행에서 `document.getElementById`를 사용할 수 없었다. 초기 fixture가 보관한 node reference로 바꿨다.
- Android에서 첫 scene supersession 응답을 최종 실패로 처리할 수 있었다. 같은 inventory state만 최대 5회 재시도하고 성공·재시도 횟수를 로그에 남긴다. 검증 실행에서는 한 차례 retry 뒤 통과했다.
- 모바일 frame 증거가 원래 build 디렉터리에만 있었고 reference suite가 이를 확인하지 않았다. 로그와 캡처를 체크인하고 47개 노드 비교를 CSS reference suite에 연결했다.

## Android 초기 표면 준비 후속 수정 검토

초기화 중 Android Surface 크기가 확정되면서 첫 C12.1 평가가 `status=-12`로 superseded되는 실패를 실기기에서 다시 발견했다. 초기 C12.1 평가를 양수 크기의 `surfaceChanged` 이후로 옮겼다. 이 변경은 C12.1 adapter 외에 다른 fixture 초기화 순서를 바꾸지 않는다. 다음 별도 관점으로 후속 diff와 실기기 경로를 대조했다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | 실제 cold start에서 첫 평가가 Surface 생성과 경합하는가 | 수정 전 실기기에서 최초 평가 `status=-12`를 재현했다. |
| 2 | 단순 재평가가 fixture DOM을 중복 생성하는가 | 초기 script는 새 root를 append하므로 자동 재시도 대신 시작 순서를 고정했다. |
| 3 | Surface가 만들어지기 전 `surfaceCreated`만으로 시작하는가 | 시작 조건은 크기까지 확정되는 `surfaceChanged`다. |
| 4 | 0 폭 또는 0 높이 Surface에서 시작하는가 | 두 값 모두 양수인지 검사한다. |
| 5 | C12.1 이외의 runtime fixture도 새 대기 조건을 받는가 | 기존 경로는 생성자에서 즉시 runtime queue에 등록된다. |
| 6 | 중복 `surfaceChanged`가 host를 여러 번 만드는가 | `runtimeInitializationQueued`와 `hostHandle`을 함께 확인한다. |
| 7 | 이미 생성된 host에 초기화 요청이 다시 들어가는가 | 비영 host 검사로 차단한다. |
| 8 | Surface 소멸 후 남은 예약 작업이 host를 시작하는가 | runtime queue 실행 직전 closing·surface·크기를 다시 확인한다. |
| 9 | 예약 작업이 Surface 소멸로 무효화된 뒤 재생성이 영구 차단되는가 | 실행 직전 조건을 잃으면 예약 플래그를 해제해 다음 유효 callback을 허용한다. |
| 10 | Activity 종료와 callback이 경합해 닫힌 queue에 작업을 넣는가 | 초기 조건과 queue 등록 양쪽에서 closing을 검사한다. |
| 11 | viewport가 임시 기본값 301×100으로 캡처되는가 | 초기화 전에 실제 `surfaceChanged` 크기를 기록하고 이를 viewport 환산에 쓴다. |
| 12 | Surface 크기 갱신과 초기화가 메인 스레드에서 동기 대기하는가 | 초기화는 기존 단일 runtime executor에 예약한다. callback에 대기·sleep을 넣지 않았다. |
| 13 | V8 소유 thread가 달라지는가 | V8 host 생성과 script 실행은 계속 같은 직렬 runtime queue에서 진행한다. |
| 14 | Surface callback이 host 생성 전에 환경 갱신을 큐잉하는가 | host가 없을 때 `refreshEnvironment`가 반환하고, 유효 크기 callback이 초기화를 예약한다. |
| 15 | 초기화 후 resize가 환경 갱신 경로를 잃는가 | host 생성 뒤 크기 변경은 기존 `refreshEnvironment` 경로를 사용한다. |
| 16 | 초기화 전 render 요청이 잘못된 renderer를 만드는가 | host 없는 reconcile 경로를 확인했고 초기 완료 뒤 draw/reconcile을 다시 요청한다. |
| 17 | 첫 단계 supersession에 대해 중복 DOM 실행을 추가하는가 | 초기 fixture 재시도는 추가하지 않았으며, 실패는 기존 오류로 노출된다. |
| 18 | Intent extra가 다른 fixture 분기를 선택하게 되는가 | 실기기 APK에 `spinon_c121_static_relative=true`만 전달해 세 C12.1 상태를 실행했다. |
| 19 | 회전·Surface 변경 뒤 이전 generation을 새 결과처럼 기록하는가 | generation 기반 장면 무효화와 이후 환경 갱신 경로는 그대로 유지했다. |
| 20 | 수정이 빌드뿐이고 실제 시작 경합을 닫지 못하는가 | SM-S731N에서 APK 재설치 후 앱 종료·재실행 3회 모두 세 상태, 각 48 frame summary, Vulkan WGPU `presented boxes=45`를 확인했다. |

수정 후 실기기 로그 세 건은 [cold start 증거](./c12-1-static-relative/)에 저장했다. 첫 실패의 원본 Android logcat은 후속 cold start 과정에서 기기 ring buffer에서 밀려났으므로, 체크인된 로그는 수정 후 세 실행만 나타낸다.

남은 절대·fixed·sticky 배치, 논리 inset·RTL, inline fragmentation, clipping, text shaping, DOM hit-test와 접근성은 후속 계약 범위다.
