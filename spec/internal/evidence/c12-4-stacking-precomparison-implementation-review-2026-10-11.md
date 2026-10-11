# C12.4 Chrome 사전 비교 도구 변경 검토

검토 대상은 C12.4 Chrome 기준 capture 도구·fixture·reference JSON·PNG와 상태 문서 동기화다. Rust painter, WGPU 런타임, Android/iOS C12.4 구현을 승인하거나 완료 처리하는 검토는 아니다.

| # | 실패 관점 | 대조 결과와 처리 |
| ---: | --- | --- |
| 1 | 다른 fixture를 C12.4 결과로 잘못 저장하는가 | schema와 `C12.4-stacking-context-v1` ID를 capture와 테스트에서 고정했다. |
| 2 | case/node/control 식별자가 중복되어 관측값을 덮는가 | 전체 fixture namespace의 고유 ID와 각 case 내부 node ID를 검사한다. |
| 3 | 부모보다 자식이 먼저 선언되어 잘못된 DOM tree를 만드는가 | preorder 부모 폐쇄를 capture와 테스트에서 거부한다. |
| 4 | fixture parentId와 실제 삽입 parent가 달라지는가 | DOM을 parentId map으로 구성하고 관측 parentId를 기준 입력과 대조한다. |
| 5 | 화면 배치용 stage CSS가 case 자체 style을 변형하는가 | stage 원점·크기는 외부 wrapper에 적용하고 node authored style은 별도로 보존·검증한다. |
| 6 | 지원값만 관측하고 computed style의 나머지 차이를 잃는가 | 선언한 computed property 묶음을 전부 기록하며, 개별 기대값만 있는 속성은 명시 assertion으로 검사한다. 이 기준은 CSS 전체 적합성 주장이 아니다. |
| 7 | 로컬 Chrome 버전만 맞고 다른 바이너리가 실행되는가 | 실행 파일 버전과 바이너리 SHA-256을 모두 고정값에 대조한다. |
| 8 | CDP가 다른 Chromium revision에 연결되는가 | DevTools `product`와 revision을 별도로 검증한다. |
| 9 | 네트워크 자원이나 외부 폰트가 기준 화면을 흔드는가 | capture 중 네트워크를 offline으로 만들고 HTML도 로컬 fixture로 고정했다. |
| 10 | locale·timezone·색상 모드·입력 media가 달라지는가 | 각 값을 emulation에서 설정하고 실제 `matchMedia`/navigator 관측값을 다시 검사한다. |
| 11 | DPR 재설정 때 viewport CSS 크기나 환경이 달라지는가 | DPR별 capture 후 inner/client 크기와 기록한 CSS geometry를 비교한다. |
| 12 | DPR별 screenshot이 다른 물리 픽셀 크기로 저장되는가 | PNG를 다시 decode해 width·height·sRGB와 표본 수를 확인한다. |
| 13 | 표본 좌표가 stage 밖 또는 겹치는 box 바깥으로 이동하는가 | inventory 좌표를 stage 안으로 제한하고 모든 후보 participant의 실제 rect 내부인지 확인한다. |
| 14 | CSS box 좌표만으로 paint 결과를 추정하는가 | screenshot PNG interior RGBA를 독립 관찰하고 색 mismatch를 실패 처리한다. |
| 15 | `elementsFromPoint()`가 paint 순서 표준 oracle로 오인되는가 | 보조 관측이라고 reference·테스트·문서에서 역할을 제한했다. |
| 16 | 겹침 fixture의 기대값을 최초 추정 그대로 유지하는가 | `row-reverse` 예상과 실제 픽셀이 달라 별도 최소 재현 후 pinned Chrome 실측값으로 수정했다. |
| 17 | 한 Chrome 결과가 Flex 전체나 모든 브라우저 규칙으로 확대되는가 | 계획·계약에 버전 한정 empirical 결과임을 적고 일반화하지 않도록 했다. |
| 18 | top-layer 표본에서 dialog가 열린 상태가 아닌가 | computed 관측과 screenshot 단계를 분리하고 screenshot 직전 modal을 열어 frame 대기 후 캡처한다. |
| 19 | negative control이 기준을 통과하는지만 저장되고 환경 영향이 누락되는가 | 23 control의 computed style과 frame을 두 DPR 모두 저장하며 사전 기대값을 검사한다. |
| 20 | 기준을 재생성하거나 손상된 이전 파일을 조용히 덮어쓰는가 | 기본 실행은 기존 reference/PNG를 거부하고 명시적 `--replace-reference`만 허용한다. WPT는 고정 hash와 `not-run`을 함께 기록한다. |

검증 실행은 Android 실기기와 iOS Simulator의 기존 C12.3 comparator 재실행(각 13 상태·364 frame 일치) 및 C12.3 회귀 3개 통과를 포함한다. 이들은 C12.4 모바일 검증이 아니다. C12.4 capture 자체는 Chrome DPR 1·2 표본을 통과했고, WPT는 미실행이다.
