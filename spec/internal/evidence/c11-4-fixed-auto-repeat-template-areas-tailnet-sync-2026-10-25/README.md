# C11.4 PR #604 후속 Tailnet 로드맵 동기화 · 2026-10-25

## 목표와 판정 경계

구현 PR [#604](https://github.com/ohah/spinon/pull/604) 병합 뒤 공개 roadmap, 상태 대장, C11.4 계획 및 Tailnet 미리보기를 맞췄다. 이 문서 동기화는 앱 runtime 검증이 아니다. C11.4 checkbox는 미완료로, 공식 완료 수는 `72/213`으로 유지한다.

## 선택 문서 빌드

전체 `bun run docs:build`은 기존 내부 evidence Markdown의 `.json`·`.log` dead link를 RSPress가 route로 판정해 실패했다. 관련 없는 문서 링크는 이 변경에서 고치지 않았으며 전체 site build가 성공했다고 표시하지 않는다. RSPress `2.0.22`를 고정한 별도 설정은 `markdown.link.checkDeadLinks`와 `markdown.image.checkDeadImages`를 모두 끈 상태로 상태 대장, 계획, 계약, runtime 근거, roadmap 및 직접 참조 이미지를 포함한 선택 문서 root를 만들었다. 따라서 이 선택 build는 dead link/image 완전성을 검사하지 않고, 선택 산출물 생성과 route hash만 확인한다. 근거는 `selected-root.json`, `selected-build-verification.json`, `rspress.config.ts`에 기록했다.

## Tailnet 배포와 자산 누락 수정

대상은 `https://macstudio.tailed42f2.ts.net/spinon/`이고, Funnel은 local preview server `127.0.0.1:8768`로 요청을 전달했다. preview root는 `/Users/yoonhb/Library/Application Support/SpinonPreview`다. 기존 파일을 지우지 않고 변경 전 경로와 hash를 확인한 뒤 선택된 경로만 갱신했다.

pass01은 문서 및 직접 이미지 11개 경로를 처리했다(생성 5, 교체 4, 동일 2). 교체 전 파일 2,322,411 bytes를 backup 경로에 보존했다. local preview와 Tailnet에서 11/11 HTTP 200 및 SHA-256 일치를 확인했다. Agent Browser에서 runtime README를 다시 확인하자 Android/iOS screenshot의 `naturalWidth`가 모두 0이었다. HTML에는 RSPress가 만든 hashed image와 lazy-loaded JS route가 연결돼 있었지만, pass01에는 이 정적 자산이 없었다.

이 문제를 pass02에서 수정했다. 선택 RSPress build의 `static/` 전체 19개 경로를 추가했고, 10개를 생성했으며 나머지 9개는 이미 hash가 같아 유지했다. 새 backup이나 삭제는 없었다. 최종 집합은 문서·직접 이미지 11개와 RSPress static assets 19개, 총 30개의 고유 경로다. 최종 확인에서 local preview와 Tailnet의 HTTP 200 및 SHA-256을 30/30 확인했고, pass01 교체 전 backup도 4/4 검증했다. `tailnet-final-verification.json`의 원격 검증을 직접 재현할 때는 위 Tailnet base URL에 기록된 route를 붙인다.

## Agent Browser 확인과 프로세스 정리

[Agent Browser 0.31.1](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html)을 사용해 Tailnet의 roadmap, STATUS, 계획, runtime README를 확인했다. 세 문서는 PR #604와 merge SHA `912c10bee4e2d88374a2a32689ac37c824b1207d`를 표시한다. 공식 `72/213`, C11.4 미완료 및 unchecked checkbox도 유지한다. runtime README의 Android PNG는 `naturalWidth=1080`, iOS PNG는 `naturalWidth=1206`으로 로드됐다. 초기 자산 누락 결과와 수정 뒤 결과 모두 `agent-browser-verification.json`에 기록했다.

roadmap screenshot은 1280×633, SHA-256 `d8153e60b10587296f96d466bee5600dcfb39c01ceae4b7fba944504171635b4`다. 세 Agent Browser session을 닫고 `agent-browser close --all` 뒤 session 0개와 agent-browser/Chrome listener 0개를 확인했다. Tailnet preview server는 계속 실행 중이다.

## 근거 파일

- 선택 site root와 route hash: `selected-root.json`, `selected-build-verification.json`
- 선택 경로와 변경 전 backup 계획: `tailnet-preflight.json`
- 문서 route 복사 및 검증: `tailnet-copy.json`, `tailnet-verify.json`
- 정적 자산 목록·사전 검사·복사·검증: `selected-static-assets.json`, `tailnet-assets-preflight.json`, `tailnet-assets-copy.json`, `tailnet-assets-verify.json`
- 최초 30개 local/remote 경로 및 backup 통합 결과: `tailnet-final-verification.json`
- remote origin을 결과에 포함한 재검증: `tailnet-final-verification-reproduced.json` (`verify-tailnet-final.mjs` 재실행)
- Agent Browser 표시·이미지 로드·프로세스 종료: `agent-browser-verification.json`
- roadmap 화면: `roadmap-agent-browser.png`
- 선택된 문서 root 준비, 정적 자산 수집, Tailnet 복사 및 검증 도구: `prepare-selected-root.mjs`, `prepare-selected-site.mjs`, `prepare-static-assets.mjs`, `sync-tailnet.mjs`, `sync-tailnet-assets.mjs`, `verify-tailnet-final.mjs`

이 근거는 위에 명시한 선택된 문서 route의 빌드와 preview 동기화에 한정된다. 전체 RSPress site build, 앱 runtime, C11.4 전체 구현의 완료 증거로 확대하지 않는다.
