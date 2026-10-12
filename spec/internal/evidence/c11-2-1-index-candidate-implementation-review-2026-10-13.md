# C11.2.1 인덱스 후보 통합 구현 검토 · 2026-10-13

검토 대상은 [`인덱스 후보 검증`](./c11-2-1-index-candidate-2026-10-13/README.md)의 staged tree `fb9411cb7cc7f767699447d29a202a8fadca1ece`다. 앞선 재검증에서는 C12.4 부분 분기, iOS draw report revision/sample 누락, C12.4 단일 표본 호출의 인수 불일치를 발견해 수정했다. 아래 관점은 수정 전 발견과 최종 후보의 동작을 각각 대조한다.

| # | 실패 관점 | 검토와 판정 |
| --- | --- | --- |
| 1 | C12 변경이 C11 후보에 넓게 섞여 다른 작업을 병합할 수 있다 | staged 경로를 검사했다. C12 전용 경로는 없고 공유 readback API 호출을 고친 `runtime_gpu/ffi/c12.rs`의 단일 호환 줄만 포함한다. 나머지 C12 작업은 unstaged 상태로 남았다. 통과. |
| 2 | iOS가 정의되지 않은 C12 Grid scene 심볼을 참조할 수 있다 | exact candidate의 Swift 컴파일이 처음 실패했다. C11 조건식에서 C12 전용 분기를 제거한 뒤 같은 후보가 Simulator용으로 빌드됐다. 통과. |
| 3 | iOS readback이 실제 장면 revision 대신 고정값이나 다른 요청의 revision을 기록할 수 있다 | draw report가 `scene.snapshot.key()`의 document·tree·style·environment revision과 generation을 출력하도록 했다. 최종 iOS 로그의 frame report와 표본 summary가 같은 장면의 값을 기록한다. 통과. |
| 4 | 여러 pixel sample이 문자열을 임의 분할하거나 값 순서를 잃을 수 있다 | Rust가 renderer 반환 순서의 숫자 좌표/RGBA를 JSON 배열로 직렬화하고 iOS가 기대 개수·shape·좌표·채널을 index별 비교한다. 두 플랫폼의 20개 표본이 exact다. 통과. |
| 5 | 표본 요청 개수가 0일 때 성공으로 오인할 수 있다 | FFI feature test가 빈 요청 거부를 확인했다. 통과. |
| 6 | 64개를 초과하는 요청이 큰 임시 버퍼를 예약할 수 있다 | 상한 64를 검사하고 65개 및 `count × 2` overflow를 거부하는 feature test가 통과했다. 통과. |
| 7 | null 좌표 또는 host를 역참조할 수 있다 | FFI 경계 테스트가 null 좌표와 null host를 거부했다. 통과. |
| 8 | null output 포인터 또는 용량 0에 쓰기를 시도할 수 있다 | C11·C12 FFI 함수가 output pointer와 capacity를 먼저 확인한다. null·0 경계 테스트와 플랫폼 feature 빌드가 통과했다. 호출자가 넘긴 비-null 포인터의 메모리 유효성은 호출자 계약이며 이 실행에서 임의 주소 주입은 하지 않았다. 통과. |
| 9 | 중복 sample ID 또는 같은 픽셀 좌표가 잘못된 입력을 모호하게 만들 수 있다 | iOS payload parser가 ID와 CSS 좌표의 고유성을 검사하고, Android 비교기는 고정 fixture allowlist를 사용한다. allowlist negative control에서 미등록 요청을 거부했다. 통과. |
| 10 | `run_id`가 로그 라인을 오염시키거나 fixture를 혼동할 수 있다 | iOS parser가 길이와 허용 문자를 확인하며, 각 scene은 별도 run ID와 process를 사용한다. matrix 비교가 예상 marker 외 결과를 받지 못하면 실패한다. 통과. |
| 11 | 성공하지 않는 scene에서 pixel capture가 시작될 수 있다 | 지원 6 scene만 pixel fixture allowlist에 있고, 기대 거부 4 scene의 로그는 readback 시작 marker가 없는지 확인한다. 양 플랫폼 행렬 통과. |
| 12 | CSS 표본 좌표와 surface texel 중심 변환이 DPR에서 한 pixel 어긋날 수 있다 | 두 host 모두 CSS 표본의 중심을 drawable scale로 변환한다. iOS는 drawable ratio와 범위를 검사하고 Android는 PixelCopy PNG의 실제 샘플을 비교한다. iOS·실기기 Android에서 20/20 exact다. 통과. |
| 13 | viewport 또는 실제 surface 크기가 달라져도 정상으로 기록할 수 있다 | iOS readback은 320×240 CSS viewport, 요청한 drawable 크기와 가로·세로 scale이 같아야 진행한다. Android 행렬도 고정 fixture viewport/frame과 PixelCopy 표면을 대조한다. 통과. |
| 14 | drawable resize 중 다른 크기에서 계산한 좌표를 쓸 수 있다 | iOS drawable 크기 읽기·쓰기에 같은 `stateLock`을 사용하고 readback 전에 현재 surface 세대를 다시 확인한다. 10 scene Simulator 재실행에서 surface mismatch가 없었다. 통과. |
| 15 | 표본 요청 뒤 다른 scene이 commit되어 stale surface를 읽을 수 있다 | renderer는 presentation sequence가 바뀌면 이전 제출을 막고 pending sample을 버린다. scene별 새 앱 process를 사용해 상태 재사용 경로를 차단했으며 각 summary/draw generation을 비교했다. 통과. |
| 16 | 표면 복사 실패가 정상 결과처럼 보이거나 후속 scene을 오염시킬 수 있다 | 매트릭스가 readback error/mismatch marker를 즉시 실패로 취급하고 앱 process를 scene마다 재시작한다. 이번 Android·iOS run의 capture error는 0건이다. 통과. |
| 17 | 잘못된 Grid 입력이 partial frame 또는 빈 오류를 제출할 수 있다 | 4개 기대 거부 scene 각각에서 정확한 오류와 partial frame·non-empty WGPU 제출·surface readback 부재를 확인했다. 양 플랫폼 모두 4/4다. 통과. |
| 18 | C11 변경이 기존 C12.4 단일 표본 feature를 깨뜨릴 수 있다 | 이 회귀가 실제로 발견됐다. 기존 C12.4 FFI 호출을 새 slice 입력 형식에 맞춘 뒤 C12 단독 feature 37 통과·2 ignored, C11+C12 동시 활성화 39 통과·2 ignored다. 통과. |
| 19 | 에뮬레이터 또는 화면 screenshot을 실기기 GPU/pixel parity로 잘못 표시할 수 있다 | Android 증거는 Samsung SM-S731N 실기기의 PixelCopy와 renderer 정보를 기록한다. iOS는 Simulator drawable이며 compositor 전 단계임을 명시했다. screenshot은 pixel equality의 근거로 쓰지 않았다. 통과. |
| 20 | 고정 fixture 성공을 전체 Grid·WPT 적합성으로 과장할 수 있다 | README·계획·상태 대장은 C11.2.1의 10 scene만 통과한 것으로 한정한다. Spinon WPT runner, 전체 Grid/CSS, compositor parity, 성능을 미완료로 남겼고 C11.2.2는 이 변경의 PR 통합 뒤로 유지했다. 통과. |

최종 후보는 전체 Rust/JavaScript/CSS reference suite, C11/C12 feature 검사, Android 실기기 및 iOS Simulator 행렬을 통과했다. 남은 항목은 기능 회귀가 아니라 이 후보의 PR 생성·검토·병합이며, 전체 C11과 WPT 적합성은 계속 미완료다.


후속 유지보수 검토에서 605줄·558줄이던 신규 Rust Grid 테스트 모듈을 책임별 하위 파일로 분리했다. 전체 테스트·Clippy·rustfmt를 최종 후보 tree에서 다시 실행했다.
