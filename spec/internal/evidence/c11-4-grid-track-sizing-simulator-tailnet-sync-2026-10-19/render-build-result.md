# 선택 문서 렌더 결과

- RSPress `2.0.22`로 선택된 C11 문서·근거와 정적 로드맵을 빌드했다. Markdown 문서 route 24개와 공용 CSS·JS/search assets, static roadmap 및 404 page를 만들었다. 최종 빌드는 성공했다. [최종 build log](./render-build-pass-2.log), [첫 반영 build log](./render-build.log).
- 선택 빌드에서는 `markdown.link.checkDeadLinks`와 `markdown.image.checkDeadImages`를 끄고, 전체 문서 build나 전체 링크·이미지 적합성을 판정하지 않았다.
- Browser 검토에서 C11.4 계획 페이지의 `../spec/...` 링크가 미리보기 경로를 벗어나는 점을 발견해 `/spinon/...html` 경로로 바로잡았다. 상태 대장의 C11 Grid 및 C11.4 계획 링크도 `/spinon/plan/...html`로 고쳤다. 수정 뒤 선택 렌더와 실제 페이지 링크를 다시 확인했다.
- [첫 pass preflight](./preflight.json) 및 [pass 2 preflight](./preflight-pass-2.json)에는 각 경로의 원본·기존 SHA-256이 있다. 각 pass는 모든 변경 파일을 복사 전에 외부 backup으로 옮겨 검증했다. 선택 파일은 임시 파일에서 원자적으로 교체했다. 어떤 파일도 삭제하지 않았다.
- pass 1: 120개 항목 중 새 파일 63개, 교체 22개, 동일 35개. backup 22/22, copy hash 120/120, HTTP 200 및 served hash 120/120.
- route 수정 후 pass 2: 120개 항목 중 새 파일 2개, 교체 26개, 동일 92개. backup 26/26, copy hash 120/120, HTTP 200 및 served hash 120/120. 두 번째 pass에서 미리보기 listener를 재시작하지 않았다.
- 상세 대조: [backup pass 1](./backup-verification.json), [copy pass 1](./copy-verification.json), [served pass 1](./served-verification.json), [backup pass 2](./backup-verification-pass-2.json), [copy pass 2](./copy-verification-pass-2.json), [served pass 2](./served-verification-pass-2.json).
