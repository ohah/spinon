# PR diff 실패 관점 검토 30개

대상은 `spec/STATUS.md`, C11.4 계획·내부 색인·공개 roadmap·구현 근거의 병합 상태 문구와 이번 Tailnet 동기화 도구·증거다. 구현 PR #362의 코드 범위는 다시 변경하지 않았다.

1. **상태 대장이 PR #362를 미병합으로 남기는가?** merge SHA와 KST 병합 상태를 반영했다. 통과.
2. **계획 문서와 상태 대장의 merge SHA가 다른가?** 둘 다 `033ea09badbf61100675df485679c6e7959d1974`. 통과.
3. **내부 evidence index가 다른 PR을 가리키는가?** #362 링크와 동일한 evidence 경로를 쓴다. 통과.
4. **공개 roadmap이 증거 없이 숫자를 올리는가?** PR #362와 13/13 근거 링크를 붙이고 C11.4 미완료를 표시한다. 통과.
5. **구현 README가 merged 상태를 반영하는가?** 기존 #362 merge SHA와 일치한다. 통과.
6. **HTML 링크가 잘못된 저장소를 가리키는가?** 링크가 `ohah/spinon/pull/362`를 가리킨다. 통과.
7. **공식 체크리스트를 완료로 바꾸는가?** 이번 diff에서 완료 체크나 합계는 증가하지 않는다. 통과.
8. **overflow·WPT·실기기 상태를 과장하는가?** 문서가 미검증 범위를 구체적으로 남긴다. 통과.
9. **sync report가 Tailnet 외부 origin을 노출하는가?** 보고서·browser JSON에는 호스트를 넣지 않았다. 통과.
10. **대상 경로가 달라지는가?** `sync-preview.mjs`의 target guard와 승인된 preview 경로가 동일하다. 통과.
11. **백업이 다른 실행과 섞이는가?** pass10 전용 날짜별 경로를 쓴다. 통과.
12. **기존 staging 파일을 지우거나 덮는가?** prepare helper는 같은 pass staging root가 있으면 중단한다. 통과.
13. **선택되지 않은 근거가 누락되는가?** precomparison·구현 근거와 세 장면의 Android/iOS 자산을 각각 포함한다. 통과.
14. **별칭 route와 `spec/` route가 불일치하는가?** 선택 근거는 두 목적 경로로 복사되고 같은 원본을 쓴다. 통과.
15. **Markdown 상대 이미지가 GitHub의 raw가 아닌 blob 페이지로 연결되는가?** 선택 내부 링크는 로컬 route로 변환하며 browser에서 이미지 여섯 장을 확인했다. 통과.
16. **선택되지 않은 상대 저장소 경로가 깨지는가?** 저장소 main의 blob 경로로 fallback한다. 통과.
17. **scheme·절대·fragment 링크를 잘못 재작성하는가?** link rewriter가 외부 scheme, absolute 및 fragment-only 링크를 그대로 둔다. 통과.
18. **query/fragment suffix가 사라지는가?** rewriter가 경로와 suffix를 분리해 보존한다. 통과.
19. **build root가 기존 docs 사이트를 덮는가?** root와 output에 pass10 전용 `build/spinon` 하위 경로를 쓴다. 통과.
20. **Rspress 설정이 기본 설정을 잃는가?** 기존 config를 spread하고 root/outDir 및 선택 정적 link 검사만 덮어쓴다. 통과.
21. **lockfile이나 의존성이 바뀌는가?** package manifest와 lockfile은 변경하지 않았다. 통과.
22. **절대 host/worktree 경로가 빌드에 남는가?** sanitizer가 redaction 뒤 잔여 경로 0개를 검사한다. 통과.
23. **입력 manifest가 자기 자신으로 순환하는가?** 새 manifest와 이전 최종 report manifest를 제외한다. 통과.
24. **SHA 목록이 실제 선택 입력과 무관한가?** 새 증거 디렉터리의 도구가 332개 선택 원본의 SHA-256을 생성한다. 통과.
25. **복사 단계가 오래된 preflight를 허용하는가?** pass, HEAD, base path 및 backup path를 비교한다. 통과.
26. **복사 도중 preview 대상이 바뀌는가?** 기존 대상의 크기와 SHA를 다시 확인하고 다르면 중단한다. 통과.
27. **교체 전 원본이 보존되지 않는가?** 교체 33개 모두 전용 backup에 기록되고 검증됐다. 통과.
28. **대상 파일이 부분 기록 상태로 노출되는가?** 같은 디렉터리 임시 파일을 쓰고 rename한다. 통과.
29. **실제 HTTP 서비스가 새 byte를 제공하는가?** 526 route 각각 HTTP 200, 크기, SHA 일치다. 통과.
30. **브라우저 증거가 종료 상태와 충돌하는가?** 캡처 후 전용 session inactive 및 process 0개를 기록했다. 통과.
