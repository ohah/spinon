# C11.4 문서 동기화 diff 적대 검토 · 30 관점

검토 기준은 origin/main 대비 plan, STATUS, 내부 계약 0059, 공개 roadmap, 실행 evidence README 두 형식, SHA256SUMS의 미커밋 변경이다. 구현 PR #632의 merge SHA 64739104e4be3e06f95c60b94a5eb1f87a4e78b0은 origin/main의 조상이며, 기록된 병합 시각은 2026-10-25T14:56:09Z다.

범위 파일: plan/c11-4-cyclic-percentage-auto-repeat-row-axis.md, spec/STATUS.md, spec/internal/0059-c11-grid-layout.md, spec/public/roadmap.html, evidence README.md/README.html/SHA256SUMS.txt.

첫 검토에서 발견한 P2 세 건(계획의 병합 전 문구, roadmap PR 순서, PixelCopy/Metal equality 과장)은 현재 diff에서 모두 정정됐다. 재검토 기준으로 남은 P1/P2는 없다. README HTML은 Markdown보다 직접 실행 링크가 적은 비차단 P3를 아래 표에 기록했다.

리뷰는 읽기 전용이었다. 빌드·테스트·시뮬레이터·브라우저는 실행하지 않았고, 기존 로그·manifest를 읽고 정적 경로와 checksum만 확인했다. 이 리뷰 파일 이외의 파일은 수정하지 않았다.

