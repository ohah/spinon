# C11.4 ASCII letter-word 상태·로드맵 PR diff 적대 검토 30개

기준: docs sync candidate의 `spec/STATUS.md`, `spec/public/roadmap.html`, 내부 계약 0059, C11.4 parent plan, letter-word plan 변경. Base `main`은 PR #1208 merge SHA `3f022b175dfb689cf721ef407a0d86b05ea96db1`이다. 이번 문서 변경은 완료 수를 올리지 않으며 구현 변경을 포함하지 않는다.

| # | 독립 실패 관점 | 판정 | 확인 |
|---:|---|---|---|
| 1 | 공식 상태 수가 변하지 않았는가 | PASS | 상태 대장과 로드맵 모두 78/214를 유지한다. |
| 2 | C11 상위 체크를 실수로 완료 처리하지 않았는가 | PASS | C11 상위 `- [ ]`를 유지했다. |
| 3 | C11.4를 전체 완료로 과장하지 않았는가 | PASS | 각 기록이 제한 slice이며 C11.4/C11 미완료라고 명시한다. |
| 4 | PR 병합 상태와 merge SHA가 실제 GitHub 상태와 맞는가 | PASS | PR #1208은 MERGED, merge SHA `3f022b175dfb689cf721ef407a0d86b05ea96db1`이다. |
| 5 | 병합 시각을 UTC로 표기했는가 | PASS | GitHub의 `2026-11-06T05:32:14Z`를 사용한다. |
| 6 | ASCII token 허용 문법이 과도하게 넓어지지 않았는가 | PASS | `[A-Za-z]+`만 허용된다고 기록한다. |
| 7 | normal whitespace 구분자가 정확한가 | PASS | SPACE/TAB/LF/CR 네 종류만 기록한다. |
| 8 | 원문·Text node identity 보존 계약을 누락하지 않았는가 | PASS | 상태와 계약 문서에 source/identity preservation을 썼다. |
| 9 | 지원 밖 입력 fail-closed 경계를 유지했는가 | PASS | 미지원 입력은 거부된다고 쓰고 수치·구두점·Unicode 제한을 계약에 보존했다. |
| 10 | 실제 route가 기본 parsed Root HTML인지 맞는가 | PASS | 문서가 default Root HTML runtime 경로로 범위를 한정한다. |
| 11 | 일반 CSS 또는 공개 CSS 지원으로 승격하지 않았는가 | PASS | 공개 CSS admission을 포함하지 않는다고 유지한다. |
| 12 | Chrome 관측과 독립 기대값을 구분했는가 | PASS | Chrome 156 독립 기대값 matrix로 구분해 기록했다. |
| 13 | Style-to-layout 결과 수가 evidence와 같은가 | PASS | 10/10으로 일치한다. |
| 14 | RuntimeSession 결과 수가 evidence와 같은가 | PASS | 14/14로 일치한다. |
| 15 | fixture source/trace 결과 수가 evidence와 같은가 | PASS | 2/2로 일치한다. |
| 16 | 전체 crate test가 통과했다고 과장하지 않았는가 | PASS | focused suites만 기록하고 broad suite green 주장은 없다. |
| 17 | Android 환경을 실기기로 오인할 수 있는가 | PASS | API 37 ARM64 AVD 및 ANGLE/SwiftShader로 한정한다. |
| 18 | iOS surface 표본을 compositor 이후 결과로 오인할 수 있는가 | PASS | Metal drawable과 compositor 제출 전 한계를 명시한다. |
| 19 | 표본 좌표와 RGBA가 evidence와 맞는가 | PASS | CSS (42,19), 지정 surface sample, `[91,168,245,255]`를 맞췄다. |
| 20 | mobile app log만으로 전체 frame matrix를 주장하는가 | PASS | mobile log가 전체 geometry를 대신하지 않는다는 한계를 유지한다. |
| 21 | fixture adapter 기본 산출물 누락 검증이 기록됐는가 | PASS | Default/fixture boundary 및 positive control을 기록한다. |
| 22 | PR CI 결과를 green으로 오인하게 쓰지 않았는가 | PASS | GitHub check 미보고를 명시한다. |
| 23 | Markdown 상대 링크가 STATUS 위치에서 유효한가 | PASS | plan/evidence/internal 계약 링크의 기준 디렉터리를 대조했다. |
| 24 | PR 링크가 올바른 저장소와 번호를 가리키는가 | PASS | ohah/spinon/pull/1208로 고정했다. |
| 25 | 내부 계약 버전을 불필요하게 올렸는가 | PASS | 0059는 0.1.0으로 유지한다. |
| 26 | 상위 track-sizing phase gate를 바꿔 C11.4를 완료 처리했는가 | PASS | 중앙 계획은 PR 사실만 갱신하고 stage completion gate는 그대로다. |
| 27 | 집중 계획의 병합 상태가 STATUS와 일치하는가 | PASS | PR #1208 merged와 같은 SHA/date를 기록했다. |
| 28 | 공개 HTML 카드가 이전 C11.4 기록을 덮어쓰는가 | PASS | 기존 hero cards는 유지하고 #1208 최신 카드를 추가했다. |
| 29 | 공개 HTML의 실행 화면 링크가 mutable branch를 가리키는가 | PASS | Android/iOS raw screenshot 링크를 immutable merge SHA에 고정했다. |
| 30 | Tailnet과 공식 저장소 상태를 혼동하거나 Pages를 배포하는가 | PASS | 이 PR은 source 문서만 갱신하며 Tailnet은 merge 뒤 별도 hash 검증하고 Pages workflow는 실행하지 않는다. |

**결과:** 30/30 PASS. GitHub PR diff가 이 candidate diff와 동일한지 merge 전 재대조한다.
