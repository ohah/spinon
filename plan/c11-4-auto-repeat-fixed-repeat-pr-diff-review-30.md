# C11.4 fixed-repeat + auto-repeat 계획 PR diff 검토 · 실패 관점 30개

**대상:** 계획-only 변경의 전체 PR diff. 규범 근거는 CSS Grid 2025 CRD §7.2.3.1–7.2.3.2, 실행 oracle은 Chrome 156 Agent Browser다. 코드 변경이나 Spinon runtime 결과는 포함하지 않는다.

| # | 독립 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | 구현 전에만 정해야 할 oracle이 코드 결과에서 역산됐을 수 있음 | 독립 formula와 Chrome observation이 별도 evidence로 먼저 고정되어 있다. implementation/runtime은 시작 전이라고 문서에 표시했다. |
| 2 | C11.4 완료 checkbox가 계획 추가로 체크됐을 수 있음 | `spec/STATUS.md`의 C11.4는 미완료 checkbox다. 변경 후에도 그대로다. |
| 3 | `72/213` 집계가 계획을 제품 완료로 세었을 수 있음 | 공식 집계는 `72/213`으로 유지한다고 상태·계획·roadmap에 명시했다. |
| 4 | API/내부 계약 버전이 초안 추가만으로 상승했을 수 있음 | 계약 0059는 기존 `0.1.0`을 유지하며 숫자 버전 변경이 없다. |
| 5 | Conformance 문서가 Chrome 관찰을 Spinon 지원 결과로 오인할 수 있음 | `spec/0001-conformance.md`는 계획/oracle 단계라고 직접 구분한다. |
| 6 | 상태 대장이 integer-repeat 혼합을 이미 지원한다고 읽힐 수 있음 | C11.4 문단은 current profile rejection과 후속 계획을 분리해 적었다. |
| 7 | Status의 plan 상대 경로가 저장소 밖을 가리킬 수 있음 | `spec/STATUS.md`에서 `../plan/...`이 저장소 `plan/`의 새 파일에 도달한다. |
| 8 | 계약 0059의 plan 상대 경로가 `spec/` 아래 잘못된 곳을 가리킬 수 있음 | `spec/internal/0059...`의 `../../plan/...` 경로가 실제 계획 파일을 가리킨다. |
| 9 | 사전 비교 evidence 상대 경로가 다른 evidence 폴더를 가리킬 수 있음 | `spec/0001`, `STATUS`, 0059, plan의 사전 비교 링크가 같은 신규 evidence directory를 대상으로 한다. |
| 10 | RSPress가 root plan 문서를 HTML route로 만들지 않을 수 있음 | 선택된 RSPress 2.0.22 build에 `plan/c11-4-auto-repeat-fixed-repeat.html`이 생성되고 Agent Browser DOM에서 열렸다. |
| 11 | public roadmap raw HTML의 plan URL이 markdown 파일 확장자를 잘못 쓸 수 있음 | `/spinon/plan/c11-4-auto-repeat-fixed-repeat.html` route를 선택된 build의 plan page와 대조했다. |
| 12 | public roadmap source route와 RSPress route가 서로 다른 문서를 가리킬 수 있음 | 원본 `spec/public/roadmap.html`과 selected-root alias에서 새 후보 문단 및 plan href를 확인했다. |
| 13 | 외부 규범 링크가 검색 페이지나 비공식 copy를 가리킬 수 있음 | link는 W3C CSS Grid 2025 CRD repeat-notation section anchor다. |
| 14 | normative text가 같은-list fixed-repeat을 금지한다고 잘못 요약할 수 있음 | W3C Agent Browser에서 `<fixed-repeat>` coexist 문장과 한 번 제한된 auto-repeat 문장을 직접 확인했다. |
| 15 | 계획 범위가 표준의 전체 repeat grammar를 지원한다고 과장할 수 있음 | 한 fixed group, 한 auto-repeat, direct `px`, no names, one-axis profile로 제한했다. |
| 16 | `fixed-repeat` 안에서 repeat nesting을 허용할 수 있음 | nested repeat은 scope와 unsupported input 관문에서 제외했다. |
| 17 | 고정 반복 안의 unsafe size가 auto-repeat용 허용 범위를 우회할 수 있음 | fixed fragment는 finite nonnegative direct `LengthPx`만 허용하고 기존 neighbor validator는 유지한다. |
| 18 | prefix 반복 전개 후 auto-repeat index를 authored component index로 계산할 수 있음 | 계획은 expanded track index로 `start/end`를 계산하고 prefix case로 경계를 판정한다. |
| 19 | suffix fixed-repeat이 auto-repeat 앞에 이동할 수 있음 | suffix columns/rows Chrome cases가 order를 각각 고정한다. |
| 20 | empty line-name groups가 expanded component의 `N+1` 길이를 잃을 수 있음 | 구현 순서와 cascade/projection criteria에서 확장된 경계 수를 요구한다. |
| 21 | unsupported authored line names가 성공적으로 projection될 수 있음 | nonempty line-name group은 현재 slice에서 거부한다. |
| 22 | auto-fill count가 fixed track breadth를 제외할 수 있음 | 독립 하한은 non-repeated 76px를 포함하며 2회와 3회 경계를 산출한다. |
| 23 | auto-fill count가 gutter 수를 하나 이상 누락할 수 있음 | `8(4+n)` 전체 gap 수를 식에 넣고 prefix/suffix에서 동일한 count를 보였다. |
| 24 | auto-fit collapse를 반복 수 산정보다 먼저 적용할 수 있음 | 계획과 evidence에서 count 후 placement·collapse 순서를 분리했다. |
| 25 | 빈 auto-fit과 점유 auto-fit을 같은 결과로 취급할 수 있음 | 0px/0px empty case와 84px/0px occupied case를 따로 고정했다. |
| 26 | repeated-range boundary gutter가 occupied case에서 잘못 collapse될 수 있음 | occupied item 좌표 92px, used track 84px를 Chrome과 독립 arithmetic으로 함께 확인한다. |
| 27 | row repeat count에 column width를 basis로 쓸 수 있음 | rows scene은 definite 200px height, columns scene은 definite 200px width를 각각 갖는다. |
| 28 | 두 axis 동시 auto-repeat cross-product까지 구현된 것처럼 읽힐 수 있음 | 한 grid에서 두 축을 동시 확장하지 않고 해당 조합을 명시적으로 제외했다. |
| 29 | Chrome fixture가 row cases를 수집하지 않아 결과표가 부분일 수 있음 | 첫 capture에서 selector가 `.row-grid`를 놓친 점을 수정했다. 최종 Agent Browser observation JSON과 full screenshot은 columns/rows 8개 ID를 모두 포함한다. |
| 30 | 스크린샷·input hash·공개 설명이 서로 다른 revision을 가리킬 수 있음 | README, JSON과 최종 SHA-256 manifest에서 fixture, screenshot, plan 및 docs source hash를 대조한다. |

## 결론

이 diff는 C11.4의 미지원 계약을 곧바로 넓히지 않는다. 문서 route, independent oracle, 계획 범위, 비용·오류 경계와 완료 집계가 서로 맞으며 계획 PR을 병합할 수 있다. Build는 `mise exec -- bun run docs:build --config /tmp/spinon-c114-fixed-repeat-selected.rspress.config.ts`로 완료했다. 이 검토는 구현이나 mobile app 실행을 승인·증명하지 않는다.