| # | 적대 실패 관점 | 판정과 근거 |
|---:|---|---|
| 1 | PR 번호가 맞아도 merge SHA가 main에 없거나 다른 변경을 가리키는가? | 통과. SHA 64739104e4be3e06f95c60b94a5eb1f87a4e78b0을 git object로 확인했고 origin/main의 조상이다. 계획 7행, STATUS 368행, 계약 0059 538행, 양쪽 README 상태 문구가 같은 SHA를 기록한다. |
| 2 | 병합 시각·제목·PR 번호가 문서마다 달라졌는가? | 통과. 계획 7행, STATUS 368행, 계약 0059 538행, README.md 3행·README.html 23행, roadmap.html 181행이 #632와 2026-10-25T14:56:09Z를 일치시킨다. |
| 3 | 계획 요약이 #631만 병합된 과거 상태로 남았는가? | 통과. 계획 7행이 계획 PR #631과 구현 PR #632의 병합 상태를 함께 쓴다. |
| 4 | 계획 끝부분에 “구현 PR 병합 전까지” 같은 낡은 완료 기준이 남았는가? | 정정 확인(P2 해소). 계획 99행은 구현 당시 기대값을 사용했다고 과거형으로 쓰고, #632 병합 뒤에도 전체 C11.4와 72/213을 유지한다고 갱신했다. |
| 5 | 공개 roadmap의 병합 기록이 시간순을 깨뜨리는가? | 정정 확인(P2 해소). roadmap.html 179–181행은 #623 04:35:52Z → #627 09:08:33Z → #632 14:56:09Z 순서다. |
| 6 | 제한 slice 기록이 공식 C11.4 체크박스를 실수로 완료 처리하는가? | 통과. STATUS.md 273행은 계속 미완료 체크다. 새 기록은 368행의 설명 이력이며 checklist 표시를 바꾸지 않는다. |
| 7 | 완료 집계가 72/213에서 증가했는가? | 통과. 계획 7·99행, STATUS 368행, 계약 0059 536·538행, README 두 형식과 roadmap 181행은 72/213을 유지한다. |
| 8 | 새 이력 행이 상위 CSS Grid 완료 판정으로 오인되는가? | 통과. STATUS.md 259·273행의 C11 및 C11.4 상위 항목은 미완료이고, 368행은 제한 runtime slice 이력으로 분리되어 있다. |
| 9 | 두 개가 넘는 runtime scene이 추가된 것처럼 쓰였는가? | 통과. 계약 0059 532·534행과 README의 target/control 표는 target 한 개와 control 한 개만 정의한다. 양 플랫폼 result.json도 각각 sceneCount 2다. |
| 10 | target의 입력 조건이나 geometry가 runtime 결과와 어긋나는가? | 통과. spec/internal/0059-c11-grid-layout.md 532행과 evidence/README.md 14·20·24행은 24×16 target, repeat count 1, 최종 basis 16px 및 Rows [8px]을 기록한다. Android result.json 8–44행과 iOS result.json의 target observation도 frame·track 결과가 맞는다. |
| 11 | cyclic pass에서 auto-repeat이 재확장됐다고 잘못 요약하는가? | 통과. spec/internal/0059-c11-grid-layout.md 532행, evidence/README.md 14행, verification/runtime-feature-tests.log 14행은 IntrinsicMaxContent → FinalDefinite, count 1 유지, intrinsic basis 없음에서 final basis 16px로 전환한다고 적는다. |
| 12 | definite control이 target 전용 예외를 사용한 것으로 섞였는가? | 통과. spec/internal/0059-c11-grid-layout.md 534행은 일반 definite-axis admission에서 count 2, basis 40px, 두 개의 20px row를 기록한다. verification/runtime-feature-tests.log 10·13행과 runtime-feature-off-tests.log 7행도 이를 확인한다. |
| 13 | trace 주장이 화면 좌표만으로 확대됐는가? | 통과. spec/internal/0059-c11-grid-layout.md 534행은 phase·axis·순서·count·basis·track list를 provenance와 대조한다고 범위를 밝힌다. 최신 Android runtime log의 SPINON_C114_GRID_TRACE_CHUNK와 result.json 43행에는 trace chunk count가 기록돼 있다. |
| 14 | 내부 계약이 일반 CSS Grid admission이나 전체 C11.4 지원을 약속하는가? | 통과. 계약 0059 532·536·538행은 두 고정 scene과 지원 밖 범위를 제한하고, 전체 C11.4는 미완료로 둔다. README.md 3·49행과 roadmap 181행도 같은 한계를 반복한다. |
| 15 | 내부 계약 숫자 버전이 제한 구현 때문에 올라갔는가? | 통과. 계약 0059 532행은 출시 전 내부 인터페이스 숫자 버전 0.1.0 유지라고 명시한다. |
| 16 | fixture가 제품 기본 경로에 포함되는 것처럼 보이는가? | 통과. spec/internal/0059-c11-grid-layout.md 532·536행과 evidence/README.md 36·45행은 opt-in fixture-test harness와 archive 경계를 구분한다. |
| 17 | feature-off에서도 target이 열린다고 읽힐 수 있는가? | 통과. verification/runtime-feature-off-tests.log 6행은 cyclic row target 거부, 7행은 definite control 일반 경로 통과를 기록한다. spec/internal/0059-c11-grid-layout.md 536행과도 일치한다. |
| 18 | mutation·추가 자식·browser marker 거부 주장이 근거 없이 붙었는가? | 통과. verification/runtime-feature-tests.log 17행과 verification/layout-feature-tests.log 7행은 mutation fail-closed 검사를 기록한다. crates/spinon-runtime/src/session/ua_cascade/runtime_layout_tests/c11_grid_tests/cyclic_percentage_auto_repeat_rows.rs 389–398행은 추가 inflow child와 browser marker-as-node 거부를 구체화한다. |
| 19 | static archive 검사 결과를 APK/IPA 전체 제외로 확대했는가? | 통과. spec/internal/0059-c11-grid-layout.md 536·538행은 archive 경계만 확인했고 APK/IPA 전체 fixture exclusion은 증명하지 않는다고 한정한다. evidence/README.md 45행도 기본 산출물과 positive control 검사만 설명한다. |
| 20 | WPT나 제품 호환성을 통과한 것으로 과장했는가? | 통과. 계약 0059 538행, README.md 49행, README.html 50행 및 roadmap 181행은 WPT·일반 admission·전체 C11.4를 미완료로 둔다. |
| 21 | Android 실기기 또는 hardware GPU로 오인될 환경 표기가 있는가? | 통과. evidence/README.md 49행은 API 37 ARM64 16 KB AVD를 명시한다. 최신 Android runtime-definite-40px-auto-fill-control-rows.log 21행은 llvmpipe renderer를 기록하고 roadmap.html 181행도 software renderer라고 한정한다. |
| 22 | iOS 결과가 실기기 또는 compositor 이후 pixel 결과로 읽히는가? | 통과. evidence/README.md 49행과 spec/internal/0059-c11-grid-layout.md 536·538행은 iPhone 17 Pro/iOS 26.2 Simulator이며 compositor 이후 pixel 검증은 없다고 밝힌다. |
| 23 | scene별 3/3 PixelCopy·Metal equality를 검증했다고 과장하는가? | 정정 확인(P2 해소). target/control scene 정의 tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs 4852–4880행에는 pixelSamples가 없고, 7217–7230행의 pixelCaptureArguments는 값이 없으면 빈 인자를 반환한다. README.md 16행, README.html 33행, STATUS.md 368행, spec/internal/0059-c11-grid-layout.md 536행, roadmap.html 181행은 이제 pixel equality 미검증이라고 명시한다. 최신 run logs에도 해당 marker가 없다. |
| 24 | 스크린샷이 pixel oracle이나 compositor 결과로 확대 해석되는가? | 통과. evidence/README.md 16행과 README.html 33행은 스크린샷을 scene 배치 확인으로 한정하고 pixel equality 근거가 아니라고 설명한다. |
| 25 | HTML 이미지가 예전 실패 run이나 다른 scene을 가리키는가? | 통과. evidence/README.md 24–25행과 README.html 36–39행의 네 screenshot 링크가 최신 Android/iOS target/control run 경로와 일치하며 네 파일이 존재한다. |
| 26 | Markdown의 상대 링크가 끊겼는가? | 통과. evidence/README.md 24–32행의 screenshot·metadata·result·runtime directory·설치 앱 JSON·manifest·checksum·verification 경로를 정적으로 확인했고 누락 대상은 없다. |
| 27 | HTML의 링크나 이미지 src가 끊겼는가? | 통과. evidence/README.html 36–48행의 네 이미지, source manifest, sums, result JSON, 설치 앱 JSON, verification 경로가 모두 존재한다. verification/evidence-html-links.log 1–3행도 14개 상대 링크가 해석됐다고 기록한다. |
| 28 | Markdown과 HTML이 병합 상태·범위·count·pixel 한계에서 서로 모순되는가? | 통과. evidence/README.md 3·16·49행과 README.html 23·33·50행은 #632, 72/213, 제한 slice 및 pixel 미검증을 일치시킨다. |
| 29 | Markdown/HTML 간 직접 근거 연결 차이가 검토를 막는가? | 비차단 P3. evidence/README.md 29행은 simulator.json과 runtime log 경로를 직접 제공하고 39–45행은 재현 명령·검증을 적는다. README.html 43–48행은 manifest·result·verification에 연결하지만 simulator metadata/runtime log와 재현 명령은 직접 나열하지 않는다. manifest와 verification index를 통한 경로는 남아 있어 사실 모순이나 깨진 링크는 아니며, HTML에서 로그로 가는 클릭 수만 늘어난다. |
| 30 | 설명 갱신이 다른 run의 checksum을 바꾸거나 오래된 해시를 남겼는가? | 통과. evidence/SHA256SUMS.txt는 이 리뷰 파일을 포함해 103개 항목이며 모든 실제 SHA-256과 일치한다. docs sync diff는 README.html·README.md checksum을 갱신하고 이 리뷰 파일 항목을 추가했으며, run screenshot/log hash는 바뀌지 않았다. |

## 최종 판정

현재 diff에서 남은 P1/P2는 없다. 정정된 두 stale/순서 문제와 PixelCopy/Metal 과장 문제는 지정 문서에서 해소됐다. 비차단 P3는 README HTML에 simulator metadata/runtime logs로 가는 직접 링크가 Markdown보다 적다는 discoverability 차이다.
