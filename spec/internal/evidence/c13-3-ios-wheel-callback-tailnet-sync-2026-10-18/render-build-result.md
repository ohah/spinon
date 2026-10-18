# 선택 문서 렌더 결과

- RSPress: `2.0.22` (프로젝트 고정 버전)
- 명령: `RSPRESS_PERSISTENT_CACHE=false rspress build --config rspress.tailnet-sync.config.ts`
- 범위: 선택한 C13.3 상태 대장·계획·내부 계약·iOS adapter evidence·Tailnet 동기화 evidence
- 결과: Markdown 15 route와 정적 roadmap 1개를 렌더해 16개 route HTML을 출력했다. `404.html`을 포함한 HTML은 17개이며 전체 파일은 53개다. Web/node 빌드 exit code는 0이다. 상세 log는 [render-build.log](./render-build.log)에 있다.
- 빌드 디렉터리는 새 경로로 시작했다. RSPress는 root 바깥 output을 자동 정리하지 않는다고 경고했으나, 이 실행의 output은 사전 존재하지 않았으므로 오래된 산출물은 섞이지 않았다.
- 선택 root에서 불러오지 않은 기존 링크와 이미지가 있어 dead-link/image checks는 비활성화했다. 따라서 전체 저장소 docs build, 전체 링크·이미지 검사, Pages 배포 결과가 아니다.
