# C11.4 Unicode 병합 상태·로드맵 PR 변경 적대 검토 · 30개 관점

검토일: 2026-11-02
검토 범위: PR #991 병합 사실을 기록한 `spec/STATUS.md`, C11.4 계획, 공개 roadmap 카드, 선택 문서 build/배포 manifest, Tailnet 동기화 결과, Agent Browser 화면·lifecycle evidence.
기준 사실: PR #991 merge SHA `108d0eed91220f81971f8c2eb7c4d2b727b26452`; 구현 source SHA `b3dadbc6ae28712b656a5ff24157cb4939d16252`.

## 판정

**PASS 30 / PARTIAL 0 / FAIL 0.** C11/C11.4 공식 checkbox와 완료 집계 `72/213`은 그대로 두고 제한 fixture candidate의 병합 사실만 추가한다. PR check run이 없고 main required status check가 설정되어 있지 않다는 점도 성공 CI로 바꾸어 쓰지 않는다.

## 적대적 검토 30개

| # | 실패 관점 | 결과와 근거 |
|---:|---|---|
| 1 | PR merge SHA가 source commit SHA로 잘못 기록되는가 | **PASS** — 구현 source `b3dadbc6`와 Rebase and merge 결과 `108d0eed`를 서로 다른 용도로 쓴다. |
| 2 | merge timestamp가 UTC/KST 사이에서 혼동되는가 | **PASS** — GitHub `mergedAt` 원본 `2026-11-01T16:59:25Z`를 사용한다. |
| 3 | status/plan이 PR을 아직 open 상태로 표시하는가 | **PASS** — 병합 사실은 `spec/STATUS.md`와 계획 상단에 기록한다. |
| 4 | 상위 C11 checkbox가 하위 candidate merge만으로 완료되는가 | **PASS** — `- [ ] **C11 CSS Grid**`를 유지한다. |
| 5 | C11.4 intrinsic track sizing checkbox가 제한 slice 때문에 완료되는가 | **PASS** — `- [ ] **C11.4 intrinsic track sizing**`를 유지한다. |
| 6 | 공식 완료 수가 merge를 이유로 증가하는가 | **PASS** — `72/213`을 유지한다. |
| 7 | 제한 fixture candidate가 일반 CSS 지원으로 오해되는가 | **PASS** — card는 `제한 fixture candidate`라고 명시하고 일반 CSS admission을 제외한다. |
| 8 | 기존 ASCII scene 수를 Unicode scene 수에 포함해 세는가 | **PASS** — 기존 12 scene과 추가 Unicode 11 scene을 별도로 표기한다. |
| 9 | 앱 검증이 실기기나 hardware GPU 결과로 과장되는가 | **PASS** — Android는 QEMU AVD, iOS는 Simulator로 기록한다. |
| 10 | source HEAD와 simulator 설치 artifact hash 검증이 누락되는가 | **PASS** — 최신 pass05/pass06 metadata와 runtime README를 연결한다. |
| 11 | root cache scene identity fix가 증거에서 빠지는가 | **PASS** — same-scene 재시도에서만 root/node IDs를 재사용하도록 고친 경계를 카드 및 evidence에서 명시한다. |
| 12 | FFI source test 결과가 runtime matrix에 합산되는가 | **PASS** — source test 1/1과 두 platform 23/23을 별도 결과로 쓴다. |
| 13 | Chrome 156의 관측과 CSS Text 기대값이 하나로 섞이는가 | **PASS** — Chrome의 `ßeta` 보존과 Unicode titlecase `Sseta`를 차이로 분리한다. |
| 14 | 구현 범위가 임의 locale로 넓혀지는가 | **PASS** — 정확한 `lang=en` fixture로 한정하고 다른 locale을 범위 밖에 둔다. |
| 15 | 임의 Unicode/font/text 입력을 허용한다고 읽히는가 | **PASS** — 고정 source allowlist와 arbitrary input/font 제외를 기록한다. |
| 16 | inline formatting 미지원이 누락되는가 | **PASS** — inline formatting을 범위 밖으로 기록한다. |
| 17 | simulator screenshot을 glyph paint 증거로 과장하는가 | **PASS** — layout/status screenshot이며 glyph raster 증거가 아님을 유지한다. |
| 18 | full WPT 검증을 완료했다고 읽히는가 | **PASS** — full WPT를 미검증 범위로 분리한다. |
| 19 | 기존 Clippy warning이 전체 green으로 요약되는가 | **PASS** — workspace/runtime all-targets의 미수정 warning과 scoped green을 구분한다. |
| 20 | GitHub check run이 없는 상태를 CI 성공으로 기록하는가 | **PASS** — PR #991에 보고된 check run이 없다고 적는다. |
| 21 | required status check가 있는데도 누락으로 오인되는가 | **PASS** — branch protection API의 404 결과는 required status check 미설정으로 기록한다. |
| 22 | 공개 Pages workflow가 Tailnet preview 갱신과 혼동되는가 | **PASS** — 선택 route만 Tailnet 미리보기에 복사하고 Pages workflow는 실행하지 않는다. |
| 23 | roadmap 최신 activity의 순서가 구현 계획보다 뒤에 배치되는가 | **PASS** — PR #991 implementation card를 #989 계획 카드 바로 뒤에 둔다. |
| 24 | #991 candidate card가 중복 삽입되는가 | **PASS** — roadmap에는 PR #991 implementation marker가 하나만 있어야 한다는 source assertion을 둔다. |
| 25 | card의 local link가 `/spinon/` base 밖으로 나가는가 | **PASS** — PR card의 local href를 build manifest에서 추출해 exact route를 검증한다. |
| 26 | card가 가리키는 계획 route가 선택 build에서 빠지는가 | **PASS** — Unicode plan route를 selected root manifest와 publication에 요구한다. |
| 27 | 내부 계약/실행 README/review route 중 일부가 누락되는가 | **PASS** — contract, Chrome precomparison, runtime README, implementation review, PR review route를 각각 요구한다. |
| 28 | RSPress dead-link 검사를 끄고도 새 card 링크를 검증하지 않는가 | **PASS** — 전체 사이트 검증이라고 과장하지 않고 새 card local href는 별도 manifest 검사로 확인한다. |
| 29 | Tailnet 대상이 현재 Tailscale server root가 아닌 다른 경로인가 | **PASS** — `/Users/yoonhb/Library/Application Support/SpinonPreview`와 8768 preview PID를 preflight에서 확인한다. |
| 30 | 교체 중 열린 파일·기존 backup을 덮어쓰거나 preview server를 종료하는가 | **PASS** — 변경 대상 open handle을 거부하고 고유 backup·atomic per-file rename을 사용하며 route 삭제와 server 종료를 하지 않는다. |

## 결과와 한계

- 공식 상태는 C11/C11.4 미완료, `72/213`이다.
- 선택한 351 route 모두 local·Tailnet HTTPS에서 HTTP 200 및 SHA-256 일치를 확인했고, 변경 21개 backup hash도 확인했다. 8개 roadmap card local href를 별도 검증했다.
- Agent Browser에서 #991 카드와 merge SHA, 공식 `72/213`을 확인했다. desktop 1280px·mobile 390px에서 문서 가로 overflow가 없고 screenshot hash와 닫힌 browser lifecycle은 `agent-browser-verification.json`에 기록했다.
- 이 검증은 선택 route와 화면 범위에 한정한다. 전체 문서 링크 검사, 전체 사이트 dead-link 검사 또는 Pages 배포를 증명하지 않는다.
