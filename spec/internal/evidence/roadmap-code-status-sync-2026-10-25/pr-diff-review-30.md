# 로드맵·코드 상태 동기화 문서 diff 검토 30관점

## 검토 범위와 판정

검토 대상은 `HEAD 0ccec669e5506b6caef53314702c7fb60090e45c` 대비 현재 작업 트리의 네 문서 diff다.

- `plan/c11-3-3-unknown-named-area-implicit-lines.md`
- `spec/STATUS.md`
- `spec/internal/0059-c11-grid-layout.md`
- `spec/public/roadmap.html`

PR #393의 상태·병합시각·merge SHA·base/head·check rollup은 GitHub CLI read-only metadata로 대조했고, merge commit이 현재 HEAD의 조상임을 확인했다. `git diff --check`는 통과했다. 코드, 빌드, 테스트, 시뮬레이터, 브라우저, Tailnet은 이 검토에서 실행하지 않았다. 단, 화면 링크의 HTTP 응답은 저장소 소유자가 별도 확인한 결과를 전달받았다.

**차단 finding은 없다.** 아래 30개 failure view 중 29개는 통과했다. 비차단 기록 사항은 §3에 적었다.

## 30개 failure view

| # | 실패 관점 | 결과와 근거 |
|---:|---|---|
| 1 | 다른 PR의 병합 사실을 C11.3.3에 잘못 붙였는가 | **통과.** GitHub PR #393 제목은 `C11.3.3 trailing implicit named line` 후보이며 네 문서의 설명과 맞는다. 계획 3행, 상태 대장 262행, 계약 0059 3·83행, 로드맵 131행을 대조했다. |
| 2 | PR base/head가 문서의 주장과 다른가 | **통과.** PR base는 `main`, head는 `feat/c11-3-3-unknown-area-implicit-tracks-20261020`이다. |
| 3 | PR이 실제로 병합되지 않았는데 병합으로 기록했는가 | **통과.** GitHub 상태는 `MERGED`이며 병합 방식은 Rebase and merge다. |
| 4 | 병합 timestamp가 잘못됐는가 | **통과.** GitHub `mergedAt`은 `2026-10-20T15:47:50Z`; 계획·상태 대장·계약·로드맵에 적힌 시각과 일치한다. |
| 5 | `2026-10-21` 문서 날짜와 UTC 병합 날짜가 충돌하는가 | **통과.** `15:47:50Z`는 한국 시간 `2026-10-21 00:47:50`이므로 21일 표기와 양립한다. |
| 6 | merge SHA를 PR head SHA나 다른 커밋과 혼동했는가 | **통과.** merge SHA `12dc2669814c4d5e7d0e2aa59d5196fdab98bb68`가 GitHub metadata와 문서에 일치한다. PR head SHA `95a5b36eca80e631365b6d9cf69ec856a72150fe`와 구분되어 있다. |
| 7 | merge SHA가 현재 통합 이력에 실제 존재하는가 | **통과.** `git merge-base --is-ancestor <merge-sha> HEAD`가 성공했고 `git show`가 같은 merge commit을 반환했다. |
| 8 | 체크 실행 결과가 있는데 rollup을 비었다고 적었는가 | **통과.** PR metadata의 `statusCheckRollup`은 빈 배열이며 `gh pr checks 393`은 checks가 보고되지 않았다고 반환했다. |
| 9 | 체크가 보고되지 않은 사실을 branch protection 설정 부재로 확대했는가 | **통과.** 신규 문구는 “필수 checks는 보고되지 않았다”고 한정한다. required-check 구성이 없었다고 새로 주장하지 않는다. |
| 10 | 빈 rollup을 CI 성공으로 오해하게 쓰였는가 | **통과.** 계획은 CI 통과로 기록하지 않는다고 명시한다. 계약과 상태 대장도 check가 보고되지 않았다고 구분하며 성공 주장을 하지 않는다. |
| 11 | 상태 요약에서 PR 추적 링크가 사라졌는가 | **통과, 수정 재확인.** 최신 diff에 계획 3행, `STATUS.md` 262행, 로드맵 131행의 명시적 PR #393 링크가 복구됐다. 계약 0059 83행에도 PR 링크가 있다. |
| 12 | 계획의 상태·상위 문서 링크가 끊겼는가 | **통과.** 계획 4행의 상대 링크는 C11.3 계획, STATUS anchor, 계약 0059에 연결된다. |
| 13 | 상태 대장의 후속 실행 근거 링크가 없는 파일을 가리키는가 | **통과.** `STATUS.md` 262행의 `c11-3-3-unknown-area-implicit-lines-followup-2026-10-21/README.md`가 존재한다. |
| 14 | 계약의 실패 관점 검토 링크가 없는 파일을 가리키는가 | **통과.** 0059 83행의 PR diff review 30개 파일과 후속 README가 존재한다. |
| 15 | 로드맵 직접 화면 링크가 잘못됐는가 | **통과.** 대상 PNG가 저장소의 `spec/internal/evidence/...` 경로에 있다. 저장소 소유자가 전달한 별도 HTTP 확인은 `200 image/png`였고, 본 검토자는 브라우저/Tailnet을 사용하지 않았다. |
| 16 | 계획 상단이 제한 후보를 공식 완료나 공개 CSS 지원으로 승격했는가 | **통과.** 계획 3행은 fixture와 문서가 공개 CSS 지원 선언이나 C11.3.3 완료 근거가 아니라고 남긴다. |
| 17 | 이전 테스트 결과를 현재 `origin/main` 실행으로 오인시키는가 | **통과, 수정 재확인.** 계획 76행은 기존의 “현재 origin/main” 및 “PR/merge 전” 표현을 제거하고 병합 전 후보 snapshot에 귀속했다. |
| 18 | Rust workspace 결과의 수치가 evidence와 다른가 | **통과.** 계획 76행의 `1,193 passed·4 ignored`는 후속 README 19행의 `1,193 passed·4 ignored·0 failed`와 일치한다. |
| 19 | CSS reference·fixture FFI 수치를 뒤섞었는가 | **통과.** 계획 76행은 CSS reference `24/24`와 C11.3.3 fixture FFI `4/4`를 별도 결과로 기록한다. |
| 20 | Clippy/rustfmt 결과를 workspace test 결과로 잘못 합쳤는가 | **통과.** 계획 76행은 이 도구들이 통과했다고 별도로 적고, evidence README 19행도 같은 경계를 유지한다. |
| 21 | 실패한 전체 `bun run test`를 성공으로 표시했는가 | **통과.** 계획 76행과 README 19행은 C04, C11.3.2, C13.3의 네 기존 source expectation 실패를 명시하며 전체 suite 통과로 보고하지 않는다. |
| 22 | 실행되지 않은 benchmark를 완료로 기록했는가 | **통과.** 두 문서는 CSS-reference 이후 benchmark 단계가 실행되지 않았다고 명시한다. |
| 23 | 병합 전 결과의 대상 소스 범위가 현재 main으로 다시 확대됐는가 | **부분, 비차단 추적성 한계.** 계획 76행의 “병합 전 후보 snapshot”은 시간 범위를 올바르게 제한하지만, 연결된 README에는 해당 Rust workspace 실행의 정확한 Git source SHA가 없다. 결과 자체와 모순되는 증거는 찾지 못했다. 기록을 재현 가능하게 만들려면 보존된 실행의 source SHA를 함께 적는 것이 가장 작다. |
| 24 | C11.3.3 checkbox가 완료로 바뀌었는가 | **통과.** `STATUS.md` 261행은 `[ ]`이고 변경된 262행은 하위 evidence 설명만 갱신한다. |
| 25 | parent C11 checkbox가 완료로 바뀌었는가 | **통과.** `STATUS.md` 259행의 C11은 계속 `[ ]`다. |
| 26 | official `72/213` 집계가 증가하거나 상충하는가 | **통과.** 로드맵 131행은 `72/213`을 유지한다고 명시한다. STATUS checkbox는 바뀌지 않았고 roadmap의 상태 fetch 로직도 diff 대상이 아니다. |
| 27 | 제한 구현의 named-line admission이 일반 숫자 입력으로 넓어졌는가 | **통과.** STATUS 262행과 로드맵 131행은 미구현 area 이름의 양의 trailing named-line fallback으로 범위를 한정하고 일반 numeric·leading·negative 경계를 미완료로 둔다. |
| 28 | named span 또는 implicit track 조건을 과장했는가 | **통과.** 계획 15–20·72행과 0059 79–81행은 named-span 부족분의 trailing 확장과 양 축 고정 px track 조건을 설명하며 `auto`/intrinsic sizing은 거부로 남긴다. |
| 29 | simulator evidence를 WPT·전체 Grid·기기 전반의 증거로 확대한가 | **통과.** 계획 3·76행, 0059 83행, STATUS 262행, 로드맵 131행은 WPT 및 전체 CSS/Grid 지원을 미완료로 두고 기기·compositor 제한을 유지한다. README 12–15행의 26 scene·65 node, 19/5/2 결과, 61 visible frame, 0 CSS px, overflow sample 3/3과 연결된다. |
| 30 | 문서 간 중복/모순 또는 renderer 한계 누락이 새로 들어왔는가 | **통과, 비차단 편집 메모 있음.** 로드맵·계획·STATUS·0059의 병합 상태와 제한 범위는 서로 일치한다. 0059 3행은 `Spinon WPT 적합성`을 미완료 목록에 두 번 포함하지만 이 중복은 기존 문구이며 이번 diff에서 생기지 않았다. Android SwiftShader와 iOS compositor 전 readback 한계는 계획 74행·README 12–16행 및 0059 83행에 보존되어 있다. |

## Finding과 필요한 수정

- **차단 finding: 없음.** 병합 사실, 날짜/SHA, check 부재 표현, `72/213`, 미완료 checkbox, 제한 구현 경계는 최신 diff에서 근거와 맞는다.
- **비차단 추적성:** 계획 76행의 역사적 테스트 결과는 “병합 전 후보 snapshot”으로 적절히 한정됐지만 exact source SHA가 근거 README에 없다. 당시 실행이 확인되는 경우에만 source SHA를 덧붙인다.
- **기존 편집 중복:** `spec/internal/0059-c11-grid-layout.md:3`의 “Spinon WPT 적합성” 중복은 이번 diff 이전부터 존재한다. 정리한다면 반복된 한 구절만 삭제하면 된다.

PR 링크는 최초 확인 때 상태 대장과 로드맵에서 텍스트로만 남아 있었으나, 검토 중 복구됐다. 최종 확인에서 세 문서의 링크가 모두 `https://github.com/ohah/spinon/pull/393`을 가리켰다.
