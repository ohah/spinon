# C11.4 direct-text min/max Tailnet 로드맵 동기화

## 범위와 입력

2026-10-30에 병합된 `origin/main` 커밋 `2102696e458e23993387d218d8932d1dcb062cba`(tree `2457267a884fb39811a8d0b22b540682e8770afd`)에서 로드맵과 선택한 C11.4 근거 문서만 렌더링해 Tailnet 미리보기로 복사했다. 전체 문서 사이트 배포가 아니다.

- 선택한 원본은 `selected-root.json`에 기록되어 있다. 108개 선택 경로, 5,645,682바이트를 고정 입력으로 만들었다.
- RSPress 2.0.22 빌드 결과는 `selected-build-verification.json`에 기록되어 있다. 123개 출력 파일, 10,551,275바이트이며 로드맵·STATUS·명세·fixture·시뮬레이터 근거 경로를 확인했다.
- 선택 빌드 설정은 `rspress.config.ts`다. 미리보기 subset에서는 dead-link 검사를 끄므로 이 결과를 전체 사이트 링크 검증으로 해석하지 않는다.
- 모든 입력 및 렌더링 출력의 개별 크기와 SHA-256은 대응 manifest에 남겼다.

## Tailnet 복사와 검증

복사 전 대상에 새 파일 102개, 교체 8개, 동일 13개가 있음을 기록했다. 교체 전 파일 8개(3,168,171바이트)는 `SpinonPreviewBackups/c11-4-direct-text-minmax-roadmap-tailnet-sync-2026-10-30/pass01` 아래에 보관했다. 삭제는 없었다. 복사 보고서는 110개 파일과 백업 8개를 기록한다.

검증 결과는 `tailnet-verify.json`에 있다. 로컬 대상 123/123, 백업 8/8, Tailnet HTTPS 응답 200 및 SHA-256 일치 123/123이다. 서버 PID 28040의 Tailnet 미리보기 프로세스는 그대로 유지했다.

첫 preflight 시도는 서버 listener의 네트워크 handle이 순간적으로 두 개로 관측되어 안전 검사에서 중단됐다. 이 시도는 preflight 단계라 파일을 변경하지 않았다. 뒤이은 handle 재확인에서 예상 listener 하나만 확인된 뒤 preflight를 다시 실행했고, 경과는 `tailnet-preflight-failure.log`에 남겼다.

## Agent Browser 화면 확인

터미널 브라우저는 사용하지 않았다. Agent Browser 세션 `spinon-c114-roadmap-sync`에서 `https://macstudio.tailed42f2.ts.net/spinon/roadmap.html`과 `/spinon/STATUS.html`을 열어 확인했다.

- 로드맵 화면 제목, PR #850 카드, 병합 SHA, `42/42`, `129/129`, 공식 `72/213` 표기를 확인했다.
- STATUS에서 공식 집계 `72/213`과 C11.4 미체크 상태를 확인했다.
- 카드에 연결된 Android 이미지(1080×2400)와 iOS 이미지(1206×2622)가 모두 로드됐다.
- 화면 캡처는 `tailnet-roadmap-agent-browser.png`와 `tailnet-status-agent-browser.png`다. 관측값, 이미지 크기·해시, 세션 종료 상태는 `agent-browser-verification.json`에 기록했다.
- 사용 후 Agent Browser 세션과 프로세스를 종료했다. Agent Browser 목록은 비었고 실행 프로세스도 없음을 확인했다. Tailnet 미리보기 서버는 계속 실행 중이다.

## 범위 한계

이 동기화는 선택 문서 미리보기와 그 HTTP 응답 해시를 확인한다. CI 성공, 전체 문서 사이트의 dead-link 검사, 앱 전체 CSS Grid/C11.4/C11 완료를 뜻하지 않는다. C11.4와 C11은 공식 상태에서 미완료이며 공식 집계는 `72/213`이다. PR check rollup도 성공 결과로 보고하지 않는다.

## 검토

복사 helper의 경로 안전성, 입력 고정, symlink 거부, 사전 상태 재확인, 백업, 원자적 파일 교체, 삭제 부재, 원격 해시 검증과 부분 실패 복구 경계를 별도 20관점으로 검토했다. 결과는 `sync-helper-review-20.md`다.
