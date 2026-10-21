# Tailnet 미리보기 동기화 근거 — pass42

2026-10-22 KST, PR #478이 병합된 최신 `main`을 기준으로 선택 문서 사이트와 Tailnet 미리보기를 갱신했다. PR #478 merge SHA는 `907390f75e0b25b399a47b6c863a19b7e5a326e7`, source tree는 `cd54a5cc38c46962caba99840bc1fc45f8833592`다. 공개 roadmap 원본에 이 slice의 병합 내용과 제한을 기록하고 집중 계획에 merge metadata를 반영했다. GitHub Pages는 배포하지 않았다.

## 선택 build

- Rspress `2.0.22`; 선택 site는 4,505 files, 189,154,819 bytes, deterministic tree SHA-256 `70ff7bf15ece6aa0e919ced40751337c710033d83579245ed70ae8a3a3561c29`다.
- 선택 root는 pass41 tracked docs를 보존한 뒤 PR #478 이후 변경된 roadmap, `STATUS`, C11.4 계획, Chrome 사전 비교 및 AVD/iOS Simulator 근거만 적용했다. unrelated untracked workspace evidence는 포함하지 않았다.
- 공개 roadmap, 상태 대장, 계획, 사전 비교 화면, Android AVD 화면, iOS Simulator 화면의 선택 route를 확인했다. C11.4 checkbox는 미완료이며 공식 완료 수 `72/213`은 변하지 않았다.
- build 입력·경로·본문 점검: [selected-root-preparation](./selected-root-preparation-pass42.json), [selected build summary](./selected-build-summary-pass42.json), [Rspress log](./selected-build-pass42.log) (line-end whitespace normalized).

## 안전 복사와 원격 검증

Preflight에서 4,505개 target을 비교해 367개 신규, 261개 교체, 3,877개 동일로 분류했다. 삭제는 하지 않았다. 교체 대상의 기존 48,588,234 bytes를 pass42 전용 backup에 보관했다. 복사는 파일별 source/target hash 재검증 후 임시 파일에서 원자 rename으로 적용했다.

로컬 최종 검증은 선택 파일 4,505/4,505개와 backup 261/261개의 byte 수·SHA-256이 일치하고 실패 0이다. Tailnet origin `https://macstudio.tailed42f2.ts.net/spinon/`의 전체 4,505개 route가 HTTP 200을 반환했으며 response byte 수와 SHA-256이 모두 선택 build와 일치했다. 실패는 0이다. 세부 내용은 [preflight](./preflight-pass42.json), [copy와 backup](./copy-and-backup-pass42.json), [최종 로컬 검증](./final-verification-pass42.json), [원격 route 검증](./served-verification-pass42.json)과 [실행 로그](./remote-verification-pass42.log)에 기록했다.

## Agent Browser 확인과 종료

Agent Browser `0.31.1`에서 Tailnet roadmap, 공식 상태 대장, C11.4 집중 계획, runtime 근거를 열었다. roadmap에 PR #478 merge SHA와 `72/213`이 보였고, 상태 대장에서 C11.4 checkbox가 unchecked임을 확인했다. 계획 페이지의 merge SHA도 확인했다. 직접 연 Simulator 이미지의 natural size는 Android AVD `1080×2400`, iPhone 17 Pro/iOS 26.2 Simulator `1206×2622`다. roadmap 화면은 [캡처](./tailnet-roadmap-agent-browser-pass42.png)에 저장했다.

Agent Browser session을 닫았고 `agent-browser session list`는 활성 세션이 없음을 보였다. 해당 session PID도 종료됐다. 미리보기 server PID 1191은 `127.0.0.1:8768` listen 상태로 보존했다. tmux에는 접근하지 않았다. 관점별 검토는 [sync review 30개](./sync-review-30-pass42.md)다.

이번 변경은 static Tailnet preview와 source roadmap 기록을 갱신한 작업이다. 새 모바일 runtime 실행은 하지 않았다. Android 실기기, hardware GPU, iOS compositor 이후 pixels, glyph paint, WPT 및 전체 C11.4 적합성은 확인하지 않았다.
