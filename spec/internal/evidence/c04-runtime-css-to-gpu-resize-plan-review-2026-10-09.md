# C04.10 surface resize 계획 적대 검토

- 검토 대상: [`C04.10 구현 계획`](../../../plan/c04-runtime-css-to-gpu.md)
- 계획 SHA-256: `6daa84918eac7f6791dd63eb9a347af20a5813193416887376162d4097254ab6`
- 새 비교 기준: [Chromium resize precomparison](c04-runtime-css-to-gpu-resize-precomparison-2026-10-09.md)

## 서로 다른 실패 관점 20개

| # | 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | Android `SurfaceHolder` pixel 폭을 CSS px로 바로 취급하는가 | density로 나눠 CSS viewport를 만들고 WGPU drawable은 pixel 크기를 유지하도록 구분했다. |
| 2 | Android density 환산에서 정수 dp 반올림이 CSS viewport에 섞이는가 | platform callback의 실제 pixel 폭/높이와 density를 부동소수점으로 환산하도록 명시했다. |
| 3 | Android CSS viewport와 화면 배율을 두 번 적용하는가 | CSS px 계산과 WGPU backing pixel 크기의 책임을 분리했다. |
| 4 | iOS `CAMetalLayer.drawableSize`를 CSS viewport로 쓰는가 | `UIView.bounds` point를 CSS viewport로, point×scale을 drawable 크기로 정의했다. |
| 5 | iOS main thread 밖에서 UIKit bounds를 조회하는가 | main-thread layout callback에서 크기를 snapshot하고 불변 값만 runtime executor로 전달하도록 정했다. |
| 6 | safe area/system bar가 viewport에 다시 포함되는가 | 기존 콘텐츠 surface bounds를 기준으로 하며 상위 레이아웃이 제외한 영역을 다시 더하지 않는다. |
| 7 | 크기 변경 없이 scale만 달라져도 device scale이 낡는가 | scale 변화도 환경 입력으로 갱신하고 drawable을 다시 구성하도록 유지했다. |
| 8 | 크기가 0인 일시적 surface에서 유효 환경을 지우는가 | 0 이하 값은 거부하고 마지막 유효 viewport와 revision을 유지한다. |
| 9 | NaN·무한대 환경 값이 FFI에서 수용되는가 | Rust 경계의 유한 양수 검사를 기존 계약으로 유지한다. |
| 10 | surface configure 전 새 크기 scene이 draw되는가 | surface generation을 무효화하고 renderer 재구성 뒤 새 환경 계산·draw를 허용한다. |
| 11 | 늦게 도착한 이전 크기 callback이 최신 크기를 덮는가 | generation이 다른 resize 작업은 플랫폼 queue에서 실행하지 않도록 규정했다. |
| 12 | 이전 viewport 계산이 새 presentation sequence 뒤 publish되는가 | surface generation과 host presentation sequence를 각각 검사하게 했다. |
| 13 | 같은 크기의 중복 callback이 잘못된 세대를 증가시키는가 | 실제 drawable 크기 변경에서만 surface generation을 증가시키며 중복 callback은 허용한다. |
| 14 | 환경 변경 task에서 mutable platform view를 다시 읽는가 | 크기·scale·scheme snapshot을 queue 제출 시 캡처하는 계약을 추가했다. |
| 15 | 색상 scheme 변경과 크기 변경의 값이 서로 덮이는가 | 두 입력을 같은 최신 환경 snapshot에서 적용하고 stale sequence 작업을 버리게 했다. |
| 16 | renderer resize 실패 뒤 CSS 환경만 새 크기로 바뀌는가 | renderer 재구성이 성공한 뒤 environment revision과 draw를 진행한다. 실패 시 화면 오류를 보인다. |
| 17 | 앱 재개나 surface recreate에서 기본값 `301×100`으로 되돌아가는가 | 기본값은 surface callback 전 임시 초기 입력에만 쓰고, 유효 bounds가 오면 즉시 교체하게 했다. |
| 18 | root `100vw/100vh` 계산이 resize 재계산 여부를 드러내지 못하는가 | Chromium 두 viewport 기준과 root computed value/frame을 별도 reference로 고정했다. |
| 19 | viewport 변경을 screenshot 색상 oracle로 잘못 판정하는가 | geometry와 computed value는 Chromium 기준으로, 화면 캡처는 surface 제출 증거로만 사용한다. |
| 20 | 개발용 크기 전환 제어가 공개 앱 API로 오인되는가 | 제어를 C04.10 내부 fixture에 한정하고 내부 계약·상태 대장에 해당 경계를 적는다. |

계획에 추가로 드러난 누락은 Android pixel→CSS px 환산, iOS point→drawable pixel 분리, 이전 resize task의 stale admission, 그리고 Chromium의 두 번째 viewport 기준이었다. 이를 계획과 고정 reference에 반영했다. 이 계획 검토는 코드 구현 완료를 뜻하지 않는다.
