# C04.12 기본 앱 root HTML 시작 경로 · 구현 전 계획 적대 검토 20개

대상은 [기본 앱 시작 계획](../../../../plan/c04-12-default-app-launch.md)이다. 아래 20개는 구현 전 서로 다른 실패 경로와 경계를 대조했으며, 계획 문구를 구현 코드 검토에 재사용하지 않는다.

| # | 실패 관점 | 계획의 판정/대응 |
| ---: | --- | --- |
| 1 | 기본 route가 새 GPU host와 기존 DOM-only root session을 둘 다 만들어 문서·메모리가 중복됨 | 기본 route는 DOM-only root session 대신 GPU host 하나에 HTML과 root script를 연결한다. 기존 `app.js` smoke session은 별도 동작으로 보존한다. |
| 2 | root script가 GPU host가 아닌 DOM-only session에서 실행되어 화면과 `data-runtime` marker가 다른 문서에 속함 | root script 실행·layout·scene이 같은 host V8/document session이어야 기본 launch 성공이다. |
| 3 | C04.12 확인용 fixture HTML을 제품 default input으로 잘못 사용함 | 기본 route의 source를 bundle `index.html`/`root-app.js`로 고정하고 fixture selector 및 synthetic document 사용을 금지한다. |
| 4 | Android intent flag의 기본값 때문에 실험 route가 일반 launch보다 먼저 선택됨 | 인수 없는 일반 실행과 기존 실험/fixture route를 분리하고 기존 route 우선순위를 보존한다. |
| 5 | iOS launch argument가 없는 경우 빈 DOM host가 남고 GPU route로 이동하지 않음 | `AppDelegate`의 일반 route도 동일한 기본 GPU root HTML surface를 선택해야 한다. |
| 6 | C04 runtime GPU가 기본 빌드에서 빠져 simulator 외 실제 기본 빌드가 빈 화면이 됨 | Android/iOS product 기본 빌드에서 `c04-runtime-gpu`를 포함하고 빌드 artifact로 확인한다. |
| 7 | product feature를 켜며 fixture-test feature나 fixture payload까지 함께 포함함 | fixture flag은 계속 false로 두고 default/product 및 positive-control artifact를 각각 대조한다. |
| 8 | `root-app.js`가 첫 layout/render 뒤 실행되어 사용자는 초기 script 상태가 빠진 장면을 봄 | 최초 environment 적용 후 root script를 평가하고 성공한 layout/render scene만 첫 게시로 삼는다. |
| 9 | root script 예외/잘못된 반환이 로그만 남기고 이전 또는 빈 scene을 게시함 | script 실패는 startup 실패로 처리하며 부분 scene을 게시하지 않는다. |
| 10 | UTF-8 잘못됨·빈 파일·1 MiB 초과에서 platform wrapper가 buffer 수명을 넘기거나 잘린 문서를 파싱함 | 일반 C ABI 제한과 소유권 계약을 재사용하고 asset bytes를 검증 후 host 생성에 넘긴다. |
| 11 | viewport를 physical pixels로 입력하여 CSS layout이 DPR만큼 커짐 | surface pixel bounds를 native scale로 나눈 CSS px와 별도 DPR을 계산해 전달한다. |
| 12 | safe-area 또는 system bar 높이를 임의로 더해 CSS viewport와 실제 GPU drawable이 다름 | CSS viewport는 native content surface의 실제 bounds를 기준으로 기록하고 영역을 임의 확장하지 않는다. |
| 13 | 첫 environment 입력 전 기본 light 값으로 scene을 만들어 dark 설정에서 순간 stale flash가 발생함 | 초기 OS scheme과 surface 환경 적용 전에는 host eval/render를 허용하지 않는다. |
| 14 | OS scheme 변경 뒤 이전 scene 또는 revision이 늦게 완료된 계산에 의해 되돌아옴 | 최신 environment revision만 게시하고 light↔dark 전환을 양 simulator에서 반복 확인한다. |
| 15 | 회전/멀티윈도우 resize 뒤 최초 viewport가 유지되거나 zero-size surface가 나눗셈 오류를 일으킴 | 유효 surface 크기에서만 계산하고 resize마다 CSS viewport/DPR을 다시 설정한다. |
| 16 | host eval을 UI thread에서 동기 대기해 시작/trait callback을 교착시킴 | 평가·레이아웃 대기는 기존 background runtime lane에 두고 UI thread는 surface 상태 전달만 한다. |
| 17 | Activity/UIViewController 종료와 async eval/renderer 종료가 경합해 해제 host를 사용함 | 종료 flag·generation을 검사한 뒤 host와 surface resource를 한 번만 해제한다. |
| 18 | 기본 route에 C0410 진단 title, fixture button 또는 fixed-size debug surface가 그대로 보임 | 일반 기본 화면은 app content surface를 채우고 debug title/control/fixture input은 표시하지 않는다. |
| 19 | scene이 화면 중앙에서는 맞지만 표면 가장자리 밖에 paint하거나 screenshot이 잘못된 content 영역만 캡처함 | Android/iOS 전체 simulator screenshot에서 content surface 네 변과 모서리를 확인하고 capture 범위를 기록한다. |
| 20 | Chrome desktop 관찰이나 simulator screenshot을 물리 기기·hardware GPU·전체 웹 호환성 근거로 과장함 | Chrome은 HTML oracle, API 37 AVD/iOS 26.2 Simulator는 별도 app 근거로 분리하고 물리 기기·hardware/compositor 이후 주장은 제외한다. |

## 결과

20개의 독립 관점을 모두 기록했다. 계획에 실제 기본 asset, route 우선순위, 단일 root document host, default product feature와 fixture 분리, 최초 frame 원자성, surface 좌표와 실패 처리, simulator 증거 경계를 넣었다. 구현은 이 계획 및 이 검토 결과를 기준으로 시작한다.
