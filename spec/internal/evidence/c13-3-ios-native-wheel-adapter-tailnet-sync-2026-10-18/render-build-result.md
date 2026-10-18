# 선택 문서 렌더 결과

- RSPress: `2.0.22` (프로젝트 고정 버전)
- 명령: `RSPRESS_PERSISTENT_CACHE=false rspress build --config rspress.tailnet-sync.config.ts`
- 선택 범위: C13.3 계획, 상태 대장, 내부 인덱스, 내부 계약 0063, iOS adapter 실행 근거, adapter 구현·PR 변경 검토, 이번 Tailnet 동기화 근거와 변경 검토
- 결과: Markdown 문서 route 10개 렌더 성공, 정적 asset 24개 생성. [build log](./render-build.log)에서 성공 종료를 확인할 수 있다.

선택 root만 빌드했다. 기존 링크와 이미지가 선택 root 밖 Tailnet 파일을 가리키는 경로가 있어 해당 root의 dead-link/image 검사는 비활성화했다. 따라서 전체 저장소 docs 빌드, 전체 링크·이미지 검사 또는 GitHub Pages 배포를 성공으로 나타내지 않는다. 누락 페이지는 기존 Tailnet 사본을 보존하고, 이번에 갱신한 route 및 정적 파일은 별도 served status/hash 확인으로 검사한다.
