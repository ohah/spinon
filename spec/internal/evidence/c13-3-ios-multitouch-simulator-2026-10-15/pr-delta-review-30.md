# iOS active-pan second-touch PR 변경 적대 검토 30개

검토 범위는 이번 PR의 계획·상태 대장 문구 및 Simulator evidence artifact다. 런타임 코드는 변경하지 않았다. 각 검토는 독립된 문서·증거 무결성 실패를 본다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | roadmap에서 C13.3 전체 완료로 오인되는가 | C13.3 미완료와 남은 gate를 명시한다. |
| 2 | Android 실제 multi-touch가 통과한 것처럼 읽히는가 | Android synthetic `MotionEvent` 한정 및 실제 입력 미검증을 명시한다. |
| 3 | iOS 실기기에서 확인한 것처럼 쓰이는가 | 기기 정보를 Simulator로 구체화하고 실기기 부재를 적는다. |
| 4 | 일반 제품 입력 구현을 검증했다고 오인하는가 | 고정 opt-in fixture 범위라고 판정과 남은 범위에서 구분한다. |
| 5 | DOM Pointer Events 검증으로 확대되는가 | public DOM event와 CSSOM을 미완료로 남긴다. |
| 6 | wheel/trackpad 지원으로 확대되는가 | wheel/trackpad 미완료를 계획과 상태에 보존한다. |
| 7 | displayed-frame hit testing 완료로 오인되는가 | S04.10 displayed-frame target 미완료로 남긴다. |
| 8 | 화면 캡처만으로 native cancel을 주장하는가 | 결론은 앱 touch log와 XCTest event를 함께 요구한다. |
| 9 | log만 있고 두 contact가 실제 겹쳤는지 불분명한가 | `active_contacts=2` 시각과 별도 UIKit touch runner를 기록한다. |
| 10 | `SecondPointer`와 다른 cancel reason을 혼동하는가 | source와 reason이 `second-pointer`/`SecondPointer`로 적혀 있다. |
| 11 | cancel보다 PanBegan이 나중인 trace를 성공으로 세는가 | README는 PanBegan 선행 후 cancel 순서를 명시한다. |
| 12 | cancel 전에 받아들인 이동을 drift로 잘못 분류하는가 | accepted `-40 CSS px`와 캡처 비교 구간을 분리한다. |
| 13 | second-touch 뒤 허용된 move가 없다는 근거가 빈약한가 | 이후 네 callback의 timestamp와 `accepted=false`를 log와 요약에 연결한다. |
| 14 | 한 contact가 남은 상태의 suppression을 생략하는가 | 두 번째 contact 종료 후에도 첫 contact가 남은 동안 거부됐다고 적는다. |
| 15 | fresh input recovery를 확인하지 않고 결함 상태를 숨기는가 | all-up 후 pointer 2 PanBegan 및 sequence 10 종료를 기록한다. |
| 16 | crop 영역과 CSS/physical pixel 배율이 맞지 않는가 | 400×240 CSS px, 3×, 1200×720 physical crop을 명시한다. |
| 17 | pixel compare가 full screen 차이를 뜻하는 것으로 과장되는가 | 비교 범위를 canvas crop으로 제한한다. |
| 18 | 0 pixel 차이를 compositor/scanout 증거로 오인하는가 | displayed-frame/scanout을 미완료로 남긴다. |
| 19 | recovery 화면 pixel 변경이 원인과 연결되지 않는가 | marker 위치 변화이며 별도 fresh gesture 이후 발생했다고 기록한다. |
| 20 | screenshot 링크/파일명이 evidence와 어긋나는가 | README 링크 다섯 개가 evidence directory의 실제 파일과 일치한다. |
| 21 | PR screenshot으로 초기 정지 화면을 잘못 선택하는가 | PR용 선택은 movement가 보이는 `ios-after-recovery.png`로 정했다. |
| 22 | XCUITest가 repository target 변경으로 오인되는가 | 임시 runner이며 tracked app/test target 변경이 없다고 범위를 적는다. |
| 23 | test 성공 수를 제품 테스트 전체로 과장하는가 | summary는 XCTest 한 건의 Simulator 통과에 한정한다. |
| 24 | simulator OS/build/architecture를 재현할 수 없는가 | `simulator-device.txt`, build identities, XCTest summary가 연결된다. |
| 25 | 앱 바이너리가 다른 source revision일 수 있는가 | 앱 실행 파일 hash 및 Swift source hash를 기록한다. |
| 26 | route flag가 불분명해 다른 fixture와 혼선이 생기는가 | `--spinon-c133-scroll-gesture` opt-in route를 기록한다. |
| 27 | screenshot/language claims가 사용자 화면보다 로그에만 의존하는가 | 원본 화면과 raw log를 모두 보존하고 결론을 분리한다. |
| 28 | relative links가 plan/STATUS 위치에서 깨지는가 | plan의 상위 경로와 STATUS의 `spec/` 상대 경로를 각각 사용한다. |
| 29 | evidence 디렉터리에 구현 검토와 PR 변경 검토가 누락되는가 | 두 검토 파일을 plan/STATUS와 서로 다른 링크로 연결한다. |
| 30 | 수정된 roadmap 상태와 실제 evidence가 불일치하는가 | 상태 문구는 이번 실행 결과만 추가하며 남은 항목을 명시한다. |

문서 변경 경로와 artifact 링크를 읽기 대조했다. 실제 링크의 merge 후 서버 제공 검증은 PR merge 뒤 Tailnet sync 단계에서 별도로 수행한다.
