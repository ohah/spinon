# C07.2 Android 실기기 증거 검토

이 문서는 기존 C07.2 구현의 재승인이나 새 border-width 기능 구현이 아니다. 병합된 PR #103 뒤에 저장된 Android 실기기 보조 smoke의 근거 범위와 상태 문서 반영을 확인한다.

| # | 실패 관점 | 검토 결과와 조치 |
| ---: | --- | --- |
| 1 | 병합된 PR을 아직 열린 것으로 기록하는가 | `gh pr view 103`에서 `MERGED`, 병합 시각 `2026-10-10T02:50:58Z`를 확인했다. simulator 근거의 오래된 “열려 있다” 문장을 완료 상태로 고친다. |
| 2 | 실기기 근거를 emulator로 오인하는가 | 기록은 Android device log와 SM-S731N 기기에서의 surface를 담는다. emulator 결과와 별도 행으로 표시한다. |
| 3 | 기기 모델이 로그 바깥 추정에만 의존하는가 | 연결된 serial의 `ro.product.model`이 `SM-S731N`이며 저장된 로그의 Xclipse/Vulkan 경로도 함께 대조했다. serial 자체는 문서에 공개하지 않는다. |
| 4 | Android 버전과 API 수준을 혼동하는가 | 기기 속성 결과 Android 16·API 36을 기록한다. API 37 emulator 실행과 별개다. |
| 5 | 화면 density를 CSS DPR이나 CSS fixture 값으로 오인하는가 | `wm density` 450과 1080×2340 physical resolution만 기기 특성으로 기록한다. 앱 summary의 2.8125 scale과 CSS viewport를 구분한다. |
| 6 | 실제 GPU backend를 근거 없이 하드웨어로 단정하는가 | 저장된 renderer 로그가 Vulkan·Samsung Xclipse 940을 명시한다. 성능 결과는 주장하지 않는다. |
| 7 | desktop Chrome comparator를 실기기 비교로 확대하는가 | 실기기 run은 layout ready와 제출 box 수만 확인한다. node별 Chrome 비교는 기존 50-node desktop fixture 결과로 분리한다. |
| 8 | 의도한 C07.2 fixture 대신 기본 앱 화면만 실행하는가 | `SPINON_C072_EVAL` tag와 `runtime-border-width.js` trigger 경로를 확인하고 근거 문서에 적는다. |
| 9 | 실패한 eval을 성공으로 기록하는가 | Logcat의 eval 요약은 `status=0`이다. |
| 10 | 계산이 완료되지 않았는데 box 수만 보고하는가 | 같은 eval 요약에서 `layout=ready`, `boxes=6`을 확인한다. |
| 11 | resize 전후 revision을 혼합하는가 | screen-size change 뒤 environment revision 1의 layout과 draw를 별도로 기록한다. 초기 revision과 resize 결과를 같은 frame처럼 합치지 않는다. |
| 12 | WGPU 제출을 화면 표시 완료 또는 scanout 증명으로 과장하는가 | 로그의 `presented`는 renderer 제출 로그로만 인용한다. 광학 scanout·frame timing을 뜻하지 않는다고 범위를 제한한다. |
| 13 | screenshot이 실제 앱 결과와 무관한 별도 mock인가 | 첨부 화면에는 `SPINON · C07.2 border width`, runtime summary, 301×100 CSS px 상태가 보인다. 같은 시각대 Logcat의 C07.2 실행과 대조한다. |
| 14 | 사각형 화면을 border stroke 구현 증거로 오인하는가 | screenshot subtitle에 테두리 페인트 제외가 적혀 있고 기존 계약도 이를 제외한다. 문서에 stroke 미구현을 다시 명시한다. |
| 15 | child frame 숫자를 직접 확인했다고 과장하는가 | 실기기 화면·로그에는 child별 CSS frame 목록이 없다. per-node 수치 비교를 주장하지 않는다. |
| 16 | 고 DPR authored fraction 보정을 실기기에서 확인했다고 오인하는가 | 이 fixture smoke는 `1.9999px` 등 authored boundary matrix가 아니다. high-DPR follow-up 검토의 Rust/Chrome 범위와 별도로 둔다. |
| 17 | 6-box smoke를 50-node fixture의 전체 실행으로 해석하는가 | 6 boxes는 모바일 런타임 제출 수로만 기록한다. 50-node comparator의 Chrome/Rust 근거는 기존 자료를 참조한다. |
| 18 | 앱 성능이나 프레임 지연을 주장하는가 | benchmark 반복·timing 계측이 없다. 지연·처리량·성능 결론을 추가하지 않는다. |
| 19 | Android 결과를 iOS 실기기까지 일반화하는가 | iOS 실기기는 검증하지 않았다. 기존 iOS Simulator 결과와 Android 실기기 smoke를 분리한다. |
| 20 | 하위 체크 완료를 이유로 C07 부모까지 완료 처리하는가 | C07.3의 다섯 min/max+ratio 조합은 현재 fail-closed다. 하위의 제한 구현 완료는 부모 상자 모델 범위 전체 완료를 뜻하지 않으므로 C07 상위 체크는 유지한다. |

실행 로그·캡처·digest는 [실기기 근거 폴더](./c07-2-android-physical-2026-10-11/README.md)에 보관한다. 수정된 C07.2 설명은 이 보조 smoke와 desktop fixture의 수치 비교를 구분한다.
