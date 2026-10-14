# U05.1 최신 main rebase delta 검토 · 20개 · 2026-10-14

이 검토는 PR #155의 최신 `main` rebase, 충돌 병합, 계약 ID 참조 변경과 재실행 근거를 대상으로 한다. PNG 구현 자체의 [실패 관점 20개](./u05-1-png-implementation-review-2026-10-14.md) 및 [PR 변경 관점 30개](./u05-1-png-pr-review-2026-10-14.md)와 구분한다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | branch가 최신 `main`을 포함하지 않음 | `origin/main`이 ancestor이고 PR 범위에는 U05의 두 feature commit만 남았다. |
| 2 | rebase 중 사용자 변경 유실 | rebase 전 feature worktree가 clean이었고 원본 remote head를 `backup/pr155-before-main-rebase-20261014` 로컬 ref로 보존했다. |
| 3 | C13.1 internal index 추가를 U05 변경이 덮음 | 두 충돌에서 C13.1 행과 U05.1 행을 모두 보존했다. |
| 4 | C13 PR #158의 오래된 draft 상태를 유지 | internal index에서 실제 rebase merge 상태로 고쳤다. PR metadata의 merge SHA는 `c551b0e`다. |
| 5 | 새 계약이 C13.1과 같은 ID `0060`을 재사용 | 충돌 상태에서 duplicate ID를 발견했다. C13.1의 기존 `0060`은 유지하고 U05.1은 새 `0061`로 이동했다. |
| 6 | U05 계약 파일 rename 뒤 일부 path가 사라짐 | `0060-u05-png-image-decode.md`를 `0061-u05-png-image-decode.md`로 추적 rename하고 참조를 갱신했다. |
| 7 | U05 contract heading과 index ID가 다름 | 새 파일 제목·internal index가 모두 `0061`로 일치한다. |
| 8 | 계약 숫자 version을 문서 ID 수정과 함께 변경 | contract body의 version을 `0.1.0`으로 유지한다. |
| 9 | Architecture의 old path가 남음 | `docs/architecture.md`를 새 `0061` path로 고쳤다. |
| 10 | U05 계획이 old ID를 계속 요구 | `plan/u05-1-png-decode.md`의 번호를 `0061`로 갱신했다. |
| 11 | 공식 U05 상태가 old contract를 연결 | `spec/STATUS.md` U05 항목 링크를 `0061`로 갱신했다. |
| 12 | C14.1 status가 old PNG contract를 연결 | C14.1의 PNG 계약 링크를 `0061`로 갱신했다. |
| 13 | C13 contract index를 U05 rename으로 바꿈 | C13.1 행은 `0060-c13-1-overflow-clip.md`를 그대로 가리킨다. |
| 14 | U05 구현/PR review가 old ID를 가리킴 | 두 review 문서의 계약 식별자를 `0061`로 갱신했다. |
| 15 | source patch가 rebase conflict 해결 중 달라짐 | 충돌 파일은 index/문서였고 U05 decoder/request/snapshot source는 자동 적용됐다. |
| 16 | 문서 충돌 해결이 변경 범위를 숨김 | latest-main diff는 U05 48개 파일이며 C13 main changes는 merge base에 남아 feature diff에서 빠졌다. |
| 17 | rebase 뒤 Rust regression이 실패 | full workspace log에서 760 passed, 0 failed, 4 ignored를 확인했다. |
| 18 | warning을 Clippy에서 놓침 | 최신 main에서 workspace/all-targets Clippy `-D warnings` 통과 로그를 보존했다. |
| 19 | format/diff 오류 | `cargo fmt --all -- --check`와 rebase 변경 뒤 `git diff --check`를 통과시켰다. |
| 20 | 기존 AVD 근거를 앱 동작으로 확대 | 새 ADB/AVD/실기기 실행은 하지 않았다. 이전 기록은 Rust test executable 결과이며 APK/runtime/화면 근거가 아니다. |

rebase delta에서 고친 결함은 중복 계약 ID와 stale #158 draft 문구다. 최신 main에서 full workspace test/Clippy/format은 통과했다. GitHub PR checks는 별도로 보고되지 않아 local 검증과 구분한다.
