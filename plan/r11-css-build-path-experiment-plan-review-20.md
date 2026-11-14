# R11 모바일 CSS 변환 경로 증거 범위 확인 · 계획 실패 관점 검토

**대상:** [R11 계획](./r11-css-build-path-experiment.md)
**검토 범위:** 실행 전 계획의 독립적인 실패 경로 20개
**검토 방식:** 서로 다른 3개 관점에서 각 7개씩 총 21개를 검토하고, 중복 경로를 합쳐 초기 20개로 정리했다. 수정 뒤 재검토는 4–20번만 별도 기록됐다. 검토자는 계획 초안만 읽었고 코드·상태 대장은 수정하지 않았다.

첫 검토는 아래 초기 판정을 남겼다. 확인된 지적을 계획에 반영한 뒤, 마지막 열의 재검토가 끝나기 전까지 계획 실행을 시작하지 않는다.

| # | 공격 관점 | 초기 판정과 확인된 실패 경로 | 계획 보정 | 수정 후 재검토 |
|---:|---|---|---|---|
| 1 | R11의 조사 질문을 제품 지원으로 확대 | 발견 없음. 계획이 기존 converter 채택·지원 범위 추가를 제외한다. | 조사 전용·R11 미완료 경계를 유지한다. | 초기 통과 · 재검토 안 함 |
| 2 | R10 Taffy 결과를 CSS 변환 의미의 증거로 사용 | 발견 없음. layout consumer 결과와 CSS AST 변환 결과를 구분한다. | 두 근거의 경계를 유지한다. | 초기 통과 · 재검토 안 함 |
| 3 | R14 Tailwind 결과를 일반 CSS 또는 R11 완료로 해석 | 발견 없음. 전체 거부와 `.flex` 대조를 제한 근거로만 둔다. | R14 재사용 한계를 유지한다. | 초기 통과 · 재검토 안 함 |
| 4 | 과거 R10/R14 실행 소스가 현행 코드와 같다고 오판 | 발견. 과거 source revision에서 converter·lockfile·input blob을 복원해 비교하지 않으면 과거 결과를 현재 실행으로 혼동한다. | 과거 revision의 blob hash를 복원하고 revision이 없으면 historical source를 미확인으로 남기도록 했다. | 통과 |
| 5 | 미커밋 source change를 기준 revision 실행으로 오기록 | 발견. HEAD만으로 실제 실행 파일을 식별할 수 없다. | converter 관련 source path의 status/diff 및 working-copy/HEAD hash를 고정하고 전체 worktree가 clean하다고 가장하지 않는다. | 통과 |
| 6 | lockfile 버전과 실제 로드된 Lightning CSS가 다름 | 발견. resolved module path/version이 없는 manifest는 다른 설치 결과를 놓친다. | Bun/Node 버전, argv, working directory, resolved path와 package version을 기록하도록 했다. | 통과 |
| 7 | 지원 CSS 입력에서 일부 rule/declaration이 누락 | 초기 발견 없음. 추가 관점에서 converter result의 warnings와 input/output completeness가 노출되지 않을 수 있음이 발견됐다. | criteria에 warnings, exact expected JSON, input/output rule completeness를 추가하고 확인할 수 없으면 미확인으로 두었다. | 통과 |
| 8 | 잘못된 CSS parse와 unsupported AST를 한 실패로 처리 | 발견 없음. | parse와 AST 거부의 분류를 유지한다. | 통과 |
| 9 | mixed valid/unsupported 입력이 부분 성공으로 분류 | 발견. transform warning 또는 누락된 rule을 성공으로 보지 않을 기준이 필요하다. | mixed control에서 warning·partial JSON을 성공으로 판정하지 않도록 했다. | 통과 |
| 10 | output JSON 자체를 보존하지 않아 결과를 재검증할 수 없음 | 발견. 새 변환의 output path/hash와 실패 진단 보존 조건이 약했다. | input/output JSON, stdout/stderr, exit, existence/size/hash를 기록하도록 했다. | 통과 |
| 11 | 실패 뒤 stale JSON을 새 결과로 소비 | 일부 발견. 이전 파일과 비정상 종료 뒤 partial output을 구분할 규칙이 부족했다. | 새 case directory, exit code gate, partial output 보존·비소비 조건을 추가했다. | 통과 |
| 12 | 출력 경로가 입력의 symlink/case alias를 가리켜 입력을 삭제 | 발견. converter는 문자열 resolve 비교 뒤 output을 먼저 삭제한다. | run-local copy와 canonical realpath/lstat, symlink target alias 거부를 추가했다. | 통과 |
| 13 | 기존 output이 존재해 converter가 unrelated artifact를 삭제 | 발견. 새 output path 부재 확인과 overwrite stop condition이 필요하다. | 새 case 안의 없는 `output.json`만 허용하고 기존 경로면 중단하도록 했다. | 통과 |
| 14 | 고정 evidence directory에 서로 다른 실행 결과가 섞임 | 발견. 재실행 시 directory collision 규칙이 없었다. | 고유 run-id directory를 만들고 이미 있으면 중단하도록 했다. | 통과 |
| 15 | 임시 입력·출력 생성/cleanup이 다른 파일을 건드림 | 발견. 임시 경로 소유·삭제 범위가 추상적이었다. | 새 run/case directory를 `mkdir`로 생성하고 소유한 scratch만 정리하도록 했다. | 통과 |
| 16 | 임시 출력 디렉터리가 `TMPDIR` 설정으로 worktree 안에 생성 | 재검토에서 추가 발견. “worktree 밖” 문구만으로는 `TMPDIR`이 repository 내부를 가리키는 설정을 배제하지 못한다. | temp path와 repository root를 realpath로 비교해 temp가 실제 worktree 밖인지 확인하고 canonical path를 기록하도록 했다. | 통과 |
| 17 | hash 목록이 부족해 source와 input 조합을 재현할 수 없음 | 발견. 실제 R14 CSS/JSON 및 lockfile 등을 경로별로 고정할 필요가 있다. | R10/R14 source/input/output/lockfile의 path별 SHA-256 manifest를 요구하도록 했다. | 통과 |
| 18 | 계획을 실행한 뒤에야 필수 계획 검토를 진행 | 발견. 최초 초안은 계획 실행 전 20개 검토를 요구하지 않았다. | 20개 검토·수정·재검토 완료 전 단계 1을 실행하지 않도록 했다. | 통과 |
| 19 | evidence가 없는 경우에도 공식 완료 체크나 roadmap count 변경 | 발견 없음. 공식 상태 변경을 계획 실행의 결과에서 제외한다. | R11 미완료와 상태 대장 불변 조건을 유지한다. | 통과 |
| 20 | 미확인 결과를 임의 구현·시뮬레이터 검증으로 확대 | 발견 없음. 후속 공백은 별도 계획에 남기며 앱·시뮬레이터를 제외한다. | Stop condition과 simulator/browser/performance 제외를 유지한다. | 통과 |

## 재검토 기록

- 최초 초안에 대해 21개 관점이 제출됐다. 발견 사항을 반영한 뒤 4–20번은 재검토했다. 1–3번은 발견이 없어 재검토하지 않았다. 이 표는 파일 변경·증거 출처·출력 경로처럼 겹치는 failure path를 합쳐 20개로 정리했다.
- 4–20번은 보정 후 독립 재검토에서 통과했다. 16번의 `TMPDIR` 경로 차단도 canonical path 조건으로 마지막 재검토를 통과했다.
- 계획 검토 후속 작업이 끝났으므로 이제 제한된 R11 조사를 실행할 수 있다. 조사 결과만으로 STATUS나 제품 지원 범위를 바꾸지 않는다.
