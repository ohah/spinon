# C13.1 전체 viewport hit identity 구현 실패 관점 검토 20개

검토 대상은 source commit 1375b7978e6d10d200b8d7b1cc0471dda35e5b73의 Rust oracle/runtime, Android·iOS reader, capture route와 각 시뮬레이터 결과다. 항목마다 다른 실패 경로 또는 불변 조건을 확인했다. 이 검토는 제한된 fixture slice의 완료를 뜻하며 C13.1/C13 완료 판정은 아니다.

| # | 공격 관점과 확인한 실패 경로 | 판정 및 근거 |
|---:|---|---|
| 1 | 체크인된 Chrome oracle이 바뀌었는데 validator가 계속 통과하는지 | 닫힘. oracle SHA-256 279364905336ef53d225c781bae68ec26d3fe4d50cfc7d0cfce689ab5354eee2를 상수와 대조하며 source mutation을 거부한다. |
| 2 | top-level JSON key 추가·삭제·중복이 허용되는지 | 닫힘. exact schema와 metadata를 검사하고 19개 negative mutation 중 schema mutation도 실패한다. |
| 3 | Chrome 버전·fixture hash·viewport·DPR·scroll을 다른 oracle에서 가져올 수 있는지 | 닫힘. pin된 값을 각각 비교한다. 고정 oracle과 앱 report 양쪽에서 동일 oracle hash를 확인했다. |
| 4 | viewport 바깥 좌표나 half-pixel query가 정수 grid에 섞이는지 | 닫힘. grid는 540×128의 정수 CSS 좌표만 조회한다. fractional 및 viewport 밖 좌표는 별도 edge controls다. |
| 5 | data-id의 누락·중복·순서 변경이 다른 NodeId로 잘못 join되는지 | 닫힘. 13개 ID 목록과 순서를 검증하고 frame inventory를 exact join한다. |
| 6 | 음수 node index, 빈 run, overflow 또는 540 초과 run이 잘못 해석되는지 | 닫힘. run index·width 범위와 누적 폭을 제한하고 row마다 정확히 540을 요구한다. |
| 7 | 같은 node의 인접 run이 비정규 입력으로 허용되는지 | 닫힘. 인접 동일-node run은 canonical encoding 위반으로 거부되며 mutation test에서 확인한다. |
| 8 | row에 gap/overlap이 있거나 row 수가 128이 아닌데 합계로 상쇄되는지 | 닫힘. 각 row를 독립적으로 540까지 소비하고 전체 128 row를 요구한다. |
| 9 | point 수와 digest가 서로 모순되어도 pass 처리되는지 | 닫힘. 69,120점·1,136 run·확장 69,120-byte stream digest를 함께 검사한다. Android/iOS 세 실행 모두 같은 digest를 보고했다. |
| 10 | 우측·하단 정수 경계와 fractional/outside 경계의 rounding이 Chrome과 다른지 | 닫힘. 11개 edge control을 검사한다. 최초 검토 중 viewport visibility의 fractional rounding 차이를 발견해 Chromium 동작에 맞추고 renderer regression test를 추가했다. |
| 11 | snapshot의 hit 결과를 다른 frame 순서나 추가 frame에 대응시키는지 | 닫힘. 고정 13개 frame inventory와 NodeId mapping을 대조하고 unknown·추가·중복 frame은 오류다. |
| 12 | 일부 점을 생략·중복 조회하고 69,120개 완료로 세는지 | 닫힘. 각 정수 좌표를 동일 immutable RuntimeRenderSnapshot에서 한 번씩 scan하며 전체 denominator와 row coverage를 검증한다. |
| 13 | fixture 밖 NodeId 또는 hit 없음이 정상 node처럼 digest에 합쳐지는지 | 닫힘. 등록되지 않은 identity는 mismatch로 세고 actual digest에 sentinel byte를 사용한다. |
| 14 | 첫 mismatch에서 조기 종료해 전체 mismatch 수를 숨기는지 | 닫힘. 전체 grid를 끝까지 비교하고 모든 count/digest를 만든 뒤 preview만 최대 16개로 제한한다. |
| 15 | oversized JSON이 잘리거나 출력 용량 초과가 성공으로 읽히는지 | 닫힘. report는 8,192 byte 한도에서 실패하며 잘린 성공 report를 내지 않는다. 플랫폼 reader도 summary 길이·필수 field를 검사한다. |
| 16 | hit evaluation과 표면 draw의 generation/document/tree/style revision이 섞이는지 | 닫힘. 두 시뮬레이터의 세 정상 process에서 revision tuple [1,65,13,0]을 기록하고 run별 frame/draw 연계를 확인했다. |
| 17 | stale document revision이 hit 성공 뒤 PixelCopy/Metal readback까지 도달하는지 | 닫힘. Android는 PixelCopy 전, iOS는 Metal readback 전에 거부한다. 원본 stale-control logs와 process ID를 보존했다. |
| 18 | fixture route 충돌 시 host 생성 또는 evaluation이 시작되는지 | 닫힘. 두 플랫폼의 route conflict negative control이 host init 전에 거부됐고 hostCreated/evalStarted는 false다. |
| 19 | Android·Swift JSON reader가 bool을 정수로 받거나 unknown key·잘못된 digest를 허용하는지 | 닫힘. 두 parser가 exact key set, canonical summary, 정수/type/count/preview/digest 제약을 검사한다. 각각 Android build/AVD, Xcode build/Simulator route로 실행했다. |
| 20 | 기본 artifact 오염이나 재현 불가능한 단일 실행을 놓치는지 | 닫힘. default archive에는 새 marker/oracle 문자열이 없고 opt-in positive control에는 있다. 각 지정 시뮬레이터에서 서로 다른 새 process 3개를 사용했다. Android 69,120/69,120과 PixelCopy 18/18, iOS 69,120/69,120과 Metal 18/18이다. |

## 검토 중 수정 및 남은 한계

- Chrome integer viewport containment가 right/bottom fractional edge에서 정수화된 점을 비교한다는 사실을 확인해 renderer hit-test의 viewport visibility 경계만 반올림하고 geometry test point는 원래 float로 유지했다. 회귀 test는 1/1 통과했다.
- iOS capture의 stale-control 오류 marker 식별, Android run ID 길이·generation 필드·중복 frame line 처리와 PNG dimension 검증을 실제 로그 형태에 맞췄다. 최종 postcommit capture 두 플랫폼 모두 통과했다.
- 이 결과는 API 37 ARM64 AVD 및 iOS 26.2 Simulator에 한정된다. 전체 workspace test, WPT, 실기기, hardware GPU, OS compositor/scanout, DOM event/CSSOM은 실행·검증하지 않았다.
