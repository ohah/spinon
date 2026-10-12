# C11.2.1 명시 Grid 배치 PR 변경 검토 · 2026-10-13

## 검토 범위

검토 대상은 `feat/c11-css-grid-plan`의 C11 구현·fixture·플랫폼 수집기와 상태 문서다. 전체 PR tree에는 검토 문서와 실행 근거가 추가되므로, 실행 증거의 `source-tree.txt`에 기록한 구현 tree `fb9411cb7cc7f767699447d29a202a8fadca1ece`를 런타임 재현의 정확한 입력으로 사용했다. unstaged C12.4 변경은 후보에 포함하지 않는다.

| # | 실패 관점 | 검토와 판정 |
| --- | --- | --- |
| 1 | 검증 대상이 바뀌었는데 과거 로그를 새 코드 결과로 오인할 수 있다 | 격리된 archive에서 구현 tree를 직접 빌드하고 전체 테스트를 다시 실행했다. Android/iOS 최종 matrix의 앱 SHA, inventory SHA, Chrome reference SHA가 각 run README에 연결된다. 구현 tree와 최종 문서 tree를 구분했다. 통과. |
| 2 | 다른 C12.4 작업이 C11 PR에 대량으로 섞일 수 있다 | staged diff에는 C12 전용 변경이 없고 새 readback API와 맞추는 `ffi/c12.rs` 호출 한 줄만 있다. 나머지 C12 변경은 unstaged다. 동시 기능 활성화 검사도 별도 통과했다. 통과. |
| 3 | 제한된 fixture 구현을 CSS Grid 또는 웹 호환 완료로 과장할 수 있다 | `RuntimeBlockFlexGridPlacementV1`은 explicit track의 numeric line/span profile로 격리했다. STATUS·계획·로드맵 모두 C11.2.1과 상위 C11을 미완료로 유지한다. 통과. |
| 4 | 기존 C11.1 자동 배치 profile 동작을 바꿀 수 있다 | 기존 `RuntimeBlockFlexGridV1`과 새 profile 진입점을 분리했고, all-auto child는 기존 경로를 유지하는 layout 및 runtime 회귀가 있다. 통과. |
| 5 | Stylo가 인식한 CSS 값이 검증 없이 Taffy 입력으로 새어 나갈 수 있다 | cascade가 typed computed value를 만들고 style-to-layout adapter가 속성·노드 문맥으로 지원 밖 값을 거부한다. 문자열 재파싱 없이 typed placement가 도달하는 테스트가 있다. 통과. |
| 6 | shorthand·longhand 우선순위가 CSS cascade와 달라질 수 있다 | `grid-area` shorthand 전개 후 longhand override를 pinned Chrome computed value와 geometry로 비교하는 fixture가 있다. 통과. |
| 7 | 음수 line이 explicit grid 반대쪽 edge가 아니라 implicit track 기준으로 계산될 수 있다 | 양·음수 line 경계의 정규화가 track 수를 입력으로 받으며 Chrome numeric-lines scene과 layout-core 단위 검사에서 양 끝을 비교한다. 통과. |
| 8 | 양 끝점 중 하나에서 span을 계산할 때 정수 overflow나 잘못된 span 방향을 숨길 수 있다 | 양수 span만 받고 checked add/sub로 반대 line을 계산한다. 양 endpoint 방식 모두 fixture에 있고 zero·음수·범위 초과는 거부한다. 통과. |
| 9 | line 0, 동일 line, 역순 line이 Taffy의 Auto나 무효 span으로 조용히 바뀔 수 있다 | line 0과 동일 line은 fail-closed하며 역순 numeric line은 CSS conflict rule에 따라 교환한다. 각각 별도 회귀가 있다. 통과. |
| 10 | implicit track 또는 mixed auto/explicit item을 성공 frame처럼 표시할 수 있다 | explicit track 밖, named line, mixed auto/explicit, 지원 밖 placement는 구체적 오류로 거부한다. 실패 뒤 부분 frame이 없는 테스트가 있다. 통과. |
| 11 | 보이지 않는 자식의 지원 밖 값이 보이는 Grid 계산을 거부하거나 결과를 오염시킬 수 있다 | `display:none` child는 배치 분류에서 제외하고 hidden item·hidden nested Grid에서 unsupported track/placement가 layout에 들어오지 않는 cascade-to-layout 회귀가 있다. 통과. |
| 12 | 형제 또는 nested Grid container의 placement mode가 서로 충돌할 수 있다 | 배치 분류는 각 Grid container의 보이는 direct child에 대해 독립 수행하며 nested/형제 container 경계 fixture가 있다. 통과. |
| 13 | 두 번째 item 오류가 먼저 계산한 item의 frame을 부분 공개할 수 있다 | layout 출력은 배치 검증과 계산 전체가 성공한 뒤 생성한다. 잘못된 두 번째 item에서 전체 layout 부재를 확인한다. 통과. |
| 14 | overlap 또는 정규화가 DOM preorder와 paint 순서를 몰래 바꿀 수 있다 | geometry·document child order·paint snapshot을 별도로 검사한다. `order`/C12 stacking 의미를 추가하지 않고 overlap source order를 검증한다. 통과. |
| 15 | min/max, intrinsic sizing, alignment 등 이번 profile 밖 속성을 기본값으로 버리고 성공할 수 있다 | unsupported constraints와 item/container alignment·`place-*`를 명시적으로 범위 밖으로 둔 negative fixtures가 있다. 통과. |
| 16 | readback FFI가 null 포인터, 0 capacity, 과도한 표본 수, 출력 버퍼 경계에서 메모리를 잘못 쓸 수 있다 | test-only FFI는 인수와 표본 개수 상한을 검사한다. null·capacity·overflow 경계 8개 검사가 통과했고, C12 단독 및 C11+C12 feature coactivation도 통과했다. 임의의 비-null 잘못된 주소는 호출자 계약이므로 주입하지 않았다고 증거 문서에 적었다. 통과. |
| 17 | 여러 표면 sample의 순서·revision이 섞여 다른 장면을 exact 결과로 오인할 수 있다 | readback report는 현재 snapshot revision/generation과 표본 배열을 함께 기록한다. 비교기는 run·좌표·RGBA·표본 수·scene allowlist를 고정하고 stale/duplicate/변형 입력을 거부한다. 통과. |
| 18 | Android emulator 결과나 PNG 화면을 실기기 GPU 표면 증거라고 표시할 수 있다 | 최종 Android run은 qemu=0 SM-S731N의 PixelCopy이고 renderer를 Samsung Xclipse 940/Vulkan 경유 ANGLE GLES로 기록한다. 화면 PNG는 보조 자료, exact 값은 PixelCopy 결과로 구분했다. 통과. |
| 19 | iOS drawable readback을 최종 compositor 화면 또는 iOS 실기기 결과로 잘못 부를 수 있다 | 최종 iOS run은 iPhone 17 Pro/iOS 26.2 Simulator의 Metal drawable texture이며 compositor 제출 전 단계라고 명시했다. iOS 실기기와 compositor 이후 parity를 미완료로 둔다. 통과. |
| 20 | 문서가 다음 단계·지원 경계를 바꾸거나 기능 결과를 과장할 수 있다 | 상태 대장·C11.2.1 계획·C11 상위 계획·내부 evidence index·공개 roadmap에 같은 exact run과 미완료 범위를 연결했다. GitHub Pages 배포는 하지 않으며 C11.2.2 runtime은 C11.2.1 통합 뒤 시작하도록 유지한다. 통과. |

## 결과

코드·테스트 모듈·Android/iOS 실행 경계·버전 있는 내부 계약과 문서가 같은 제한 profile을 가리킨다. 전체 후보 검증은 [실행 요약](./c11-2-1-index-candidate-2026-10-13/README.md), 구현 단계의 세부 수정·실패 관점은 [인덱스 후보 통합 구현 검토](./c11-2-1-index-candidate-implementation-review-2026-10-13.md)를 따른다. 전체 C11, Spinon WPT 적합성, 다른 기기군, iOS 실기기와 compositor 이후 픽셀 비교는 완료로 판정하지 않는다.
