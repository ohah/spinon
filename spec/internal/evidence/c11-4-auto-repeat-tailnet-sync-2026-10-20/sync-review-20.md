# Tailnet 동기화 실패 관점 검토 20개

기준 commit `033ea09badbf61100675df485679c6e7959d1974`, 선택 site `pass10`, 대상 `/spinon/`을 검토했다.

1. **다른 사이트 경로가 덮이는가?** 대상 helper가 `~/Library/Application Support/SpinonPreview`만 허용한다. 통과.
2. **다른 백업 위치를 쓸 수 있는가?** 이번 날짜와 pass10으로 고정된 backup 경로 외에는 거부한다. 통과.
3. **선택 밖 파일을 삭제하는가?** copy helper는 create·replace·same만 처리하고 삭제 단계가 없다. preflight와 copy 기록 모두 `no_deletes: true`. 통과.
4. **기존 사용 파일을 교체하는가?** 교체 33개에 `lsof`를 실행했고 열린 경로 0개였다. 통과.
5. **열린 파일을 확인한 뒤 대상이 바뀌었는가?** copy 직전 대상의 존재·크기·SHA를 preflight와 다시 대조한다. 통과.
6. **원본 build가 preflight 뒤 바뀌었는가?** 복사 직전 각 파일 크기와 SHA를 preflight 값과 재대조한다. 통과.
7. **잘못된 source directory를 sync하는가?** `build/spinon/c11-4-auto-repeat-tailnet-site-pass10`만 허용한다. 통과.
8. **staging Markdown을 사이트 파일로 복사하는가?** sanitized site root만 source로 받고, staging root는 거부한다. 통과.
9. **임시 파일이 다른 파일과 충돌하는가?** 목적 파일과 같은 디렉터리에 PID suffix 임시 파일을 배타 생성한 뒤 rename한다. 통과.
10. **교체 전 byte를 잃는가?** 교체 전 파일 33개를 전용 backup에 기록하고 byte·SHA를 각각 대조했다. 33/33 통과.
11. **이미 존재하는 backup을 잘못 덮는가?** 같은 SHA인 기존 backup만 재사용하고 내용이 다르면 중단한다. 통과.
12. **심볼릭 링크를 따라 쓰는가?** source·target·backup 경로 구성요소와 선택 build 항목에서 symlink를 거부한다. 통과.
13. **selected output에 host path가 남는가?** sanitizer 뒤 검사에서 잔여 host path가 0개였다. 통과.
14. **선택 문서의 상대 링크가 `/spinon/` 밖으로 나가는가?** 준비 단계에서 매핑되는 내부 문서는 local route로, 비선택 저장소 링크는 GitHub `main`으로 재작성했다. 실제 브라우저 네 페이지를 확인했다. 통과.
15. **PR #362를 미병합으로 표시하는가?** 공개 로드맵·상태·계획·실행 근거에 merge SHA `033ea09...`가 표시된다. 통과.
16. **C11.4 전체 완료로 과장하는가?** 체크와 완료 수를 유지하고 남은 범위 및 미검증 조건을 함께 표시한다. 통과.
17. **필수 CI 미설정을 CI 통과로 표현하는가?** 상태 페이지에는 required status check가 설정되지 않았다고 표시한다. 통과.
18. **Simulator 화면 링크가 깨지는가?** 구현 근거 페이지에서 Android·iOS 여섯 장이 모두 자연 크기로 로드됐다. 통과.
19. **복사 완료가 서버 반영을 증명하지 못하는가?** Tailnet HTTP 응답 526개 모두 로컬 byte·SHA와 대조했다. 526/526 통과.
20. **검증 브라우저가 남는가?** 전용 Agent Browser 세션을 닫고 session inactive 및 agent-browser process 0개를 확인했다. 통과.
