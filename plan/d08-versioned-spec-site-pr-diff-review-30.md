# D08 버전별 명세 사이트 PR diff 적대 검토 30개

- **대상:** [PR #1389](https://github.com/ohah/spinon/pull/1389)
- **검토한 코드 head:** `bd766fd4a46f3422ea481c956b1dca5ca60ed3bd`
- **base:** `main` `0aac7f00e1fda333d9cfa0e5c5d5082a8533dfef`
- **검토 범위:** 계획 초안, 계획 검토 기록, PR 본문·라벨·파일 범위와 기존 preview/process 경계. D08 사이트 구현이나 배포 검증은 아니다.
- **방식:** dependency/contract, file/scope, runtime/resource의 세 독립 관점에서 각각 10개 항목을 확인했다. 총 30개 항목 모두 **PASS**다.
- **주의:** 이 결과 파일은 위 head 검토가 끝난 뒤 PR에 기록용으로 추가했다. 이 파일 추가 외에 계획과 계획 검토 기록의 내용은 바뀌지 않았다. CI status와 workflow run은 비어 있다.

| # | 적대 관점 | 근거와 최종 판정 |
|---:|---|---|
| 1 | 앱·내부 계약·API·문서 snapshot 버전을 하나의 축으로 합침 | 계획 14·20·35·64행이 버전 축을 구분한다. **PASS** |
| 2 | `spec/`, `spec/api/`, `STATUS.md`의 source ownership 혼합 | 계획 22–23·34행이 제안 명세, 지원 API, 공식 상태의 역할을 나눈다. **PASS** |
| 3 | 제안 API를 지원 계약으로 승격 | 계획 14·23·48행은 제안을 지원 API에서 제외하며 현재 지원 API 0개와 맞는다. **PASS** |
| 4 | 아직 정해지지 않은 release tag/source를 이미 확정된 것으로 기술 | 계획 20·46·49·64행은 tag/revision pinning을 결정 사항으로 남긴다. **PASS** |
| 5 | 이전·대상 버전이 불연속인 migration을 완료로 표시 | 계획 24·52행은 source/target과 건너뛴 전환을 다룬다. **PASS** |
| 6 | 기존 URL·redirect·snapshot retention을 무시 | 계획 21·27·50·63행은 URL inventory와 보존 정책을 결정 대상으로 둔다. **PASS** |
| 7 | 현재 Rspress·Pages gate와 다른 배포 절차를 전제 | 계획 12·28·37·57·67·72–73행은 현재 설정과 수동 승인 경계를 보존한다. **PASS** |
| 8 | 계획만으로 D08 완료 수나 API 지원 수를 변경 | 계획 3·74행은 D08 미완료와 `78/214` 집계 유지를 명시한다. **PASS** |
| 9 | 미지정 owner의 제품 결정을 대신 내림 | 계획 26·32–38·70행은 역할과 실제 owner 결정을 분리한다. **PASS** |
| 10 | D08의 선행 의존성을 만들거나 완료를 주장 | 계획 3·6·38·70·74행과 STATUS D08 행은 계획 범위와 미완료 상태를 유지한다. **PASS** |
| 11 | root의 dirty C11/C15/C19 WIP가 PR에 섞임 | main 기준 별도 worktree이며 PR diff에는 root WIP, `Cargo.lock`, `mise.toml`이 없다. **PASS** |
| 12 | 의도하지 않은 source/config/lock/workflow 변경 | 검토한 PR diff는 계획과 20개 계획 검토 기록 두 Markdown 파일뿐이다. **PASS** |
| 13 | 계획·검토 기록 또는 PR 본문 링크가 없거나 다른 head를 가리킴 | 상대 링크가 해석되고, 검토한 PR 본문의 네 GitHub 파일 링크는 당시 head `bd766fd4…`를 가리킨다. **PASS** |
| 14 | STATUS/API/version 파일이 계획 변경으로 수정 | 해당 파일은 diff에 없고 D08 체크는 미완료다. **PASS** |
| 15 | Rspress 경로·버전·Pages workflow를 잘못 설명 | 계획은 `spec/`, `/spinon/`, `packages/docs/build/site`, Rspress `2.0.22`와 수동 workflow를 현재 설정에 맞춰 기록한다. **PASS** |
| 16 | PR 본문이 계획을 구현 증거로 과장 | 구현 근거 체크는 비워 두고 계획 전용 범위와 site QA 미실행을 구분한다. **PASS** |
| 17 | PR base/head/라벨이 계획 scope와 불일치 | base는 `main`, 대상 라벨은 `documentation`, `planning`, `area: docs`다. **PASS** |
| 18 | 계획 검토 기록이 최종 PR 파일 수를 잘못 표시 | 검토 기록 19행은 계획과 검토 기록 두 파일만 추가된다고 정확히 적는다. **PASS** |
| 19 | 과거 공개 집계 `72/213`을 현재값으로 표시 | 계획 74행의 `78/214`는 현재 STATUS·README 집계와 일치한다. **PASS** |
| 20 | D08 사이트 구현·publication·완료를 계획으로 주장 | 계획 3·6·10·42·74행과 PR 본문은 후속으로 남긴다. **PASS** |
| 21 | 검토 중 활성 Tailscale preview를 종료 | 계획은 기존 process/output을 보존하고, PID `90711`은 검토 중 계속 실행됐다. **PASS** |
| 22 | site build가 기존 preview output을 덮음 | 계획 12·27·55·72행은 별도 output과 실패 시 기존 산출물 보존을 요구한다. **PASS** |
| 23 | Pages publish 또는 넓은 `rsync --delete`를 자동 실행 | 계획 28·37·57·67·73행은 수동 승인, destination 제한, delete 범위를 둔다. **PASS** |
| 24 | 선택한 문서 버전과 다른 버전/draft 검색 결과 혼합 | 계획 20–25·46–51행은 버전과 검색 index 범위를 분리할 결정을 남긴다. **PASS** |
| 25 | snapshot 저장·삭제·검색 비용과 책임 누락 | 계획 27·59행은 quota, retention과 삭제 권한을 owner 결정으로 둔다. **PASS** |
| 26 | 예제·migration·license/notice의 출처를 보증 없이 표시 | 계획 24·26·52–54행은 출처, lock, 실행 환경, 담당자와 publish gate를 연결한다. **PASS** |
| 27 | 설정된 `/spinon/` base를 live URL 성공으로 간주 | 계획 21·50·63행은 주소 inventory를 요구하고 가용성 검증을 주장하지 않는다. **PASS** |
| 28 | 별도 agent-browser 집계 확인을 D08 site QA로 오인 | 계획 검토 6행과 PR 본문은 공개 로드맵 수 확인과 site QA를 분리한다. **PASS** |
| 29 | 실행하지 않은 build/test/browser/deploy 또는 CI green을 주장 | 계획·PR 본문은 미실행을 적고 GitHub status/workflow도 비어 있다. **PASS** |
| 30 | 실패·취소·source drift 때 기존 배포가 손상될 수 있음 | 계획 49·55–57·59행은 drift, rollback/alias, delete 범위와 quota를 acceptance로 둔다. **PASS** |

## 최종 판정

검토한 PR head `bd766fd4a46f3422ea481c956b1dca5ca60ed3bd`의 30개 적대 관점은 모두 PASS다. 이 검토는 정적 문서·PR 범위 확인이며 CI, site build, browser QA, deployment 성공을 뜻하지 않는다. D08은 계획 단계이고 공식 완료 수는 바뀌지 않았다.
