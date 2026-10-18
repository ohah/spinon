# Tailnet 미리보기 선택 route 렌더 결과

- RSPress `2.0.22`로 선택 root만 빌드했다. 임시 config는 `/spinon/` base와 저장소 theme/CSS를 사용했다.
- 입력은 Markdown 30 route 및 정적 HTML 2개다. 출력은 `404.html` 포함 HTML 33개, 전체 정적 파일 107개, source 총 `7,885,046` bytes다.
- Build exit code `0`. `markdown.link.checkDeadLinks`와 `markdown.image.checkDeadImages`는 선택 root의 제한된 문서 집합에 필요한 범위만 만들기 위해 끄었다. 전체 저장소 문서·링크·이미지 검사나 Pages 배포 결과가 아니다.
- Tailnet 사전 비교에서 파일 40개 생성, 28개 교체, 39개 동일을 확인했다. 교체 전 사전 manifest와 backup hash를 기록하고, 삭제는 하지 않는다. 사전 대상은 [preflight manifest](./preflight-pass-1.json)에서 확인한다.
- Build log는 [render-build.log](./render-build.log)이고 실행 종료 코드는 [render-build.exit](./render-build.exit)다.
