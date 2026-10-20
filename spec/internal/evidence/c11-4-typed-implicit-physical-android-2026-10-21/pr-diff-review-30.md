# C11.4 typed implicit track 실기기 근거 PR diff 검토

검토 대상은 실기기 실행 로그·화면, 기기/APK 정보, typed track/frame 비교 JSON, 공식 상태 대장·계획·내부 인덱스·공개 roadmap 갱신이다. 실행 대상은 기존 opt-in fixture이며 제품 코드와 공개 API는 수정하지 않았다.

| # | 독립 실패 관점 | 판정 |
| ---: | --- | --- |
| 1 | 근거 PR에 제품 구현 변경이 섞여 새 기능 범위가 되지 않는가 | 제품 코드 diff가 없다. evidence와 roadmap 문서만 변경했다. 통과. |
| 2 | C11.4 부분 검증을 전체 완료로 바꾸는가 | 상태 체크를 미완료로 유지하고 한계를 기록했다. 통과. |
| 3 | 공식 완료 수나 public CSS 지원 범위를 잘못 늘리는가 | 완료 수와 지원 API를 바꾸지 않았다. 통과. |
| 4 | fixture APK를 제품 릴리스 APK로 표현하는가 | <code>c04-runtime-gpu-fixture-test</code> opt-in build라고 명시했다. 통과. |
| 5 | emulator 결과를 실기기 결과로 오인하는가 | serial, 모델, <code>ro.kernel.qemu=0</code>을 함께 기록했다. 통과. |
| 6 | 실행 명령이 다른 연결 기기를 우연히 선택하는가 | 모든 install·launch·capture 명령에 serial <code>R5KYB06SMPE</code>가 있다. 통과. |
| 7 | 기기 Android/API와 ABI가 빠져 재현 대상을 특정할 수 없는가 | Android 16/API 36, arm64-v8a를 저장했다. 통과. |
| 8 | GPU·driver를 시뮬레이터 경로와 혼동하는가 | SurfaceFlinger의 Xclipse 940/Vulkan·driver 식별값을 기록했다. 통과. |
| 9 | 논리 좌표와 물리 화면 배율을 혼동하는가 | 1080×2340 및 density 450을 기록했다. 통과. |
| 10 | 빌드 APK와 실제 설치 APK가 다를 수 있는가 | pull한 <code>base.apk</code>와 로컬 APK SHA-256이 같다. 통과. |
| 11 | 빌드가 실패했거나 경고를 숨겼는데 성공으로 쓰는가 | <code>BUILD SUCCESSFUL</code>을 확인하고 compile SDK 경고도 README에 제한으로 남겼다. 통과. |
| 12 | 보고한 source revision과 checkout tree가 불일치하는가 | commit, tree, branch, tracked-tree cleanliness를 기록했다. 통과. |
| 13 | 14개 scene 범위에 빠진 typed sizing 분기가 있는가 | fixed cycle, typed functions, percentage/calc, auto-fit occupied/empty/gutter ID를 각각 남겼다. 통과. |
| 14 | 재실행 사이에 run ID가 겹쳐 로그를 잘못 합치는가 | scene별 고유 run ID와 process ID를 저장했다. 통과. |
| 15 | 장면이 실행되지 않았는데 앱 프로세스 시작만으로 성공 처리하는가 | 각 scene의 <code>status=0 layout=ready</code> summary를 확인했다. 통과. |
| 16 | 일부 trace chunk가 누락됐는데 trace 존재만으로 통과 처리하는가 | 14/14 scene에서 선언된 part 수와 실제 part 수를 비교했다. 통과. |
| 17 | chunk 결합 순서 때문에 손상된 JSON을 track 결과로 읽는가 | part index로 정렬한 전체 JSON을 parse해 result를 만들었다. 통과. |
| 18 | Columns 또는 Rows 한쪽만 검사하는가 | 양축의 final sizing pass를 각각 비교했다. 통과. |
| 19 | gutter를 track으로 세거나 collapse 상태를 버리는가 | gutter를 제외한 track과 collapsed bool을 함께 비교했다. 통과. |
| 20 | implementation output 자체를 새 oracle로 삼는가 | CSS 기대값 검증이 끝난 기존 AVD matrix를 교차 비교 기준으로 밝혔고 직접 Chrome 검증이라고 주장하지 않았다. 통과. |
| 21 | fractional CSS px 차이를 과도하게 허용하는가 | 프로젝트의 1/64 CSS px tolerance를 사용했다. 통과. |
| 22 | float 직렬화 noise를 기능 편차로 과장하는가 | 최대 frame delta 0.000006 CSS px를 실제 수치로 보고했다. 통과. |
| 23 | node frame 수나 좌표 누락을 놓치는가 | 실기기 process별 raw node frame과 AVD frame을 대응시켰다. 통과. |
| 24 | summary는 있지만 frame callback marker가 누락된 scene이 있는가 | <code>SPINON_C114_FRAME_SUMMARY marker=present</code> 14/14를 확인했다. 통과. |
| 25 | layout 완료를 GPU surface 표시로 오인하는가 | 각 scene의 <code>SPINON_C0410_DRAW status=0 presented</code>를 별도로 확인했다. 통과. |
| 26 | screenshot이 AVD 또는 다른 scene의 이미지로 섞이는가 | 이미지마다 scene 이름을 맞추고 ADB capture를 해당 실행 직후 했다. 대표 화면과 1080×2340 크기를 기록했다. 통과. |
| 27 | overflow 표본을 typed scene의 일반 trace 결과와 혼동하는가 | <code>implicit-min-content-overflow-border</code>를 독립 run ID·로그·화면으로 분리했다. 통과. |
| 28 | PixelCopy sample payload가 scene whitelist와 맞지 않아 다른 입력을 읽는가 | 기존 고정 sample ID·좌표·node ID를 사용하고 <code>status=matched</code>를 확인했다. 통과. |
| 29 | root 밖 overflow가 실제 surface에서 확인되지 않았는데 geometry만으로 주장하는가 | CSS x=91의 item RGBA와 x=92의 뒤쪽 배경 RGBA를 실기기 PixelCopy에서 각각 확인했다. 통과. |
| 30 | 고정 표본 결과를 모든 테두리·compositor pixel equality로 확대 해석하는가 | 표본 다섯 점·해당 fixture만 말하고 전체 border stroke·Chrome pixel equality는 미검증으로 명시했다. 통과. |

후속 확인도 README·STATUS·계획·공개 roadmap에서 동일한 14-scene 및 5-sample 범위를 유지한다. 결함은 발견되지 않았다.
