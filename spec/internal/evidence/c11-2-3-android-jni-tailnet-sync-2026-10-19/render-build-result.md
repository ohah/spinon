# 선택 문서 빌드 결과

- 기준 소스: PR #328 병합 커밋 `d50c6cc73317050fe078299ce0f63618b83869bc`.
- 렌더러: Rspress 2.0.22.
- 입력: 공식 `STATUS.md`, C11.2.3 JNI gate 계획, 계획·구현·PR 검토 및 실행 근거 문서, 공개 `roadmap.html`과 필요한 정적 자산.
- 결과: 선택 빌드 성공. `404.html`을 제외한 출력 34개 파일, 총 2,078,379 bytes.
- 설정: 선택 root에서만 dead-link/image 검사를 비활성화했다. 전체 저장소 docs 적합성 검증으로 해석하지 않는다.
- 전체 저장소 빌드: 별도로 시도했지만 기존 문서 dead-link 검사에서 569개 source-file 진단으로 실패했다. 대표 경로와 요약은 [오류 요약](./full-site-build-failure-summary.json)에 있다. 원본 bundler trace는 사용자별 절대 경로를 포함해 저장소에 넣지 않았다.
- 로그: [선택 빌드 로그](./selected-build.log).
