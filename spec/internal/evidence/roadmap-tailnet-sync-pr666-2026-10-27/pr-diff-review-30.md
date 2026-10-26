# PR #667 Tailnet 선택 경로 동기화 증거 적대 검토 · 30개 관점

- 검토 브랜치: `docs/c11-4-pr666-tailnet-sync-20261027`
- 기준: merged `origin/main` / HEAD `fb3f1e5a307c9678d6b70af017858fe7e03f435a`, tree `3e7c6e643d501a70e53aae1780729c1c25c81898`
- 검토 대상: `spec/internal/evidence/roadmap-tailnet-sync-pr666-2026-10-27/`의 staged diff 21개 파일
- 대조 자료: GitHub PR #666 메타데이터와 merge commit, staged JSON/log/스크립트, 기존 selected build·Tailnet backup 파일, Agent Browser 결과와 캡처
- 방식: read-only 검토. 새 docs build나 browser를 실행하지 않았고 stage/commit하지 않았다.

## 판정

**PASS 30 / PARTIAL 0 / FAIL 0. 차단 항목은 없다.** PR #666의 merged SHA와 build source가 같고, 253개 경로의 selected build·copy 분류·backup·Tailnet 결과가 각각 경로별 byte 수와 SHA-256으로 맞는다. 세 open-handle 경로는 byte-exact로 확인된 동일 파일이라 copy가 건너뛰었다. 로드맵 및 이미지 주장은 Agent Browser 관찰 JSON, 해시가 일치하는 캡처, selected route 파일을 대조했다.

preflight의 쓰기 범위는 `noPreviewTargetWritesPerformed`로 한정됐고 README는 이를 copy 이전 단계로 설명한다. preflight가 `/tmp` 응답과 자체 증거 파일을 기록하는 것과 preview target을 변경하지 않은 주장은 구분된다. 이 검토에서는 새 문서 build·browser·app·simulator를 실행하지 않았다.

## 적대적 검토 30개

