# C14.2 PR 재기반 적대 검토 · 2026-10-14

대상은 부모 PR #155 병합 뒤 최신 `main`에 재기반한 [PR #156](https://github.com/ohah/spinon/pull/156)이다. 원격 branch `feat/c14-2-image-ratio-sizing`의 관찰 head는 `3042b7f68c63710fe53385322af0e0657a44b3c6`, base는 `main`이었다. 계획, 구현, 최초 PR 검토에서 사용한 sizing 관점을 반복하지 않고, rebase topology, 문서 조정, GitHub 통합 상태와 merge gate에 대해 별도 20개를 확인했다.

| # | 공격 관점 | 판정과 근거 |
|---:|---|---|
| 1 | PR base가 아직 부모 feature branch라서 병합 때 parent diff가 누락될 수 있다 | PASS — GitHub PR base가 `main`이고 최신 원격 head `3042b7f`를 가리킨다. |
| 2 | 재기반 target이 PR #155의 merge 결과보다 뒤일 수 있다 | PASS — `origin/main`은 PR #155 merge commit `4d97add`이며 C14.2 변경은 그 위에 있다. |
| 3 | 최초 stacked PR의 부모 U05 commit이 C14.2 변경에 중복 포함될 수 있다 | PASS — `git diff origin/main...HEAD`에 U05 decoder/source 파일이 없고 변경 범위는 C14.2 layout·계약 참조·계획·근거다. |
| 4 | rebase 과정에서 C14.2 feature commit 중 하나가 사라지거나 중복될 수 있다 | PASS — 두 C14.2 commit을 main 위에서 확인했고 별도 검토/검증 문서 commit은 그 뒤에 선형으로 놓였다. |
| 5 | STATUS 충돌 해결이 C14.1 최신 내용을 되돌릴 수 있다 | PASS — C14.1은 main의 PNG 계약 0061 참조를 유지하고 C14.2 행만 선택했다. |
| 6 | 내부 계약 번호 충돌 0060이 재기반 결과에 다시 들어올 수 있다 | PASS — U05 PNG 계약은 0061이며 `0060-u05` 또는 C14.1의 구 계약 참조가 diff에 없다. |
| 7 | C14.2 계획과 internal index가 parent branch를 여전히 현재 base로 표시할 수 있다 | PASS — 계획은 최신 main 재기반 및 현재 검증을 설명하고 index는 PR #155 병합 이후를 기록한다. |
| 8 | STATUS가 최신 main 재기반 workspace 결과를 예전 742 count와 혼동할 수 있다 | PASS — C14.2 행은 773 passed, 0 failed, 4 ignored 및 해당 새 log 경로를 명시한다. |
| 9 | 구현 검증 README가 이전 stacked 상태를 계속 주장할 수 있다 | PASS — rebase 후 새 test/Clippy/rustfmt log와 기존 AVD 범위를 구분하고 C14 미완료를 유지한다. |
| 10 | full workspace test log가 부분 로그이거나 최종 suite를 누락할 수 있다 | PASS — 저장한 전체 log의 모든 suite 결과 합계는 773 passed, 0 failed, 4 ignored다. |
| 11 | Clippy warning이 `-D warnings` 밖에서 누락될 수 있다 | PASS — `cargo clippy --locked --workspace --all-targets -- -D warnings`가 exit 0으로 끝났다. |
| 12 | rustfmt나 merge diff 공백 오류가 rebase 뒤 남을 수 있다 | PASS — `cargo fmt --all -- --check` 및 `git diff --check`가 통과했다. |
| 13 | Cargo dependency/lockfile 변경이 기능 검토와 무관하게 섞일 수 있다 | PASS — PR diff에 Cargo manifest와 lockfile 변경이 없다. |
| 14 | 과거 Android 결과를 새 rebase에서 다시 실행한 것으로 잘못 쓸 수 있다 | PASS — rebase 후 AVD/실기기를 다시 실행하지 않았다고 evidence README에 구분했다. 기존 AVD 자료는 ARM64 Rust executable만 증명한다. |
| 15 | Chrome 화면을 Spinon 앱 runtime 또는 GPU 출력으로 오해할 수 있다 | PASS — PR 본문은 캡처를 Chrome 155 browser oracle 비교 자료로 설명하고 앱·GPU 실행 주장으로 쓰지 않는다. |
| 16 | 이미지 링크가 GitHub PR에서 실제로 렌더링되는지 확인되지 않을 수 있다 | PASS — agent-browser로 PR 화면을 열어 본문 캡처의 로드 상태와 natural dimensions를 확인했다. |
| 17 | 캡처가 PR 설명에서 빠져 사용자가 동작 사례를 볼 수 없을 수 있다 | PASS — Chrome 155 비교 화면을 PR 본문에 inline image로 넣고 로컬 fixture 근거 README와 연결했다. |
| 18 | repository label 또는 draft 상태가 통합 전에 틀릴 수 있다 | PASS — PR은 ready 상태이며 `enhancement`, `area: layout`, `area: testing`, `area: docs` 라벨을 갖는다. |
| 19 | 보호된 main이 요구하는 CI/review gate가 충족되지 않았을 수 있다 | PASS — branch protection은 required status check를 선언하지 않고 approving review count는 0이다. 현재 PR check rollup은 비어 있으며 mergeability는 최종 병합 직전 다시 조회한다. |
| 20 | rebase 병합이 프로젝트의 선형 history 계약을 어길 수 있다 | PASS — main은 linear history를 요구하고 PR 병합 방식은 rebase로 확인한다. |

재기반 경계 20개를 확인했고 미해결 코드 P1/P2를 발견하지 못했다. GitHub의 최신 mergeability는 이 문서 작성 뒤 병합 직전에 다시 조회한다. C14.2는 제한 구현 후보다. C14.1, 상위 C14, async image lifecycle, 앱 runtime, GPU/실기기 동작과 WPT 범위는 완료로 표시하지 않는다.
