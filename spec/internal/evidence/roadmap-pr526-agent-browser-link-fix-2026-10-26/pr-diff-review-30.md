# PR #526 Agent Browser 근거 링크 수정 적대 검토 · 30개 관점

검토일: 2026-10-26
검토 worktree: `/private/tmp/spinon-roadmap-pr526-link-fix-20261026`
기준 `HEAD` 및 `origin/main`: `8247a12b94d6e5d75475c16977fa1f11ed983a42`
검토 대상 source diff: `spec/public/roadmap.html` 한 href의 `.html` → `.json` 변경
검토 한계: source tree와 RSPress root/base 설정을 확인했다. 이번 검토에서 build·browser는 실행하지 않았다.

## 판정

**PASS 29 / PARTIAL 1 / FAIL 0.** 바뀐 정확한 `.json` 파일은 현재 `origin/main`에 tracked 상태로 존재하고, 동일 basename의 `.html` 및 `.md` source는 없다. 기존 `.html` href는 존재하지 않는 source를 가리켰다. 수정된 주소는 JSON 근거 경로와 맞는다. 유일한 PARTIAL은 수정된 href가 이번 변경으로 만들어진 현재 site output 또는 배포본에서 실제 열리는지는 build/browser 없이 확인하지 않았다는 점이다.

## 적대적 검토 30개

