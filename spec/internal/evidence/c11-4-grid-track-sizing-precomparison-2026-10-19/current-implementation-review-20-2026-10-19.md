# C11.4 현재 구현 실패 관점 검토 20개

**검토 날짜:** 2026-10-19
**대상:** `feat/c11-4-track-sizing-algorithm-20261019`의 현재 미커밋 변경. C11.4 typed track/projection, 제한 auto-repeat, Taffy 0.15.0 trace, FFI·Android/iOS Simulator 연결을 검토했다. 계획 검토와 기존 contribution/input review를 재사용하지 않았다.

| # | 실패 관점 | 확인 및 판정 |
|---:|---|---|
| 1 | 기존 profile에 intrinsic track keyword가 새어 들어감 | `validate_tracks`와 cascade gate가 `TrackSizing` profile에서만 `auto`, min/max-content, minmax, fit-content, calc를 받는다. 기존 template profile이 intrinsic 값을 거부하는 회귀 테스트가 있다. |
| 2 | `minmax()` minimum 위치에 flex가 허용됨 | CSS typed projection에서 minimum `Fraction`을 거부하고 low-level API 타입에도 minimum flex variant가 없다. maximum flex만 보존한다. |
| 3 | 유효한 `0fr`이 잘못 거부되거나 양의 track으로 변환됨 | 최소·최대 함수 검증이 0을 허용하고 `minmax(auto, 0fr)` projection 회귀가 실제 zero 값을 확인한다. |
| 4 | auto-repeat에서 명명 line이 Taffy 입력 변환 중 사라짐 | CSS subset은 repeat fragment와 외부 line-name metadata를 거부한다. 별도 low-level Grid 입력도 columns auto-fill과 rows auto-fit의 named-line 조합을 거부하는 회귀 테스트가 있다. |
| 5 | 한 축에 auto-repeat 두 개 또는 양 축 동시 반복이 들어감 | cascade와 layout validator가 반복 component를 한 축의 단일 항목으로 제한하고 양 축 반복을 거부한다. |
| 6 | template areas와 auto-repeat이 충돌함 | cascade에서 두 기능이 함께 입력되면 속성 오류로 fail-closed한다. |
| 7 | auto-repeat의 고정 최소 크기 조건을 우회함 | CSS와 low-level validator가 고정 길이 또는 허용된 fixed-minimum minmax pattern만 통과시킨다. calc·percentage·intrinsic minimum은 이 subset에서 거부된다. |
| 8 | 빈 fragment·과도한 명시 track·반복 metadata 크기에서 overflow/할당 오류 | 빈 목록, 10,000 track 상한, checked arithmetic과 `try_reserve`를 확인했다. projection 오류는 partial `LayoutOutput`을 내지 않는다. |
| 9 | zero-size fixed minimum auto-repeat에서 분모가 0이 됨 | Taffy 0.15.0의 repeat count 상한과 10,000-track 제한을 확인했다. 다만 zero-size 반복의 앱 fixture는 없다. 제한 profile에서 관찰된 결과로 미측정 조합을 일반 지원으로 확대하지 않는다. |
| 10 | explicit placement 외의 auto-placement에서 Grid 영역 검사가 달라짐 | `TrackSizing`만 CSS Grid placement 경로를 허용하고 기존 mode 분기는 유지된다. workspace Grid placement 회귀 전체가 통과했다. |
| 11 | item 수·cell 수 산술 overflow 또는 implicit track 생성 누락 | checked multiplication/addition 및 기존 implicit-track gate를 확인했다. TrackSizing mode의 implicit behavior는 Taffy 경로로 넘기며 전체 placement 조합은 미완료로 남긴다. |
| 12 | item `width`/`height`의 `%`, intrinsic keyword가 이전 profile에 적용됨 | 새 dimension allowlist는 `TrackSizing` 분기 안에 있고 기존 profile 검증은 unchanged behavior를 가진다. 새 profile 외 입력은 고정 회귀 테스트에서 거부된다. |
| 13 | item min/max constraint의 지원 경계가 무제한으로 넓어짐 | C11.4는 `auto`, fixed, percentage만 허용하고 다른 constraint를 거부한다. cyclic percentage와 모든 min/max 조합은 지원 완료로 기록하지 않았다. |
| 14 | replaced image aspect-ratio/automatic minimum 경로가 텍스트와 혼동됨 | 기존 intrinsic-size provenance와 C14.3 context gate를 확인했다. 새 Chrome frame matrix에 image case 8개가 있지만 앱 fixture는 image contribution을 다루지 않는다. |
| 15 | spanning item의 minimum contribution이 span 밖 track 영향을 받음 | Ahem text span 회귀가 span 안·밖 auto/flex track 차이, minimum source trace와 80 CSS px min-content 값을 확인한다. Chrome 155 span case 3개도 frame 대조에 포함됐다. |
| 16 | minimum contribution의 preferred/min/overflow/automatic branch source가 잘못 표시됨 | Taffy trace source variant를 Grid item 계산 분기와 대조했다. trace는 질의 관찰 자료이며 CSS contribution의 최종 normative 판정으로 주장하지 않는다. |
| 17 | sizing stage 순서 또는 fixed-track early return의 관찰 누락 | Taffy stage hook 위치를 11.4~11.8 계산 경계와 대조하고 fixed early return을 별도 기록한다. 현재 trace test는 단계 존재를 확인하며 독립 normative stage 전체 oracle은 없다. |
| 18 | percentage/cross-axis 재실행 pass가 같은 실행으로 합쳐짐 | pass마다 sequence와 axis를 보존하며 trace 예산·u16 변환 실패를 오류로 처리한다. Chrome 비교에는 indefinite-percentage case 1개가 있지만 모든 cyclic percentage 조합은 미검증이다. |
| 19 | trace buffer 상한/할당 실패에서 부분 진단 layout을 반환하거나 OOM 범위를 과장함 | Spinon/Taffy trace buffer 예약·예산 오류는 layout을 실패시킨다. 반면 Taffy 내부 working vector 일부는 infallible `reserve`/`with_capacity`를 사용한다. 전역 OOM 복구를 보장하지 않는 한계를 계획·상태·vendor patch에 적었다. |
| 20 | 앱 화면이 코드 frame과 다르거나 overflow 경계가 캡처에서 잘림 | Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 7 scene frame이 통과했다. overflow screenshot은 item RGB가 100px Grid보다 약 22 CSS px 더 나감을 보이나 border stroke 색과 실기기는 미검증이다. |

**결론:** 이번 검토에서 추가로 고칠 코드 결함은 발견하지 않았다. 확인된 범위는 후보 implementation과 고정 simulator/Chrome 입력에 한정된다. 전체 CSS Grid contribution, 독립 normative stage oracle, 앱 trace 표시, 모든 auto-repeat grammar, WPT와 C11.4 공식 완료는 여전히 열려 있다. 이번 검토 뒤 문서화된 제한은 Taffy 내부 infallible working allocation과 실기기·border stroke 미검증이다.
