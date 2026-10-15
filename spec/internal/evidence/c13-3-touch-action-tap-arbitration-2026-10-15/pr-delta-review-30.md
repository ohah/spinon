# C13.3 native tap arbitration PR 변경 적대적 검토 30개 · 2026-10-15

검토 대상은 Android/iOS fixture tap adapter, 좌표 복원 capture harness, opt-in artifact 경계, simulator evidence와 상태·계획·roadmap 문서다. 각 관점은 서로 다른 실패 경로를 다룬다. 시뮬레이터에서 확인하지 않은 제품 입력·공개 event·실기기 동작은 통과 근거로 확대하지 않았다.

| # | 적대적 관점 | 확인한 실패 경로와 근거 | 판정 |
|---:|---|---|---|
| 1 | Android tap flag 단독 진입 | `MainActivity`가 tap probe를 gesture fixture route와 opt-in build 양쪽에 종속한다. 단독 flag는 화면과 진단을 내고 일반 route로 진행하지 않는다. | 통과 |
| 2 | iOS tap flag 단독 진입 | `initializeRuntime`가 tap probe에 fixture route가 없으면 준비 전에 거부하고, fixture-test 미사용 build에서도 disabled 진단으로 중단한다. | 통과 |
| 3 | 잘못된 scenario가 기본값으로 대체 | iOS tap probe는 scenario 인자 하나와 ID 3/4만 허용한다. 중복·누락·범위 밖 입력은 ready 전에 거부한다. Android도 동일하게 scenario 3/4를 검사한다. | 통과 |
| 4 | tap probe 인자가 route 충돌로 오인 | iOS route 충돌 검사에서 tap flag와 scenario flag를 gesture fixture route 안에서만 허용한다. 다른 C13 route와의 조합은 계속 거부된다. | 통과 |
| 5 | 최초 draw 전에 입력 성공 집계 | 두 플랫폼 모두 fixture-ready와 최초 draw 이후에만 Begin을 받는다. capture는 준비 report와 초기 revision/offset을 확인한 뒤 입력한다. | 통과 |
| 6 | 요청한 CSS action을 실제 computed 값으로 오인 | iOS recognizer 설정은 scenario 번호가 아니라 runtime report의 `touch_action_effective`를 읽는다. log와 capture assertion도 effective action을 기록·검사한다. | 통과 |
| 7 | iOS swipe가 tap과 pan으로 동시 집계 | `UITapGestureRecognizer.require(toFail:)`로 같은 pan recognizer의 실패를 기다린다. pan-x swipe 후 tap activation 합계는 1에 머문다. | 통과 |
| 8 | `touch-action:none` 설정 경쟁 | computed `none` 확인 뒤 main queue에서 pan recognizer를 비활성화하고 draw/readiness를 요청한다. 실제 none tap log에는 `native_pan_enabled=false`가 남는다. | 통과 |
| 9 | `pan-x`가 수평 swipe까지 막음 | 실제 computed `pan-x`일 때 native pan recognizer를 유지한다. 두 simulator에서 horizontal pan과 inner X 이동을 관찰했다. | 통과 |
| 10 | Android 거부된 Begin 뒤 tap 허용 | `c133TapDownAccepted`와 active gesture를 함께 요구하며 Begin 거부 시 후보가 만들어지지 않는다. capture는 실제 OnClick activation이 없으면 실패한다. | 통과 |
| 11 | End enqueue 실패 뒤 activation 허용 | Android 후보는 ACTION_UP에서 End가 queue에 수락된 경우만 만든다. 로그의 `up_candidate` 및 실제 listener count를 함께 대조했다. | 통과 |
| 12 | stale candidate가 다음 입력을 클릭으로 귀속 | 새 ACTION_DOWN에서 후보를 먼저 지우고, 실제 OnClick은 한 번 소비한 뒤 후보 없이 재호출되면 오류 로그를 낸다. | 통과 |
| 13 | native slop 이후 두 번째 click 발생 | 수정 전 pan 종료 뒤 `OnClickListener` 재호출을 재현했다. 첫 slop crossing에서 pressed/long-press를 취소하고 남은 stream을 clickable view에 전달하지 않도록 고쳤으며 최종 count는 1이다. | 통과 |
| 14 | sub-slop jitter가 tap을 잃게 함 | slop 아래 Move는 pan을 시작하지 않고 native clickable 경로를 유지한다. 최종 `none` 및 `pan-x` tap 모두 activation 1, revision 0이다. | 통과 |
| 15 | slop crossing callback이 clickable view에 누락 | crossing callback은 내부 gesture에서 pan 시작을 확인한 바로 그 호출에서 forwarding을 끊는다. 이후 up은 tap candidate가 되지 않는다. | 통과 |
| 16 | ACTION_CANCEL 뒤 click candidate 잔존 | Android cancel은 C13.3 gesture 취소와 tap forwarding/down/candidate 상태를 함께 지운다. 구현 검토와 swipe/cancel 경로를 대조했다. | 통과 |
| 17 | 두 번째 pointer 뒤 첫 pointer up을 tap으로 집계 | `ACTION_POINTER_DOWN`은 gesture를 취소하고 모든 pointer 종료까지 stream을 suppress한다. 실제 iOS multi-touch와 이번 tap run의 Android multi-touch는 검증하지 않았다고 범위를 제한했다. | 범위 명시 |
| 18 | 앱 background 뒤 tap 상태 재사용 | Android host pause는 gesture 취소와 tap state reset을 수행한다. iOS background observer는 기존 gesture cancellation 경로를 호출한다. | 통과 |
| 19 | Android local 좌표와 screen 좌표 혼합 | callback은 `event.getX()` local point, `getRawX()` screen point, SurfaceView screen origin과 크기를 별도 기록한다. capture가 변환을 각각 비교한다. | 통과 |
| 20 | clipped UIAutomator bounds를 전체 view origin으로 사용 | 수정 전 ADB 입력 오차를 재현했다. harness가 전체 view 크기와 clipping으로 origin을 복원하고 runtime origin/size/screen point 불일치 시 중단한다. 최종 좌표는 CSS `(148,132)`와 일치한다. | 통과 |
| 21 | Android scale X/Y 불일치가 조용히 진행 | 입력 시작 시 CSS viewport의 X/Y scale이 다르면 reject한다. capture는 SurfaceView size와 runtime scale도 교차 확인한다. | 통과 |
| 22 | iOS safe-area/canvas origin 무시 | `UITapGestureRecognizer.location(in:)`은 canvas 기준으로 얻고, capture 입력 좌표는 canvas origin·bounds·screen scale을 포함한 manifest 값에서 계산한다. | 통과 |
| 23 | tap이 scroll snapshot을 바꾸고도 통과 | 각 post-tap 상태에서 revision과 root/outer/inner X/Y를 모두 0과 대조한다. 이후 swipe는 inner X만 양수여야 한다. | 통과 |
| 24 | screenshot이 log의 offset과 무관한 화면 | 캡처 도구가 PNG를 decode해 scene background와 예상 inner marker 위치를 검사한다. full screenshot과 별도로 검토 가능한 crop을 보존한다. | 통과 |
| 25 | screenshot이 다른 실행 또는 다른 기기에서 옴 | `run.json`은 source revision/diff hash, source hash, simulator ID·OS, binary hash, 좌표와 상태를 연결한다. simulator 종류/API/ABI를 capture 시작 때 검사한다. | 통과 |
| 26 | fixture 코드가 기본 artifact에 포함 | Android APK와 iOS app 기본 빌드에서 네 gesture FFI export 및 고정 payload 부재를 확인했고, opt-in artifact는 네 export/payload 양성 대조를 제공한다. | 통과 |
| 27 | status 문구가 실제 activation을 대신 | Android는 `View.OnClickListener`, iOS는 recognized `UITapGestureRecognizer` callback만 activation으로 센다. 두 simulator 모두 실제 callback count를 log로 확인했다. | 통과 |
| 28 | iOS pan 내부 이동이 tap으로 오인 | tap callback은 `.recognized` 상태만 세며 swipe 종료 뒤 인식 count가 증가하지 않는다. pan `.began`과 revision/inner X 증가는 별도 조건이다. | 통과 |
| 29 | fixture 결과가 public DOM/API 지원으로 게시 | 계획·상태·내부 계약·evidence에서 public `click`, Pointer Events, WPT, 일반 제품 route와 C13.3 전체 완료를 명시적으로 제외한다. 미완료 상태를 유지했다. | 통과 |
| 30 | PR/roadmap 이미지와 정적 링크가 배포 후 깨짐 | Tailnet roadmap, status, evidence/PR review HTML과 두 hashed crop에 HTTP 200 및 source/served SHA-256 일치를 확인했다. Agent Browser에서 evidence page의 Android `1080×720`, iOS `1206×820` 이미지를 모두 로드했다. PR 본문 이미지 URL은 PR 생성 뒤 같은 브라우저에서 추가 확인한다. | Tailnet 통과 · PR 확인 대기 |

## 검토 결론

구현 범위의 실제 재현 결함 두 건(Android clipped accessibility bounds 좌표 오차, slop 이후 중복 click)은 수정했고 양 플랫폼 Simulator 결과에서 다시 확인했다. 남은 판정 경계는 실기기·hardware GPU·compositor 이후 scanout·iOS 실제 multi-touch·public event와 제품 전체 policy 행렬이다. Tailnet preview 동기화와 browser image load는 통과했다. PR 본문 이미지 URL은 생성 후 마지막으로 확인한다.
