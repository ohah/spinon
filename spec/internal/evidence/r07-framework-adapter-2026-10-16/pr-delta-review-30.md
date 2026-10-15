# PR #204 변경 적대 검토 30회

**대상:** PR #204, `origin/main...feat/r07-vue-svelte-host-adapter-20261016` diff와 PR 설명. 구현 검토 20개와 별도로, 상태 원본·preview 동작·저장소 통합·merge gate까지 확인했다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | R07 체크가 공식 상태 parser에서 무시되거나 중복되는가 | `R07`은 기존 상위 ID 한 번만 바뀌었고 `roadmap.html`의 status fetch가 이를 읽는다. |
| 2 | 총 작업 수가 HTML 숫자 상수로 남아 카운터가 틀리는가 | 카운터는 STATUS의 base/detail tasks를 계산한다. `taskCount` 수동 변경은 없다. |
| 3 | R07 완료 때문에 R03도 간접 완료처럼 보이는가 | R03 checkbox는 미완료로 남고 R07 문구에 FFI·제품 지원 미완료를 적었다. |
| 4 | 공식 상태 날짜가 결과보다 오래된가 | `STATUS.md` 기준 날짜를 2026-10-16으로 갱신했다. |
| 5 | `현재 제품 지원 완료: 없음`과 R07 `[x]`가 충돌하는가 | R07은 가능성 조사이며 제품 지원/API 완료가 아니라고 명시했다. 제품 지원 완료 0개와 일치한다. |
| 6 | 공개 API index에 조사 결과를 지원 API로 추가하는가 | API 변화가 없고 public API 목록은 수정하지 않았다. |
| 7 | 내부 계약 0003 초안의 버전/상태를 조사만으로 올리는가 | 0003은 `0.1.0-draft` 그대로이며 문서나 ABI 계약 변경은 없다. |
| 8 | API 없는 조사를 완료 처리하면서 산출물·판정 oracle이 없는가 | API 해당 없음, 사전 oracle, 관찰 결과, 실행 명령과 남은 한계를 연결했다. |
| 9 | roadmap task 문구가 FFI/Svelte custom renderer 통합을 이미 제공한다고 암시하는가 | client DOM-shaped facade를 사용했고 제품 어댑터와 FFI는 미완료로 기술했다. |
| 10 | roadmap page의 설명 버전과 STATUS 판정이 다른가 | 양쪽 모두 Vue 3.5.44/Svelte 5.57.2의 한 counter 조사 및 제품 지원 미완료를 말한다. |
| 11 | Tailnet에서 열릴 증거 경로가 오타 또는 다른 evidence 날짜를 가리키는가 | R07 STATUS 링크들이 존재하는 2026-10-16 README, JSON trace, plan, review로 연결된다. |
| 12 | README 상대 링크가 branch-root 기준으로 잘못 해석되는가 | spike package/fixtures, oracle, plan review와 review files가 저장소 경로에 존재한다. |
| 13 | SHA-256 manifest에 capture 후 바뀐 trace 값이 기록됐는가 | 현재 lock, fixtures, oracle, observation의 SHA-256을 재계산해 표와 대조했다. |
| 14 | screenshot 없음이 빠진 화면 검증으로 오해되는가 | 실제 OS/UI surface를 만들지 않은 recorder spike라 PR에 화면 근거가 없다고 설명했다. |
| 15 | 구현 전에 사전 판정 기준이 없거나 결과 뒤에 oracle을 바꾸는가 | oracle과 plan이 implementation commit보다 먼저 만들어졌고 캡처 tree와 일치한다. |
| 16 | 계획 review 20개를 구현 review에 재사용하는가 | plan review와 post-implementation review가 별도 파일이며 관점이 다르다. |
| 17 | 구현 review 20개가 실제 코드/trace를 보지 않고 계획만 반복하는가 | runtime event, listener cleanup, ownership limits, operation mapping과 test outputs를 대상으로 기록했다. |
| 18 | 이 PR의 30개 delta review가 review 횟수만 맞추는가 | 각 항목이 서로 다른 status, contract, test, preview, branch-protection 실패 경로를 확인한다. |
| 19 | nested Bun test가 root lock에 없는 임의 framework 버전을 받는가 | package-local exact versions/integrity와 `bun install --frozen-lockfile`을 확인했다. |
| 20 | Svelte Node server export가 client 통합 성공처럼 보이는가 | 테스트 명령에 browser condition이 고정됐고 capture를 반복해 byte-identical 결과를 얻었다. |
| 21 | Vue가 두 번째 render 때 이전 click callback까지 누적하는가 | 반복 click oracle은 0→1→2이며 이전 listener 제거·새 listener 등록을 trace로 확인했다. |
| 22 | Svelte SSR을 실행했지만 client host path를 검증했다고 적는가 | `generate: "client"` output과 Svelte browser runtime을 실행했다. |
| 23 | unsupported attribute/name/owner를 silently ignore하면서 R07 완료를 주장하는가 | strict recorder negative control과 no-mutation 결과를 포함했다. |
| 24 | JS fake owner token 결과를 Rust OwnerId ABI 연결이라고 확대하는가 | README와 STATUS에서 ABI 미연결이라고 분리했고 recorder string token을 실 OwnerId로 부르지 않는다. |
| 25 | fake recorder owner test가 실제 HostDocument 거부 근거를 대체하는가 | 기존 Rust owner-conflict 및 cross-owner fail-closed 테스트를 각각 실행해 1/1 통과했다. |
| 26 | no checks를 green CI로 표시하는가 | PR에 checks가 보고되지 않았고 main protection의 required checks가 null임을 확인했다. 로컬 focused tests만 pass로 기록한다. |
| 27 | PR을 merge commit/squash로 합쳐 linear history를 깨뜨리는가 | branch protection은 linear history를 요구한다. 이 저장소 규칙에 맞춰 Rebase and merge를 사용한다. |
| 28 | GitHub 라벨이 작업 종류 또는 주요 영역을 빠뜨리는가 | `spike`, `area: frameworks`, `area: docs` 라벨이 붙었다. |
| 29 | 기존 사용자 worktree의 340개 변경과 PR 변경이 섞이는가 | 새 `origin/main` worktree와 전용 branch를 사용했고 원래 checkout을 수정하지 않았다. |
| 30 | main을 읽는 Tailnet preview가 PR branch의 62/197을 미리 보여준다고 오인하는가 | preview는 main merge 뒤에 동기화한다. PR은 source page/STATUS를 갱신하고, served 62/197 검증은 merge 후 별도 기록한다. |

## 판정

30개 서로 다른 PR 변경 관점에서 검토했다. PR은 현재 mergeable이며 required status checks가 설정되어 있지 않다. 로컬 R07 adapter 2/2와 Rust ownership 2/2 검증은 이미 통과했다. merge는 Rebase and merge 후에만 수행한다.
