# C04.10 현재 계획 적대 검토

- 대상: [`plan/c04-runtime-css-to-gpu.md`](../../../plan/c04-runtime-css-to-gpu.md)
- 대상 SHA-256: `aef3566cbddca2a4037b06ffaf292ea1867248a678663d2dbbede9a146929205`
- 이전 검토 이후 UIKit resize의 main-thread 제약과 Android `SurfaceHolder` 해제 callback의 렌더 큐 drain 조건을 실제 호출 순서에 맞게 다시 적었다.

## 서로 다른 실패 관점 20개

| # | 공격 관점 | 현재 계획 판정 |
|---:|---|---|
| 1 | 제한된 GPU fixture가 전체 웹/CSS 지원으로 오인되는가 | 범위를 단일 DOM fixture와 제한된 inline CSS로 묶고 제품 전체 지원은 명시적으로 제외한다. |
| 2 | Android/iOS가 Chromium HTML을 직접 읽어 실제 V8 경로를 우회하는가 | 앱은 저장된 단일 JavaScript를 V8에 평가하고 HTML은 비교 기준으로만 쓴다. |
| 3 | JS fixture와 Chromium HTML의 노드·style·viewport가 달라지는가 | 고정 JS/HTML inventory 검사와 입력별 hash를 두어 대응을 잠근다. |
| 4 | C04.9 layout JSON의 허용 목록이 paint 지원 때문에 넓어지는가 | 별도 `RuntimeFlexPaintV1` profile만 `background-color`를 계산하고 기존 계약의 오류 동작을 보존한다. |
| 5 | fixture hash가 필요한 static snapshot에 동적 scene이 잘못 묶이는가 | `StaticRenderSnapshot`과 runtime snapshot 자료형을 분리한다. |
| 6 | style과 layout이 서로 다른 revision에서 합쳐지는가 | generation/document/render-tree/style/environment 전체 key를 동일하게 요구한다. |
| 7 | 계산 도중 새 DOM 또는 환경 입력이 들어와 오래된 장면이 publish되는가 | 현재 presentation sequence와 완성 key를 확인하고 오래된 publish를 거부한다. |
| 8 | 노드 ID 정렬이 DOM paint order로 잘못 쓰이는가 | ID와 preorder를 분리하고 CSS stacking 지원을 주장하지 않는다. |
| 9 | 중복·누락 ID나 순서 불일치가 부분 화면으로 보이는가 | 유효성 실패 시 장면 전체를 거부한다. |
| 10 | NaN·무한대·음수 크기·좌표 합산 overflow가 GPU 정점으로 넘어가는가 | finite frame, 크기, `x+width`/`y+height` 검증을 완료 조건에 둔다. |
| 11 | `display:none`이나 0 면적 노드가 그려지는가 | 둘 다 draw primitive에서 제외한다. |
| 12 | transparent paint가 기본 불투명 상자로 바뀌는가 | paint 없음과 host clear color를 분리한다. |
| 13 | 부분 alpha·gradient·image 등 지원 밖 paint가 조용히 근사되는가 | 불투명 sRGB와 투명값만 허용하고 나머지는 실패/미지원으로 둔다. |
| 14 | CSS 색을 재파싱하거나 sRGB 전환을 두 번 적용하는가 | Stylo 계산값을 쓰고 surface format별 shader encoding을 정하며 내부 readback 기준을 별도로 둔다. |
| 15 | CSS px, Android dp/iOS point, backing scale 또는 safe area가 중복 적용되는가 | viewport 원점·콘텐츠 영역·scale 적용 경계를 각 플랫폼에서 한 번으로 정한다. |
| 16 | Android `SurfaceView` 수명 종료 뒤 렌더 thread가 surface를 계속 사용하는가 | `surfaceDestroyed`에서 신규 작업을 막고 render queue의 이전 작업과 destroy 완료를 기다리도록 정했다. 근거는 Android [SurfaceView 공식 계약](https://developer.android.com/reference/kotlin/android/view/SurfaceView.html)이다. |
| 17 | iOS UIKit 객체를 background queue에서 읽거나 `CAMetalLayer` configure를 잘못된 queue에서 하는가 | view/layer 접근과 configure/resize는 main thread에서 수행하고 GPU 작업은 전용 serial queue에서 한다. |
| 18 | iOS main-thread resize가 이전 draw와 동시에 진행되는가 | sequence 무효화와 draw admission 차단 뒤 render queue barrier를 거쳐 configure하고 generation 확인 후 draw를 재개한다. |
| 19 | queue shutdown·pending draw·동기 JS 무한 실행이 가려지는가 | 요청 차단 → runtime/session 종료 → render queue drain → surface 해제 순서를 명시하고 JS 평가 자체는 timeout/cancel 대상이 아님을 적는다. |
| 20 | offscreen·시뮬레이터 결과가 실제 surface, 하드웨어 GPU, 성능 또는 release 지원으로 과장되는가 | Android/iOS 실제 앱 화면, 별도 readback, 성능 미주장, 실기기 미검증, 내부 계약 `0.1.0` 고정을 각각 구분한다. |

## 남은 검증 경계

- 이 문서는 현재 계획의 모순과 빠진 수명주기 계약을 확인한 기록이다. 구현 검증을 대신하지 않는다.
- 현재 완료 조건에는 실제 surface resize, draw 실패, 종료 시 pending 작업, 빈 root readback이 남아 있다. 각각의 실행 근거를 추가하기 전에는 C04.10을 완료 표시하지 않는다.
