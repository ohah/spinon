# C11.4 `hyphens:auto` 로드맵 문서 PR 적대 검토 30개

검토 대상은 `spec/public/roadmap.html`의 PR #982 진행 기록 추가와 PR #977의 역사적 범위 설명, 이 문서의 링크다. 구현 범위는 PR #982 병합 기록과 `spec/STATUS.md`, C11.4 계획·내부 계약·런타임 근거를 대조했다. 로드맵은 참고 진행 기록이고 구현 상태의 원본은 상태 대장이다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 구현 PR 주소가 다른 작업을 가리키는가 | #982 링크가 anonymous Grid text `hyphens:auto` 구현 PR을 가리킨다. 통과. |
| 2 | 병합 시각이 로컬 시간과 UTC 사이에서 혼동되는가 | `2026-11-01T10:38:00Z`를 UTC 표기로 유지한다. 통과. |
| 3 | 병합 SHA가 후보 커밋이나 문서 PR SHA로 바뀌었는가 | 구현 병합 SHA `b54d4853fe7cd5c70393c7083eea2c3d8f6f223f`를 기록한다. 통과. |
| 4 | 언어 범위가 임의 언어로 일반화되는가 | 정확한 `lang=en`만 명시한다. 통과. |
| 5 | 폰트/크기 범위가 제품 전체로 오해되는가 | Ahem 10px direct text profile로 한정한다. 통과. |
| 6 | 어휘 목록이 늘어난 것으로 읽히는가 | 세 단어 `hyphenation`, `internationalization`, `representation`을 열거한다. 통과. |
| 7 | Hypher가 기본 제품 의존성/경로로 읽히는가 | opt-in English Hypher 0.1.8 feature로 한정한다. 통과. |
| 8 | 임의 CSS 입력이 지원된다는 인상을 주는가 | 일반 CSS admission과 임의 text/language/font가 미완료라고 적는다. 통과. |
| 9 | 원본 DOM/문서 텍스트가 바뀌는 것으로 오해되는가 | HostDocument와 UTF-16 source range 보존을 기록한다. 통과. |
| 10 | synthetic hyphen이 원문 문자로 노출되는가 | marker는 normalized layout text에만 들어간다고 구분한다. 통과. |
| 11 | 30개 행렬 결과가 플랫폼을 합쳐 30개로 오해되는가 | Android와 iOS에서 각각 30/30이라고 명시한다. 통과. |
| 12 | Android 결과가 실기기나 hardware GPU 결과로 읽히는가 | API 37 ARM64 AVD와 실기기/hardware GPU 미완료를 함께 적는다. 통과. |
| 13 | iOS 결과가 실기기로 확대 해석되는가 | iPhone 17 Pro/iOS 26.2 Simulator라고 명시하고 실기기는 미완료로 둔다. 통과. |
| 14 | 30개 행렬이 Auto 양성만 포함한다고 오해되는가 | regression·Auto·expected reject 장면을 포함한다고 적는다. 통과. |
| 15 | 설치 결과가 검증 대상 빌드와 다른가 | 설치 APK/executable hash를 대조했다고 기록한다. 통과. |
| 16 | 비어 있는 GitHub check rollup이 CI 통과로 오해되는가 | rollup이 비어 있고 CI 통과로 세지 않는다고 명시한다. 통과. |
| 17 | fixture 결과가 glyph/pixel 렌더링을 증명한다고 오해되는가 | glyph pixels는 미완료로 명시한다. 통과. |
| 18 | WPT 적합성까지 입증한 것으로 보이는가 | WPT 미완료를 명시한다. 통과. |
| 19 | 새 제한 slice가 C11.4 전체 완료를 의미하는가 | 전체 C11.4 미완료를 명시한다. 통과. |
| 20 | 하위 slice 완료가 C11 전체 완료로 확대되는가 | C11 미완료를 명시한다. 통과. |
| 21 | 공식 완료 수가 진행 기록 때문에 증가하는가 | 공식 `72/213`을 유지한다고 적는다. 통과. |
| 22 | 진행 기록이 상태 대장을 대체하는가 | 상태 대장 링크를 두고 기록을 비규범 진행 요약으로 유지한다. 통과. |
| 23 | PR #977의 당시 제한 설명이 현재도 절대적인가 | #977 범위에 한정된 historical baseline임을 설명하고 #982 후속을 연결한다. 통과. |
| 24 | 이전 PR #977의 증거 링크가 새 항목 때문에 사라지는가 | 기존 계획·계약·상태·근거·화면 링크를 보존한다. 통과. |
| 25 | 새 계획 링크의 경로가 오탈자이거나 아직 미병합 후보로 표시되는가 | `c11-4-anonymous-grid-text-hyphens-auto` slug와 #982 병합 SHA·상태를 계획에서 대조한다. 통과. |
| 26 | 내부 계약 링크가 다른 계약을 가리키는가 | 0059 C11 Grid layout 계약 경로를 유지한다. 통과. |
| 27 | 런타임 README가 미병합으로 남거나 화면/행렬 링크가 디렉터리 URL을 가리키는가 | 병합 SHA·빈 check rollup을 기록하고, matrix·simulator JSON·대표 PNG의 실제 파일 경로를 각각 연결한다. 통과. |
| 28 | 20/30 검토 링크가 계획/구현 검토와 혼동되는가 | 구현/runtime 20개와 구현 PR diff 30개를 각각 구분해 연결한다. 통과. |
| 29 | 로드맵과 계획/실행 README가 서로 다른 PR 병합 상태를 말하는가 | `hyphens:auto` 계획과 실행 README를 #982 병합 시각·SHA·빈 check rollup에 맞춰 갱신한다. 통과. |
| 30 | 로드맵 변경 자체의 검토 근거가 빠지거나 Pages 배포/삭제가 따라오는가 | 이 30개 항목 문서를 같은 PR에 연결한다. 소스 문서 PR만 만들고 Pages 자동 배포는 실행하지 않으며 Tailnet 미리보기는 별도 동기화한다. 경로/파일은 삭제하지 않는다. 통과. |

## 판정

30개 관점에서 과장, 상태 변경, 누락된 범위 및 링크 경로 문제를 확인했다. 공식 체크 수는 변경하지 않았고, 문서 빌드와 Tailnet 미리보기 검증은 PR 병합 후 별도 기록한다.
