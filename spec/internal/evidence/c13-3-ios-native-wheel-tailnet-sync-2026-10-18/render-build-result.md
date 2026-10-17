# C13.3 Tailnet 문서 렌더 빌드 결과

## 선택 빌드

Rspress `2.0.22`로 Tailnet에서 갱신할 C13.3 계획 페이지, 내부 인덱스, iPadOS preflight 자료와 기존 Tailnet의 C04.12 선택 문서 root를 렌더링했다. `selected-build-final.log`는 성공 종료를 기록하고 29개 HTML route를 만들었다. 빌드는 전체 저장소 문서 적합성 검사가 아니다.

## 전체 root 빌드 제한

`bun run docs:build`는 저장소에 이미 존재하는 dead-link 오류 때문에 실패했다. 전체 root에서 dead-link 검사를 끈 별도 시도도 기존 MDX 이미지 import와 Shiki language token 문제로 실패했다. 전체 root 결과를 성공으로 표시하거나 전체 문서 링크가 검증됐다고 주장하지 않는다.

선택 root에 모든 기존 C13.3 evidence를 추가한 중간 시도는 기존 `c13-3-style-invalidation` 자료가 참조하는 저장소 누락 이미지에서 실패했다. 실패 출력은 `selected-build-broad-c13-failed.log`에 남겼다. 이번 배포 root에는 변경된 C13.3 계획·내부 인덱스와 새 preflight 자료만 선택했고, 기존 C13.3 실행 자료 route는 이미 Tailnet에 제공되던 파일을 유지했다. 공개 로그에서는 Tailnet DNS, 사용자 경로와 process ID를 가렸으며 build 오류·HTTP status·hash 결과는 보존했다.

## 판정

선택한 Tailnet 변경 페이지 렌더는 성공했다. 전체 문서 build, 전체 링크·이미지 적합성, GitHub Pages 배포는 수행하지 않았다.
