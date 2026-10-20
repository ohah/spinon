# C11.4 auto-fit post-merge Tailnet 동기화 실패 관점 검토 20개

**검토 대상:** PR #372 병합 뒤 공식 대장·공개 로드맵을 고정 Rspress 선택 빌드로 Tailnet /spinon/에 반영하고, 파일·화면 근거를 확인하는 과정. 제품 runtime 재검증이 아니다.

| # | 독립 실패 관점 | 결과와 근거 |
| ---: | --- | --- |
| 1 | 동기화 서버가 다른 root를 제공하는가? | **통과.** 기존 Tailnet server PID 1191이 127.0.0.1:8768에서 제공하며 target은 SpinonPreview다. 서버를 재시작하지 않았다. |
| 2 | 이전 브랜치나 미병합 SHA가 선택 빌드의 기준이 되는가? | **통과.** source branch는 docs/c11-4-auto-fit-minimum-tailnet-sync-20261020, HEAD는 최신 origin/main과 같은 9996ee2089eaa4c0e86506af7fcea8f0cec3491c다. |
| 3 | 초기 source selection이 이전 PR 결과만 담는가? | **발견·수정.** 최초 경로 매핑이 구현 근거 폴더를 잘못 가리켰다. README가 링크한 sibling runtime evidence 경로로 수정한 뒤 121개 mapping, 1,840개 staging file을 만들었다. 초기 실패 기록은 prepare-initial-failure-pass15.json에 있다. |
| 4 | iOS 캡처를 Android 캡처 위치에 잘못 연결하는가? | **발견·수정.** 두 번째 준비 시도에서 iOS 경로도 잘못된 부모 폴더를 사용했다. 같은 Git 경로 기준으로 바로잡고 최종 preparation이 성공했다. |
| 5 | 임시 bunx가 repo의 고정 Rspress를 우회하는가? | **발견·수정.** 초기 실행은 임시 Rspress 1.47.2를 내려받았다. 기존 workspace 의존성 2.0.22를 연결해 최종 빌드를 다시 했고 selected-build-pass15.log가 버전을 기록한다. 실패 로그 두 개를 별도 보존했다. |
| 6 | Rspress output에 checkout 절대 경로가 새어 나가는가? | **통과.** 빌드 뒤 텍스트 산출물을 정리했고 sanitize-pass15.json은 1개 asset 수정, 남은 host/worktree 경로 0개를 기록한다. |
| 7 | 바뀐 입력이 SHA manifest에 빠지는가? | **통과.** 71개 고유 selected source, 823개 파일에 SHA-256을 계산해 SHA256SUMS-preview-inputs-pass15.txt에 저장했다. |
| 8 | 새 C11.4 구현 계획이나 실행 근거가 Tailnet build에서 빠지는가? | **통과.** roadmap, STATUS, 계획, 구현 README, Chrome 사전 관찰, 실행 screenshot이 선택 root에 들어갔고 Agent Browser에서 해당 route를 열었다. |
| 9 | Tailnet 경로가 /spinon/ base를 벗어나는가? | **통과.** 확인한 다섯 route에서 same-origin 링크가 /spinon/ 바깥으로 나간 경우는 0개다. |
| 10 | C11.4 제한 구현을 전체 지원 완료로 표시하는가? | **통과.** roadmap과 STATUS의 C11.4 checkbox는 모두 false이며 상위 C11 완료로 승격하지 않았다. |
| 11 | 공식 완료 수를 임의로 +1 하는가? | **통과.** Agent Browser가 공식 상태에서 집계한 완료 칸 72를 렌더링했다. PR #372 관련 체크는 추가하지 않아 이 동기화로 수가 늘지 않았다. |
| 12 | Chrome 관찰을 normative 기대와 섞는가? | **통과.** 154/162px 관찰과 162/170px normative 기대의 차이를 공개 문구에서 분리했다. |
| 13 | out-of-flow Grid item이 앱에서 지원되는 것처럼 보이는가? | **통과.** C11.1 거부를 기록하고 in-flow second-repeat 화면으로 runtime 범위를 한정했다. |
| 14 | 앱 화면과 문서 화면을 혼동시키는가? | **통과.** 네 개의 동작 캡처는 AVD/Simulator runtime 장면이며 추가 Agent Browser screenshot은 Tailnet 문서 화면이다. 각 용도를 README에서 분리했다. |
| 15 | 캡처가 엉뚱한 기기나 크기에서 온 것인가? | **통과.** Android API 37 ARM64 AVD 이미지 2장은 1080×2400, iPhone 17 Pro/iOS 26.2 Simulator 이미지 2장은 1206×2622로 모두 로드됐다. |
| 16 | 일부 image href만 성공하고 실제 image가 깨지는가? | **통과.** Tailnet 동기화 README의 네 runtime image와 구현 README의 두 overflow image가 모두 naturalWidth > 0, broken image 0개였다. |
| 17 | Chrome route만 갱신되고 STATUS 또는 plan이 stale한가? | **통과.** /spinon/roadmap.html, /spinon/STATUS.html, plan, sync README, implementation README를 각각 열었다. |
| 18 | 복사 전 열린 파일을 덮어써 서버 응답을 손상시키는가? | **통과.** 교체 대상 76개에 대한 즉시 전 lsof 검사에서 열린 descriptor는 0개였다. 각 교체는 backup 검증 뒤 임시 파일 원자 rename으로 적용했다. |
| 19 | backup이 없거나 원본과 다른가? | **통과.** 변경 전 76개 파일의 backup SHA-256이 전부 일치했다. |
| 20 | 일부 asset만 이전 state로 남는가? | **통과.** 선택 파일 1,571개 모두 HTTP 200과 local/served byte·SHA-256 일치를 확인했다. 변경 76, 생성 104, 동일 1,391, 삭제 0이다. |

이 검토는 Tailnet 정적 문서 배포와 Agent Browser 화면 확인에 한정한다. Android/iOS runtime, WPT, 실기기, hardware GPU, iOS compositor 이후 표면 검증은 다시 실행하지 않았다.
