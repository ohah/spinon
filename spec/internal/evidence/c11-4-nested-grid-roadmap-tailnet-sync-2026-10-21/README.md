# C11.4 nested Grid 로드맵 Tailnet 반영

2026-10-21 기준 C11.4 일반 nested Grid intrinsic contribution 집중 계획과 Chrome 156 사전 관찰을 구현 로드맵에 연결하고, 선택 문서 사이트를 Tailnet 미리보기에 동기화했다. 이 변경은 계획·문서 미리보기 갱신이며 C11.4 구현이나 공식 완료 상태를 바꾸지 않는다.

## 원본과 빌드

- 소스 기준: `0650281cf82c8295375e32895f8577f1e171bba3` (`docs/c11-4-nested-grid-roadmap-20261021`)
- Tailnet 기준으로 선택한 이전 사이트: `44c18e61d2328753726cd120eeca3cff465a29a4`
- 선택 문서 root: 6,094개 파일에서 변경된 계획·명세·증거 32개를 적용했고, 증거용 RSPress 설정 파일 1개만 route 대상이 아니어서 명시적으로 제외했다. 심볼릭 링크와 삭제는 허용하지 않았다.
- RSPress `2.0.22` 선택 빌드: 3,269개 파일, 161,165,381 bytes, site tree SHA-256 `de853d59965b881766d00216e4805bc5316e9b33a4bf181a106b31b7329f35dc`.
- 필수 route에서 roadmap, STATUS, C11.4 계획, precomparison README/JSON/PNG가 생성됐고 새 로드맵 안내와 공식 `72/213` 집계가 포함됐다. C11.4는 미완료로 남아 있다.
- 저장소 전체 dead-link 검사는 이 선택 빌드에서 실행하지 않았다. 이전 pass01은 증거 폴더의 `rspress-pass05.config.ts`가 route 콘텐츠로 읽혀 `node:*` 모듈을 클라이언트 번들에 포함하려다 실패했다. pass02는 그 설정 파일만 선택 root에서 제외하고 성공했다. 원본 파일은 보존했다.

## 미리보기 동기화와 확인

- 미리보기 경로는 `/Users/yoonhb/Library/Application Support/SpinonPreview`다. 사전 비교는 생성 251개, 교체 202개, 동일 2,816개, 삭제 0개였고 교체 전 202개 파일(30,626,812 bytes)을 별도 백업 경로에 저장했다.
- 복사 후 로컬 대상 3,269/3,269와 백업 202/202가 SHA-256으로 확인됐다. Tailnet URL에서도 3,269/3,269 route가 HTTP 200이고 기대한 SHA-256과 일치했다.
- Agent Browser로 `/spinon/roadmap.html`을 열어 새 안내와 `72/213` 표시를 확인했다. 캡처는 `tailnet-roadmap-agent-browser-pass02.png` (1280×633, SHA-256 `be2649fd4bc93c356e5830b7895a1087845a0fd33d65f5ddbd534080de41ed64`)이다.
- PR #441 본문 이미지 주소에서 commit SHA 오타를 Agent Browser가 `naturalWidth=0`으로 잡아 수정했다. 고친 주소의 화면은 1280×633으로 실제 렌더링됐다.
- 기존 미리보기 서버 PID 1191은 `127.0.0.1:8768`에서 계속 실행 중이다. Tailnet 동기화 도구는 서버 프로세스나 tmux를 종료하지 않았다. Agent Browser 세션은 이 확인 뒤 닫았다.

## 범위

Android API 37 AVD와 iOS Simulator에서 이 변경 또는 새 nested Grid 동작을 실행하지 않았다. 미리보기의 플랫폼 캡처는 기존 문서에 포함된 자료다. nested Grid 동작의 구현 전 Chrome 156 관찰과 판정 기준은 [precomparison 증거](../c11-4-nested-grid-intrinsic-precomparison-2026-10-21/README.md), 구현 계획과 남은 범위는 [C11.4 nested Grid 계획](../../../../plan/c11-4-nested-grid-intrinsic.md)에 있다.

## 산출물

- `manifest-archive-pass02.json`: 네 개 전체 manifest의 핵심 집계와 압축 전·후 byte 길이 및 SHA-256.
- `preflight-pass02.json.gz`, `copy-and-backup-pass02.json.gz`, `local-verification-pass02.json.gz`, `served-verification-pass02.json.gz`: 복사 전 예상값, 백업, 로컬 복사 사후 점검과 Tailnet 3,269개 파일의 HTTP 상태·길이·SHA-256 원본. 압축을 풀면 검증 당시 JSON 바이트를 복원한다.
- `selected-root-preparation-pass01.json`, `selected-build-pass01.log`: 첫 선택 빌드 입력과 실패 원인.
- `selected-root-preparation-pass02.json`, `selected-build-pass02.log`, `selected-build-summary-pass02.json`: 설정 파일 제외 후 성공한 선택 빌드.
- `prepare-selected-root-pass*.mjs`, `rspress-pass*.config.ts`, `summarize-selected-build-pass*.mjs`, `sync-tailnet-pass*.mjs`, `verify-tailnet-pass*.mjs`: 이번 동기화에서 사용한 고정 입력 스크립트. 현재 checkout 및 지정된 macOS 미리보기 경로에 묶인 증거 도구이며 범용 배포 CLI가 아니다.
- `agent-browser-tailnet-pass02.json`, `browser-close-pass02.json`: 브라우저 페이지 결과와 종료 확인.
- `pr-diff-review-30.md`: 이 PR diff에서 확인한 독립 실패 관점.
