# Tailnet 미리보기 원본 근거 링크 검증 · pass31

2026-10-21 pass29에서는 RSPress 본문 이미지가 변환된 정적 이미지 URL로 로드됐지만, README의 상대 원본 PNG URL은 404였다. pass31은 같은 run 디렉터리와 재현 스크립트를 `public/internal/evidence/`에도 복사해 브라우저 본문 표시와 원본 파일 직접 열기를 모두 지원한다.

## 빌드와 동기화

| 검사 | 결과 |
| --- | --- |
| 입력 | pass29 선별 문서 root를 복제한 뒤 최신 STATUS·로드맵·계획·실기기 근거를 반영 |
| RSPress | `2.0.22` build 성공, 1,760개 파일, 총 75,153,242 bytes |
| raw evidence | result JSON, 화면·surface PNG 4개, 기기 정보·로그, 재현 스크립트 포함 |
| preflight | create 116, replace 100, same 1,544, 삭제 0 |
| 교체 백업 | 100/100 SHA-256 일치 |
| 로컬 최종 확인 | Tailnet 대상 1,760/1,760 일치, 백업 100/100 일치 |
| 원격 HTTP·SHA-256 | 1,760/1,760 HTTP 200, 1,760/1,760 해시 일치, 실패 0 |

교체 직전 검사에서 열린 파일 두 개를 확인했다. 둘 다 Tailnet preview PID `1191`의 idle listener만 열고 있었고, 다른 client socket은 없었다. 각 기존 파일은 rename 전에 전용 백업 경로에 보존했다. pass31은 이전 preview 파일을 삭제하지 않았다.

## Agent Browser 확인

- 로드맵에서 `Android 실기기 보충 실행·화면` 링크가 새 evidence README를 가리켰다.
- README에 장치·frame/trace 정보와 두 Android 화면 이미지가 표시됐다. 변환 이미지 크기는 각각 1080×2340이었다.
- README의 원본 row 화면 PNG 주소를 직접 열어 1080×2340 이미지가 로드됨을 확인했다.
- 원본 `result.json` 주소를 직접 열어 `application/json`, run ID `c114-fg-device-20261020T220621187Z`, `status: passed`를 확인했다.
- STATUS 페이지에서 실기기 근거와 C11.4 미완료 문구를 확인했고, 계획 페이지에서 실기기 보충과 미완료 상태를 확인했다.
- Agent Browser 세션을 닫은 뒤 실행 중인 Agent Browser/Chrome 프로세스가 없음을 확인했다. preview server PID `1191`은 유지했다.

브라우저 화면: [근거 README와 표시 이미지](./tailnet-evidence-agent-browser-pass31.png) · [원본 PNG 직접 열기](./tailnet-direct-device-image-agent-browser-pass31.png) · [result JSON 직접 열기](./tailnet-direct-result-agent-browser-pass31.png).

## 한계

이 빌드는 전체 `spec/`을 다시 구성한 결과가 아니라 이전 선별 root 기반의 미리보기 갱신이다. 별도로 수행한 전체 root 빌드는 기존 S04 iOS 이미지 누락 때문에 실패했으며 해당 로그는 이 변경에서 수정하지 않았다. 이 pass는 Tailnet preview 경로와 파일 일치를 확인하며 GitHub Pages 배포나 CSS 적합성을 주장하지 않는다.

세부 원장: [preflight](./preflight-pass31.json) · [복사·백업](./copy-and-backup-pass31.json) · [최종 로컬 확인](./final-verification-pass31.json) · [원격 확인](./served-verification-pass31.json) · [HTTP 진행 로그](./remote-verification-pass31.log).
