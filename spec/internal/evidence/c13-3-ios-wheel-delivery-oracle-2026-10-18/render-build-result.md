# C13.3 standalone UIKit 대조 선택 문서 렌더 결과 · 2026-10-18

- RSPress: 저장소 고정 버전 `2.0.22`를 workspace `docs:build` script로 실행했다.
- 출력 경로는 빌드 전 존재하지 않았고, 전체 문서 output과 분리된 새 경로에서 생성했다.
- 결과: Markdown 19 route, 정적 HTML 2개, `404.html` 포함 HTML 22개, 전체 파일 72개다. 현재 output은 4,235,740 bytes이며 정확한 route와 파일 수는 [manifest](./route-manifest.json)에 있다.
- 선택 페이지에는 `STATUS`, 명세 인덱스와 필요한 상위 링크 페이지, 내부 계약 0063, C13.3 wheel 계획, 실험 README·검토 3개·이 렌더 결과, `public/roadmap.html`, 기존 정적 runtime-evidence 1개가 포함됐다. 새 Agent Browser roadmap 캡처도 정적 이미지 자산으로 포함했다.
- 출력은 전체 문서 경로·링크 검사 결과가 아니다. 선택 설정에서 선택 범위 밖의 문서 링크와 이미지 전체 확인을 껐다. GitHub Pages 배포는 하지 않았다.
- [빌드 log](./render-build.log)에는 고정 RSPress 실행의 성공 결과가 있다.

저장소 전체 RSPress build 시도는 별도로 기존 `.js` capture route의 SSR 전역(`window`/`innerWidth`), Shiki `rust,ignore`, 상대 image import 오류로 실패한 기록이 있다. 위 선택 route build 성공을 전체 문서 build나 전체 링크 적합성으로 확대하지 않는다.
