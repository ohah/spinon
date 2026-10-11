# C12.4 계획 PR 변경 검토

검토 범위는 `docs/c12-4-stacking-plan`의 계획·내부 계약·상태 대장·인덱스·공개 로드맵이다. CSS paint 의미 자체의 계획 검토는 [별도 20개 관점 기록](./c12-4-stacking-plan-review-2026-10-11.md)에 두었으며 여기서 다시 세지 않는다.

| # | PR 변경 실패 관점 | 확인 결과와 조치 |
|---:|---|---|
| 1 | 작업 브랜치의 기준 main이 오래되어 PR에 C12.3 완료 커밋까지 중복 포함되는가 | 브랜치 시작점 `a7b212c`가 `origin/main`과 같은지 확인했다. PR 변경은 그 이후 C12.4 문서 파일로 제한한다. |
| 2 | 현재 작업이 실제 다음 roadmap item이 아닌가 | C12 master와 상태 대장에서 C12.3 다음 미완료 단계가 C12.4임을 대조했다. |
| 3 | 문서 계획 수립만으로 공식 완료 체크가 켜지는가 | `spec/STATUS.md`의 C12.4 체크를 미완료로 유지하고 기능 미구현 문구를 적었다. |
| 4 | C12 전체가 하위 단계 계획 추가로 완료 처리되는가 | C12 parent, C12.5 sticky, C10.3.5 관계를 모두 미완료/완료 경계에 맞게 유지했다. |
| 5 | 내부 계약 0058을 공개 API 또는 구현 완료 명세로 오해하는가 | 문서 상단에 제안·미구현·비공개 계약이라고 표시했다. |
| 6 | 내부 계약 ID나 crate/API 숫자 버전이 계획 때문에 올라가는가 | 다음 ID `0058`만 사용하고 계약 숫자 버전은 `0.1.0`으로 고정했다. |
| 7 | 상태 대장 링크와 계획 본문 링크가 서로 다른 경로를 가리키는가 | 계획·계약·계획 검토 경로를 status 행과 동일하게 맞췄다. |
| 8 | internal README가 새 계약을 누락하거나 plan 검토만 계약으로 오해하는가 | 0058 표 항목과 별도의 plan review evidence 항목을 각각 추가했다. |
| 9 | C12 master plan의 다음 단계가 여전히 “계획 작성”으로 남는가 | 전용 계획/계약/실패 검토 완료와 다음 Chrome 사전 비교 단계를 명시했다. |
| 10 | 공개 roadmap만 갱신하고 내부 SSOT는 그대로 두는가 | status, C12 master, 0058, internal index, review와 roadmap 원본을 함께 갱신했다. |
| 11 | 공개 roadmap에서 C12.4 기능을 구현된 것처럼 소개하는가 | 계획 수립과 미구현 상태, 다음 사전 비교를 분리해 적었다. |
| 12 | roadmap 새 링크가 맞지 않는 route 이름을 가리키는가 | `/plan/c12-4-stacking-context.html`, `/spec/internal/0058-c12-4-stacking-context.html`, 두 evidence route를 갱신한 Tailnet preview에서 agent-browser로 열어 제목·본문 응답을 확인했다. |
| 13 | 외부 URL·WPT 고정 revision 경로가 implementation test 통과 주장으로 바뀌는가 | WPT commit SHA와 후보 경로를 계획에 고정하고 WPT 실행을 아직 안 했다고 표시했다. |
| 14 | plan review와 PR 문서 동기화 review를 중복 검토로 세는가 | 서로 다른 파일과 목적을 명시했다. CSS 의미 관점은 계획 검토, SSOT·route·diff 관점은 PR 검토다. |
| 15 | 내부 문서에서 실제 저장소 상대 경로가 사라지는가 | 새 contract·plan review 링크와 변경된 internal README 상대 경로의 대상 파일 존재를 확인했다. RSPress는 sibling `plan/` 디렉터리를 포함하지 않는다. |
| 16 | 전체 docs build 실패를 이번 변경의 성공으로 숨기는가 | RSPress `2.0.22` 전체 빌드는 기존 다수 dead link와 sibling plan 경로 때문에 실패했다. 성공으로 기록하지 않고 새 contract에서도 같은 plan 링크 경계가 확인되어 PR에 남긴다. |
| 17 | RSPress 오류 때문에 GitHub Pages를 임의 배포하거나 설정을 바꾸는가 | GitHub Pages workflow·RSPress 설정·의존성 버전을 변경하지 않았다. 요청된 Tailnet preview의 기존 파일을 유지하면서 필요한 HTML·asset만 갱신했다. |
| 18 | 계획 PR에서 불필요하게 Android/iOS 앱을 다시 빌드하거나 실행했다고 주장하는가 | 이번 diff는 문서 산출물이며 native build/runtime 검증을 했다고 보고하지 않는다. 플랫폼 확인은 C12.4 구현 게이트로 남겼다. |
| 19 | 무관한 C07.2 증거나 두 Stylo spike가 staging/commit에 섞이는가 | `git status`의 untracked spike 두 디렉터리는 보존하고 PR 파일 추가 대상에서 제외한다. |
| 20 | PR 라벨·제목·본문이 저장소 규칙과 다른가 | 기존 `planning`, `area: docs`, `area: layout`, `area: renderer` 라벨을 쓰고 conventional prefix 뒤 본문은 한글로 작성한다. PR 본문은 기능 미구현·전체 docs build 실패·다음 게이트를 드러내야 한다. |

## 현재 검증 경계

- `git diff --check`는 통과했다.
- `mise exec -- bun run docs:build`는 실패했다. 로그에는 기존 링크 문제 185건과 Shiki `rust,ignore` 언어 오류가 있다. 현재 계약의 `../../plan/c12-4-stacking-context.md` 경로도 RSPress 기본 `root: spec` 바깥이다. 이 RSPress 설정에서 sibling plan 링크가 실패하는 기존 문서 경계는 확인했으며, 전체 사이트 문제가 해결됐다고 보고하지 않는다.
- 관련 계획·계약·상태·인덱스·검토 문서만 넣은 격리 입력으로 RSPress `2.0.22`를 빌드해 여섯 HTML route 생성을 확인했다. 전체 문서 사이트 빌드 통과를 의미하지 않는다.
- Tailnet preview의 roadmap, C12 계획, 계약, 계획 검토, PR 변경 검토 route를 기존 출력에 추가 반영했다. agent-browser에서 새 제목과 계획 미구현 상태를 확인했다. GitHub Pages는 배포하지 않았다.
