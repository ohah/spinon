# PR #1299 병합 후 문서 변경 실패 관점 검토 30개

검토 대상은 PR #1299 병합 결과를 반영한 `spec/STATUS.md`, 내부 계약 `0059`, C11.4 계획, 구현 근거 README, 공개 `roadmap.html`, Tailnet 게시 기록이다. 각 항목은 서로 다른 잘못된 상태·범위 확장·게시 오류 경로를 확인한다.

1. **PR 상태 확인:** GitHub PR #1299를 직접 조회해 `MERGED`인지 대조했다. `MERGED`다.
2. **병합 시각:** GitHub의 `mergedAt`과 문서의 `2026-11-08T23:28:36Z`가 일치한다.
3. **병합 커밋:** PR merge commit과 문서 SHA `f0c18c8fead17fb379b77eb6191193a066a5115d`가 일치한다.
4. **병합 방식:** 프로젝트 규칙의 선형 `Rebase and merge` 흐름과 병합 기록이 일치한다.
5. **CI 과장 방지:** PR status-check rollup은 빈 배열이다. 문서가 CI 성공으로 기록하지 않는다.
6. **C11 상위 상태:** 공식 C11 checkbox가 미완료로 남아 있다.
7. **C11.4 상태:** intrinsic track sizing checkbox가 미완료로 남아 있다.
8. **완료 집계:** 공식 구현 완료 수 `78/214`를 유지한다.
9. **내부 계약 버전:** 후보 구현만으로 내부 계약 `0.1.0`을 올리지 않는다.
10. **후보와 공식 지원 구분:** 제한 구현을 일반 CSS/Grid 지원으로 표현하지 않는다.
11. **입력 문맥 경계:** Root HTML 경로에 한정된 조건이 계획과 실행 근거에 남아 있다.
12. **Track 위치 경계:** `fit-content()` 위치 2·3만 추가한 것으로 기록한다.
13. **허용 순서 경계:** 각 위치에서 정해진 tail 순서 30개만 허용한 것으로 설명한다.
14. **기존 position-1 보존:** 기존 위치 1 slice의 회귀 결과를 구분해 기록한다.
15. **Auto-placement 경계:** 새 위치에서 9-item auto-placement 입력은 거부한다는 근거가 남아 있다.
16. **기본 runtime 분모:** 기본 RuntimeSession matrix는 1,920 scene이며 전체 CSS 적합성으로 해석하지 않는다.
17. **Trace 분모:** trace-enabled matrix도 별도 1,920 scene으로 적는다.
18. **거부·복구 경로:** 실패 atomicity와 같은 session 복구를 4/4로 제한해 표현한다.
19. **회귀 분모:** position-1 960 및 선행 810 scene 회귀를 새 matrix와 섞지 않는다.
20. **Android 증거 경계:** API 37 ARM64 AVD 화면을 simulator smoke로만 표현한다.
21. **iOS 증거 경계:** iPhone 17 Pro/iOS 26.2 Simulator 결과를 실기기 결과로 부풀리지 않는다.
22. **PixelCopy 한계:** Android PixelCopy가 수행된 것처럼 쓰지 않는다.
23. **Metal readback 한계:** iOS drawable/compositor pixel equality를 주장하지 않는다.
24. **기기 매트릭스 한계:** 네 sentinel 화면을 전체 native 1,920-scene matrix로 표현하지 않는다.
25. **Roadmap의 병합 카드:** 공개 원본에 PR 번호, merge 시각과 SHA가 한 카드에 연결된다.
26. **Roadmap의 다음 항목:** 다음 C11.4 bounded slice 선정이 queue에 남고 현재 PR을 진행 중으로 중복 표시하지 않는다.
27. **동적 공식 상태:** 칸반은 main의 공식 상태를 읽는 기존 경로를 유지한다.
28. **링크 route:** 계획·사전 비교·구현 근거·검토 링크가 Tailnet `/spinon/` 기준 route를 가리킨다.
29. **미리보기 서버 보호:** Tailnet 제공본 교체는 선택 경로의 해시 확인·백업 뒤 수행하고 서버를 재시작하지 않는다.
30. **범위 밖 자원 보호:** 이번 문서 작업은 iOS Simulator 정의, 기존 다른 프로젝트 Agent Browser, 무관한 Tailnet 경로를 삭제 대상으로 삼지 않는다.

## 결과

30개 관점에서 문서 범위와 근거를 확인했다. Tailnet 게시 이후 제공 route의 HTTP·SHA 및 Agent Browser desktop/mobile 화면 확인 결과는 같은 디렉터리의 publication 기록과 README에 별도로 남긴다.
