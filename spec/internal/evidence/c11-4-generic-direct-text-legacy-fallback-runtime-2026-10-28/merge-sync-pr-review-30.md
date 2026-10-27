# C11.4 legacy fallback merge-sync PR diff 검토 30개

**검토 시점:** 2026-10-28

**대상:** [PR #707](https://github.com/ohah/spinon/pull/707), PR #706 병합 정보와 기존 13-scene partial 결과를 `STATUS.md`, 집중 계획, 상위 계획, evidence 및 공개 roadmap에 반영하는 문서 변경

**비교 기준:** 실제 GitHub PR 상태·merge SHA와 `origin/main`의 공식 집계 및 C11.4 checkbox

## 검토 결과

| # | 실패 관점 | 결과 | 확인 |
| ---: | --- | --- | --- |
| 1 | 잘못된 merge SHA를 공식 문서에 넣는가 | PASS | `gh pr view 706`의 merge commit `440359c89593047aae5ab8914573568a5a037568`와 일치한다. |
| 2 | UTC merge time을 로컬 시간으로 잘못 기록하는가 | PASS | GitHub `mergedAt=2026-10-27T18:17:27Z`를 그대로 사용한다. |
| 3 | 실제 적용한 merge strategy를 잘못 표기하는가 | PASS | PR은 `Rebase and merge`로 병합됐고 문서도 같은 방식으로 쓴다. |
| 4 | PR 번호나 링크가 다른 작업을 가리키는가 | PASS | 모든 새 링크가 구현 PR #706을 가리킨다. |
| 5 | PR이 아직 open인데 merge로 잘못 표기되는가 | PASS | GitHub API 관찰 결과 state는 `MERGED`다. |
| 6 | 원래 head SHA를 merge SHA처럼 기록하는가 | PASS | source/test commit과 실제 rebase merge SHA를 별도로 둔다. |
| 7 | 빈 GitHub check rollup을 CI pass로 확대하는가 | PASS | 병합 시 checks가 보고되지 않았다고 기록하고 CI 통과 주장은 하지 않는다. |
| 8 | Rust 결과가 21/21과 다르게 기록되는가 | PASS | focused test log는 21 passed, 0 failed를 기록한다. |
| 9 | 앱 matrix 결과가 Android/iOS 각 13/13과 다르게 기록되는가 | PASS | pass03 post-run verification과 README가 두 플랫폼 모두 13/13을 기록한다. |
| 10 | 제한된 legacy fallback slice를 C11.4 전체 완료로 승격하는가 | PASS | 문서 모두 제한된 test/evidence slice라고 설명한다. |
| 11 | 공식 완료 집계가 실수로 변경되는가 | PASS | `STATUS.md`, plan, public roadmap은 계속 `72/213`을 표시한다. |
| 12 | C11.4 checkbox를 잘못 체크하는가 | PASS | 공식 `- [ ] C11.4 intrinsic track sizing` 상태가 유지된다. |
| 13 | 계획 PR #705와 구현 PR #706 역할을 혼동하는가 | PASS | #705는 계획 병합, #706은 runtime evidence/test 병합으로 구분한다. |
| 14 | parent plan이 새 후속 기록을 누락하거나 중복하는가 | PASS | 상위 C11.4 계획에 한 개 closure 절과 #706 링크만 남는다. |
| 15 | 공식 상태 대장에 같은 10/28 결과가 두 번 생기는가 | PASS | 기존 단일 C11.4 sub-bullet를 찾아 merge metadata만 갱신했다. |
| 16 | 상태 대장의 실행 한계가 merge sync에서 사라지는가 | PASS | glyph paint, 실기기, hardware GPU, 전체 WPT/C11.4 미검증 범위를 유지한다. |
| 17 | 공개 roadmap이 다른 PR state나 이전 merge SHA를 보여주는가 | PASS | HTML note의 PR #706 link, timestamp, SHA가 GitHub 결과와 맞는다. |
| 18 | HTML 안에서 URL·SHA markup이 깨지거나 잘못 escape 되는가 | PASS | PR link는 `a`, SHA는 `code` element로 표기한다. |
| 19 | implementation source commit을 branch merge commit으로 오인하는가 | PASS | app runtime source `5df6da56`와 docs PR의 최종 merge SHA를 다른 역할로 설명한다. |
| 20 | pass03이 후속 Rust negative tests 뒤 빌드됐다고 과장하는가 | PASS | pass03 source가 `5df6da56`이고 `7af1322f`는 Rust test만 바꾼 뒤라는 제한을 유지한다. |
| 21 | 제품 layout/FFI/native matcher가 수정된 것처럼 보이는가 | PASS | PR #706 diff 및 evidence는 test, runner, docs, artifacts 범위로 기술한다. |
| 22 | strict workspace Clippy failure를 숨기거나 범위를 바꾸는가 | PASS | 수정하지 않은 `validation.rs:329`의 기존 `type_complexity` warning이 명시돼 있다. |
| 23 | feature-specific Clippy 예외를 workspace 전체 통과로 과장하는가 | PASS | 단 하나의 기존 warning을 허용한 `spinon-runtime` command만 통과로 설명한다. |
| 24 | Android/iOS app build 결과를 실제 simulator run과 혼동하는가 | PASS | build와 runtime 13-scene matrix 결과를 별도 항목으로 유지한다. |
| 25 | AVD·Simulator 종류나 ABI/model이 잘못 옮겨지는가 | PASS | Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator를 유지한다. |
| 26 | screenshots를 glyph paint 또는 final pixel equality 증거로 표현하는가 | PASS | box/surface screenshot 한계와 0 telemetry를 유지한다. |
| 27 | negative controls 또는 runner correction이 merge summary에서 빠지는가 | PASS | unknown IDs/text, extra item, wrong track과 pass01→pass03 보정 기록을 보존한다. |
| 28 | 새 계획·evidence 링크가 저장소 내부 경로를 벗어나거나 누락되는가 | PASS | plan, README, implementation review, PR diff review 경로를 확인했다. |
| 29 | 문서 동기화가 아직 하지 않은 Tailnet 배포를 완료로 주장하는가 | PASS | 변경은 source docs 상태만 기록하며 Tailnet refresh 여부를 완료로 표현하지 않는다. |
| 30 | 문서 변경이 unrelated local untracked evidence를 stage하거나 삭제하는가 | PASS | stage 대상은 merge SHA 관련 다섯 문서와 이 전용 review file로 한정한다. |

## 결론

차단 결함은 없다. PR #706의 실제 merge commit/time, 빈 check rollup, 미완료 C11.4 상태와 `72/213`이 서로 일치한다. 변경은 문서 상태 동기화이며 API 계약이나 제품 동작을 바꾸지 않는다.
