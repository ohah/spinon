# C04.10 UIKit surface 스레드 분리 계획 재검토

- 대상: [`plan/c04-runtime-css-to-gpu.md`](../../../plan/c04-runtime-css-to-gpu.md)
- 검토 대상 SHA-256: `637e3bb66a0104c3721b6aaa74c43d6103e495eeebabc280464c16a2c0c5d395`
- 근거: 잠금된 `wgpu 30.0.1` 및 전이 의존성 `raw-window-metal 1.1.0`의 실제 UIKit surface 생성 경로를 확인했다. `Layer::from_ui_view`는 `MainThreadMarker`를 요구한다.

| # | 별도 실패 관점 | 계획에서 확인한 계약 |
| ---: | --- | --- |
| 1 | UIKit view를 background thread에서 읽는가 | `UIView.layer`에서 WGPU surface를 준비하는 단계만 메인 스레드에 둔다. |
| 2 | 문서 설명만으로 메인 스레드 제약을 추측하는가 | 잠긴 WGPU 의존성의 실제 `from_ui_view` 경로와 메인 스레드 확인을 근거로 삼는다. |
| 3 | Surface 생성과 GPU 장치 초기화를 한 단계로 묶는가 | surface 준비와 adapter·device·pipeline 생성을 별도 FFI 호출로 분리한다. |
| 4 | CSS/layout 대기가 UI thread로 이동하는가 | V8 평가와 CSS 완료 대기는 runtime background executor에 남긴다. |
| 5 | GPU adapter 선택이 UI를 막는가 | adapter 요청과 완료 대기는 serial render queue에서 수행한다. |
| 6 | GPU device 생성이 UI를 막는가 | device와 pipeline 초기화를 serial render queue에서 수행한다. |
| 7 | UIKit layer가 renderer보다 먼저 해제되는가 | surface 준비 호출 중 view가 유효하고, controller가 renderer 종료까지 view를 소유한다. |
| 8 | 같은 host에 중복 준비 surface가 쌓이는가 | host당 활성 renderer 하나와 UIKit pending surface 하나로 제한한다. |
| 9 | surface 준비 실패가 pending 상태를 남기는가 | 준비 실패는 상태를 만들지 않고, 보고 버퍼 실패는 pending surface를 비운다. |
| 10 | renderer 초기화 실패 후 pending surface를 재사용하는가 | 초기화 시작 때 pending surface를 소비하며 실패 시 해당 surface를 버린다. |
| 11 | 생성 중 surface generation이 바뀌는가 | 렌더 queue의 생성 전후 현재 generation을 검사하고 오래된 생성 결과를 파괴한다. |
| 12 | 오래된 준비 surface가 재시도를 막는가 | stale 생성 경로가 host의 renderer와 pending surface를 모두 파괴한 뒤 재시도한다. |
| 13 | UI thread가 긴 렌더 mutex를 기다리는가 | UIKit 준비 전 renderer mutex는 `try_lock`만 사용하고 사용 중이면 즉시 오류로 돌아간다. |
| 14 | WGPU 객체를 thread 간 이동할 수 있는가 | `PendingRuntimeSurface` 이동 가능 여부를 Rust 타입 검사에서 확인한다. |
| 15 | Android surface 생성도 불필요하게 main으로 이동하는가 | Android는 기존 render executor에서 surface와 renderer를 함께 만든다. |
| 16 | backend 선택이 준비/초기화 사이에 달라지는가 | backend 제한이 걸린 Instance와 Surface를 pending 객체가 함께 소유한다. |
| 17 | surface를 준비한 queue와 파괴 queue가 달라지는가 | renderer와 pending surface 파괴는 render queue에서 직렬화한다. |
| 18 | 종료 중 runtime과 render 작업이 역순으로 해제되는가 | 새 요청을 막고 runtime 작업을 끝낸 다음 renderer queue를 비운다. |
| 19 | output 부족을 성공이나 조용한 null로 오인하는가 | prepare/create는 안정된 상태 코드를 쓰며 실패 보고와 자원 정리를 함께 수행한다. |
| 20 | 구조 변경을 제품 지원 완료로 과장하는가 | C04.10은 고정 fixture 구현이며 실제 simulator 화면 검증 전에는 완료로 표시하지 않는다. |

의존성 소스 검토와 계획 계약은 일치한다. 앱 빌드와 Android·iOS Simulator 실행은 구현 검증 단계에서 별도로 확인한다.
