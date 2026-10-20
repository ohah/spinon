# C11.3.3 PR #393 병합 후 Tailnet 로드맵 동기화

PR [#393](https://github.com/ohah/spinon/pull/393)의 Rebase and merge 결과 `12dc2669814c4d5e7d0e2aa59d5196fdab98bb68`를 기준으로 선택 문서 빌드를 만들고, 지정된 Tailnet 미리보기 `/spinon/`을 갱신했다. 이번 작업은 미리보기 파일만 동기화한다. 공식 C11.3.3 완료 체크와 전체 CSS Grid 상태는 변경하지 않았다.

## Android 실기기 동작 근거

Samsung SM-S731N, Android 16/API 36에서 C11.3.3 runtime matrix 26 scene·65 node를 실행했다. 성공 19 scene, cascade fallback 5 scene, 기대 거부 2 scene이며, visible frame 61개의 최대 geometry 오차는 0 CSS px다. 별도 overflow 장면에서 PixelCopy 표본 3/3이 정확히 일치했다. 기본 표면 표본도 2/2가 일치했다. 실제 SurfaceFlinger renderer는 ANGLE/Xclipse 940/Vulkan 1.3.279이고, APK SHA-256은 `482e5b5452b3d62f8ad20f51247d6f93622b43c0d3db3ed7fa8d79e6c3122470`이다.

이 근거는 C11.3.3 전용 opt-in의 고정 px trailing implicit-line 범위에 한정한다. 일반 numeric extent, leading/negative implicit placement, `auto`·intrinsic implicit track, WPT 적합성 및 전체 CSS Grid 완료를 뜻하지 않는다. iOS 근거는 iPhone 17 Pro/iOS 26.2 Simulator이며 iOS 실기기나 compositor 이후 표면 검증이 아니다.

## Tailnet 동기화 결과

선택 원본 1,723개(60,634,877 bytes)로 생성한 정적 파일 1,832개(74,077,697 bytes)를 대상으로 사전 비교했다. 신규 315개, 교체 98개, 기존과 동일 1,419개였고 삭제는 없었다. 교체 전 백업 98/98개가 이전 byte 수와 SHA-256으로 확인됐다. 복사 후 1,832/1,832개가 선택 산출물과 byte 수·SHA-256이 일치했다.

copy 직전 `lsof`로 교체 대상의 열린 파일을 조사했다. 열려 있던 대상은 sibling temporary file 작성 후 같은 디렉터리에서 atomic rename으로 바꿔 기존 서버 descriptor를 보존했다. Tailnet preview server와 tmux 세션은 종료하지 않았다.

Tailnet origin에서 1,832개 경로를 모두 요청했다. HTTP 200, 로컬 제공 파일과의 byte 수, SHA-256이 모두 일치했다(불일치 0). Agent Browser에서 `/spinon/roadmap.html`, `/spinon/STATUS.html`, C11.3.3 evidence README를 열어 PR #393 링크, C11.3.3 미완료 상태, 실기기 이미지 로드를 확인했다. evidence 페이지의 Samsung PixelCopy PNG는 900×675로 완전히 로드됐다. 새로 연 Agent Browser 세션을 닫은 뒤 활성 세션 0개와 관련 프로세스 부재를 확인했다.

## Agent Browser 화면

![Tailnet 로드맵의 C11.3.3 PR #393 항목](https://raw.githubusercontent.com/ohah/spinon/docs/c11-3-3-pr393-tailnet-20261021/spec/internal/evidence/c11-3-3-pr393-tailnet-sync-2026-10-21/roadmap-tailnet-pass1.png)

![Tailnet C11.3.3 evidence 페이지](https://raw.githubusercontent.com/ohah/spinon/docs/c11-3-3-pr393-tailnet-20261021/spec/internal/evidence/c11-3-3-pr393-tailnet-sync-2026-10-21/evidence-tailnet-pass1.png)

![Samsung SM-S731N 실기기 overflow PixelCopy](https://raw.githubusercontent.com/ohah/spinon/main/spec/internal/evidence/c11-3-3-grid-template-areas-runtime-2026-10-21/android-physical-c1133-matrix-20261020T152039508Z-5886c27a/unknown-area-overflow-pixelcopy.png)

## 재현·감사 파일

- `source-inputs-pass1.json`: 선택 원본 경로·byte 수·SHA-256
- `prepare-root-pass1.json`: 선택 root 구성과 링크 정리 결과
- `preflight-pass1.json`: 전체 파일 사전 비교, 백업 대상과 no-delete 판정
- `open-handles-before-copy-pass1.json`: 교체 직전 열린 파일 조사
- `copy-and-backup-pass1.json`: 원자 교체, 사후 복사 및 백업 검증
- `served-verification-pass1.json`: Tailnet에서 전체 파일 HTTP·byte·SHA-256 검증
- `sanitized-built-output-pass1.json`: 빌드 산출물 정리 결과
- `sync-review-20.md`: Tailnet 동기화 절차 실패 관점 검토
- `pr-diff-review-30.md`: 이 문서 변경 실패 관점 검토

선택된 RSPress 빌드만 사용했다. GitHub Pages workflow는 실행하지 않았다. GitHub required checks가 없는 상태를 CI 통과로 표시하지 않는다. 계획, 내부 계약 버전, 공식 완료 집계, 앱 코드는 변경하지 않았다.
