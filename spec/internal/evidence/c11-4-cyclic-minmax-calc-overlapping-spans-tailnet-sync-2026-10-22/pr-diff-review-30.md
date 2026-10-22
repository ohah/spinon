# C11.4 Tailnet preview PR diff adversarial review · 30 checks

**범위:** PR #500 병합 상태를 C11.4 공식 문서와 Tailnet 정적 미리보기에 반영하는 문서 변경. 공개 API, 앱 런타임 또는 완료 상태는 바꾸지 않는다. 각 검토는 별도의 실패 경로를 다룬다.

| # | 실패 경로 / 검토 질문 | 판정 근거 |
|---:|---|---|
| 1 | Rebase and merge 전 head SHA를 merge SHA로 잘못 기록하는가? | gh pr view 500의 mergeCommit.oid와 대조한다. |
| 2 | 동기화 branch가 PR #500 merge commit에서 분기하지 않는가? | branch base와 merge SHA를 기록한다. |
| 3 | dirty checkout을 빌드 입력으로 사용해 미추적 파일이 유입되는가? | git archive commit/tree와 selected-root manifest만 입력으로 허용한다. |
| 4 | selected-root에 symlink나 비정규 파일이 섞이는가? | 준비 스크립트가 모든 archive entry를 검사한다. |
| 5 | initial/final build가 같은 root나 output을 덮어쓰는가? | pass 이름을 root·output 경로에 포함하고 기존 경로는 실패 처리한다. |
| 6 | docs config가 저장소 RSPress가 아닌 다른 버전을 실행하는가? | packages/docs 잠금 버전과 빌드 로그를 대조한다. |
| 7 | preview 전용 dead-link 설정이 저장소 기본 config까지 변경되는가? | 설정은 evidence의 선택 config에만 두고 package/config diff를 확인한다. |
| 8 | roadmap에 계획값과 구현값을 혼동해 기록하는가? | [34,39,39], [42,47,47], [32,32,32] 세 값을 원문과 대조한다. |
| 9 | roadmap이 C11.4 전체 또는 지원 완료를 선언하는가? | note가 전체 C11.4 미검증과 77/213 유지라고 명시한다. |
| 10 | STATUS가 #500을 미병합으로 남기거나 다른 SHA를 기록하는가? | UTC 병합 시각과 merge SHA를 PR API에 대조한다. |
| 11 | STATUS 수정이 parent checkbox 또는 완료 카운트를 올리는가? | 공식 counter와 C11.4 checkbox를 전후 비교한다. |
| 12 | 계획 문서가 simulator scene 수를 과장하는가? | 계획의 두 플랫폼 6/6 표기를 원본 실행 manifest와 대조한다. |
| 13 | 내부 문서 인덱스 링크가 plan/evidence 경로를 잘못 가리키는가? | 상대 Markdown 링크의 목적 파일 존재를 검사한다. |
| 14 | roadmap에서 PR 링크가 잘못된 PR이나 pull URL을 가리키는가? | 렌더 DOM에서 #500 href를 확인한다. |
| 15 | RSPress가 roadmap/STATUS/plan의 URL 경로를 바꾸어 링크가 깨지는가? | 생성 route와 Agent Browser 최종 URL을 확인한다. |
| 16 | #500 두 simulator screenshot이 Tailnet에서 404 또는 깨진 이미지로 보이는가? | Agent Browser complete와 naturalWidth를 확인한다. |
| 17 | evidence page가 runtime 근거와 다른 scene screenshot을 포함하는가? | 이미지 URL을 #500 runtime manifest의 Android/iOS artifact와 대조한다. |
| 18 | roadmap의 값이나 한글이 entity/Markdown 처리로 잘못 렌더링되는가? | Agent Browser 표시 텍스트와 source note를 비교한다. |
| 19 | STATUS 자동 카운터가 예상과 달리 78개 또는 다른 수를 표시하는가? | 렌더 페이지에서 77/213 표시를 직접 확인한다. |
| 20 | 노트가 CSS border stroke나 compositor 이후 결과를 주장하는가? | 문서가 미검증 범위를 명시하는지 확인한다. |
| 21 | 문서 업데이트가 GitHub Pages 또는 gh-pages 배포로 이어지는가? | workflow dispatch와 branch 변경이 없음을 PR diff/실행 기록으로 확인한다. |
| 22 | preview 대상의 기존 파일이 백업 전에 교체되는가? | preflight SHA와 교체 전 backup SHA를 비교한다. |
| 23 | preflight 이후 source가 달라진 채 복사되는가? | copy 단계가 manifest byte 수와 SHA를 다시 검사한다. |
| 24 | preflight 이후 target이 외부에서 바뀐 채 덮어써지는가? | copy 단계가 이전 target 존재 여부·byte 수·SHA를 다시 비교한다. |
| 25 | target 또는 backup 경로 구성요소의 symlink를 따라 쓰는가? | sync script가 각 경로를 lstat로 검사하고 symlink에서 중단한다. |
| 26 | 다른 프로세스가 읽는 열린 파일을 atomic rename으로 교체하는가? | rename 직전 lsof; 허용은 idle preview PID 1191만이다. |
| 27 | 임시 파일이 기존 파일을 덮거나 부분 기록이 노출되는가? | exclusive temp create, fsync, 동일 디렉터리 atomic rename을 확인한다. |
| 28 | 동기화가 기존 Tailnet 파일을 삭제하는가? | manifest와 summary에서 no_deletes=true 및 create/replace/same만 허용한다. |
| 29 | 로컬 복사만 성공하고 HTTP 서버가 다른 bytes를 내보내는가? | 모든 선택 파일의 HTTP 200, byte 수, SHA-256을 비교한다. |
| 30 | 브라우저 cleanup이 사용자 Chrome 또는 Tailnet listener를 종료하는가? | 전용 Agent Browser session만 닫고 PID 20311·1191 생존을 확인한다. |

최종 결과와 각 확인의 수치는 아래 Tailnet 동기화 기록 및 manifest를 기준으로 채운다. 30개 모두 실행 근거가 연결되기 전에는 검토를 완료로 표시하지 않는다.
