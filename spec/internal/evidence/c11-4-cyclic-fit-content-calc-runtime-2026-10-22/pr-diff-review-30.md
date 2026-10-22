# C11.4 cyclic `fit-content(calc())` PR diff 검토 30개

구현/runtime 실패 관점 검토와 구분해 PR diff의 컴포넌트 연결, 예상값 중복 관리, fixture 경계, 근거 문서 정확성을 각각 확인했다. 코드는 고정 scene allowlist만 노출하고 아래 모든 교차 검토는 현재 최종 diff와 Android/iOS Simulator 근거를 기준으로 했다.

| # | 별도 실패 관점 | diff 대조 및 결과 |
| ---: | --- | --- |
| 1 | FFI dispatcher에서 새 모듈이 조건부 fixture 트리에 연결되지 않는가 | `fixtures/c11_4.rs`의 `#[path]`와 dispatch 양쪽에 모듈이 등록되어 feature-gated fixture 구성에서만 접근된다. |
| 2 | FFI 생성과 runtime validator의 scene 목록이 갈라지는가 | 두 Rust allowlist 모두 같은 네 ID를 사용하고 각 모듈 테스트에서 목록과 미등록 ID 거부를 확인한다. |
| 3 | Android 또는 iOS에서 한 scene이 빠져 일부 장면만 검증되는가 | 두 host의 scene set, root/item frame, box count, 표본 설정이 네 ID 모두를 포함한다. 양쪽 capture에서 4/4 통과했다. |
| 4 | CSS reference capture가 scene을 인식하지 못해 요청 입력을 거부하는가 | capture script의 requested scene set과 223개 정적 scene 수가 새 네 장면을 포함한다. node 문법 검사와 실제 simulator capture가 통과했다. |
| 5 | C11.4 전체 scene 수를 넷 추가하지 않고 예전 상수를 남기는가 | 219에서 223으로 상수를 갱신했고 고유 ID 검사가 기존 중복 방지와 함께 유지된다. |
| 6 | 유사한 ID를 새 fixture로 묵인하는가 | 새 runtime helper는 등록되지 않은 `c114-cyclic-fit-content-calc-*`와 `c114-definite-fit-content-calc-*` root를 오류로 처리한다. |
| 7 | FFI가 id 외 입력을 받아 scene DOM을 임의화하는가 | FFI helper는 네 ID의 고정 scene 자료만 직렬화하며 텍스트와 크기를 호출자 인자로 받지 않는다. |
| 8 | HTML scene에 item 외 다른 node가 생겨 node index가 변하는가 | 생성 DOM은 root와 item 두 element 및 단일 직접 Ahem text node로 고정되고 native 기대 node frame도 두 항목이다. |
| 9 | test와 simulator가 서로 다른 CSS 선언을 사용해 기대값만 일치하는가 | fixture source의 네 track, root size, text는 Rust layout test와 capture 표에 같은 입력으로 각각 기록되어 있다. |
| 10 | cyclic column의 max-content root가 definite 기준값으로 오인되는가 | cyclic scene은 root 50px/item 29px, definite 40px control은 item 24px로 분리되어 두 플랫폼 결과와 일치한다. |
| 11 | cyclic row만 기존 column 경로에 묻혀 확인되지 않는가 | row scene은 20×40 root에서 Rows `[22]`, 20×22 item을 별도 기대하고 Android/iOS 모두 통과했다. |
| 12 | 작은 definite limit에서 item 폭 clamp와 root 폭이 섞이는가 | width 10px root의 item/track 폭 9px을 Rust, host frame assertion, simulator 로그가 함께 기대한다. |
| 13 | layout test가 frame만 보고 잘못된 track을 통과시키는가 | 각 scene에서 root/item 위치·크기와 Columns·Rows track을 따로 assertion한다. |
| 14 | subpixel 허용 오차가 runtime host의 정수 출력과 불일치하는가 | 순수 layout test는 0.05px 오차를 허용하고 simulator host는 고정 정수 CSS px frame 문자열을 검증한다. 현재 네 기대값은 정수이고 양쪽이 일치한다. |
| 15 | Ahem 측정 결과가 시스템 폰트 설치에 따라 달라지는가 | runtime fixture는 시스템 font 없는 engine에 저장소 Ahem font를 등록하고 고정된 10px 글꼴·line-height로 측정한다. |
| 16 | text overflow 표본이 잘못된 글리프 위치를 읽는가 | 표본은 Ahem 10px cell의 내부, item 바깥, text 끝 바깥 좌표로 구성되며 작은 limit scene에서 red/red/black을 확인한다. |
| 17 | Android PixelCopy 검증이 표본 수만 맞고 중복 ID를 허용하는가 | validator는 ID별 단일 매칭, target node, CSS 좌표를 확인하고 예상 표본 수와 모두 찾았는지도 검사한다. |
| 18 | Android RGB만 맞고 alpha나 배경 표면을 무시하는가 | capture 기대값은 RGBA 4채널이다. 내부·overflow 표본은 opaque red, text 이후는 opaque black으로 3개 값을 고정했다. |
| 19 | iOS host가 표본 위치를 CSS px 대신 drawable px으로 잘못 해석하는가 | Metal 표본 설정은 CSS 좌표와 drawable scale factor 3을 사용하고 최종 로그는 960×720 drawable의 3/3 exact를 기록한다. |
| 20 | iOS 결과를 compositor 제출 이후 화면 근거로 과장하는가 | 코드와 README가 읽기 지점을 제출 전 Metal drawable로 한정하고 compositor 이후 결과는 명시적으로 남은 범위에 둔다. |
| 21 | 각 플랫폼 sample validator가 기존 C11.4 장면 분기와 충돌하는가 | 새 ID만 별도 분기로 처리하며 기존 cyclic percent/minmax/fit-content 분기는 보존된다. Android와 iOS 앱 빌드 및 이 fixture capture가 통과했다. |
| 22 | root 배경색을 overflow 글리프 색으로 혼동하는가 | root 배경은 `#101820`, item과 glyph는 `#e5484d`다. item 내부 및 외부 글리프를 배경과 구분하는 표본이 exact하게 읽혔다. |
| 23 | fixture painter 연결이 일반 앱 paint를 활성화하는가 | 새 adapter 준비는 runtime paint가 활성이고 root가 exact 네 ID일 때만 실행되며 `c11-4-grid-track-sizing-fixture-test` cfg 아래에 있다. |
| 24 | 기존 text input 준비 순서에서 다른 C11.4 fixture input을 빼앗는가 | 기존 fixture 우선 조건에서 text input을 얻고, 새 render input은 해당 allowlist scene만 소비한다. 나머지 fixture 분기와 capture는 기존 dispatch에 남아 있다. |
| 25 | 공통 painter helper가 fixture scene 의미를 runtime 렌더 모듈에 누출하는가 | 공통 helper는 검증된 owner·style map·handle map·색·engine을 `Input`으로 옮기고 scene별 입력과 CSS 검증은 fixture 모듈에 둔다. |
| 26 | helper 입력이 다른 snapshot generation의 node를 참조하는가 | helper가 root generation, owner의 computed style와 document handle 소속을 확인하고 attach 단계는 render revision을 확인한다. |
| 27 | item의 기본 최소 크기 때문에 small-limit 검사가 우연히 통과하는가 | 생성 CSS와 computed typed style 모두 `min-width:0;min-height:0`을 명시하고 validator도 두 값을 검증한다. |
| 28 | 문서 상태가 한정된 simulator slice를 공식 완료로 표시하는가 | `STATUS.md`는 C11.4 checkbox와 `72/213`을 유지하고 네 고정 scene과 미검증 범위를 progress note로만 추가했다. |
| 29 | PR 증거가 시각 화면과 재현 가능한 원본 로그를 연결하지 못하는가 | README는 Android/iOS 화면, scene별 로그, 기기·빌드 식별자와 artifact hash를 연결한다. 두 캡처 디렉터리에 실제 PNG와 runtime log가 있다. |
| 30 | 기본 제품 FFI artifact에 검증 fixture가 포함되는데 문서가 과장하는가 | 기본 release archive의 marker 부재와 fixture-test positive archive의 marker 존재를 별도로 확인했다. 근거는 host static archive 비교로 한정하고 앱 bundle 크기 증거로 주장하지 않는다. |

**결과:** 이 30개 diff 관점에서 새 결함은 남지 않았다. Android 물리 기기, hardware GPU, iOS compositor 이후 pixel, WPT, 일반 CSS admission과 전체 C11.4는 이 PR의 검증 대상이 아니다.
