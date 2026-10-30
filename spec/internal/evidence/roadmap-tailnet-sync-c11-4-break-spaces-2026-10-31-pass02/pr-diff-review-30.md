# C11.4 break-spaces roadmap sync PR diff 검토 30개

## 검토 범위

기준 source HEAD는 `32ba37b18dc5a8fc553667f52cc47d46dae3952f`다. 로드맵, 공식 상태, 내부 계약 0059, 계획과 runtime README의 병합 증거 변경 및 선택 Tailnet publication/evidence를 서로 다른 실패 관점에서 검토했다. 구현 PR의 20개 runtime 관점 검토와 별개로, 이 문서는 문서 동기화 PR의 변경만 다룬다.

1. **공식 완료 수 유지:** `spec/STATUS.md`와 공개 로드맵이 모두 `72/213`을 유지하고 완료 수를 증가시키지 않는다.
2. **C11.4 상태 유지:** 상태 대장의 C11.4 체크박스는 미완료다. 병합 사실을 기능 완료 표기로 오해하게 만들지 않는다.
3. **C11 상위 상태 유지:** C11 상위 항목도 완료로 바뀌지 않았다.
4. **병합 PR 식별:** #873 링크가 구현 PR을 가리킨다. 병합 사실은 문서에 기록하되 docs PR 자체와 혼동하지 않는다.
5. **merge SHA 정확성:** 계획, 상태, 계약, runtime 근거, 로드맵에서 `32ba37b18dc5a8fc553667f52cc47d46dae3952f`가 일관된다.
6. **병합 시간 정확성:** 문서의 `2026-10-30T20:02:37Z`는 PR의 병합 시각과 비교 확인한다.
7. **CI 과장 금지:** 비어 있는 check rollup과 required check 부재를 성공으로 바꾸어 쓰지 않았다.
8. **Android 행렬 범위:** 로드맵은 API 37 ARM64 AVD에서 26/26을 기록하고 실기기라고 부르지 않는다.
9. **iOS 행렬 범위:** 로드맵은 iPhone 17 Pro / iOS 26.2 Simulator에서 26/26을 기록하고 실기기로 확대하지 않는다.
10. **Android renderer 경계:** ANGLE/Vulkan/SwiftShader software renderer라는 한계를 유지한다.
11. **iOS 제출 경계:** compositor 제출 전 관찰이라는 한계를 유지한다.
12. **glyph pixel 주장 방지:** 화면 캡처를 glyph raster 정합성 증거라고 표현하지 않는다.
13. **workspace 실패 공개:** 변경 밖 C07.3 비교 실패와 기대·실제 computed string이 유지된다.
14. **일반 CSS admission 경계:** 제한 opt-in fixture를 일반 author CSS 지원으로 확대하지 않는다.
15. **전체 C11.4/C11 경계:** root HTML mapping, 전체 WPT, 상위 단계의 미완료 범위를 그대로 명시한다.
16. **로드맵 순서:** 신규 BreakSpaces 카드는 이전 PreWrap 항목보다 앞에 있다. publication staging에서 순서를 assertion으로 확인했다.
17. **상태 페이지 렌더링:** Agent Browser에서 `72/213`, merge SHA, C11.4 미완료 checkbox를 확인했다.
18. **계획·상태 일치:** 계획 문서와 공식 상태가 동일한 PR 병합 사실과 미완료 수치를 말한다.
19. **계약 버전 불변:** 내부 계약 0059의 내용 보강만 했고 숫자 버전을 바꾸지 않았다.
20. **runtime 증거 중복 방지:** runtime README에 merge note가 한 번만 있다.
21. **이미지 상대 경로:** Rspress 실패 재현으로 드러난 두 PNG 링크를 `./screenshots/...`로 고쳤고 빌드 결과에 둘 다 포함됐다.
22. **이미지 실제 로드:** Agent Browser에서 Android 1080×2400과 iOS 1206×2622 이미지의 `complete` 및 `naturalWidth`를 확인했다.
23. **페이지 링크 경로:** 공개 로드맵 링크는 `/spinon/` base 아래의 plan·contract·runtime·screenshots·STATUS 경로를 사용한다.
24. **선택 route 범위:** publication manifest 104개 route와 필요한 BreakSpaces 페이지·PNG를 검사했다. 전체 문서 링크 검증으로 주장하지 않는다.
25. **Rspress 버전 고정:** 문서 빌드는 2.0.22이며 의존성 파일의 버전을 바꾸지 않았다.
26. **빌드 산출물 격리:** Rspress root와 output은 전용 `build/spinon/.../pass02` 아래에 있고 추적 중인 문서 산출물을 덮어쓰지 않는다.
27. **원격 콘텐츠 일치:** HTTPS 104/104 route의 응답 길이와 SHA-256이 선택 publication manifest에 일치한다.
28. **기존 파일 복구 자료:** 11개 교체 파일을 사전 백업했고 결과에서 backup SHA 11/11을 확인했다.
29. **복사 안전성:** preflight의 open handle은 0개였고 copy/verify 결과 모두 삭제 없음 및 server 미재시작을 기록한다.
30. **작업 세션 정리·범위:** terminal-browser를 쓰지 않고 Agent Browser를 사용했다. `close --all` 뒤 session은 inactive/PID 없음이며 unrelated untracked 경로는 stage 대상에 포함하지 않는다.

## 확인 결과

30개 관점에서 문서 변경의 병합 식별, 공식 상태와 검증 한계, 선택 publication 및 브라우저 결과가 서로 모순되지 않음을 확인했다. PR 병합 직전에는 GitHub에서 #873의 `state`, `mergedAt`, `mergeCommit`과 문서 PR의 repository policy/check state를 다시 조회한다.
