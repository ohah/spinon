# PR diff 적대 검토 30개

대상: C11.4 nested Grid 로드맵 안내, 선택 사이트 생성·Tailnet 동기화 산출물 및 브라우저 근거. 각 항목은 다른 실패 경로 또는 경계다.

1. **공식 완료 상태를 앞당기는 문구** — 로드맵 안내에 계획 단계와 미완료를 표시하고 `spec/STATUS.md` 집계가 `72/213`인지 대조했다. 통과.
2. **계획을 구현 성과로 오인** — Chrome 사전 관찰·계획을 실제 Spinon 런타임 결과와 분리하고 Simulator 미검증을 README에 적었다. 통과.
3. **기존 최신 로드맵 기록 덮어쓰기** — 변경이 기존 C11.3.3 기록을 바꾸지 않고 문단 하나를 추가하는지 확인했다. 통과.
4. **잘못된 계획 링크** — 신규 note의 집중 계획 링크가 저장소 파일 `plan/c11-4-nested-grid-intrinsic.md`를 가리키는지 확인했다. 통과.
5. **잘못된 precomparison 링크** — 증거 경로가 생성 사이트에 노출되는 동일 evidence 디렉터리로 매핑되는지 확인했다. 통과.
6. **문서의 route 충돌** — 선택 route 목록에 roadmap, 상태, 계획, README, 관찰 JSON과 oracle PNG를 각각 포함했다. 통과.
7. **문서 베이스 경로 누락** — Tailnet `/spinon/` prefix가 링크와 브라우저 URL에 일관되게 적용된 배포물을 확인했다. 통과.
8. **RSPress 버전 변경** — 선택 빌드가 고정 정책 `2.0.22`를 사용했는지 로그와 요약에서 확인했다. 통과.
9. **전체 문서 검사 통과로 과장** — README에 전체 dead-link 검사가 실행되지 않았다고 구분했다. 통과.
10. **pass01 실패를 숨김** — 실패 로그와 선택 root manifest를 남기고 `node:*` client bundle 오류 및 pass02 조치를 적었다. 통과.
11. **pass02 필터 범위 확대** — 선택 root 준비 스크립트가 지정한 RSPress config evidence 하나만 제외하고 source를 유지하는지 확인했다. 통과.
12. **선택 root 쓰기 중 소스 변경** — 스크립트가 고정 HEAD, clean tracked `plan/spec` 상태, HEAD blob 일치를 검사한다. 통과.
13. **경로 순회로 선택 root 탈출** — 입력 경로의 절대 경로와 `..`를 거부하고 출력도 root 안인지 검사한다. 통과.
14. **심볼릭 링크 공격/누락** — 선택 root 파일 순회가 symlink를 거부하고 결과 요약에서 symlink 없음 조건을 확인한다. 통과.
15. **삭제 전파로 미리보기 자료 손실** — source 삭제는 거부하며 Tailnet preflight와 README 모두 삭제 0개를 기록한다. 통과.
16. **잘못된 기준 사이트 혼합** — base commit이 기존 pass05 미리보기 근거의 commit `44c18e61…`와 같은지 확인했다. 통과.
17. **RSPress 결과가 다른 입력에서 생성됨** — source commit/tree, selected root 해시와 build summary가 동일 pass02 입력을 식별한다. 통과.
18. **부분 빌드 결과 배포** — Tailnet copy 대상은 성공한 완성 site root만 가리키고 source file count와 manifest를 비교한다. 통과.
19. **복사 전 사용자 데이터 백업 누락** — 덮어쓸 모든 202개 기존 파일을 별도 경로에 백업하고 해시를 검증했다. 통과.
20. **사전 점검 이후 target 변경** — copy 도구가 preflight 파일 집합, 해시와 bytes를 다시 확인한 다음 파일 단위로 적용한다. 통과.
21. **원자적 복사 실패로 부분 파일 노출** — 대상 임시 파일을 fsync 후 rename하며 복사 뒤 전 파일을 재검증한다. 통과.
22. **다른 로컬 서비스 덮어쓰기** — 복사 도구가 8768 listener PID와 명령 이름, socket 수를 검사한다. 관측된 PID 1191은 지정 preview server다. 통과.
23. **예상 밖 열린 파일 핸들** — 미리보기 서버 네트워크 핸들을 별도로 확인하고 서버를 종료하지 않은 채 파일만 갱신했다. 통과.
24. **원격 응답 성공이나 오래된 본문** — HTTP 200 외에 각 경로의 byte 길이와 SHA-256이 선택 빌드 manifest와 맞는지 확인했다. 3,269/3,269 통과.
25. **화면은 열리지만 새 안내 누락** — Agent Browser 렌더링 결과에서 제목, 신규 안내와 완료 수 노출을 확인하고 PNG를 저장했다. 통과.
26. **캡처 파일이 빈 이미지/다른 크기** — PNG가 1280×633이고 SHA-256 `be2649fd…`임을 검사했다. 통과.
27. **PR 본문 raw 이미지 경로 오류** — 첫 주소의 commit SHA 오타로 Agent Browser에서 `naturalWidth=0`을 발견했다. 실제 HEAD SHA로 본문 주소를 고친 뒤 `complete=true`, `naturalWidth=1280`, `naturalHeight=633`, 표시 상태를 확인했다. 통과.
28. **대규모 manifest에 토큰·비밀 포함** — 원격 manifest가 경로·상태·길이·해시만 담는지, 쿠키·인증 헤더·credential 필드가 없는지 확인하고 원본 JSON을 gzip으로 보존한다. 통과.
29. **근거가 재현 입력과 분리** — README가 선택 root, 로그, preflight, 사후 검증, PNG와 원격 전체 manifest를 함께 링크할 수 있도록 파일명을 적었다. 통과.
30. **검증 브라우저 잔류** — `spinon-pr441-image-verify` 종료 후 `agent-browser session list`가 빈 배열이고 `ps -axo pid,comm`에서 Agent Browser/Google Chrome 프로세스가 없는 것을 확인했다. 통과.

## 검토 결과

30개 관점 모두에서 병합 전 조치를 확인했다. PR 이미지 오타는 본문에서 수정했고 브라우저 프로세스는 종료했다. 증거는 `agent-browser-tailnet-pass02.json`과 `browser-close-pass02.json`에 기록했다.
