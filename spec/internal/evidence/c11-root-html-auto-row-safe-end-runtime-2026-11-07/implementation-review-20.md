# C11.4 `safe|unsafe end` 구현/runtime 실패 관점 20개

최종 후보 소스와 Android API 37 ARM64 AVD·iPhone 17 Pro/iOS 26.2 Simulator 실행을 대상으로 서로 다른 실패 경로를 확인했다. 계획 검토표를 재사용하지 않았다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | CSS 값의 공백·computed serialization이 정확 allowlist를 우회하거나 거부하지 않는가 | 정확히 `safe end`·`unsafe end`만 이 경로에 추가됐다. 통과. |
| 2 | 기준 결과를 Spinon 출력에 맞춰 사후 구성하지 않았는가 | Chrome 157 관측과 독립 Grid 계산이 사전 비교에 별도로 고정됐다. 8/8 scene·96/96 좌표 일치. 통과. |
| 3 | 양의 free space에서 end 위치가 잘못 계산되지 않는가 | `row-gap:0` 기대 `[10,20,30]px`를 layout/style/runtime 경로에서 확인. 통과. |
| 4 | safe overflow가 container 끝에서 역방향 overflow하지 않는가 | `safe end` 기대 `[0,56,112]px` 확인. 통과. |
| 5 | unsafe overflow가 start fallback으로 바뀌지 않는가 | `unsafe end` 기대 `[-82,-26,30]px` 확인. 통과. |
| 6 | plain `end` 결과가 제한값 추가로 변하지 않는가 | positive와 overflow의 plain end control이 기대값과 일치. 통과. |
| 7 | align-content가 track 크기까지 바꾸는가 | 모든 정렬에서 used row size `[10,10,10]px`; fixed track과 gutter 유지. 통과. |
| 8 | `%` basis가 viewport 또는 다른 축으로 잘못 잡히는가 | definite 40px block basis의 `calc(120% - 2px)` 결과 46px를 사용. 통과. |
| 9 | 기존 safe/unsafe center 값이 end 변경으로 회귀하는가 | overflow 기대 safe `[0,56,112]`, unsafe `[-41,15,71]` 및 RuntimeSession 회귀 통과. |
| 10 | unsupported safe/unsafe start가 허용되거나 잘못된 CSS 속성으로 보고되는가 | 둘 다 거부. 발견된 `grid-auto-rows` 오귀속을 `align-content`로 수정하고 정확한 진단 테스트 추가. 통과. |
| 11 | 안전 modifier가 distributed/baseline 값까지 넓어지는가 | safe/unsafe start 외 `safe space-between`, baseline, unknown value는 기존 profile 밖이다. 통과. |
| 12 | generic style projection의 전역 allowlist가 의도치 않게 넓어지는가 | 허용값을 `RootHtmlAutoRowStretchV1` computed allowlist에만 추가했다. 통과. |
| 13 | legacy intrinsic-row profile이 새 정렬을 무조건 허용하는가 | 후보 mode는 고정 Root HTML profile에만 설정한다. legacy 경로는 변경하지 않았다. 통과. |
| 14 | 다른 track/item topology가 후보로 잘못 분류되는가 | 고정 computed grid, definite dimensions, direct item 모양과 row-gap gate를 유지한다. layout/profile rejection controls 통과. |
| 15 | text item DOM 순서·소유 관계가 정렬로 섞이는가 | fixed empty item 다음 direct text `A`, `B`의 source/order를 유지했다. 통과. |
| 16 | unsupported projection이 일부 frame만 노출하는가 | RuntimeSession 오류 뒤 completed snapshot을 부분 갱신하지 않는 검사가 통과. |
| 17 | 실패 뒤 같은 session에서 합법적 입력으로 회복하지 못하는가 | 실패 다음 지원 alignment에서 layout 재계산 및 frame publish를 확인. 통과. |
| 18 | 시뮬레이터 검증이 fixture-only ABI나 다른 경로에 의존하는가 | 두 플랫폼에서 기본 Root HTML route, C11 fixture flags 비활성 상태로 실행. 통과. |
| 19 | viewport 밖 unsafe item을 양성 pixel 영역으로 잘못 판정하는가 | Rust 좌표가 음수인 첫 두 item을 보이고 OS screenshot은 마지막 visible item만 검출한다. 수단과 한계를 분리했다. 통과. |
| 20 | 캡처·build가 기존 앱이나 bootstrap 자산을 남기고 바꾸는가 | 각 pass가 설치 앱·APK·iOS executable·bootstrap 입력 해시를 복원했다. 세 pass 모두 restoration PASS. 통과. |

확인한 발견은 #10의 속성 오귀속이다. 이를 수정한 뒤 allowlist projection 회귀를 다시 실행했다. Simulator screenshot은 PixelCopy/Metal surface readback이 아니며, 이 검토는 physical device, WPT 또는 전체 C11 완료를 주장하지 않는다.
