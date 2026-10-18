# C14.1 Tailnet 로드맵 변경분 실패 관점 검토 30개

**검토 대상:** `spec/public/roadmap.html`에 추가한 U05.1 파싱 루트 HTML PNG 입력 경로 안내. 기능 코드와 C14.1 실행 검토를 재사용하지 않고 로드맵 문구·링크·지원 경계를 별도로 확인했다.

| # | 실패 가능성 | 확인 결과 |
|---:|---|---|
| 1 | 부분 입력 경로를 U05.1 전체 완료로 읽는가 | 미완료를 명시했다. |
| 2 | 상위 C14.1이 완료로 표시되는가 | C14.1 미완료를 명시했다. |
| 3 | C14가 완료로 오해되는가 | C14 미완료를 명시했다. |
| 4 | fixture 경로를 기본 앱 동작으로 오해하는가 | opt-in C14.1 앱 fixture라고 범위를 한정했다. |
| 5 | 앱 사용자가 기본 경로에서 동작한다고 오해하는가 | 기본 앱 일반 HTML 렌더 경로에 포함되지 않는다고 적었다. |
| 6 | 앱 캡처가 HTML image paint의 화면 증거로 읽히는가 | 캡처가 fixture report 화면이며 이미지 픽셀 paint가 아니라고 적었다. |
| 7 | `<img>`가 일반 이미지 요소 전체 동작을 뜻하는가 | 고정 PNG 입력 경로로 한정했고 일반 loader는 제외했다. |
| 8 | 모든 PNG 변형 지원으로 확대 해석되는가 | 고정 PNG fixture로 표현했다. |
| 9 | HTTP URL이 네트워크 fetch 지원을 암시하는가 | 원격 resource loader가 범위 밖임을 적었다. |
| 10 | 비동기 로딩·교체 지원을 암시하는가 | 비동기 resource loader가 범위 밖임을 적었다. |
| 11 | intrinsic size를 화면 픽셀로 오해하는가 | 단위를 `CSS px`로 표시했다. |
| 12 | 24×16을 일반 이미지 결과로 일반화하는가 | 고정 PNG 입력 및 fixture 문맥에서만 수치를 제시했다. |
| 13 | 일반 HTML layout 완료로 오해되는가 | 일반 HTML 렌더 경로 미포함을 명시했다. |
| 14 | 일반 text layout 지원을 오해하는가 | text layout/paint 미포함을 명시했다. |
| 15 | CSS image paint 지원을 암시하는가 | 이미지 픽셀 paint 결과가 아님을 명시했다. |
| 16 | Android AVD를 실기기로 오해하는가 | `AVD`를 명시했다. |
| 17 | iOS Simulator를 실기기로 오해하는가 | `Simulator`를 명시했다. |
| 18 | 물리 기기 검증을 암시하는가 | 두 검증 환경을 각각 simulator로 특정했다. |
| 19 | iOS compositor 이후 표시를 입증한다고 오해되는가 | fixture report 한계와 paint 제외를 명시했다. |
| 20 | Android hardware GPU 결과로 읽히는가 | AVD 및 report 화면이라고 특정하고 GPU 주장 없이 작성했다. |
| 21 | DPR 또는 device scale 적용을 암시하는가 | 단위를 CSS px로만 기록하고 surface 변환을 주장하지 않았다. |
| 22 | root HTML parser의 보편적인 UA 동작을 뜻하는가 | 문구를 이 고정 opt-in 경로로 제한했다. |
| 23 | live eval과 parsed input 경로가 같은 것으로 보이는가 | parsed initial HTML 입력을 별도 경로로 표현했다. |
| 24 | media query나 color-scheme 작업과 혼동되는가 | 색상 스키마나 viewport 지원 주장을 포함하지 않았다. |
| 25 | C04.12 root document 환경 범위가 확대되는가 | 별도 C04.12 완료 주장 없이 C14.1 slice만 기록했다. |
| 26 | 계획 링크가 없는 문서로 연결되는가 | 병합된 `plan/c14-1-root-html-image-source.md`가 존재한다. |
| 27 | evidence route에서 source fixture가 base 밖 URL로 빠지거나 Simulator 이미지가 누락되는가 | fixture는 stable GitHub blob URL을 사용하고, screenshot은 `./` relative import로 고쳤다. Rspress 2.0.22 선택 build에서 evidence route와 두 image asset을 확인했다. |
| 28 | PR 링크가 다른 번호나 잘못된 repository를 가리키는가 | 링크는 `ohah/spinon`의 병합 PR #295다. |
| 29 | 날짜와 병합 사실이 맞지 않는가 | PR은 UTC 2026-10-17, KST 2026-10-18에 병합되어 표기 날짜와 일치한다. |
| 30 | 안내 문구가 공식 완료 집계를 수동 고정하거나 바꾸는가 | 노트는 설명만 추가하며 완료 수는 기존 대로 공식 `STATUS.md`에서 동적으로 읽는다. |

**결론:** 이 노트는 PR #295에서 확인한 bounded parsed-root image input slice만 소개하며 공식 상태 체크나 제품 지원 계약을 올리지 않는다.
