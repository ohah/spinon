# C11.4 이미지 intrinsic 행렬 PR diff 검토 30개

구현 후 20개 검토와 별도로, PR diff가 릴리스 경계·모바일 harness·증거 전달에서 끊기는 경우를 다시 확인했다. 아래 검토는 서로 다른 입력·통합·문서 실패 경로를 다룬다.

| # | 실패 관점 | 확인 및 판정 |
| --- | --- | --- |
| 1 | PR 설명의 이미지 링크가 작업 브랜치 삭제 후 깨짐 | 발견. 브랜치 이름을 경로에 쓰면 PR 병합 뒤 안정적이지 않다. 검토 커밋을 푸시한 후 이미지와 증거 링크를 불변 commit SHA URL로 바꾼다. |
| 2 | GitHub가 PR 본문 이미지 요청을 인증/경로 문제로 거부 | Agent Browser에서 PR 본문 DOM의 실제 이미지 로드를 확인했다. Android 1080×2400, iOS 1206×2622이며 둘 다 `complete`다. |
| 3 | 플랫폼별 링크가 서로 반대 스크린샷을 가리킴 | DOM의 alt와 `currentSrc` 경로를 대조했다. Android는 `android-avd`, iOS는 `ios-simulator` PNG를 가리킨다. |
| 4 | 이미지 대체 텍스트가 시뮬레이터/overflow 범위를 숨김 | 두 alt가 Android API 37 AVD와 iPhone 17 Pro Simulator를 각각 구별한다. |
| 5 | README나 PR이 화면을 decoded PNG texel paint로 과장 | PR 본문과 evidence README가 red CSS background box 및 overflow 경계 표본으로만 설명하고 texel paint는 범위 밖으로 둔다. |
| 6 | C11.4 제한 fixture가 상태 대장에서 전체 구현 완료로 올라감 | `spec/STATUS.md`의 완료 checkbox는 비어 있고 PR·README가 C11.4/C11 미완료를 명시한다. |
| 7 | 구현 계획과 내부 계약이 이 matrix를 전체 Grid 지원으로 확대 해석 | plan과 `0059`는 9개 direct-leaf scene 및 unsupported 조합 한계를 추가한다. 내부 계약 버전은 0.1.0 그대로다. |
| 8 | 외부에 보고한 Android/iOS 결과가 실기기·hardware GPU 결과처럼 읽힘 | 증거는 API 37 ARM64 AVD/SwiftShader 및 iOS 26.2 Simulator/Metal drawable로 환경을 구분하고 실기기·hardware GPU를 미검증으로 둔다. |
| 9 | 제품 기본 빌드에 fixture 전용 이미지 의존성이 전파 | `spinon-runtime`의 `spinon-image` 의존성과 runtime adapter가 전용 fixture feature 뒤에 있다. |
| 10 | fixture feature만 켜고 style/render 지원 feature를 빠뜨려 잘못된 컴파일 경계를 주장 | runtime feature가 style-to-layout과 style-to-render의 같은 이름 opt-in feature를 함께 활성화한다. |
| 11 | FFI scene 제공 함수가 신규 장면을 무조건 제품 API로 노출 | image scene 선택은 `c11_4_grid_track_sizing_runtime_source`의 fixture 경로에 있으며 외부 C 헤더/API 추가는 없다. |
| 12 | 기본 artifact에 고정 fixture URL이나 이미지 payload가 포함 | default release staticlib 검사에서 FFI symbol, scene string, URL, 97-byte PNG가 absent이고 opt-in positive control에는 present다. |
| 13 | artifact boundary 로그가 실제 아카이브 대신 경로 이름만 확인 | 저장된 검증은 심볼·문자열과 고정 PNG 전체 byte sequence를 검사하며 아카이브 크기/SHA-256도 기록한다. |
| 14 | fixture scene 문자열이 길이 제한을 넘거나 임의 scene 입력이 코드 생성으로 전달 | FFI 테스트가 생성 source를 4 KiB 미만으로 제한하고 allowlist 밖 scene ID를 오류로 닫는다. |
| 15 | image scene 식별자를 문자열 보간으로 주입해 생성 JavaScript 구문이 바뀜 | scene/root/item 값은 컴파일 시 고정 테이블에서 오고 ID 포맷은 Rust debug quoting으로 문자열 리터럴을 만든다. |
| 16 | FFI 생성 scene과 runtime validator가 다른 root/item ID를 사용 | 9개 항목의 scene·root·item 테이블을 cross-platform comparison과 런타임 node frame 로그에 대조했다. |
| 17 | 이미지 fixture가 기존 C11.4 text scene 허용 목록을 가리거나 충돌 | FFI source 함수는 기존 17 text scene과 신규 9 image scene을 순서대로 조회하고 text scene 회귀 테스트도 그대로 둔다. |
| 18 | text와 image를 섞은 문서가 한 종류의 intrinsic source만 반영한 채 통과 | image adapter는 root direct child가 단일 img인지 검사하므로 혼합 child tree를 제한 fixture로 처리하지 않는다. 이 범위 제한은 README에 적혀 있다. |
| 19 | 앱 실행 결과가 이전 scene의 로그와 run이 섞임 | Android/iOS 로그는 매 scene마다 고유 run ID와 scene ID를 포함하며 script는 같은 식별자의 성공 summary를 요구한다. |
| 20 | Android scene launch 인자가 scene별 viewport 치수와 어긋남 | 12×40 고정 overflow, 100×80 row, 100×30 spanning의 별도 크기 mapping을 C11.4 harness와 대조했다. |
| 21 | iOS scene launch가 image pixel readback 인자를 모든 9 scene에 적용 | surface pixel 인자는 고정 overflow scene에만 전달되고 다른 scene은 layout/trace assertion만 실행한다. |
| 22 | 각 플랫폼이 natural size는 비교해도 document input revision을 누락 | Android/iOS report parser가 `intrinsicSize == 1`, grid 1, node 2, source order 0 및 replaced natural 24×16을 함께 요구한다. |
| 23 | Columns만 맞고 Rows 단계가 누락되거나 순서가 바뀜 | 양 harness가 정확히 두 pass를 요구하고 Columns 다음 Rows에서 최종 stage track을 대조한다. |
| 24 | fixed 12px track에서 item 기여도를 track 크기로 잘못 보고 | 최종 column base는 12px, item frame은 24px이며 24px column contribution은 요구하지 않는다. 비교 JSON은 track overflow로 분리 기록한다. |
| 25 | overflow 표본에서 item과 root 배경을 구별하지 못해 false positive | x=5 item 내부, x=20 root 밖 item 내부, x=6/y=25 root 배경 control의 세 RGBA 값을 각 플랫폼에서 exact 비교한다. |
| 26 | Android 표면 pixel assertion이 일부 누락된 표본을 성공으로 처리 | PixelCopy summary는 expected/observed/matched sample 수가 모두 3일 때만 script가 계속한다. |
| 27 | iOS drawable이 다른 scene/run의 stale pixel buffer를 반환 | Metal request는 scene/run ID, 좌표, node ID가 고정 표본과 일치해야 하며 3개 결과를 exact 비교한다. |
| 28 | iOS drawable 결과를 화면 compositor 이후 결과라고 오해 | README와 PR 본문이 drawable readback 시점을 compositor 제출 전이라고 명시한다. |
| 29 | Chrome·Android·iOS geometry 허용오차나 scene 누락이 감춰짐 | 비교 JSON은 Chrome 155 기준의 9 scene을 각각 기록하고 geometry 판정은 0.05 CSS px 이내다. |
| 30 | 검토 파일 추가 뒤 SHA256 manifest가 기존 92개 파일만 가리킴 | 이 파일을 포함해 evidence 폴더의 manifest를 다시 생성한다. PR 설명 링크도 최종 commit SHA로 갱신한 뒤 URL을 재확인한다. |

## 판정

30개 검토에서 새로 확인한 수정은 PR 본문 asset URL의 브랜치 의존성이다. 최종 commit을 기준으로 PR 설명 링크를 고정하고 manifest를 갱신한 뒤, 해당 commit의 링크 로드를 다시 확인한다. 현재 PR에 등록된 CI/status check는 없고 `main`에는 required status check 설정도 없다. 이는 시뮬레이터·정적 검사 결과를 CI로 확대해 말한다는 뜻은 아니다.
