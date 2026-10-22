# C11.4 교차 축 span groups 공개 로드맵 PR diff 실패 관점 검토 30개

**대상:** `spec/public/roadmap.html`의 PR #510 런타임 요약 1개와 연결된 증거 경로. 구현/runtime 검토 및 구현 PR diff 검토를 반복하지 않고, 공개 진행 문구의 사실성·수치·링크·완료 상태 경계를 확인했다.

| # | 실패 관점 | diff 판정 |
|---:|---|---|
| 1 | 공개 로드맵 변경이 계획 문단을 런타임 결과로 잘못 바꾸거나 다른 C11.4 작업을 덮는가? | 기존 cross-axis span groups 계획 행 하나만 #510 런타임 요약으로 교체했다. |
| 2 | 요약 제목이 검증한 `minmax(calc())` 교차 축 span groups 작업과 일치하는가? | 계획과 runtime README의 작업명을 그대로 유지했다. |
| 3 | 연결한 PR 번호가 이 구현 slice를 가리키는가? | PR 링크와 공개 PR 메타데이터 모두 #510이다. |
| 4 | 병합 방식이 프로젝트 규칙과 일치하는가? | PR 메타데이터상 Rebase and merge다. |
| 5 | 공개한 병합 시간이 UTC로 정확한가? | PR의 `mergedAt` `2026-10-22T09:59:19Z`와 일치한다. |
| 6 | 공개한 merge SHA가 실제 main merge commit과 일치하는가? | PR 메타데이터와 `origin/main` 최신 커밋 `e9f5f45579841c1fd78b1dc981687b468531cd64`가 일치한다. |
| 7 | 빈 check rollup을 CI 통과로 오해하게 쓰거나 필수 check가 있다고 암시하는가? | rollup이 비었고 main required status checks가 설정되지 않았음을 함께 적었다. |
| 8 | 공식 완료 수가 구현 slice만으로 증가하는가? | 기존 `77/213`을 그대로 표시한다. |
| 9 | 부모 C11.4 항목이 이 slice만으로 완료된 것처럼 표시되는가? | C11.4 checkbox를 유지한다고 적고 완료 주장을 하지 않는다. |
| 10 | 네 개 고정 scene을 실행한 근거보다 scene 수를 부풀리는가? | 두 intrinsic 및 두 definite forward/reverse의 네 scene이라고 적었다. |
| 11 | DOM 역순 장면이 별도 기능이나 다른 입력으로 오기재되는가? | 순서 변형을 forward/reverse로만 설명해 비교 목적을 유지한다. |
| 12 | intrinsic scene의 sizing mode가 계획·runtime evidence와 일치하는가? | intrinsic 결과로 분류하고 수치는 README의 intrinsic row와 대조했다. |
| 13 | intrinsic root 크기가 content box와 border box를 혼동하는가? | evidence 표의 root border box `55×24px`를 그대로 썼다. |
| 14 | 독립 기대 Columns 값을 실제 구현 출력으로 잘못 표기하는가? | 기대 배열과 실제 배열을 별도로 표시했다. |
| 15 | 실제 intrinsic Columns 값이 캡처 결과와 다른가? | `[15.666666]*3`으로 runtime README의 실제값과 일치한다. |
| 16 | float 오차 또는 허용치를 과장해 정확 일치라고 주장하는가? | 약 `0.01042px`와 `1/64px` 허용치를 구분해 적었다. |
| 17 | intrinsic Rows 값이 누락되거나 definite 결과와 섞이는가? | intrinsic Rows `[9,9]`를 별도 표기했다. |
| 18 | definite control의 입력 폭이 잘못 설명되는가? | 40px content width로 runtime 입력과 일치한다. |
| 19 | definite root의 content 크기를 border-box 크기로 오기재하는가? | root border box `44×34px`로 README와 일치한다. |
| 20 | definite Columns 결과가 oracle 기대값과 혼동되는가? | 실제/기대가 같은 exact 배열 `[12,12,12]`로 적었다. |
| 21 | definite Rows 결과가 다른 scene의 값을 가져오는가? | `[14,14]`이며 README 표와 일치한다. |
| 22 | Android 환경이 실기기 또는 hardware GPU로 확대 해석되는가? | API 37 ARM64 AVD라고 명시하고 실기기·hardware GPU를 미검증에 둔다. |
| 23 | iOS Simulator 환경이 실기기로 표현되는가? | iPhone 17 Pro / iOS 26.2 Simulator로 한정했다. |
| 24 | Android 표본 수가 두 좌표·두 definite scene의 합계와 맞는가? | PixelCopy definite forward/reverse 표본 4/4 exact로 evidence의 2×2 결과와 일치한다. |
| 25 | Metal 표본을 compositor 이후 화면 검증으로 오해할 수 있는가? | 제출 전 drawable이라고 특정하고 compositor 이후 pixels는 미검증으로 남겼다. |
| 26 | sample 결과가 CSS border stroke를 확인했다고 확대되는가? | border stroke를 미검증으로 명시했다. |
| 27 | Ahem 기반 geometry 장면이 glyph raster까지 증명한다고 읽히는가? | glyph raster를 미검증으로 명시했다. |
| 28 | opt-in fixture 확인이 기본 제품 archive에 포함된 것으로 표현되는가? | 기본 archive 제외와 opt-in archive 포함을 분리해 적었다. |
| 29 | 네 scene archive 검사 결과가 일반 CSS admission이나 WPT 지원으로 확대되는가? | WPT·일반 CSS admission과 전체 C11.4를 미검증으로 명시했다. |
| 30 | 공개 링크가 계획, 증거, 두 Simulator 화면, PR, 상태 원본으로 연결되는가? | 각 경로와 PR 링크가 대상 파일·페이지를 가리키며 상태 링크는 완료 수/checkbox 경계를 유지한다. |

**결론:** 이 공개 문구에서 수정할 오표기는 발견하지 못했다. 런타임 수치와 한계는 [runtime README](README.md), 병합 정보는 [PR #510](https://github.com/ohah/spinon/pull/510), 공식 완료 상태는 `spec/STATUS.md`에 대조했다.
