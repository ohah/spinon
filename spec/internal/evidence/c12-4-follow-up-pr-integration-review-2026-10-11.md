# C12.4 후속 검증 PR 통합 검토

**대상 PR:** [#142](https://github.com/ohah/spinon/pull/142) · `docs/c12-4-verification-reconciliation`

1. **base/head가 의도한가?** `main` 대상의 문서·근거 갱신 브랜치다.
2. **GitHub가 병합 가능으로 판단하는가?** 확인 시 PR은 `OPEN`, `MERGEABLE`, `CLEAN`이었다.
3. **커밋 규칙을 지키는가?** 커밋 제목은 `docs:` 접두어 뒤 한국어다.
4. **유형·영역 label이 있는가?** `documentation`, `area: renderer`, `area: docs`를 적용했다.
5. **PR 본문이 변경 범위·명세·잔여 범위를 설명하는가?** 상태 ID, API 미변경, 실행 근거와 미완료 항목을 분리해 적었다.
6. **캡처 이미지가 실제 PR asset으로 첨부됐는가?** `gh pr create --attach`로 첨부했고 본문 참조가 GitHub user-attachments 주소로 바뀌었다. 업로드 자산은 HTTP redirect를 반환한다.
7. **PR 본문에 Tailnet 주소가 섞였는가?** `tailed42f2.ts.net`이나 Tailnet URL은 없다.
8. **GitHub Pages를 배포했는가?** Pages workflow와 `gh-pages`는 건드리지 않았다.
9. **문서 PR이 API 계약을 임의 변경하는가?** 공개·내부 API 변경이 없고 숫자 계약 버전 `0.1.0`도 그대로다.
10. **C12.4를 과도하게 완료 처리하는가?** `spec/STATUS.md`의 C12.4 체크박스는 미완료다.
11. **PR #141 병합 사실과 현재 세 문서가 일치하는가?** GitHub 병합 상태를 확인한 뒤 상태 대장·내부 인덱스·공개 roadmap을 갱신했다.
12. **iOS 표면 비교가 offscreen/screenshot 대체인가?** 새 비교 JSON의 8개 case가 `metal-surface` renderer로 기록됐다.
13. **PR 첨부 이미지가 결과와 연결되는가?** 이미지 파일과 case ID `flex-row-reverse-paint-order-0-0`이 새 iOS Metal surface run에 속한다.
14. **새 iOS matrix가 수정된 fixture를 담는가?** 입력 JSON에 `flex-row-reverse-paint`가 있고 결과 상태는 `matched`다.
15. **matrix 집계와 오차 범위가 맞는가?** 26 scene, 지원 21, 기대 거부 5, 64 node, hit-test 21, 최대 geometry 오차 0 CSS px다.
16. **동시 session 자료가 양 플랫폼·모든 stage를 포함하는가?** iOS Simulator·Android API 37 AVD에서 0·1·2·4·8 stage를 각각 세 번 측정했다.
17. **메모리 수치의 귀속이 안전한가?** README 수치는 raw process sample과 일치하며 worker stack 전용 commit이나 누수로 해석하지 않는다.
18. **AVD 결과를 hardware GPU 또는 실기기로 부르는가?** renderer는 ANGLE/SwiftShader이며 실기기 GPU·RSS는 미측정으로 표기했다.
19. **남은 실패와 적합성 경계를 숨기는가?** Chrome WPT는 oracle로만 분리하고, 기본 test stack 256-depth overflow와 global allocator OOM 미검증을 유지했다.
20. **독자가 결과를 재확인할 수 있고 검사를 과장하지 않는가?** 변경 문서 상대 링크 801개와 최신 Tailnet 페이지·JSON의 HTTP 200을 확인했다. `git diff --check`는 문서 범위에서 통과하고 원본 OS 로그의 trailing whitespace 경고는 원문 보존에 따른다. GitHub에는 이 브랜치 checks가 보고되지 않아 CI 통과라고 쓰지 않았다.

**판정:** 문서 상태·측정 범위·첨부 증거 간 차단 문제가 없다. 원본 플랫폼 로그의 공백은 수집기 출력 그대로 보존했다. PR은 측정되지 않은 하드웨어·전역 OOM·Spinon WPT runner 결과를 주장하지 않는다.
