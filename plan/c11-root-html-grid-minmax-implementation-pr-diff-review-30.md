# C11.1 implementation PR diff hostile review · 30 perspectives

**대상:** 현재 stage된 C11.1 implementation candidate diff. review inventory 파일 자체는 대상에서 제외했다. PR 번호·CI·merge는 아직 없다.

이 30개 patch-level 관점은 구현/runtime의 별도 20관점 검토와 겹치지 않도록 변경 경계·호환성·검증 주장·제출 품질을 검사했다.

| # | Patch 관점 | 확인한 실패 경로 | 판정 |
|---:|---|---|---|
| 1 | 변경 범위 경계 | C11.1 외 roadmap·status 기능을 구현 완료로 섞음 | `spec/STATUS.md`는 수정하지 않았고 이번 plan만 진행 중으로 바꿨다. 통과. |
| 2 | 기반 revision | 최신 base와 변경 candidate를 혼동 | base HEAD `773cfe232`를 evidence에 기록하고 candidate가 미커밋 상태임을 분명히 했다. 통과. |
| 3 | 공식 집계 | 증거 추가를 공식 완료 항목 수에 반영 | C11/C11.1 checkbox를 그대로 두고 수치를 고정 기입하지 않았다. Tailnet의 현재 `72/213`과 원본 대장의 차이는 별도 후속으로 기록했다. 통과. |
| 4 | API surface | 내부 런타임 기능에 새 public API·ABI를 끼워 넣음 | 새 제품 API·scene route·public ABI가 없다. Pixel capture ABI는 test feature 경계 안이다. 통과. |
| 5 | 계약 버전 | 계획·구현만으로 internal contract version 상승 | 내부 계약 버전을 바꾸지 않았다. 통과. |
| 6 | computed-style admission | Root HTML profile flag가 다른 profile로 확산 | 별도 option을 RuntimeRootHtmlTextPaintV1에만 설정했다. 통과. |
| 7 | CSS conversion reuse | 기존 track converter를 broad admission으로 전환 | 기존 intrinsic 여부와 Root HTML bounded-minmax 여부를 분리 전달한다. 통과. |
| 8 | cascade scope | 무효 negative `fr` 진단 무시가 모든 profile에 적용 | diagnostic suppression은 RuntimeRootHtmlTextPaintV1 profile로 한정했다. 통과. |
| 9 | cascade source parity | stylesheet와 inline style의 recovery 차이 | 두 diagnostic 경로에 같은 좁은 matcher를 적용하고 회귀 테스트를 추가했다. 통과. |
| 10 | `!important` parsing | 대소문자·comment 사이 important에서 fallback 실패 | 단순 음수 `fr` recovery 테스트가 대소문자와 주석 변형을 다룬다. 통과. |
| 11 | declaration parser overreach | helper가 색상·shorthand·multi-track까지 삼킴 | 네 허용 longhand와 단일 음수 simple `fr` 외 입력은 matcher가 거부한다. 통과. |
| 12 | track variant bounds | `minmax(auto, …)` 등 intrinsic breadth 혼입 | typed gate와 layout validator 모두 허용 breadth를 별도로 제한한다. 통과. |
| 13 | axis constraints | percentage·flex maximum이 width/height 제약 검사 밖으로 빠짐 | nested breadth 탐지와 기존 axis gate를 공유한다. 통과. |
| 14 | resolved arithmetic | px·percentage·gap 합산 overflow | 축별 resolved range 검사가 fixed/gap 누계를 f32 상한에서 막는다. 통과. |
| 15 | flex arithmetic | 개별 유한 `fr` 누적합 overflow | direct와 minmax `fr` factor 합계를 검사하는 rejection case가 있다. 통과. |
| 16 | layout model boundary | 새 mode가 일반 Grid 레이아웃에서 minmax 허용 | Taffy 계층 gate는 RootHtmlImplicitRowsV1에서만 bounded minmax를 연다. 통과. |
| 17 | implicit row semantics | default auto row를 minmax 지원에 따라 조용히 수락 | implicit row 확장은 별도 validator가 막고 runtime rejection test를 추가했다. 통과. |
| 18 | column flow | row용 bounded minmax가 column auto-placement로 누출 | RootHtmlColumnFlowV1와 row implicit mode가 분리되고 기존 거부 경계를 유지한다. 통과. |
| 19 | named areas | template-area 완화를 통해 지원 확대 | 이름 있는 area 경로는 별도 fixed-track contract에 남았다. 통과. |
| 20 | repeat accounting | integer repeat expansion에서 minmax metadata·quota 손실 | 확장 후 direct track과 같은 converter·line metadata 예약을 사용한다. 통과. |
| 21 | hidden/order behavior | display none·document order·overflow가 minmax와 결합해 변경 | 기존 auto-placement/rejection tests를 유지하고 동일 parsed DOM 경로에서 검사한다. 통과. |
| 22 | snapshot identity | DOM/class/index 기반 성공 branch 또는 revision 갈림 | 테스트는 source·computed cascade·layout key를 대조하고 제품 경로는 일반 computed style을 사용한다. 통과. |
| 23 | failure publication | unsupported track 실패 뒤 partial render snapshot 게시 | 실패 runtime case에서 `completed.is_none()`를 확인한다. 통과. |
| 24 | Android build default | capture argument/BuildConfig가 제품 기본 실행에서 활성화 | environment flag default 0, opt-in 값 검증과 C04 runtime 의존 조건을 추가했다. 통과. |
| 25 | Android launch route | capture extras가 fixture override로 오해되어 Root HTML route 이탈 | test flag + true capture 요청일 때 capture extras만 override 검사에서 제외한다. 통과. |
| 26 | Android surface geometry | grid 안쪽만 샘플해 overflow assertion을 거짓 양성 | 두 번째 표본은 grid width 밖 CSS x=44이고 AVD PixelCopy에서 exact-match했다. 통과. |
| 27 | Android capture lifecycle | stale surface/generation에서 capture하거나 실패 상태를 조용히 숨김 | generation/state guard 유지, opt-in 로그로 configured/dispatch/skipped/error/summary를 남긴다. 통과. |
| 28 | iOS build flag | feature-off product compilation에 test parser 포함 | root HTML special argument parser는 `SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST_1` 조건 안이고 양쪽 Swift parse 및 feature-on Simulator build가 통과했다. |
| 29 | iOS route/parser | 중복 ID·잘못된 ID·다른 spinon argument 혼입 | 단일 argument, ASCII ID, 길이·문자 제한과 Root HTML 기본 route 조건을 적용한다. 통과. |
| 30 | evidence / PR claims | simulator를 hardware로 표현하거나 미실행 gate를 통과 처리 | AVD SwiftShader와 iOS Simulator Metal 경계를 기록하고 PR/CI/merge가 남았음을 명시했다. Tailnet 표시 수는 현재 원본 매핑 오류가 있어 official state와 분리했다. 통과. |

**판정:** 현재 staged diff에서 해결되지 않은 patch-level 결함을 찾지 못했다. GitHub PR의 CI·required checks·최종 merge 상태는 이 문서의 판정에 포함하지 않았고, PR을 만들고 나서 별도로 확인해야 한다.
