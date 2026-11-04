# C11.1 aspect-ratio 값 확장 PR 적대 검증 30개

병합 전 PR 단위 적대 검증이다. 계획 20개, 구현/runtime 20개, PR diff 20개를 재사용하지 않고 CSS 값 경계·owner 상태·실행 경계·문서 계약을 따로 확인했다.

| # | 공격 입력·실패 경로 | 확인 및 판정 |
|---:|---|---|
| 1 | 단일 numeric `1.5`와 pair `3/2`를 계산 결과에서 다르게 취급 | typed ratio와 frame이 같고 computed serialization 차이는 유지한다. 통과. |
| 2 | 유효 ratio의 계산 source를 computed string로 바꾸기 | projection은 typed ratio로 계산하며 string parse 경로가 없다. 통과. |
| 3 | ratio `0/1`이 numeric positive로 admission | cascade/layout negative는 aspect-ratio error로 닫힌다. 통과. |
| 4 | 최소 subnormal 근처 양수의 transfer가 0으로 underflow | `1e-38/1` width-locked negative가 finite-positive transfer guard에서 거부된다. 통과. |
| 5 | 큰 양수 ratio의 반대축 결과가 무한대 | `1e38/1` height-locked negative가 거부된다. 통과. |
| 6 | transfer는 유한하지만 여러 frame 누적으로 절대 위치가 무한대 | 두 scene `3e38px` 회귀가 right/bottom edge 검사에서 거부된다. 통과. |
| 7 | width-locked `1/2`에서 나눗셈 방향 오류 | target 60×120 및 auto row 120을 matrix가 확인한다. 통과. |
| 8 | width-locked `5/2`에서 auto row가 잘못 계산 | target 60×24 및 row 24를 확인한다. 통과. |
| 9 | height-locked `4/3`에서 곱셈 방향 오류 | height 30에 target width 40을 확인한다. 통과. |
| 10 | height-locked `5/2`를 60px column/Grid 안으로 clamp | target width 75 및 visible overflow 15를 확인한다. 통과. |
| 11 | width-locked `10/1`이 text line-height보다 작아짐 | 6px transfer여도 row/target 10px minimum을 유지한다. 통과. |
| 12 | width-locked `100/1`이 subpixel height로 text를 압축 | 0.6px transfer와 10px minimum 대조를 Rust/runtime에서 확인한다. 통과. |
| 13 | height-locked `1/100`이 min-content 폭을 0.3px로 축소 | Ahem 기반 width 40px automatic minimum을 확인한다. 통과. |
| 14 | 자동 최소 폭을 호스트 폰트의 Chrome 숫자로 고정 | Chrome 24.09375px은 보충 관찰로만 남기고 Rust Ahem oracle과 구분한다. 통과. |
| 15 | `aspect-ratio:auto` control이 이전 ratio transfer를 유지 | numeric ratio가 없는 별도 control의 기존 behavior를 확인한다. 통과. |
| 16 | `auto <ratio>`의 ContentBox sizing을 BoxSizing으로 오인 | typed sizing box guard와 기존 cascade negative를 유지한다. 통과. |
| 17 | Grid root의 ratio를 item ratio로 귀속 | Grid root ratio negative가 `aspect-ratio` 오류로 닫힌다. 통과. |
| 18 | block root의 ratio를 Grid target ratio로 귀속 | Root HTML block root negative가 projection에서 거부된다. 통과. |
| 19 | owner 안에 두 ratio item이 있어 일부만 처리 | owner-scoped one-ratio 조건이 전체 candidate를 거부한다. 통과. |
| 20 | 여러 Grid 중 한 Grid의 NodeId가 다른 Grid target set에 섞임 | target set은 각 `is_supported` container의 exact child `NodeId`에서만 만들어진다. 통과. |
| 21 | anonymous/nested/replaced item으로 허용 경로 우회 | 기존 fixed direct-text ownership/topology 검사가 candidate가 되지 않게 한다. 통과. |
| 22 | height 80px 예외를 ratio 1/2 외에 사용 | 80px/3:2와 height 40px/1:2 negative를 독립 검사한다. 통과. |
| 23 | min/max constraint로 ratio result를 재조정 | candidate style gate가 미지원 constraint를 거부하고 negative를 유지한다. 통과. |
| 24 | writing mode나 item placement를 바꿔 ratio축 의미가 뒤집힘 | 고정 horizontal LTR, auto placement, start-alignment profile 외는 지원하지 않는다. 통과. |
| 25 | wrapped text에서 line-height 하나를 전체 min-content로 취급 | eligibility는 고정 nowrap paragraph이며 다른 text topology는 거부한다. 통과. |
| 26 | ratio 없는 target에서 새 최소 크기 helper가 전혀 작동하지 않아 자동 최소 behavior 깨짐 | same exact text target의 intrinsic automatic minimum은 적용하고 ratio transfer는 생략한다. control frame 테스트가 통과한다. |
| 27 | Android runner가 host/default serial로 엉뚱한 기기에서 실행 | 고정 `emulator-5554`의 emulator property 및 device state를 검사한다. 통과. |
| 28 | iOS runner가 임의 부팅된 Simulator에서 실행 | 고정 iPhone 17 Pro UDID가 booted device list에 있는지 검사한다. 통과. |
| 29 | fixture가 실제 bundle에 들어가지 않거나 실행 후 임시 원본이 남음 | APK/app bytes 검증, 네 실행 캡처, 마지막 bootstrap 원본 SHA 복원 검사를 통과했다. 앱도 실행 뒤 종료했다. |
| 30 | 제한된 simulator draw-present 또는 broad green 결과를 일반 CSS/공식 C11 완료로 발표 | README의 pixel/compositor 한계와 broad test 실패를 명시하며 status/계약 완료 및 공식 수를 올리지 않는다. 통과. |

**판정:** 30개 관점에서 구현·fixture·증거의 실패 경로를 대조했다. focused 6+1+1+1, Clippy/rustfmt 및 simulator 네 조합은 통과했지만 broad suite에는 별도 기존 실패가 있어 전체 성공으로 판정하지 않았다. PR의 required CI와 merge state가 확인되기 전에는 병합 완료로 보고하지 않는다.
