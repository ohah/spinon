# C11.4 계획·사전 oracle PR 변경 적대 검토 · 30개 관점

**대상:** 이 PR의 변경 전체, PR 본문 이미지 미리보기, [C11.4 계획](../../../../plan/c11-4-grid-track-sizing.md).
**검토 기준:** PR 생성 뒤 diff와 표시된 Chrome 캡처를 병합 전에 다시 확인한다. 이 계획 PR은 제품 구현 PR의 20관점 검토와 별도다.

| # | 독립 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | C11 상위 항목이 계획 PR만으로 완료될 수 있음 | `spec/STATUS.md`의 C11 parent는 unchecked로 유지한다. |
| 2 | C11.4 실행 항목을 계획 승인과 동시에 완료로 읽음 | C11.4 계획 subtask도 unchecked이며 코드·Rust·Android/iOS 미착수를 적었다. |
| 3 | 새 계획·evidence 링크가 이동 또는 오타로 끊김 | 계획, fixture, README, review 상대 경로를 존재 확인한다. |
| 4 | PR에서 완료 집계 수가 늘어남 | 체크된 완료 항목과 ID를 추가하지 않아 공식 완료 수를 바꾸지 않는다. |
| 5 | phase 분리로 C11.4 목표를 축소 | A–D를 합쳐야 완료하며 auto-repeat·text intrinsic sizing을 남기면 미완료라고 명시한다. |
| 6 | 기존 C11.1–3 profile에 새 성공 의미 누출 | 별도 track-sizing profile과 fail-closed 경계를 요구한다. |
| 7 | columns만 구현하고 rows를 방치 | rows/columns 및 min-content row image oracle을 계획에 포함한다. |
| 8 | explicit track만 지원하고 implicit track을 누락 | positive implicit track과 `grid-auto-rows/columns` 확장을 B 단계에 둔다. |
| 9 | Stylo parser 인식만으로 기능 지원 판정 | typed projection·track stages·frames·diagnostics를 요구한다. |
| 10 | C14.3 natural image contribution을 재구현 또는 무시 | 기존 replaced-size source/revision과 natural `24×16` 입력 재사용을 요구한다. |
| 11 | C15 indefinite measure 오류를 0으로 대체 | `IndefiniteTextAvailableWidth` 경계를 계획에 기록하고 0 fallback을 금지한다. |
| 12 | text intrinsic measure가 definite C15 snapshot을 오염 | 같은 font/revision 입력과 별도 intrinsic result를 요구하고 기존 snapshot 유지 조건을 적었다. |
| 13 | Grid item automatic minimum을 track `auto`와 혼동 | specified/transferred/content suggestion, span-flex 예외와 track auto-minimum을 별개로 다룬다. |
| 14 | padding, border, box sizing을 contribution에 중복 또는 누락 | Chrome 80px edge case와 단 한 번 외곽 contribution 조건을 확인한다. |
| 15 | min/max width 제한이 contribution에 반영되지 않음 | `min-width:90px`와 `max-width:40px` case 및 expected track을 고정했다. |
| 16 | fit-content cap이 자동 최소보다 우선 | image24/cap12와 text70/cap40 결과로 lower bound를 확인한다. |
| 17 | percentage/calc argument가 잘못된 basis에서 계산 | definite 100px의 `calc(10% + 5px)` → 15px oracle과 indefinite control을 둔다. |
| 18 | cyclic percentage 최종 크기가 intrinsic pass로 역류 | 24px max-content grid, 12px final track, 24px image overflow를 분리한다. |
| 19 | spanning item 크기를 track 수로 단순 평균 | span length/function stage와 gap 중복 방지, multi-span controls를 요구한다. |
| 20 | flex track을 span하는 item의 automatic minimum이 track을 밀어냄 | Chrome 48px+48px tracks, 100px item box, 200px `scrollWidth`를 oracle로 둔다. |
| 21 | auto-repeat 문법에 intrinsic/non-fixed track을 허용 | 한 repeat 위치와 fixed-size grammar preflight 및 unsupported error를 요구한다. |
| 22 | auto-repeat 수에서 gap 또는 definite min/max 우선순위 누락 | preferred/max → minimum → fully-indefinite 순서를 normative stage로 기록한다. |
| 23 | indefinite auto-repeat이 viewport에 종속 | 세 basis가 모두 indefinite일 때 1회 반복, viewport 의존 금지를 명시한다. |
| 24 | auto-fit의 empty 판단이 span placement보다 먼저 실행 | placement 후 span 점유와 빈 반복 track·인접 gutter collapse를 각각 검사한다. |
| 25 | 10,000 track 한도·예약 실패 뒤 partial output | 기존 상한, fallible reserve, output atomicity와 같은 worker 복구를 계획 관문으로 둔다. |
| 26 | 내부 계약 숫자 버전을 계획 변경만으로 인상 | 출시 전 `0.1.0` 고정, 실제 인터페이스 변경 시 문서만 갱신한다. |
| 27 | Chrome/WPT 결과를 Spinon 적합성으로 과장 | pinned WPT는 후보·hash만 기록하고 Spinon WPT runner 부재를 명시한다. |
| 28 | browser/resource provenance 또는 수집 timing이 불명확 | Chrome full version·macOS·DPR, Ahem/image hash, image decode/font-ready 조건을 기록한다. |
| 29 | screenshot이 일부 viewport만 담거나 overflow가 잘림 | Agent Browser full-page `1280×1363` PNG에서 26 case와 200px text overflow를 확인한다. |
| 30 | PR 캡처·시뮬레이터·Tailnet 상태를 잘못 표시 | PR 본문 이미지는 Chrome oracle로 명시하고 raw committed screenshot을 표시한다. Simulator는 미실행, Tailnet은 병합 후 갱신 대상으로 분리한다. |

이 문서는 계획 PR 전용이다. 코드 변경 뒤에는 새로운 implementation 20관점, 구현 PR diff 30관점과 실제 앱/시뮬레이터 근거를 따로 수행한다.
