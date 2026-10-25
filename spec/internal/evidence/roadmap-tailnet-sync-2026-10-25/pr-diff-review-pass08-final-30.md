# PR #625 pass08 최종 30관점 적대적 diff 검토

- 고정 대상: PR #625, head 48d51a194de2b805767cea7bb5d723a51f38f651, base main f197239b0168e2356f4d0ece03f5d2e4c40278fe.
- GitHub의 고정 head diff·본문·라벨과 참조 JSON/script/log를 검토했다. 테스트와 빌드는 실행하지 않았고, Agent Browser로 PR 본문 스크린샷의 실제 로드를 확인한 뒤 세션과 프로세스를 닫았다.
- 판정: **29 PASS / 1 PARTIAL / 0 FAIL**.

| # | 실패 관점 | 판정 | 근거와 결과 |
|---:|---|---|---|
| 1 | 검토 대상 PR이 요청된 최종 head인가 | PASS | PR #625는 OPEN이고 현재 head가 48d51a194de2b805767cea7bb5d723a51f38f651이다. |
| 2 | 비교 base와 전체 diff 경계가 정확한가 | PASS | base는 f197239b0168e2356f4d0ece03f5d2e4c40278fe이며 PR API 파일 목록은 34개 모두 추가 파일로 표시한다. statusCheckRollup은 비어 있고 CI 통과를 주장하지 않는다. |
| 3 | PR 파일이 문서 근거 범위에 한정되는가 | PASS | 34개 파일 모두 spec/internal/evidence/roadmap-tailnet-sync-2026-10-25/ 아래다. STATUS·roadmap·제품 코드·API 명세를 직접 수정하지 않았다. |
| 4 | PR 본문이 프로젝트 양식의 핵심 정보를 갖추고 라벨이 맞는가 | PASS | C11.4 ID, 변경 범위, 명세 해당 없음, 근거 링크, 남은 범위와 documentation/area: docs 라벨을 제공한다. |
| 5 | pass08의 기준 main이 PR #624 병합 결과와 일치하는가 | PASS | PR #624는 MERGED이며 merge SHA f197239b0168e2356f4d0ece03f5d2e4c40278fe, 병합 시각 2026-10-25 05:00:30Z다. |
| 6 | source root의 main commit/tree provenance가 base와 맞는가 | PASS | corrected root manifest의 sourceHead/originMainHead는 f197239b…이고 sourceTree/originMainTree는 28556f278dd82a1fde00a1e59afdb1d6c7a2267b로 일치한다. |
| 7 | root overlay 원본들이 main 추적 파일의 동일 bytes인가 | PASS | corrected prepare script는 main tree에서 추적 여부를 확인하고 각 source에 대해 git show bytes와 로컬 bytes를 비교한다. manifest는 sourceFiles 38개를 기록한다. |
| 8 | main에 없던 review 초안이 corrected source에 섞였는가 | PASS | untracked review markdown의 SHA-256 5b884466…를 기록해 제외했고 corrected selected build에도 이 경로가 없다. |
| 9 | runtime evidence 밖에서 추가한 raw source도 main에 고정되는가 | PASS | 최신 stage script가 parallel-next-slices review 파일의 tracked 여부와 git show byte equality를 검사한다. 선택 route SHA-256 12a083a6…는 f197 main 원본의 독립 SHA-256과 같다. |
| 10 | 전체 build와 Tailnet 선택 수가 혼동되는가 | PASS | 본문·README·manifest가 전체 Rspress HTML 757개, 선택 generated HTML route 24개, 전체 selected route 258개를 구분한다. manifest의 fullBuildHtmlFileCount와 selectedGeneratedRouteCount가 각각 757/24다. |
| 11 | build 결과와 비활성 검사가 과장되는가 | PASS | 저장 log에는 Pages rendered와 ready built가 있고 config는 dead-link/image 검사를 끈다. 본문은 전체 docs:build 통과라고 주장하지 않는다. |
| 12 | selected route manifest의 count·경로·byte 합이 일치하는가 | PASS | 258개 경로가 모두 고유하고 byte 합 25,913,401이 totalBytes와 같다. 잘못된 review 별칭 경로는 0개다. |
| 13 | 생성·교체·동일 preflight 분류가 258개와 맞는가 | PASS | corrected preflight와 copy 결과는 create 25, replace 24, same 209이며 합계는 258이다. |
| 14 | 첫 262-route 오류 산출물이 최신 근거처럼 보이는가 | PASS | README와 본문은 초기에 잘못 선택한 262-route 자료와 이전 prepare/stage/preflight/sync/verify scripts를 history-only·재실행 금지로 명시하고 corrected 절차를 현재 기준으로 둔다. |
| 15 | 제거 대상 별칭이 실제로 처음에는 새 경로였는가 | PASS | 최초 preflight는 네 경로 각각 status=create, target_exists=false, 8,133 bytes와 동일 source SHA-256 5b884466…를 기록한다. |
| 16 | 네 별칭의 백업이 byte-exact였는가 | PASS | correction manifest는 별칭 네 개의 backup 경로·bytes·SHA-256을 기록하고 all_backups_byte_exact=true다. |
| 17 | 삭제 범위가 네 별칭에만 한정되고 404가 확인됐는가 | PASS | correction script는 최초 create였던 네 정확한 경로·hash와 lsof 닫힘을 확인한 뒤만 제거한다. manifest는 local 4/4 removed, Tailnet 4/4 HTTP 404, only initially new paths removed를 기록한다. |
| 18 | 정상 교체 24개 파일의 기존 bytes가 백업됐는가 | PASS | corrected verification manifest는 backup 24개 전부의 경로·bytes·SHA-256과 verified=true를 기록하고 all_backups_verified=true다. |
| 19 | Tailnet sync가 race·symlink·부분 쓰기에 안전한가 | PASS | sync script는 preflight source/target hash를 재확인하고 symlink 경로를 거부하며 교체 직전 lsof를 검사한다. 임시 파일 fsync 후 link/rename으로 반영하고 결과 hash를 다시 검증한다. |
| 20 | 로컬 preview가 선택 258개 route와 일치하는가 | PASS | verify manifest는 local_targets 258개 전부 verified=true이며 selected source hash/bytes와 일치한다고 기록한다. |
| 21 | 원격 Tailnet 응답이 HTTP 200과 본문 hash를 만족하는가 | PASS | remote_targets 258개 전부 HTTP 200, byte/hash 일치와 verified=true이며 remote hash/byte mismatch가 없다. |
| 22 | no-deletes 동작을 전체 preview 완전 동기화로 과장하는가 | PASS | selected upload의 no_deletes=true를 명시하고 검증을 선택한 258 route에 한정한다. 기존 선택 밖 route 삭제나 전체 사이트 동기화를 주장하지 않는다. |
| 23 | 과거 preview PID 종료와 현재 서버 liveness가 구별되는가 | PARTIAL | 최신 근거는 새 PID 28040의 127.0.0.1:8768 listener, local HTTP 200, Tailnet Running/online을 확인한다. 이전 PID 1180 SIGTERM은 역사 기록이며 원본 종료 출력이 없다고 README/본문이 공개한다. 따라서 새 서버 liveness는 확인되지만 과거 PID 종료 사건은 독립 재검증할 수 없다. |
| 24 | PR #623의 병합 상태와 SHA가 정확한가 | PASS | GitHub PR #623은 MERGED, merge SHA 56daefe96e8143959bc99a89c7a8c80e7f1cc4ba, mergedAt 2026-10-25T04:35:52Z로 corrected Browser 관측과 일치한다. |
| 25 | #623 테스트에 사용한 source SHA가 이후 변경과 혼동되는가 | PASS | source SHA c5b8ef75804e39d36a37aa61b82b1f028115fefd는 PR #623 head의 조상이다. source SHA부터 해당 head까지의 두 commit은 runtime evidence 문서 변경이며 제품 코드 변경은 없다. |
| 26 | #623 테스트·simulator 주장이 기기 범위를 넘는가 | PASS | #623 본문은 runtime/FFI 수치와 Android API 37 AVD·iOS 26.2 Simulator target/control 결과를 기록한다. AVD software renderer, iOS Metal compositor 제출 전 표본, 실물 Android 미사용 한계를 밝히며 #625 자체는 제품 테스트/simulator를 했다고 주장하지 않는다. |
| 27 | 공식 C11.4 상태와 72/213 집계가 유지되는가 | PASS | base STATUS와 roadmap 근거는 C11.4 checkbox 미완료, 공식 집계 72/213을 보이며 PR 본문도 상태 변경을 부인한다. |
| 28 | orthogonal row blocker가 완료로 오인되는가 | PASS | Browser 관측은 vertical-rl 축이 현재 layout 경로에 전달되지 않아 별도 writing-mode 작업 전까지 보류라고 표시한다. 완료 승격이 없다. |
| 29 | PR 본문의 수정 screenshot 링크가 실제 표시되는가 | PASS | 최종 head의 PR 페이지를 Agent Browser로 열어 raw PNG의 complete=true, naturalWidth 1280, naturalHeight 633을 확인했다. 관측 JSON에는 Tailnet roadmap URL과 DOM checks가 기록돼 있다. |
| 30 | Agent Browser 세션·프로세스 종료 주장이 맞는가 | PASS | lifecycle JSON은 close 전 PID와 page 수, close --all 이후 빈 session 목록과 pgrep no-match를 구조적으로 기록한다. 이 리뷰도 Agent Browser로 이미지를 확인한 뒤 닫았고 세션 목록과 browser/Chrome 프로세스 검색이 비었다. |