| # | 독립 실패 관점 | 결과와 근거 |
|---:|---|---|
| 1 | 수정이 계획된 roadmap 링크 한 곳에 한정됐는가 | **PASS** — 검토 대상 source diff는 `spec/public/roadmap.html`의 1 insertion/1 deletion이다. 이 표는 별도 review artifact다. |
| 2 | 수정이 PR #526 항목이 아닌 다른 항목의 link를 바꿨는가 | **PASS** — 문맥은 `2026-10-23 C11.4 cyclic minmax(calc()) max-content 혼합 text/image span` 항목이다. |
| 3 | 변경 전 href가 보고서에서 정확히 식별됐는가 | **PASS** — `origin/main`의 href는 `/spinon/internal/evidence/c11-4-cyclic-minmax-max-content-mixed-text-image-span-runtime-2026-10-23/pr526-agent-browser-verification.html`이다. |
| 4 | 변경 후 href가 파일 경로·basename을 실수로 바꿨는가 | **PASS** — 같은 디렉터리와 basename을 유지하고 확장자만 `.json`으로 바꾼다. |
| 5 | 변경된 정확한 JSON source가 current main에 없는가 | **PASS** — `origin/main` tree에 `.../pr526-agent-browser-verification.json`이 tracked entry로 있다. |
| 6 | 변경된 JSON source가 worktree에만 우연히 남아 있는가 | **PASS** — local path도 존재하고 `git cat-file -e origin/main:<path>`가 성공했다. |
| 7 | source 확인 기준이 낡은 local main ref인가 | **PASS** — worktree `HEAD`와 `origin/main`은 동일 SHA `8247a12b94d6e5d75475c16977fa1f11ed983a42`다. |
| 8 | 경로 대소문자가 파일시스템에 맞지 않는가 | **PASS** — 모든 directory와 basename의 대소문자가 tracked JSON path와 일치한다. |
| 9 | 같은 basename의 다른 확장자 산출물이 링크 대상을 모호하게 하는가 | **PASS** — main tree 검색 결과 exact basename은 `.json` 하나뿐이다. |
| 10 | 잘못된 기존 `.html` source가 여전히 존재하는가 | **PASS** — `origin/main`에 `pr526-agent-browser-verification.html`은 없다. |
| 11 | `.html` source가 worktree에서만 별도로 생성되어 혼동되는가 | **PASS** — 동일 경로의 `.html`은 현재 worktree에도 없다. |
| 12 | `.md` page가 있어 RSPress가 `.html`로 생성할 대상인가 | **PASS** — exact basename의 `.md` source도 main tree에 없다. |
| 13 | `.json` 확장자와 실제 payload 형식이 맞는가 | **PASS** — target은 agent-browser metadata object와 `page`, `images`, `cleanup` field를 담은 JSON 문서다. |
| 14 | JSON이 엉뚱한 PR의 검증 기록인가 | **PASS** — `page.url`은 `https://github.com/ohah/spinon/pull/526`을 가리킨다. |
| 15 | 페이지 제목이 다른 PR을 식별하는가 | **PASS** — title은 `Pull Request #526`과 cyclic max-content mixed text/image span 주제를 명시한다. |
| 16 | 검증 도구 provenance가 다른 browser 도구로 잘못 표시되는가 | **PASS** — payload의 `tool`은 `agent-browser`다. |
| 17 | browser 버전 정보가 target JSON에서 빠져 근거 식별이 불가능한가 | **PASS** — `version`은 `0.31.1`로 기록돼 있다. |
| 18 | PR 본문의 요구 요소 확인 값이 다른 scope를 가리키는가 | **PASS** — `bodyHasSimulatorHeading` 및 `bodyHasScopeLimits`가 JSON에 기록돼 있다. |
| 19 | roadmap의 “PR 이미지” 링크가 이미지가 아닌 unrelated metadata인가 | **PASS** — JSON은 이미지 검증 항목과 화면 파일명을 함께 기록하는 Agent Browser 확인 산출물이다. |
| 20 | 이미지 개수가 roadmap이 연결하려는 근거와 어긋나는가 | **PASS** — JSON의 `imageCount`는 4이며 `images` 배열에도 4개 항목이 있다. |
| 21 | 이미지가 로드되지 않았는데 검증 성공으로 표시되는가 | **PASS** — 각 항목은 `complete: true`, `rendered: true`다. |
| 22 | AVD/iOS 이미지 치수가 비어 있거나 잘못된 장면을 가리키는가 | **PASS** — 두 Android 항목은 1080×2400, 두 iOS 항목은 1206×2622 치수를 기록한다. |
| 23 | JSON이 참조하는 screenshot이 다른 directory로 해석되는가 | **PASS** — screenshot basename은 `pr526-agent-browser-images.png`로 같은 evidence directory 기준이다. |
| 24 | 참조 screenshot 파일 자체가 빠졌는가 | **PASS** — 해당 PNG는 worktree와 `origin/main`에 존재한다. |
| 25 | JSON의 screenshot hash가 실제 PNG bytes와 맞지 않는가 | **PASS** — 저장된 SHA-256 `89ecfeecc6afd6719639948679d9e3da8dbac628e223d19193b89de966f68be6`과 파일 계산값이 일치한다. |
| 26 | browser cleanup 값이 세션 종료를 주장하지 않거나 모순되는가 | **PASS** — cleanup 결과는 `Browser closed`, `pageCount: 0`을 기록한다. |
| 27 | cleanup metadata가 마지막 확인 때 browser process가 남았다고 기록하는가 | **PASS** — `agentBrowserProcessPresentAtFinalCheck`는 `false`다. |
| 28 | terminal browser를 agent-browser 근거와 혼동하는가 | **PASS** — JSON은 `terminalBrowserProcessPresentAtFinalCheck: false`도 기록해 process 경계를 구분한다. |
| 29 | href가 현재 docs base prefix와 불일치하는가 | **PASS** — RSPress config의 기본 base는 `/spinon/`; absolute href도 같은 prefix로 시작한다. |
| 30 | 현재 생성 site 또는 배포본에서 새 `.json` URL 응답이 확인됐다고 과장하는가 | **PARTIAL** — `spec/` root와 tracked source path는 확인했지만 이 변경으로 build/browser를 실행하지 않아 실제 생성·배포 route 응답은 입증하지 않았다. |

## 결과와 한계

- 기존 `.html` source는 없고, 정확한 `.json` source는 현재 main에 있다. 같은 basename의 `.md` source도 없어 잘못된 HTML 변환 경로는 발견되지 않았다.
- JSON이 참조한 `pr526-agent-browser-images.png`도 존재하며 기록된 SHA-256과 일치한다.
- RSPress 설정은 문서 root `spec/`, 기본 base `/spinon/`이다. 따라서 source 경로와 href의 prefix는 정합적이나, 실제 생성물·live route는 이번에 확인하지 않았다.
- roadmap source는 수정하지 않았다. 새로 쓴 것은 이 검토 보고서뿐이며, build/browser는 실행하지 않았다.
