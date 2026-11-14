# R11 최종 PR 변경분 적대 검토 30개

**검토 대상:** `codex/r11-css-build-path-plan-2026-11-14`의 최종 staged diff
**기준:** `401911e6b78b543e3c9553c3e6e162e43ca381b2`
**관점:** 범위·공식 상태 10개, 근거 계보 10개, 재현·실행 경계 10개. 세 관점은 같은 변경을 서로 다른 실패 경로로 확인했다. 첫 검토 뒤 이 기록 자체와 링크를 추가했고, 그 최종 diff를 다시 확인했다.

## 범위와 공식 상태

1. **R11 공식 완료 체크를 올렸는가? — PASS.** `spec/STATUS.md`의 체크박스는 미완료이고 변경은 조사 근거 연결뿐이다.
2. **공식 완료 집계를 바꿨는가? — PASS.** README가 기록한 78개 상위 항목·214개 표시 작업과 `spec/README.md` 집계가 유지된다.
3. **converter 재현을 제품 CSS 지원으로 확대했는가? — PASS.** 실행 README는 Rust consumer, browser, mobile app 실행을 범위 밖으로 둔다.
4. **R10 Taffy 소비 결과를 AST 변환 검증으로 합쳤는가? — PASS.** Coverage matrix는 기존 소비자 근거와 이번 converter 실행을 별도 질문으로 분리한다.
5. **R14 Tailwind 실행을 이번 run 또는 일반 지원으로 썼는가? — PASS.** R14 산출물은 역사적 참고 자료로 설명되고 지원 계약으로 승격되지 않는다.
6. **positive sample 한 건을 CSS 전반으로 일반화했는가? — PASS.** R10 sample 재현과 mixed unsupported 거부만 확인한 제한 범위를 명시한다.
7. **12개 verifier 입력 사례와 14행 coverage matrix를 혼동했는가? — PASS.** 계획과 matrix가 각각 12개 입력 경계, 기존 질문 8개와 후속 경계 6개를 구분한다.
8. **계획 20점 검토가 수정 후 전부 재검토됐다고 과장했는가? — PASS.** 계획은 4–20번 재검토와 1–3번 미재검토를 그대로 밝힌다.
9. **임시 경로 정리를 전체 cleanup 입증으로 설명했는가? — PARTIAL.** 지정 경로의 삭제·부재만 기록하며 `lsof` 원본/종료 코드와 완전한 목록이 없다는 한계를 명시한다.
10. **계획·실행·인덱스가 같은 evidence를 가리키는가? — PASS.** R11 계획, STATUS, internal index가 같은 run README로 연결된다.

## 근거와 출처 계보

11. **실행 기준 HEAD·branch가 혼동됐는가? — PASS.** 초기 snapshot과 README는 HEAD `401911e6…` 및 기준 branch를 기록한다.
12. **실행 시 converter source가 dirty였는가? — PASS.** 초기·사후 snapshot의 관련 diff는 비어 있고 source hash는 HEAD blob과 맞는다.
13. **역사 R10 결과가 다른 source revision에서 왔는가? — PASS.** R10 converter·lock·input·output의 역사 hash와 조상 관계를 확인했다.
14. **R14 evidence commit의 비조상 관계를 숨겼는가? — PASS.** 비조상 commit과 조상 commit의 parent/tree 관계를 구별한다.
15. **추가한 R14 생성기 입력 hash가 누락되거나 틀렸는가? — PASS.** 생성기·lock·input·fixture·preview CSS·verifier 파일 여섯 개의 기록 hash가 대조됐다.
16. **R14 JSON hash가 개별 raw 실행 로그를 대신하는가? — PARTIAL.** 결과 파일 무결성은 확인되지만 per-control raw logs 부재를 문서가 공개한다.
17. **lockfile 버전과 실제 resolved Lightning CSS가 다른가? — PASS.** Bun/Node 버전과 실제 package version·resolved path가 설치·resolution 기록과 연결된다.
18. **positive와 mixed unsupported 결과가 뒤바뀌었는가? — PASS.** positive JSON hash는 expected와 일치하고 mixed case는 exit 1·output 없음·diagnostic으로 기록된다.
19. **보존된 manifest 기록의 수량을 재구성 가능한 사실보다 넓게 주장했는가? — PARTIAL.** cleanup note는 삭제 시점 manifest가 32개였다고 기록하지만 원본 bytes와 검증 출력을 보존하지 않았다. `manifest-verification-before-log.txt`는 별도로 보존된 33개 검증 출력을 담는다. 이용 가능한 HEAD·로그에서는 34개 중간 상태를 확인할 수 없으므로 그 수량이나 시점을 주장하지 않는다. 현재 final manifest는 자신을 제외한 35개 파일을 검증했고 모두 통과한다.
20. **staged diff와 검토 대상 worktree가 다른가? — PASS.** 최종 기록에서 index와 worktree가 일치하며 기존 staged manifest는 역사 자료라고 구분한다.

## 재현성과 실행 범위

21. **도구 버전만 고정하고 다른 module을 실행했는가? — PASS.** 설치 명령·종료 코드와 resolved module path/version이 보존됐다.
22. **positive 변환이 byte-exact 재현됐는가? — PASS.** run output, tracked JSON, expected JSON hash가 일치한다.
23. **유효 선언과 미지원 선언의 혼합 입력이 부분 성공했는가? — PASS, 해당 case 한정.** exit 1, output 미생성, 미지원 property 진단이 기록됐다.
24. **12개 verifier case별 결과를 보존 stdout만으로 재현할 수 있는가? — PARTIAL.** verifier source와 합계 출력은 있으나 개별 case 원문 진단은 보존되지 않았다고 README가 한정한다.
25. **warning 완전성이나 강제 종료 중 partial write를 검증했는가? — 미검증.** transform 반환 warning은 소비되지 않고 중단/I/O 오류 case도 실행하지 않았음을 계획과 matrix가 명시한다.
26. **temp cleanup이 다른 경로나 프로세스까지 안전하게 정리했다고 주장하는가? — PARTIAL.** 정확한 task temp 경로와 사후 부재만 확인하며 원시 `lsof`/전체 inventory는 없다고 한다.
27. **ignored `node_modules`가 이번 실행에서 만들어졌다고 단정했는가? — PASS.** 실행 전 상태가 알려지지 않아 보존했고 isolated worktree cleanup도 미실행으로 남겼다.
28. **현재 evidence manifest가 모든 evidence file을 검증하는가? — PASS, 현재 snapshot 한정.** 이용 가능한 근거로 첫 review 당시 34개 manifest가 있었다고 확인할 수 없다. 현재 최종 manifest는 자신을 제외한 35개 파일의 hash를 담고 있으며 전부 통과했다.
29. **staged-files manifest를 현재 verifier로 오인하는가? — PASS.** 두 후속 수정 뒤에는 historical manifest이며 final manifest가 현재 검사 기준이라고 설명한다.
30. **14행 matrix가 converter·제품 전체 동작을 주장하는가? — PASS.** 8개 기존 질문과 6개 후속 경계를 나누고, R11 미완료 및 제품/browser/mobile 미검증을 유지한다.

## 종합 판정

최종 변경분에서 blocker는 찾지 못했다. `PARTIAL`은 삭제 당시 전체 process·filesystem 상태, 과거 32-entry manifest 원본, 34-entry 중간 상태를 입증할 근거의 부재, R14 per-control logs, 개별 verifier 출력, warning/partial-write 동작이 보존 또는 실행되지 않은 한계를 가리킨다. 이 경계들은 본문에도 명시되어 있으며 R11 공식 완료의 근거로 사용하지 않는다.
