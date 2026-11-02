# PR #1042 이후 C11 로드맵 Tailnet 동기화 · 2026-11-02

## 범위

PR [#1044](https://github.com/ohah/spinon/pull/1044)이 `2026-11-02T08:03:05Z`에 Rebase and merge됐다(merge SHA `734663a19b72b59ff422993a258359b4ab46f970`). 병합된 `main`의 `spec/public/roadmap.html`에 C11.1 parsed Root HTML fixed-px `grid-auto-rows` 목록 slice와 한계가 표시된다. C11.1/C11은 미완료이며 공식 완료 집계는 `72/213`이다.

Tailscale 미리보기는 `/Users/yoonhb/Library/Application Support/SpinonPreview`를 제공한다. Tailnet 쪽 원본은 저장소 파일과 자동 동기화되지 않으므로 선택 publication 103개 route를 만들고 별도로 반영했다. 공개하지 않은 untracked raw log는 보존했고 publication에 포함하지 않았다.

## 동기화 결과

| 확인 | 결과 |
| --- | --- |
| 소스 `main` | `734663a19b72b59ff422993a258359b4ab46f970`, tree `acf5299f623e07b0a5229d7180400576322745ae` |
| Tailscale backend | `Running` |
| 기존 preview server | PID `28040`, `127.0.0.1:8768`; 재시작하지 않음 |
| 선택 파일 | 103 routes, 7,561,395 bytes |
| 변경 분류 | 80 created, 11 replaced, 12 identical |
| 교체 전 백업 | 11 routes, `~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1042-2026-11-02/pass01` |
| 열린 대상 파일 | 교체 전 0개 |
| 로컬·HTTPS 검사 | 로컬 file hashes 103/103, HTTPS HTTP 200 및 SHA-256 103/103 |
| 삭제 | 0 |
| 공식 상태 | `72/213`, C11 unchecked |

각 route의 source·size·SHA-256은 [selected publication manifest](./selected-publication.json), 실행별 preflight·backup·HTTPS 검증은 각각 [preflight](./preflight.json), [copy report](./copy-and-backup.json), [served verification](./served-verification.json)에 있다. `build-publication.mjs`는 tracked source와 선택 Rspress build만 publication으로 복사하고, `sync-tailnet.mjs preflight|copy|verify`는 symlink·열린 대상·기존 hash를 검사하고 교체 전 backup 후 atomic rename을 한다. 어느 단계에서도 route를 삭제하지 않는다.

Rspress 2.0.22 build는 `STATUS.html`, `plan/c11-root-html-grid-auto-row-list.html`, C11 overview·runtime·precomparison 문서를 포함한 7개 선택 문서에서 성공했다. 전체 문서 사이트 검증은 아니다. 전체 build는 별도 [route 검증 기록](../roadmap-tailnet-sync-pr1042-2026-11-02/selected-route-verification.json)에 적힌 기존 dead-link, Shiki language, 상대 이미지 import 오류로 실패했다.

## Agent Browser 확인

Tailnet의 roadmap, 전용 계획, runtime README를 Agent Browser로 열어 PR #1042 문단과 evidence marker를 확인했다. `STATUS.html`에서 공식 `72/213`과 C11 unchecked를 확인했다. PR #1044 본문에 삽입한 Android·iOS Simulator PNG도 Agent Browser에서 각각 1080×2400 및 1206×2622로 로드됐다.

1280px desktop과 390px mobile에서 roadmap document width가 각 viewport와 같았다. mobile에서 새 C11 문단은 보이지만 아래 overview SVG의 오른쪽 일부가 화면에 잘려 보인다. 이 시각 한계는 [desktop](./tailnet-roadmap-agent-browser-desktop.png)·[mobile](./tailnet-roadmap-agent-browser-mobile.png) capture와 [기계 판독 결과](./agent-browser-verification.json)에 남겼다. `agent-browser close --all` 뒤 두 번째 process scan에서 Agent Browser process 0개를 확인했다.

## 검토 한계

[문서·동기화 PR diff 실패 관점 30개](./tailnet-pr-diff-review-30.md)를 별도로 기록했다. 선택 route의 HTTP/hash 및 세 문서 렌더링을 검증했으며, 전체 문서 사이트의 모든 링크를 검증한 것은 아니다. 화면 capture는 Agent Browser의 웹 화면이며 native app pixel equality 근거가 아니다.