| # | 실패 관점 | 판정 및 근거 |
|---:|---|---|
| 1 | 다른 문서나 앱 코드가 staged diff에 섞인다. | **PASS** — staged diff는 지정 evidence 디렉터리 아래 21개 파일뿐이다. 앱/runtime source와 공개 roadmap/status 문서는 staged 변경에 없다. 다른 untracked 산출물은 staged 대상에 포함하지 않았다. |
| 2 | PR #666이 실제 merge되지 않았거나 다른 SHA로 merge됐다. | **PASS** — GitHub PR #666 메타데이터는 `merged=true`, merge SHA `fb3f1e5a307c9678d6b70af017858fe7e03f435a`를 반환한다. |
| 3 | 증거가 PR head SHA를 merge SHA인 것처럼 사용한다. | **PASS** — PR head `a7ff36c34ed6fec644e0f1393975311240caccdd`와 merge SHA가 구분되고, build/root/copy/verify는 merge SHA를 사용한다. |
| 4 | build의 checkout이 PR #666 merge 결과가 아니다. | **PASS** — 이 checkout의 `HEAD`와 `origin/main`은 모두 `fb3f1e5a…`; build/root/copy/verify의 source head도 같다. |
| 5 | tree manifest가 다른 source tree를 가리킨다. | **PASS** — Git commit tree, selected-root manifest, selected-build manifest, copy·verify manifest의 tree 값이 모두 `3e7c6e643d501a70e53aae1780729c1c25c81898`이다. |
| 6 | PR #666이 수정한 원인과 README 설명이 다르다. | **PASS** — merged PR diff는 runtime README 2개 및 precomparison `SHA256SUMS`와 이 PR diff review를 수정한다. README의 네 로컬 Markdown 이미지 링크에 `./`를 추가하고 checksum을 갱신했다. |
| 7 | pre-fix 로그가 Rspress module-resolution 원인을 뒷받침하지 않는다. | **PASS** — 보존된 압축 로그는 Rspress 2.0.22, `Module not found: Can't resolve`와 `./` 없는 PNG 요청을 기록한다. |
| 8 | 성공 build 로그가 metadata의 log hash와 다르거나 실패를 성공으로 적는다. | **PASS** — 압축·해제 로그 SHA-256이 build result의 두 hash와 맞고, 로그에 `success Pages rendered in 11149 ms`가 있다. result의 `exitCode`는 0이다. |
| 9 | build가 다른 Rspress 버전 또는 command를 사용했다. | **PASS** — result는 Rspress `2.0.22`와 staged selected config를 지정한 실행 command를 기록하며 raw log에도 같은 버전이 나온다. |
| 10 | selected config가 기본 문서 root인 `spec`을 그대로 build하면서 selected-root라 주장한다. | **PASS** — 기본 config는 `root: spec`; staged config는 이를 덮어써 `build/spinon/roadmap-tailnet-sync-pr666-20261027-root-pass01-selected`를 root로 사용한다. 따라서 이 결과는 기본/full docs build를 의미하지 않는다. |
| 11 | build output 또는 public assets가 의도치 않게 전체 사이트 범위로 설정됐다. | **PASS** — selected config는 별도 output root를 쓰고 `publicDir: false`, `cleanDistPath: true`를 설정한다. clean 경로도 이 run의 전용 build output이다. |
| 12 | 죽은 링크·이미지 검사가 꺼졌는데 이를 전체 문서 검증으로 과장한다. | **PASS** — config/result는 `checkDeadLinks=false`, `checkDeadImages=false`를 밝히며 README가 전체 문서·repository-wide link 검증이 아니라고 한정한다. |
| 13 | generated HTML 791개 주장이 실제 build 결과와 불일치한다. | **PASS** — result의 `generatedHtmlCount=791`이고 기존 full build output에서 실제 `.html`도 791개다. |
| 14 | 선택한 페이지 수가 791개 전체를 publish한 것처럼 보고된다. | **PASS** — stage script의 HTML allowlist는 40개이며 selected-build manifest도 `generatedRouteCount=40`, 전체 selected route는 253개다. README는 791개 모두 복사하지 않았다고 구분한다. |
| 15 | selected root의 기존 파일을 merged main과 비교하지 않고 새 source로 취급한다. | **PASS** — root manifest는 80개 tracked main source를 기록한다. prepare script가 각 파일을 `git show <merge>:<path>`와 byte-for-byte 비교한다. |
| 16 | 307 overlay alias 중 일부가 원본 source와 다른 bytes를 가진다. | **PASS** — 307개 overlay path가 고유하고, 각 실제 selected-root 파일·manifest bytes/hash·merged-main source bytes/hash가 일치한다. 합계도 6,291,567 bytes다. |
| 17 | source root 또는 route 복사에 symlink가 섞여 경로 밖 파일을 참조한다. | **PASS** — staged 파일 mode는 일반 파일이며 base-tree 복사는 symlink를 거부한다. selected root, full output, overlay 307개와 selected output 253개에서 symlink가 발견되지 않았다. copy/stage 스크립트도 source/target에 `lstat` 후 regular non-symlink를 요구한다. |
| 18 | manifest route에 절대경로, `.`/`..`, 중복 항목이 있어 traversal 또는 덮어쓰기가 생긴다. | **PASS** — 253 route는 모두 상대 경로이고 path segment에 빈 값·`.`·`..`가 없다. route path가 253개 고유하다. |
| 19 | selected-build의 개별 경로 hash가 실제 selected output과 다르거나 파일이 빠진다. | **PASS** — 실제 selected output에는 manifest와 정확히 같은 253개 파일이 있고, 각 파일 size/SHA-256가 일치하며 추가·누락·symlink 파일은 없다. |
| 20 | route별 수치가 manifest·copy·verify 사이에서 서로 다르다. | **PASS** — 253개 selected bytes 합계는 `32,309,889`; build, copy, verify의 선언 합계와 같다. |
| 21 | shared logo/roadmap SVG가 새 selected build에 없는 상태로 함께 검증됐다고 적는다. | **PASS** — 두 기존 shared asset은 별도 external-existing 목록에 있으며 build와 Tailnet 결과의 size/hash가 각각 일치한다. selected route 253개에 이 둘을 숨겨 중복 계상하지 않는다. |
| 22 | preflight 뒤 copy가 실행돼도 `no writes`의 대상이 불분명하다. | **PASS** — JSON/스크립트 필드가 `noPreviewTargetWritesPerformed`로 명시됐고 README는 preflight가 copy 전에 preview target에 쓰지 않았다고 설명한다. `/tmp` HTTP 응답·증거 파일 기록과 preview target 변경을 혼동하지 않는다. |
| 23 | preflight의 route inventory가 253개 또는 기존/신규 수와 맞지 않는다. | **PASS** — 51개 existing + 202개 missing이 selected 253개를 이룬다. preflight snapshot에는 Tailscale Running, Mac Studio online, PID 28040의 `127.0.0.1:8768` listener와 roadmap HTTP 200이 기록돼 있다. 이는 해당 pre-copy 시점의 관찰이다. |
| 24 | lsof의 세 open handle이 잘못 집계되거나 hash 비교 없이 덮어써진다. | **PASS** — CSS 1개와 JS 2개, 총 3개가 비교 JSON에서 현재/선택 bytes 및 SHA가 각각 같고 `exactMatch=true`다. |
| 25 | open-handle 경로가 copy 후 교체됐거나 skip 목록과 verify가 불일치한다. | **PASS** — 세 경로 모두 29개 identical 분류 안에 있고 `openHandleSkipped`와 verify 목록이 같다. |
| 26 | created/replaced/identical 분류가 중복·누락으로 253개를 채우지 못한다. | **PASS** — 세 집합은 경로 기준 서로 겹치지 않고 202+22+29=253이며 각 경로의 byte 수/SHA가 selected manifest와 일치한다. |
| 27 | 신규 202개가 preflight 시점부터 있던 파일이거나 missing 목록과 다르다. | **PASS** — copy의 created 경로 202개 집합이 preflight의 missingTargets 202개 집합과 정확히 같다. existing 51개도 replaced 22 + identical 29와 일치한다. |
| 28 | 교체된 기존 파일이 백업되지 않았거나 백업 bytes가 보존되지 않는다. | **PASS** — replaced 22개 각각의 backup metadata가 copy/verify와 맞으며 지정 backup root에서 실제 파일 22개를 확인했다. 전부 크기·SHA가 일치하고 symlink는 없다. |
| 29 | preview 삭제 또는 symlink target 덮어쓰기가 copy 동작에 숨어 있다. | **PASS** — preview root/조상/대상은 symlink 및 비정규 파일을 거부한다. 교체 전 backup을 만들고 임시 파일을 쓴 뒤 rename하며 preview 파일을 unlink/rm하지 않는다. manifests와 run logs의 `noDeletes=true`도 일치한다. |
| 30 | local/Tailnet/browser 결과가 selected bytes 또는 공개 문구와 연결되지 않는다. | **PASS** — verifier는 local 253/253 및 Tailnet HTTP 200·size/SHA 253/253을 기록하고, 경로별 remote hash가 selected manifest와 전부 맞는다. Agent Browser 0.31.1 관찰 JSON은 `/roadmap.html`에서 PR #664 merge SHA `bc681ad0…`, `[10,290]`, C11.4 미완료·`72/213`을 기록한다. 같은 JSON의 캡처 SHA는 staged PNG와 일치하고, 여기에 보이는 로드맵 문단도 맞다. 네 evidence image URL은 selected route에 있고 PNG IHDR dimensions도 Android `1080×2400`, iOS `1206×2622`, Chrome 두 장 `1280×633`으로 관찰값과 일치한다. README는 새 앱·simulator run을 주장하지 않는다. |

## 결론과 범위

- 검토된 수량과 경로별 hash는 build, copy, backup, local preview, Tailnet 응답 evidence 사이에서 일치한다.
- symlink는 staged 변경 및 확인한 selected root/output/backup 경로에 포함되지 않았다. preview 경로 삭제를 수행하는 copy 동작도 보이지 않는다.
- PR #666 merge 후 선택 경로를 publish한 문서 사이트 evidence다. 새 앱 실행이나 simulator 검증으로 확대 해석하지 않는다.
- 새 docs build, browser, app/simulator 실행은 하지 않았고 stage/commit도 하지 않았다.
