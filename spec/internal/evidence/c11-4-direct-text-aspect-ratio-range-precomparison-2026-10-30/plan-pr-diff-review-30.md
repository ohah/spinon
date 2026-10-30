# C11.4 direct-text finite-positive ratio 계획 PR diff 실패 관점 검토 30개

**대상:** 계획, 계획 검토, 사전 비교 fixture/evidence, 공식 상태 대장, 상위 계획, 공개 로드맵의 작업 트리 diff
**기준:** `main` / `origin/main` `3fb170450d82614d35c772845fa68c1b880505d0`
**판정:** 계획 PR diff 30개 관점 검토 완료. 런타임 구현 또는 C11.4 완료 검증은 아니다.

| # | 별도 실패 관점 | 판정 |
|---:|---|---|
| 1 | 계획 브랜치가 현재 main보다 뒤처지거나 의도하지 않은 commit을 포함 | PASS — merge base는 `3fb170450d82614d35c772845fa68c1b880505d0`이며 현재 변경은 아직 작업 트리 diff다. |
| 2 | 계획 PR에 runtime 제품 코드가 들어와 계획/구현 경계가 흐려짐 | PASS — Rust runtime/style/layout 파일은 바뀌지 않았고, 새 HTML은 사전 비교 입력뿐이다. |
| 3 | 새 internal contract 숫자 버전을 계획 단계에서 올림 | PASS — `0059`는 참조만 하며 계약 문서나 버전을 수정하지 않는다. |
| 4 | 계획 항목을 공식 상태에서 완료 처리 | PASS — C11, C11.3, C11.3.3, C11.4 checkbox는 그대로 미완료다. |
| 5 | 프로젝트 공식 집계를 새 Chrome fixture 때문에 증가 | PASS — `72/213`을 상태 대장, 계획, roadmap card에서 유지한다. |
| 6 | 완료인 C11.2.2/C11.2.3을 C11 전체 완료로 일반화 | PASS — 두 자식 상태를 건드리지 않고 상위 C11 미완료를 명시한다. |
| 7 | 제한 fixture를 public CSS admission 또는 브라우저 적합성으로 표현 | PASS — 일반 author CSS, 전체 WPT와 public support를 범위 밖으로 둔다. |
| 8 | 새 route가 기존 opt-in fixture-test 경계를 우회 | PASS — `c11-4-grid-track-sizing-fixture-test`의 기존 direct-text route만 대상으로 한다. |
| 9 | fixture 입력이 의도하지 않은 Grid 구조·placement·text 형태를 허용 | PASS — 두 direct block child와 A/AAAA direct Text, source order 등 제한 조건을 계획한다. |
| 10 | 사용자 폰트 또는 OS fallback이 독립 기대값을 바꿈 | PASS — 저장소 Ahem 10px과 고정 line-height를 요구하고 font load/hash를 기록한다. |
| 11 | Oracle matrix의 scene 수와 ratio/no-ratio 조합이 계획과 불일치하거나 기존 2/1 regression이 새 matrix 일부로 오인됨 | PASS — JSON의 row 4+control 및 column 4+controls 4는 총 13 scene이고, 2/1 네 회귀 scene은 #834 집합으로 분리했다. |
| 12 | `aspect-ratio` 방향을 height/width로 적용 | PASS — CSS의 width/height 방향을 기준으로 네 row와 네 column 결과를 고정한다. |
| 13 | Row ratio 계산이 max-content inline width와 분리되지 않음 | PASS — 모든 row 장면의 두 column track `[10,40]`과 40px AAAA를 별도로 기대한다. |
| 14 | 3/2의 분수 used size를 반올림 오차로 잘못 처리 | PASS — `26.65625px` 관측을 기록하고 1/64 CSS px quantization을 허용한다. |
| 15 | Definite-row column 장면에서 preferred width만으로 track을 결정 | PASS — text max-content 40px floor와 ratio-derived item size를 별개로 계산한다. |
| 16 | `column-1-2`의 root 밖 item overflow를 잘라내거나 숨김 | PASS — `100×60` root, `40×80` item, 20px overflow를 독립 관찰로 기록한다. |
| 17 | Ratio 없는 baseline을 ratio 경로와 섞어 regression 감지 불가 | PASS — auto row control 및 definite row 네 개 모두에 no-ratio control이 있다. |
| 18 | Ratio가 짧은 A 또는 두 item에 동시에 설정돼 profile 한계를 넘음 | PASS — short-item ratio와 다중 ratio owner를 fail-closed 입력으로 둔다. |
| 19 | Non-replaced text를 replaced image의 transferred-size-suggestion 규칙으로 오분류 | PASS — Grid §6.6 shortcut을 금지하고 §6.2/§11.5 경로를 지정한다. |
| 20 | Draft source를 확정 규범으로 표시하거나 최신 문서 URL이 시간에 따라 변함 | PASS — Grid 2025 CRD와 버전 지정 CSS Sizing 2026-09-04 WD를 구별한다. |
| 21 | `auto <ratio>` content-box 동작을 typed ratio로 조용히 축소 | PASS — 현재 Taffy mapping 한계를 이유로 cascade rejection을 유지한다. |
| 22 | Degenerate ratio가 `None`으로 투영돼 no-ratio control로 오인 | PASS — computed serialization이 `auto`인지 함께 확인하며 degenerate numeric serialization은 거부한다. |
| 23 | Computed serialization과 typed ratio의 값이 다른데 source string을 사실로 채택 | PASS — typed snapshot만 ratio source로 삼고 serialization은 정합성 검사로만 사용한다. |
| 24 | finite-positive `f32`에서 derived dimension overflow 발생 | PASS — `f32::from_bits(1)`의 40px/ratio와 `f32::MAX`의 60px×ratio를 Taffy 전 거부한다. |
| 25 | Invalid CSS declaration이 computed `auto`가 된 것을 NaN ratio로 오판 | PASS — source declaration과 computed value의 경계를 테스트 범위에 둔다. |
| 26 | 기존 #834 `2/1` four scene이나 no-ratio/legacy route 회귀를 놓침 | PASS — 구현 plan에 기존 장면, controls, one-item fallback 및 generic route 회귀를 유지한다. |
| 27 | Chrome evidence fixture와 계획의 입력·기대값이 다른 revision 또는 font를 가리킴 | PASS — fixture/font/관측 JSON/screenshot SHA-256 및 browser metadata를 기록한다. |
| 28 | Roadmap 캡처를 배포 또는 Tailnet proof로 확대 | PASS — 로컬 정적 HTML임을 표시하고 icon 404, 미검증 link 범위를 metadata에 남긴다. |
| 29 | Android/iOS simulator 또는 Chrome oracle 결과를 Spinon runtime 결과로 표현 | PASS — 아직 runtime 실행이 없고 AVD/Metal 제약은 구현 단계 계획에 한정한다. |
| 30 | 계획 링크, 상태 링크, PR screenshot이 리뷰자가 재확인할 수 없음 | PASS — local markdown 링크 대상이 존재하며 Chrome/roadmap PNG와 hash가 있다. |

## 발견 후 수정

초기 계획 문구는 Stylo가 degenerate ratio를 오류로 거부한다고 잘못 설명했다. 소스를 확인한 결과 `computed_layout_aspect_ratio`는 이를 CSS 규칙에 맞게 `None`으로 투영한다. 계획을 수정해 fixture profile은 computed serialization을 검사하고 degenerate numeric input을 no-ratio control로 오인하지 않게 했다. 또한 finite-positive typed ratio의 derived geometry overflow 대조를 추가하고, 해당 경로를 다룬 계획 검토 항목 #11을 재확인했다.

검토는 계획 diff와 고정된 Chrome 사전 비교 자료에 대한 것이다. Runtime tests, Android/iOS 실행, CI, WPT conformance 및 전체 C11 동작은 평가하지 않았다.
