# D08 버전별 명세 사이트 계획 · 적대 검토 20개

- **대상:** [D08 계획 초안](./d08-versioned-spec-site.md)
- **기준:** `origin/main` `0aac7f00e1fda333d9cfa0e5c5d5082a8533dfef`에서 분리한 문서 전용 worktree
- **검토 범위:** 버전/지원 상태 경계, 문서 owner, URL·검색·배포, 파일 범위와 자원 격리. D08 사이트 구현이나 출시 검증은 아니다.
- **수행 방식:** dependency/contract lens가 1–7, file/scope lens가 8–14, runtime/resource lens가 15–20을 독립적으로 검토했다. 첫 검토에서 부분 판정은 없었다.
- **실행 범위:** D08 계획·계획 검토에서는 build/test/site preview/Pages deployment를 실행하지 않았다. 공개 로드맵 완료 수는 이 작업과 분리해 agent-browser로 확인하고 그 브라우저를 닫았다.

| # | 적대 관점 | 근거와 최종 판정 |
|---:|---|---|
| 1 | app release, docs snapshot, API first-supported version, internal contract version을 하나로 합침 | 계획 14·20·35·64행은 버전 축을 분리하고 출시 전 `0.1.0` 고정을 보존한다. **PASS** |
| 2 | D08 계획 초안을 구현·배포된 versioned site로 표시 | 계획 3·10·42행은 D08 미완료와 사이트 미시작을 명시한다. **PASS** |
| 3 | 제안 `spec/`, 지원 API, 공식 상태 대장을 같은 지원 source로 취급 | 계획 14·22–23·34행은 각 source 역할을 분리하고 현재 지원 API가 0개임을 보존한다. **PASS** |
| 4 | draft source와 released snapshot을 섞어 오래된 문서를 바꿈 | 계획 20·46–49·63행은 immutable source tag, source SHA와 release gate를 결정 사항으로 남긴다. **PASS** |
| 5 | 공식 완료 수·지원 표를 site가 별도 규칙으로 재계산 | 계획 23·58·74행은 STATUS 기준을 따르고 공식 집계를 바꾸지 않도록 한다. **PASS** |
| 6 | 검증하지 않은 예제나 API migration을 실행 가능·호환으로 표시 | 계획 24·52–53행은 source version, dependency lock, 대상 release와 실행 검증 상태를 연결한다. **PASS** |
| 7 | 이전/대상 release가 불연속인데 migration 완료를 주장 | 계획 21·24·50·52행은 URL inventory, 지원 기간과 건너뛴 migration 안내를 요구한다. **PASS** |
| 8 | root의 dirty C11/C15/C19 WIP를 D08 계획에 섞음 | 별도 worktree는 main `0aac7f0` 기준이고 root 작업은 보존됐다. **PASS** |
| 9 | 계획 PR에 문서 site 설정·source·lockfile·workflow 변경이 포함 | 검토 당시 head `bd766fd4`의 PR diff에는 D08 계획과 이 검토 기록 두 파일만 있었고, 이후 추가된 PR 검토 기록까지 포함한 현재 세 파일 diff에도 site source/config/lockfile/workflow 변경은 없다. **PASS** |
| 10 | 계획 상대 링크가 없는 파일을 가리킴 | STATUS, README, 명세/API 문서, Rspress config와 Pages workflow의 일곱 링크가 모두 존재한다. **PASS** |
| 11 | D08 체크, API 지원 수, contract version이 계획 작성만으로 바뀜 | STATUS/API/version 파일을 수정하지 않으며 D08은 미완료로 남는다(계획 3·14·74행). **PASS** |
| 12 | 현재 public URL·preview URL을 검증된 계약처럼 단정 | 계획 21·63행은 현재 주소 inventory와 owner를 먼저 확인하도록 하고 새 URL/redirect 선택은 미결로 둔다. **PASS** |
| 13 | Rspress output/base/version을 잘못 설명하거나 dependency를 갱신 | 계획 12·27–28행은 현재 `spec/`, `/spinon/`, output path와 고정 `2.0.22`를 기록하고 config/lockfile 변경을 제외한다. **PASS** |
| 14 | 역할 목록이 실제 승인 owner 지정인 것처럼 보임 | 계획 32–38행은 책임 영역만 두고, 계획 70행은 실제 owner 확인을 구현 gate로 둔다. **PASS** |
| 15 | 계획 중 현재 Tailscale preview server를 종료하거나 output을 갱신 | 계획 12·63·72행은 process/output을 보존하고 충돌이 있으면 build를 보류하도록 한다. preview server는 실행 상태로 보존하고 종료하지 않았다. **PASS** |
| 16 | 미래 build가 active preview의 기본 `packages/docs/build/site`를 덮음 | 계획 12·27·55·72행은 output 격리와 기존 산출물 보존을 acceptance/gate로 둔다. **PASS** |
| 17 | version-aware 검색이 다른 release나 draft의 결과를 섞음 | 계획 25·46·51행은 index, selected version, draft 정책을 미결 결정으로 둔다. **PASS** |
| 18 | 첫 release 전 자동 Pages publish 또는 넓은 `rsync --delete` 발생 | 계획 28·37·57·67·73행은 수동 승인, destination allowlist, 삭제 범위와 자동 게시 금지를 연결한다. **PASS** |
| 19 | 오래된 snapshot 저장·검색·license 산출물이 자원을 소진하거나 출처가 누락 | 계획 26–27·54·59행은 owner, retention, storage quota, notice source를 결정하게 한다. **PASS** |
| 20 | 계획 검토를 사이트/browser/runtime evidence나 official completion으로 오인 | 계획 3·6·42행과 이 기록 6행은 구현·site 실행 근거가 아님을 분명히 한다. **PASS** |

## 최종 판정

위 20개의 서로 다른 계획 실패 관점을 검토했고 모두 PASS다. D08은 계획 단계만 진척됐으며, 버전 축·URL·support source·migration·license owner·retention·manual publish 승인 등은 release/docs owner의 결정 전까지 미결이다. 사이트 구현, 빌드, preview, 배포, D08 완료 표시는 이 계획 검토로 대체되지 않는다.
