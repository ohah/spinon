# C15.3 scroll-transform PR diff 실패 관점 20개

계획 검토 및 구현 로직 review와 별개로 이 PR의 Rust, C ABI, Java/Swift adapter, CSS fixture, 계약·roadmap·evidence 변경 전체를 diff 관점에서 대조했다.

| # | PR diff 실패 관점 | 확인과 결과 |
| --- | --- | --- |
| 1 | non-zero text scroll 구현이 기본 제품 경로에 노출됨 | C15.3 FFI/앱 route는 기존 opt-in feature guard 안에 있다. 기본 profile API를 추가하지 않았다. |
| 2 | `RuntimeRenderText`에 현재 origin만 두고 이전 값을 덮어쓰기 어려움 | base origin과 base clip을 private snapshot state로 보존했다. public API surface를 넓히지 않았다. |
| 3 | `with_scroll_offsets`가 active text origin에서 누적 이동 | constructor 재진입에서 base origin과 현재 offset map을 사용한다. 반복/초기화 test가 이를 고정한다. |
| 4 | boxes가 scroll update 뒤 text보다 늦게 변환되어 clip이 한 revision 낡음 | base boxes에 scroll state를 먼저 적용한 뒤 해당 revision의 active owner clip을 text에 교차한다. |
| 5 | missing/duplicate scroll owner 또는 비유한 합이 성공 report로 흘러감 | 누락, duplicate ancestor, 비유한 합에서 명시 오류를 반환하고 negative test를 추가했다. |
| 6 | self-owned scroll container 지원이 계획을 넘어 활성화됨 | 기존 fail-closed rejection을 남겼고 계약·계획에 같은 제외 범위를 기록했다. |
| 7 | nested fixture의 CSS parent와 runtime parent graph가 서로 다름 | scroll range용 투명 extent sibling과 outer 직계 child inner scrollport를 사용해 owner chain을 맞췄다. API 37 runtime report에서 실제 2개 ancestor와 offset을 확인했다. |
| 8 | nested outer clip/inner clip 중 하나가 잘못 이동 | CSS fixture geometry와 synthetic fixture metadata를 같은 port로 묶고 unit test와 두 simulator 표면에서 nested ink를 확인했다. |
| 9 | WGPU에는 six text batch를 제출하지만 앱 gate는 세 batch를 기대 | Android/Swift draw assertion을 `runtime_text_count=6`, `glyph_batch_draw_count=6`으로 맞췄다. 두 app route가 성공했다. |
| 10 | C ABI 표본 개수 변경 시 header 및 Rust caller 계약이 어긋남 | Rust FFI는 정확히 6개를 받고 header comment, iOS/Android adapter와 negative counts test를 동기화했다. |
| 11 | iOS parser가 scroll 표본 필드를 읽지 않거나 임의 이름 허용 | 두 report field를 파싱하며 6개 명시 이름, 중복 이름, viewport 범위를 검사한다. |
| 12 | Android parser가 overflow field만 읽어 새 사례가 누락 | `scroll_samples_css_px`까지 합쳐 6개를 요청하고, 이름·좌표 중복·범위·각 scrollport 영역을 검사한다. |
| 13 | Android readback 결과 중 일부 표본이 미검증 | 여섯 sample ID를 모두 별도 ink/background boolean으로 판단하며 observed count도 6이어야 통과한다. |
| 14 | iOS Metal drawable 결과의 순서·샘플 개수·실측 좌표 불일치가 pass | sample 목록 6개, JSON 6개, CSS→drawable 중심 변환 좌표를 모두 검사하고 각 pixel 결과를 분류한다. |
| 15 | Android DPR 비율을 무시하거나 물리 좌표를 CSS 좌표로 오해 | `SurfaceViewPixelCapture.requestForCssViewport`에 viewport 360×240과 여섯 CSS sample을 전달하고 PixelCopy 결과가 모두 기대 ink/background였다. |
| 16 | 스크롤 표본이 실제 glyph ink 바깥 또는 clip 밖을 가리킴 | runtime helper는 bitmap alpha mask와 active clip 안에서만 표본을 선택하며 app readback에서 opaque ink exact를 확인했다. |
| 17 | C15.3 부모 상태가 완성으로 체크되거나 total completion이 증가 | `STATUS.md` C15 체크를 유지하고 이번 제한 slice를 별도 하위 evidence로 기록한다. roadmap 전체 완료 count는 `61/197` 그대로다. |
| 18 | 새 계약이 upstream 공개 API 또는 잘못된 숫자 버전을 주장 | 내부 계약 `0074`, `0.1.0` 고정으로 등록하고 공개 API 아님을 명시했다. |
| 19 | plan/status/internal index의 상대 링크나 범위가 서로 어긋남 | 계약·근거·계획·색인 링크 대상은 `targeted-links.log`에서 모두 확인했다. 전체 Rspress 빌드는 다른 문서의 dead link 490건과 C13.1 누락 이미지 1건으로 실패했으며, 새 0074 계약과 이 근거 README는 오류 목록에 없다. |
| 20 | simulator screenshot이 log/sample과 다른 binary에서 왔거나 한쪽 플랫폼만 누락 | 최신 소스 빌드 뒤 Android AVD 15:58 PixelCopy와 iPhone Simulator 15:59 Metal drawable이 각 6/6 exact였다. 양쪽 캡처·실행 로그·환경·APK/executable/hash manifest가 이 evidence 폴더에 고정되어 있다. PR #261 head `56e26d184f726aa96e08b83b887b5ff3f468b377`를 Agent Browser로 열어 본문 raw 이미지 두 장이 `complete=true`와 자연 해상도 Android 1080×2400, iOS 1206×2622로 렌더되는 것을 확인했다. |

실기기·iOS compositor 이후 scanout·전체 C15 적합성은 이 PR diff의 주장 범위가 아니다.
