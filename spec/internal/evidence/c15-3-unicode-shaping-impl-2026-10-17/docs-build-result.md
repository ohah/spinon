# C15.3 문서 빌드 결과

- 실행: `mise exec -- bun run docs:build`
- 도구: Rspress 2.0.22
- 결과: 실패. 전체 저장소에서 dead link가 있는 Markdown source 473개를 보고했다.
- C15.3 구현 README는 실패 source 목록에 나타나지 않았다. 변경된 `spec/STATUS.md`와 `spec/internal/README.md`는 이미 있던 다른 plan 경로와 PNG/JSON/log 첨부 링크를 보고했다. 이번 변경으로 추가한 C15.3 링크는 오류 목록에 없다.
- 변경된 Markdown 8개 파일의 상대 링크 1,349개를 로컬 파일 대상으로 따로 확인했다. 누락 5개는 `STATUS.md`의 기존 `/roadmap.svg`, C12.4 로그 2개, C11.3.1 Android/iOS README 2개다. 새 C15.3 링크는 모두 해석됐다. 이 결과는 전체 문서 빌드를 통과한 것으로 취급하지 않는다.

1.6 MiB 원본 Rspress 출력은 473개 오류 source를 반복해 포함하므로 PR evidence에서 제외했다. 정확한 재현 명령과 범위는 위에 보존했다.
