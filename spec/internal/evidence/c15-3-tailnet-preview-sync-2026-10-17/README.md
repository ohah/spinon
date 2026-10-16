# C15.3 Tailnet 미리보기 동기화 근거

## 범위

PR #246에서 병합한 C15.3 계획 설명과 Chrome DOM oracle 근거를 Tailnet 문서 미리보기에 반영한다. 이 작업은 정적 문서·fixture 자산 동기화이며 C15.3 제품 구현이나 C15 완료를 뜻하지 않는다.

- [Tailnet 로드맵](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html)
- [공식 구현 상태](https://macstudio.tailed42f2.ts.net/spinon/STATUS.html)
- [C15.3 Chrome 155 사전 비교](https://macstudio.tailed42f2.ts.net/spinon/internal/evidence/c15-3-unicode-shaping-precomparison-2026-10-17/README.html)
- [이 Tailnet 동기화 PR 변경 적대 검토 30개 관점](https://macstudio.tailed42f2.ts.net/spinon/internal/evidence/c15-3-tailnet-preview-sync-2026-10-17/pr-delta-review-30.html)
- [C15.3 고정 font fixture 안내](https://macstudio.tailed42f2.ts.net/spinon/tests/fixtures/resources/c15/c15-3/README.html)
- [C15.3 oracle fixture](https://macstudio.tailed42f2.ts.net/spinon/tests/fixtures/css/c15/c15-3-unicode-shaping-oracle.html)

## 산출물과 안전 경계

Tailnet 사이트는 저장소를 직접 제공하지 않으므로, 선택한 Rspress 산출물과 연결된 문서·fixture·고정 글꼴을 `/Users/yoonhb/Library/Application Support/SpinonPreview`로 복사한다. 아래 JSON 기록은 저장소의 이 evidence 디렉터리에 보관한다. Tailnet 정적 route에는 sidecar JSON을 직접 복사하지 않는다.

- [최종 preflight](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c15-3-tailnet-preview-sync-2026-10-17/preflight.json)
- [최종 sync manifest](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c15-3-tailnet-preview-sync-2026-10-17/manifest.json)
- [복사 후 bytes/hash 검증](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c15-3-tailnet-preview-sync-2026-10-17/copy-verification.json)
- [HTTP served bytes/hash 검증](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c15-3-tailnet-preview-sync-2026-10-17/served-verification.json)
- [덮어쓰기 백업 검증](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c15-3-tailnet-preview-sync-2026-10-17/backup-verification.json)

동기화는 copy-only로 수행하고 삭제는 하지 않는다. 기존 내용이 다른 대상은 전용 백업 디렉터리에 먼저 보존하고, 복사 전후 source/target hash를 다시 비교한다. Rspress는 C15 문서에 한정한 scoped build로 확인하며 저장소 전체 문서 빌드 통과로 확대 해석하지 않는다.

## 화면 및 제공 확인

Agent Browser에서 Tailnet의 로드맵, C15.3 근거 페이지와 oracle fixture를 열어 본문·정적 이미지·고정 글꼴이 제공되는지 확인한다. 페이지가 참조하는 이미지의 실제 크기와 각 manifest 경로의 HTTP 상태·응답 바이트·SHA-256은 served verification 기록에 보존한다. 화면 캡처는 같은 증거 디렉터리에 둔다.

확인 시 기존 preview listener `127.0.0.1:8768`은 재시작하거나 종료하지 않는다. Agent Browser 실행 뒤 해당 브라우저 프로세스를 닫는다. 이 문서 동기화에는 Android/iOS 앱 변경이 없어 네이티브 Simulator 동작 검증을 주장하지 않는다.

## 완료 경계

C15.3 자료는 고정 입력의 Chromium DOM 사전 관찰이다. Spinon shaping, 실제 glyph ID/paint, Taffy 소비처, 부모 테두리 밖 ink, clip·hit-test의 앱 실행 검증은 별도 구현 단계다. 공식 상태 대장의 C15, C16, C19 미완료 상태는 유지한다.
