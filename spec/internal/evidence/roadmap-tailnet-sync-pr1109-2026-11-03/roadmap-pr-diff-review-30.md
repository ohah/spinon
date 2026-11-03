# C11.1 PR #1109 병합 상태·공개 로드맵 후속 PR diff 검토 30개

검토 대상은 구현 PR #1109 병합 뒤 공식 상태 대장, 내부 계약, 런타임 근거와 공개 로드맵의 기록을 맞춘 문서 변경이다. 구현 코드는 변경하지 않는다. PR #1109의 실제 병합 상태는 GitHub `MERGED`, 시각 `2026-11-03T06:35:47Z`, merge SHA `9aa813727c79e274ce4d230ede2b0accd261fd87`로 확인했다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | PR 번호가 구현이 아닌 후속 문서 PR을 가리킨다 | 통과 — 기록은 구현 PR #1109의 링크와 별도 후속 변경을 구분한다. |
| 2 | 구현 PR이 실제로 병합되지 않았는데 병합으로 적는다 | 통과 — GitHub PR 상태가 `MERGED`다. |
| 3 | 병합 시각이 GitHub 값과 다르다 | 통과 — `2026-11-03T06:35:47Z`와 일치한다. |
| 4 | merge SHA를 PR head 또는 이전 base와 혼동한다 | 통과 — 전체 SHA `9aa813727c79e274ce4d230ede2b0accd261fd87`를 기록한다. |
| 5 | `spec/STATUS.md` 완료 수를 단일 slice 병합 때문에 올린다 | 통과 — 공식 `72/213`을 유지한다. |
| 6 | C11 상위 체크를 제한 구현 때문에 완료한다 | 통과 — 체크 상태를 미완료로 유지한다. |
| 7 | 내부 계약에서 bounded candidate를 전체 CSS Grid 지원으로 넓힌다 | 통과 — section 제목과 본문 모두 bounded PR #1109로 한정한다. |
| 8 | C11.1도 완료됐다고 확대 해석한다 | 통과 — 계약은 C11.1/C11 checkbox를 변경하지 않는다고 명시한다. |
| 9 | 공개 CSS 제품 지원이나 안정 API를 새로 선언한다 | 통과 — 공개 CSS 지원 상태와 공개 API 여부를 바꾸지 않는다. |
| 10 | 계약의 `0.1.0`을 문서 수정으로 올린다 | 통과 — 숫자 버전 `0.1.0`을 유지한다. |
| 11 | 실험 경로의 `minmax(auto, fr)` 범위를 일반 grammar로 서술한다 | 통과 — 일반 `minmax(auto, …)` 문법은 지원하지 않는다고 남긴다. |
| 12 | 단일 track profile의 두 축 범위를 누락한다 | 통과 — columns와 rows의 제한 경로를 기록한다. |
| 13 | Rust focused test 4/4가 더 넓은 test suite 결과로 표현된다 | 통과 — focused 결과만 적고 전체 suite 통과를 주장하지 않는다. |
| 14 | style-to-layout 6/6 결과를 C11 전체 검증으로 표현한다 | 통과 — bounded 검증 수치로만 기재한다. |
| 15 | Android AVD를 실기기 또는 hardware GPU 결과로 오인한다 | 통과 — API 37 ARM64 AVD와 ANGLE/SwiftShader 소프트웨어 한계를 유지한다. |
| 16 | iOS Simulator drawable을 compositor 이후 사용자 화면으로 오인한다 | 통과 — Metal drawable은 compositor 제출 전이라고 명시한다. |
| 17 | PixelCopy 4/4를 Android 전체 surface 또는 기기 호환성으로 일반화한다 | 통과 — 네 표본의 고정 AVD 근거로 한정한다. |
| 18 | simulator 4/4를 iOS 실기기 pixel parity로 일반화한다 | 통과 — 실기기 및 compositor 이후 픽셀은 미검증으로 유지한다. |
| 19 | PR check 부재를 CI 성공으로 적는다 | 통과 — check가 없고 required checks도 없어서 CI 통과로 세지 않는다고 명시한다. |
| 20 | default artifact의 capture 경계를 문서 갱신에서 빠뜨린다 | 통과 — runtime README에 기존 artifact 경계와 iOS scan 결과가 유지된다. |
| 21 | 기본 Android capture가 실제 실행됐다고 잘못 읽힐 수 있다 | 통과 — capture BuildConfig가 `false`이고 HTML boot payload가 APK에 없다는 사실을 보존한다. |
| 22 | 원본 runtime 근거가 문서만으로 대체된다 | 통과 — 로그, 화면, hash, artifact 링크와 원본 실행 기준 HEAD를 유지한다. |
| 23 | C11 plan 링크가 잘못된 public route를 가리킨다 | 통과 — `plan/c11-root-html-grid-auto-minimum.md`가 tracked source에 있다. |
| 24 | runtime README route가 틀리거나 다른 slice로 연결된다 | 통과 — `c11-root-html-grid-auto-minimum-runtime-2026-11-03/README.md`가 정확한 대상이다. |
| 25 | 공개 상태 링크가 일반 문서 루트 대신 다른 상태로 연결된다 | 통과 — 기존 `/spinon/STATUS.html#css-구현-체크리스트` route를 사용한다. |
| 26 | 공개 카드의 새 PR 링크에 번호·저장소가 틀리다 | 통과 — `ohah/spinon/pull/1109`로 연결한다. |
| 27 | 공개 로드맵 최신 카드의 시간 순서가 오래된 카드 뒤에 놓인다 | 통과 — 06:35Z 병합 기록을 04:59Z #1106 앞에 둔다. |
| 28 | 로드맵 설명이 merge SHA를 누락하거나 일부만 기록한다 | 통과 — 전체 40자리 merge SHA를 표시한다. |
| 29 | 후속 문서 PR 기록 자체가 자동 GitHub Pages 배포를 암시한다 | 통과 — 변경은 사용자 지정 Tailnet preview sync 대상이며 Pages workflow는 실행하지 않는다. |
| 30 | 구현 범위 밖의 WPT·실기기·hardware GPU·최종 합성 픽셀을 완료로 표현한다 | 통과 — 네 경계를 공개 카드와 내부 계약에서 미완료로 남긴다. |

검토는 병합 상태, 계수, 범위, 플랫폼 증거, artifact 경계와 문서 route를 서로 다른 실패 관점으로 대조했다. Tailnet에 복사한 파일의 HTTP 상태·SHA 및 화면 배치는 PR 병합 뒤 별도 게시 근거로 기록한다.
