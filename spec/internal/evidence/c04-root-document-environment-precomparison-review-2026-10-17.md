# C04.12 사전 비교 결과·계획 변경 적대 검토

계획에 실제 Chromium 관측값과 파서 후보 및 구현 경계를 추가한 변경을 서로 다른 실패 관점으로 다시 확인했다. 계획 수립 검토 20개와 별도이며, 앱 기능 구현 검토가 아니다.

| # | 실패 관점 | 확인 및 판정 |
| ---: | --- | --- |
| 1 | Chromium 버전이 빠져 재현 불가 | Chrome `155.0.8059.85`와 Chromium revision을 근거에 고정했다. 통과. |
| 2 | 데스크톱 Chrome 설정을 모바일 결과로 오인 | iPhone 12 Agent Browser emulation, UA, `390×844`, DPR 3을 별도로 기록했다. 통과. |
| 3 | 모바일 user agent를 실기기로 오인 | 기기 emulation이라고 명시했고 iOS 실기기 근거로 취급하지 않았다. 통과. |
| 4 | layout viewport와 visual viewport를 합침 | viewport 없음·scale 0.5 사례에서 두 값을 따로 기록했다. 통과. |
| 5 | DPR을 page scale로 오인 | DPR 3과 meta scale 결과를 분리하고 구현에서도 별도 상태로 두도록 계획했다. 통과. |
| 6 | viewport meta 부재의 fallback 누락 | width `980`, scale `0.39795917` 관측을 기록했다. 통과. |
| 7 | `width=device-width` 기준값 누락 | `390×844`, scale 1, width media match를 기록했다. 통과. |
| 8 | 고정 width를 그대로 적용한다고 잘못 일반화 | width 320이 scale 1에서 390으로 확장된 이 Chrome 관측을 명시했다. 통과. |
| 9 | initial-scale과 layout width 관계 누락 | scale 0.5에서 layout width 780과 width media 결과를 기록했다. 통과. |
| 10 | 중복 viewport의 관측을 표준 요구로 승격 | Chrome case 한정이라고 표시하고 구현 계약 결정을 남겼다. 통과. |
| 11 | orientation capture를 rotation 증거로 오인 | viewport override가 device emulation을 초기화해 회전 확인이 아니라고 기록했다. 통과. |
| 12 | OS preference와 문서 지원 scheme을 합침 | `prefers-color-scheme`와 Canvas 선택을 별도 결과로 기록했다. 통과. |
| 13 | meta dark가 media query까지 dark로 바꾼다고 가정 | OS light + meta dark에서 media가 light인 관찰을 기록했다. 통과. |
| 14 | root CSS dark가 OS 선호를 변경한다고 가정 | OS light에서 CSS dark여도 media가 light인 관찰을 기록했다. 통과. |
| 15 | `light dark` 메타 동작 누락 | OS dark/light 두 상황의 Canvas 결과를 나눠 기록했다. 통과. |
| 16 | `only dark`를 일반 `dark`와 혼동 | OS light에서도 Canvas dark, media light인 관찰을 기록했다. 통과. |
| 17 | invalid scheme 값을 유효 값처럼 처리 | 잘못된 `unknown`의 기본 Canvas 결과를 기록했다. 통과. |
| 18 | 중복 color-scheme이 허용 입력처럼 보임 | 중복 입력은 표준상 uniqueness 요구를 어긴다고 근거와 결과에 명시했다. 통과. |
| 19 | Chrome 캡처를 Spinon 화면으로 오인 | 이미지 역할을 Chrome oracle로 표시하고 runtime 미검증을 반복해서 명시했다. 통과. |
| 20 | parser library의 test helper를 production에 채택 | `markup5ever_rcdom`의 공식 docs 경고를 확인했고 자체 `TreeSink` 선택을 계획에 기록했다. 통과. |

전체 20개 관점을 확인했다. 남은 실제 회전, malformed markup, runtime, Android/iOS Simulator 검증은 계획상 미완료로 유지한다.
