# C11.4 cyclic `minmax(calc())` replaced image PR 적대적 검토 30개

검토 대상은 기능 코드, 두 모바일 harness, 계획·사전 비교, runtime 근거와 공개 roadmap 변경이다. 각 관점은 다른 실패 경로 또는 계약 경계를 대조했다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | 제품 코드가 네 고정 scene 밖의 일반 CSS admission을 새로 허용 | 새 FFI scene builder와 Android allowlist는 네 exact ID만 허용한다. 임의 CSS parsing/admission은 계획의 제외 범위에 남는다. |
| 2 | fixture 코드가 기본 제품 artifact에 섞임 | 기록된 static archive 비교에서 기본 `spinon-ffi`에는 새 fixture symbol/payload가 없고 opt-in fixture archive에는 포함된다. APK/IPA 전체 분리로 확대하지 않았다. |
| 3 | Rust 모듈 경로 밖에 중복된 신규 구현이 생김 | 신규 style/runtime/FFI fixture는 기존 `fixtures` 계층 아래 모듈로 연결되어 있고, capture adapter는 테스트 scene 계약에만 연결된다. |
| 4 | forward/reverse scene에서 DOM 순서가 image identity를 바꿈 | 네 scene validator가 item identity와 node index를 함께 비교한다. reverse run은 C/B/A node order를 사용하면서 identity별 geometry를 유지한다. |
| 5 | source order가 intrinsic track contribution 순서에 따라 결과를 바꿈 | forward/reverse AVD·Simulator run에서 intrinsic columns `[5,8,8]`, rows `[7,7]`이 일치했다. |
| 6 | row sizing이 column pass보다 먼저 intrinsic contribution을 확정 | recorded intrinsic trace는 Columns → Rows → Columns → Rows 네 pass를 보이고 각 초기·최종 track을 검증한다. |
| 7 | definite cyclic basis가 intrinsic cycle에 잘못 적용 | intrinsic와 definite를 분리한 네 scene에서 root와 track 기대값이 따로 고정돼 있다. definite는 columns `[10,10,10]`, rows `[10,10]`, root `84×84px`다. |
| 8 | image aspect ratio가 cyclic second-axis sizing에 누락 | 세 image의 `24×16` natural size와 `12×8`, `18×12`, `24×16` CSS size가 입력 진단과 frame에서 확인된다. |
| 9 | decode/source가 바뀌었는데 같은 fixture로 판정 | precomparison의 fixed image source hash 및 runtime intrinsic input의 natural dimensions를 대조한다. decode 실패나 다른 item 수는 거부한다. |
| 10 | reversed input에서 image intrinsic diagnostics가 DOM 순서라고 가정 | Android harness는 진단 node order `[2,3,4]`와 reverse frame order를 별도 계약으로 비교한다. 해당 mismatch는 AVD 재실행에서 수정·통과했다. |
| 11 | root의 min/max/aspect-ratio style이 달라져도 fixed frame만 보고 통과 | 고정 root style 값과 computed root frame을 별도로 확인한다. 이 검사가 빠졌던 초기 검토 결함은 fail-closed validation에 반영됐다. |
| 12 | root width가 tracks 합과 다르거나 root height가 clamp 범위를 벗어남 | intrinsic root `29×20px`와 definite root `84×84px`를 각 adapter 및 캡처 validator가 비교한다. |
| 13 | root box count가 image child를 포함하지 않거나 숨은 wrapper를 놓침 | Java capture validator는 root plus three image nodes인 box count 4를 검사하고 iOS run metadata도 네 node frame을 기록한다. |
| 14 | scene 수 확장 때 기존 C11.4 capture matrix만 부분 갱신 | shared capture matrix의 기존 scene 수와 새 4 scene count가 함께 검사된다. |
| 15 | Java host가 신규 scene ID를 잘못 거부하거나 임의 ID를 허용 | API 37 AVD 설치·실행에서 네 exact scene을 통과했고 미등록 ID 경로는 기존 whitelist rejection을 유지한다. |
| 16 | Java trace 검증이 final tracks만 보고 중간 span batch 누락을 놓침 | Java harness는 pass axis/count, span 목록, initialize tracks, final track sizes를 확인한다. |
| 17 | Swift host의 expected scene/frame matrix가 Java와 다름 | iOS Simulator의 네 scene에서 track trace·root-relative frames·`drawPresented=true`가 pass했고 scene별 Metal 표본이 3/3이었다. |
| 18 | AVD PixelCopy가 image 내부·root 테두리·바깥을 혼동 | 각 scene 세 sample의 node ID와 CSS 좌표를 검사한다. root 마지막 border 좌표는 `width-1`, 바깥 좌표는 root width로 분리한다. |
| 19 | PixelCopy 성공만으로 잘못된 색을 승인 | Android sample은 image interior `255,255,64`, border `240,198,116`, outside `0,0,0`을 각각 비교한다. |
| 20 | Metal drawable 결과를 화면 제출 이후 또는 실기기 결과로 과장 | evidence와 계획은 3/3 drawable sample을 compositor 제출 전으로 한정하고 실기기·hardware GPU·후속 compositor pixel을 제외한다. |
| 21 | Android 설치 APK가 검증한 소스 산출물과 다름 | 설치 APK와 캡처 APK hash가 동일하다고 run metadata에 기록되어 있고 네 scene capture가 그 설치물에서 성공했다. |
| 22 | Android 실패 원인을 Spinon runtime 버그로 오인 | 첫 ADB install 실패는 `device is still booting`이었다. API descriptor, RAM/userdata, TCG 설정을 고친 뒤 같은 작업 AVD에서 boot/install/capture가 성공해 원인과 수정 범위를 분리했다. |
| 23 | iOS·Android 실행을 동일 source revision 근거 없이 합침 | 두 run metadata는 checkout HEAD `2c1a06e…` 및 캡처 당시 tracked diff SHA `3b761f6d…b69522e7`을 기록한다. 이후 바뀐 계획 설명은 이 실행 hash와 구분한다. |
| 24 | 새 scene의 iOS/Android screenshot 경로가 깨져 외부 근거에서 열리지 않음 | pass56 Tailnet remote verify는 1,034개 route 전부 HTTP 200과 byte hash가 일치했다. Agent Browser에서 runtime 페이지와 iOS `1206×2622`, Android `1080×2400` PNG를 직접 열었다. |
| 25 | 문서 표의 PNG 링크가 이미지 파일이 아닌 잘못된 alias로 향함 | Agent Browser의 rendered README DOM에서 두 플랫폼의 상대 PNG href가 expected run directory로 해석됐고 각 PNG 렌더링 크기를 확인했다. |
| 26 | public roadmap이 feature card를 빠뜨리거나 완료 수를 앞당김 | Tailnet roadmap DOM에서 C11.4 feature card와 공식 `77/213`이 확인됐다. 공식 checkbox와 `spec/STATUS.md`는 수정하지 않았다. |
| 27 | PR 문서가 이 slice를 전체 C11.4 또는 제품 CSS 지원으로 서술 | 계획은 네 고정 scene 범위, 제외 시나리오, PR/CI/merge 전 미완료 상태를 유지한다. |
| 28 | precomparison 기대값이 Chrome 관측 또는 제품 결과와 뒤섞임 | 독립 CSS Grid 기대값, Chrome reference 관측, 앱 runtime 표본의 역할과 한계를 각 문서에서 분리했다. |
| 29 | 기기·시뮬레이터 근거가 앱 설치/build 구성을 숨김 | Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator를 별도 명시했다. 실제 Android 기기와 iOS 실기기는 검증하지 않았다. |
| 30 | PR merge 정책, 필수 CI, 공식 상태 변경이 증거보다 앞섬 | PR은 `main` 기반으로 별도 생성하고 required checks 확인 뒤 Rebase and merge한다. 이 부분 검토 시점에는 PR·CI·merge가 없고 공식 완료 수는 `77/213`으로 유지한다. |

## 판정과 한계

검토 중 발견된 root style assertion 및 Android scene allowlist/reverse-input/PixelCopy matrix 누락은 수정 후 해당 simulator capture에서 재검증했다. 이번 증거는 고정 fixture의 style/layout/runtime과 AVD·Simulator 관찰 범위에 한정된다. PR required checks, merge, physical-device 및 full C11.4 지원은 별도다.
