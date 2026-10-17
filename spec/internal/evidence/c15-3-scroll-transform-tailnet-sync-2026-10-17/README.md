# C15.3 scroll transform Tailnet 미리보기 동기화

## 범위

병합된 [PR #261](https://github.com/ohah/spinon/pull/261)의 C15.3 non-zero scroll ancestor glyph transform 결과를 Tailnet 정적 미리보기에 반영한다.

- [Tailnet 로드맵](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html)
- [공식 상태 대장](https://macstudio.tailed42f2.ts.net/spinon/STATUS.html)
- [내부 계약 0074](https://macstudio.tailed42f2.ts.net/spinon/internal/0074-c15-3-runtime-glyph-scroll-transform.html)
- [구현·Simulator 실행 근거](https://macstudio.tailed42f2.ts.net/spinon/internal/evidence/c15-3-text-scroll-transform-2026-10-17/README.html)
- [Chrome 사전 비교 근거](https://macstudio.tailed42f2.ts.net/spinon/internal/evidence/c15-3-text-scroll-transform-precomparison-2026-10-17/README.html)

원본은 `origin/main`의 `740b64032a68cd24ae258d341cefcfc2a3fe8984`이다. Tailnet은 저장소를 직접 제공하지 않으므로 선택한 Rspress 산출물과 참조 자산을 `/Users/yoonhb/Library/Application Support/SpinonPreview`에 copy-only 방식으로 반영한다. 현재 내용과 다른 대상은 전용 backup 경로에 원본을 먼저 보존한다. 대상 삭제나 preview listener 재시작은 하지 않는다.

## 반영한 내용

- 공개 roadmap 두 경로에 PR #261의 수평·수직·nested scroll 결과를 표시한다.
- 최신 `STATUS.html`, C15 계획, 내부 색인, 계약 0074와 C15.3 실행·사전 비교 근거를 렌더한다.
- 실행 로그·시뮬레이터 화면·Chrome fixture 화면의 참조 파일을 같은 route 아래 제공한다.
- 공식 C15.3/C15 미완료 상태와 `61/197` 완료 수를 유지한다.

## 검증 결과와 한계

선택된 C15 문서 route만 Rspress 2.0.22로 빌드했다. 이 작업은 저장소 전체 문서 빌드나 Markdown 링크 전체 검사를 통과했다고 주장하지 않는다. 선택 build의 입력·명령·경계는 [render-build-result.md](render-build-result.md)와 [build log](selected-build.log)에 기록한다.

preflight에서 133개 경로를 확인했다. 97개를 새로 만들고, 19개 변경 경로의 이전 bytes를 preview root 밖 backup에 보존한 뒤 교체했으며, 동일한 17개 경로는 건드리지 않았다. 삭제는 없었다. 133개 경로 모두 local listener와 public Tailnet URL에서 HTTP 200, byte length 및 SHA-256이 source manifest와 일치했다. 기록은 [preflight](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c15-3-scroll-transform-tailnet-sync-2026-10-17/preflight.json), [manifest](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c15-3-scroll-transform-tailnet-sync-2026-10-17/manifest.json), [copy verification](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c15-3-scroll-transform-tailnet-sync-2026-10-17/copy-verification.json), [backup verification](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c15-3-scroll-transform-tailnet-sync-2026-10-17/backup-verification.json), [served verification](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c15-3-scroll-transform-tailnet-sync-2026-10-17/served-verification.json)에 남겼다.

Agent Browser에서 Tailnet roadmap, 계약 0074, 실행 근거를 열었다. 실행 근거의 Android 1080×2400 및 iOS 1206×2622 이미지는 모두 완전히 로드됐다. 페이지 검증 뒤 Agent Browser 세션을 닫았고, preview listener는 HTTP 200으로 유지됐다. PR 본문과 이 디렉터리에 보관한 Agent Browser 캡처는 정적 문서 전달 증거이며 제품 앱의 PixelCopy/Metal 표면 증거가 아니다.

- [Tailnet roadmap 화면](tailnet-roadmap-agent-browser.png)
- [계약 0074 화면](tailnet-contract-0074-agent-browser.png)

Android 근거는 API 37 ARM64 AVD, iOS 근거는 iPhone 17 Pro/iOS 26.2 Simulator의 compositor 제출 전 Metal drawable이다. 실기기, hardware GPU, iOS compositor 이후 출력, 일반 DOM text admission과 C15 완료를 의미하지 않는다.

## 동기화 기록

- [사전 inventory](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c15-3-scroll-transform-tailnet-sync-2026-10-17/preflight.json)
- [파일·SHA-256 manifest](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c15-3-scroll-transform-tailnet-sync-2026-10-17/manifest.json)
- [복사 후 검증](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c15-3-scroll-transform-tailnet-sync-2026-10-17/copy-verification.json)
- [덮어쓰기 backup 검증](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c15-3-scroll-transform-tailnet-sync-2026-10-17/backup-verification.json)
- [served HTTP 검증](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c15-3-scroll-transform-tailnet-sync-2026-10-17/served-verification.json)
- [Tailnet 동기화 변경 실패 관점 30개](pr-delta-review-30.md)
