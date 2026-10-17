# 선택 Rspress 렌더 결과

## 실행 정보

- 기준 커밋: `54a39a3e7403dd7fc9178de17daeafec4f00ec88` (PR #272 rebase merge 뒤 `origin/main`)
- 선택 route의 문서 overlay: `spec/internal/README.md`의 동기화 근거 링크와 이 evidence 폴더의 `README.md`. 둘 다 후속 동기화 근거 PR의 변경이며 런타임 코드는 overlay에 포함하지 않았다.
- Rspress: `2.0.22`
- 빌드 결과: 종료 코드 `0`; web/node bundle 빌드와 정적 페이지 렌더 성공
- 산출물: `/private/tmp/spinon-c04-12-root-html-roadmap-20261017/out-v7`
- 산출물 수: 정적 파일 56개, HTML route 22개, 총 2,785,889 bytes
- 원본 로그: [selected-build.log](selected-build.log)

## 선택 경로와 한계

임시 문서 트리에는 홈페이지, STATUS, 명세 인덱스, root `_meta.json`에 필요한 일곱 개 기본 문서, API index, internal index, 계약 0077, C04.12 계획·관련 검토 및 이 동기화 근거를 넣었다. 기본 문서는 병합된 main의 파일을 사용하고, 위 두 route 문서만 후속 evidence PR의 overlay를 포함했다. Rspress 설정은 dead-link 검사를 끄고 이 문서 트리만 대상으로 했다. `public` 폴더에는 icon, logo, roadmap HTML/SVG만 포함했다.

전체 `mise exec -- bun run docs:build`는 저장소의 변경 밖에 있는 기존 dead image link·Shiki `rust,ignore` 언어 오류 등으로 실패했다. 따라서 이 선택 빌드는 전체 사이트·저장소 dead-link 통과를 뜻하지 않는다. 선택 출력 경로가 문서 root 바깥이므로 Rsbuild clean 경고가 있었고, 고유한 비공개 `/private/tmp` 출력 경로에서만 만들었다.

## 확인한 페이지

- `/spinon/roadmap.html` 및 `/spinon/STATUS.html`
- `/spinon/internal/README.html`, `/spinon/internal/0077-c04-12-root-html-document.html`
- C04.12 기본 앱 실행·계획·구현 검토·PR diff 검토·Tailnet 동기화 근거
- root/API 탐색에 필요한 Rspress page와 선택 route들의 CSS/JS chunk

앱의 HTML 화면 렌더를 검증한 빌드가 아니다. 문서 페이지의 로드와 Tailnet에 복사한 정적 파일만 확인한다.
