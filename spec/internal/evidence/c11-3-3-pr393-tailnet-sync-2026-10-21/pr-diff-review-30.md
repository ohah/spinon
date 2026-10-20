# C11.3.3 PR #393 Tailnet 문서 변경 실패 관점 검토 30개

**검토 대상:** 이 폴더의 동기화 설명, 결과 manifest, 브라우저 캡처와 Tailnet/실기기 이미지 링크. 구현 코드는 변경하지 않는다.

| # | 독립 실패 관점 | 판정 근거 |
| ---: | --- | --- |
| 1 | PR #393의 merge commit이 잘못 연결된다. | README가 실제 merge SHA `12dc2669814c4d5e7d0e2aa59d5196fdab98bb68`를 기록한다. |
| 2 | PR의 병합 상태를 체크 통과로 오해하게 쓴다. | README는 required checks 부재를 CI 통과로 표시하지 않는다고 명시한다. |
| 3 | 구현 후보를 제품 전반 완료로 넓혀 말한다. | 설명을 C11.3.3 전용 opt-in과 명시된 고정 px trailing 범위에 한정한다. |
| 4 | 전체 CSS Grid 지원으로 읽힌다. | WPT·일반 numeric·leading/negative·auto/intrinsic 제한을 분리해 쓴다. |
| 5 | 실기기 모델이나 OS 버전이 빠진다. | Samsung SM-S731N, Android 16/API 36을 함께 적는다. |
| 6 | 26개 장면 수가 다른 플랫폼 결과와 혼합된다. | Android hardware matrix 결과를 먼저 별도 단락에 기록한다. |
| 7 | 성공·fallback·기대 거부 수를 모두 성공으로 합산한다. | 19 success, 5 cascade fallback, 2 expected rejection을 별도로 표시한다. |
| 8 | visible-frame 기준에서 오차 0을 전체 픽셀 정확도로 과장한다. | geometry 오차와 PixelCopy 표본 수를 별도 수치로 쓴다. |
| 9 | overflow 장면 표본을 기본 장면 표본과 혼합한다. | overflow 3/3과 기본 표면 2/2를 각각 구분한다. |
| 10 | Software renderer를 실제 GPU 경로로 부른다. | 관찰한 ANGLE/Xclipse 940/Vulkan 1.3.279만 기기 renderer로 기록한다. |
| 11 | APK가 다른 빌드일 수 있는데 결과를 귀속한다. | 검증에 사용한 APK SHA-256을 기록한다. |
| 12 | AVD 결과를 실기기 결과로 오해한다. | Android 실기기와 동반 simulator 근거를 분리해 설명한다. |
| 13 | iOS simulator를 iOS 실기기 또는 compositor 검증으로 읽는다. | iOS 26.2 Simulator이며 compositor 이후 미검증이라고 한정한다. |
| 14 | iOS evidence가 Android matrix와 같은 gate처럼 표현된다. | 플랫폼·검증 한계를 별도 문장으로 적는다. |
| 15 | 미실행 WPT를 통과했다고 오해한다. | WPT 적합성은 완료 범위가 아니라고 명시한다. |
| 16 | official STATUS 완료 체크가 바뀐 것으로 오해한다. | README가 완료 checkbox와 공식 집계를 변경하지 않았다고 밝힌다. |
| 17 | roadmap 항목이 Tailnet 외부에만 존재한다고 오해한다. | 실제 `/spinon/roadmap.html`을 검증한 결과와 캡처를 연결한다. |
| 18 | 서버가 Tailnet 외부 URL에서 stale 상태다. | served verification은 Tailnet origin에서 전 경로를 요청해 검사했다. |
| 19 | 1,832개가 전체 웹의 경로 수라는 과장이다. | 수치를 이번 선택 빌드와 pass1 대상에 한정한다. |
| 20 | 바이트 총량이 선택 입력과 출력에서 혼동된다. | 원본 1,723개/60,634,877 bytes와 출력 1,832개/74,077,697 bytes를 분리한다. |
| 21 | create/replace/same 합계가 총계와 맞지 않는다. | 315 + 98 + 1,419 = 1,832로 일치한다. |
| 22 | 기존 사용자 파일이 삭제될 수 있다. | README와 sync manifest가 삭제 없음으로 기록한다. |
| 23 | 교체 전 백업이 없거나 해시가 맞지 않는다. | 98개 백업의 byte 수·SHA 결과 파일을 연결한다. |
| 24 | 열려 있는 파일의 원자 교체를 안전하지 않게 표현한다. | `lsof` 확인, sibling temp와 atomic rename을 기술하고 서버 유지 사실을 적는다. |
| 25 | 미리보기 서버나 tmux가 종료되었다고 오해한다. | 두 프로세스를 종료하지 않았다고 명시한다. |
| 26 | Agent Browser를 확인하지 않고 스크린샷을 첨부한다. | 캡처는 Agent Browser에서 실제 Tailnet origin을 연 결과다. |
| 27 | evidence 페이지의 이미지 파일이 요청만 되고 실제 로드는 실패한다. | DOM 확인에서 Samsung PNG `complete=true`, 900×675 natural dimensions다. |
| 28 | 사용자 요청한 실행 화면이 PR에서 깨진다. | PR 설명은 현재 branch의 raw screenshot과 main에 병합된 PixelCopy PNG를 사용한다. |
| 29 | GitHub Pages가 배포된 것으로 잘못 안내한다. | README는 Pages workflow를 실행하지 않았다고 기록한다. |
| 30 | 증거 파일이 변경되지 않은 코드·공식 상태까지 포함하는 것으로 보인다. | 이 PR 범위를 동기화 근거·리뷰·캡처로 제한하고 앱 코드/명세 상태는 바꾸지 않는다. |

## 결과

30개 관점에서 수치, 범위, 링크와 검증 주장을 대조했다. 병합 상태와 official completion을 분리했고, 실제 기기·simulator·Tailnet 제공 파일의 검증 경계를 유지했다. 발견한 상충 항목은 없다.
