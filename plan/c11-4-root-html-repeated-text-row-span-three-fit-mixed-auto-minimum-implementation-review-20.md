# C11.4 three-fit row-span mixed minimum 구현·런타임 검토 20개

대상: Root HTML parsed Grid의 three-fit repeated-text row-span item별 `min-height:0|auto` 구현, RuntimeSession 관측, Android/iOS 화면 smoke. 계획 검토와 PR diff 검토를 대체하지 않는다.

서로 다른 실패 경로를 기준으로 style admission, Layout trace, scene 생성기, 비교 oracle, 회복 동작과 simulator 근거를 대조했다. RuntimeSession 실행은 Chrome 전체 138,240 scenes가 아니라 별도 6,912-scene 고정 표본이며, 모바일 캡처는 pixel parity가 아니다.

| # | 실패 관점 | 확인 결과와 판정 |
|---:|---|---|
| 1 | Root HTML에 Grid owner가 없거나 여러 개인데 제한 profile이 적용되는가 | Style-to-layout은 visible Grid owner를 정확히 하나 요구하고, Layout 입력은 고유 owner·row shape를 확인한다. 복수 owner는 거부한다. |
| 2 | 잘못된 implicit row 수나 track 개수가 지원으로 들어오는가 | 세 fit cap과 `min-content`, `max-content`, `auto`를 각 하나씩 가진 6-member multiset만 받는다. 길이·중복·누락 음성 사례가 layout 단위 테스트에 있다. |
| 3 | cap 값 또는 authored 타입이 비슷한 값으로 느슨하게 허용되는가 | 허용 cap은 `0px`, `10px`, `20%`뿐이다. `1px`, `30%`, calc, 다른/중복 cap을 거부하고 720개의 위치·순서 조합을 positive로 검사한다. |
| 4 | fit 위치나 tail 순서 일부만 구현되어 전체 순열처럼 보이는가 | 20 fit-position set, 여섯 cap 순서, 여섯 intrinsic tail 순서를 전부 unit-test하고 Chrome 사전 비교 metadata/verifier가 720개 list를 확인한다. |
| 5 | `min-height:auto`가 item mask 대신 전체 grid에 적용되는가 | semantic A–E/A–D ID mask와 32/16 mask 범위를 비교한다. RuntimeSession sample은 네 topology/order별 mask 전체와 computed contribution source를 대조한다. |
| 6 | 명시 행 empty control이 mask에서 빠져 자동 최소 높이를 받는가 | 첫 control item을 항상 `0px`로 제한하고 Chrome scene, computed style, 10px row geometry를 확인한다. |
| 7 | reversed DOM 순서가 mask bit를 다른 item으로 옮기는가 | mask bit는 semantic ID에 묶고 forward/reverse scene을 짝지어 row·item·payload geometry를 비교한다. |
| 8 | auto minimum 자격이 overflow·covered track·flexible track 조건을 무시하는가 | eligible item만 content floor를 요구한다. visible overflow positive profile, scrollable-overflow 및 flexible/unsupported input 거부를 확인한다. |
| 9 | 비대상 explicit item의 자동 최소 기여가 spanning contribution에 섞이는가 | item trace의 실제 span·minimum source·0 기여분을 확인하고, control과 auto span item을 구별한다. |
| 10 | text wrapping 또는 payload 크기가 달라 floor 비교가 무효가 되는가 | 고정 text/font/line-height fixture와 Chrome payload/line rectangles를 비교한다. Eligible `auto` item의 content floor를 직접 assert한다. |
| 11 | span item frame이 덮은 used row 합과 어긋나는데 통과하는가 | 각 item frame·payload frame·used rows를 oracle과 0.01 CSS px 안에서 대조한다. 역순 pair도 semantic ID별로 비교한다. |
| 12 | `align-content:stretch`가 fit-content 행을 잘못 늘리는가 | start/stretch pair에서 auto maximum row만 늘고 그 외 row는 0.01 CSS px 안에서 유지되는지 표본 전체에서 확인한다. |
| 13 | percentage cap 기준이 220px content box가 아닌 222px border box인가 | fixture는 42×222 border box 안에 40×220 content box를 고정한다. percentage cap의 Chrome used geometry와 simulator 대표 scene을 비교한다. |
| 14 | Blink 1/64px rounding replay가 trace와 다른 중간 상태를 수용하는가 | intrinsic sizing 단계마다 trace base size를 확인하고, 재생 실패 시 mixed three-fit 경로는 일반 근사 분배로 묵인하지 않고 fail-closed한다. 6,912개 Chrome 기하 비교가 rounding 차이를 검사한다. |
| 15 | border origin을 content origin으로 혼동해 1px 기하 오차를 숨기는가 | row normalization은 Grid border를 빼 content frame을 사용하고, 대표 RenderSnapshot은 네 변의 1px opaque red border와 Chrome 행 geometry를 확인한다. |
| 16 | 프로파일 외 cap·placement 입력이 이전 completed output을 유지하는가 | 비지원 cap과 `row dense`는 layout을 실패시키고 partial completed output을 게시하지 않는 negative test가 있다. |
| 17 | invalid control/item mutation 뒤 같은 RuntimeSession이 복구되는가 | control `auto` 및 item `1px` mutation을 각각 실패시킨 다음 valid mixed scene으로 되돌려 같은 session의 Ready 상태를 확인한다. |
| 18 | failed revision이 stale render snapshot이나 partial result를 반환하는가 | invalid layout snapshot의 `completed`가 `None`인지 확인한다. 복구 revision은 새 Ready layout으로 관측한다. |
| 19 | simulator 앱이 Root HTML route를 실제로 그렸는가 | Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 로그에 같은 fixture hash, `layout=ready`, text draws, present가 있다. 두 캡처에서 border와 overflow sentinel을 육안 확인했다. |
| 20 | 임시 기기·앱 산출물·process를 남기거나 smoke를 pixel parity로 과장하는가 | 임시 iOS 기기는 삭제, read-only Android AVD는 중지, 원본 HTML asset과 기존 앱 산출물은 해시로 복구했다. 캡처는 surface readback이 아니므로 색·pixel parity 주장은 하지 않는다. |

### 확인한 실행 범위

- Three-fit mixed RuntimeSession Chrome sample: 6,912/6,912 pass; full oracle은 138,240 scenes이며 전체 RuntimeSession 실행으로 부르지 않는다.
- Three-fit uniform endpoint regression: 11,520 scenes pass.
- Auto-placement 5,760, one-fit 2,880, two-fit 3,840, four-fit mixed 3,840, five-fit mixed 3,984 scenes: pass.
- Six-fit mixed는 metadata가 전체 138,240 scene 조합을 확인하고 rejection/recovery 및 첫 11,520-scene matrix chunk가 통과했다. 나머지 11개 RuntimeSession chunk는 실행하지 않았다.
- Android/iOS 결과는 simulator smoke다. real device, PixelCopy/Metal readback, compositor pixel parity는 확인하지 않았다.
- 공식 상태는 `78/214`; C11.4/C11과 전체 three-fit profile은 완료로 표시하지 않는다.

결론: 검토 범위에서 구현 결함은 재현되지 않았다. 남은 제한은 문서화한 RuntimeSession 표본, simulator-only 시각 smoke, C11.4 상위 profile admission 및 정식 spec/status 후속이다.
