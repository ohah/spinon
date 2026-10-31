# C11.4 computed line-height 공개 로드맵 동기화 PR diff 적대 검토 30개

**검토 범위:** `spec/STATUS.md`의 #903 병합 기록과 `spec/public/roadmap.html`의 C11.4 computed `line-height` 계획 진행 항목.

| # | 독립 실패 관점 | 판정과 근거 |
| ---: | --- | --- |
| 1 | 잘못된 계획 PR 번호 또는 저장소 링크 | PASS — #903이 계획 PR로 연결된다. |
| 2 | Rebase and merge 결과를 구현 PR로 표시 | PASS — 링크와 문구가 계획 단계임을 명시한다. |
| 3 | 병합 시각 불일치 | PASS — `2026-10-31T04:27:15Z`가 병합 기록과 일치한다. |
| 4 | 병합 SHA 누락 또는 오기 | PASS — `fd5dc6d4a511912e1cafa483e4bae28ba02722ed`를 기록한다. |
| 5 | C11.4 candidate 체크를 완료로 올림 | PASS — C11.4 체크는 미완료로 유지된다. |
| 6 | C11 상위 항목을 완료로 올림 | PASS — C11 체크도 미완료로 유지된다. |
| 7 | `72/213`을 C11 전용 집계로 오해하게 함 | PASS — 공식 전체 완료 수로 명시한다. |
| 8 | 계획 완료를 Spinon runtime 구현으로 확대 | PASS — 구현/runtime은 시작하지 않았다고 밝힌다. |
| 9 | Chrome oracle을 Android 실행 증거로 확대 | PASS — Android/iOS 실행이 아니라고 명시한다. |
| 10 | Chrome oracle을 iOS 실행 증거로 확대 | PASS — 같은 범위 문구가 iOS에도 적용된다. |
| 11 | 16개 표본을 16개 독립 CSS 속성으로 오해 | PASS — 값·장면 reference로 표현하고 완전한 속성 지원을 주장하지 않는다. |
| 12 | row track과 root height 관찰을 혼동 | PASS — 두 관찰값을 각각 적고 독립 기대값 대조로 한정한다. |
| 13 | fixture 화면을 glyph ink/pixel 동일성으로 오해 | PASS — 해당 주장을 하지 않는다. |
| 14 | Ahem 10px reference의 고정 입력을 생략 | PASS — 브라우저 기준 입력을 짧게 식별한다. |
| 15 | 계획 문서 링크가 다른 candidate를 가리킴 | PASS — computed line-height 계획 경로와 이름이 일치한다. |
| 16 | 사전 비교 README 링크가 빠지거나 다른 자료를 가리킴 | PASS — 해당 작업의 precomparison README로 연결한다. |
| 17 | Chrome screenshot 링크가 존재하지 않는 경로를 가리킴 | PASS — 저장소의 `chrome156-line-height-matrix.png`를 가리킨다. |
| 18 | 계획 실패 관점 20개 링크가 잘못됨 | PASS — #903에 병합된 계획 검토 문서 경로를 사용한다. |
| 19 | 계획 PR diff 검토 30개 링크가 잘못됨 | PASS — #903에 병합된 diff 검토 경로를 사용한다. |
| 20 | `STATUS.html` 상태 anchor 링크 형식이 기존 사이트 규칙과 다름 | PASS — 기존 공개 로드맵에서 쓰는 동일한 상태 anchor를 사용한다. |
| 21 | Tailnet `/spinon/` route 접두사를 누락 | PASS — 문서·이미지 링크가 기존 공개 route 규칙을 따른다. |
| 22 | HTML inline code 태그가 잘못 중첩됨 | PASS — `line-height`, SHA와 완료 수만 유효한 `<code>` 요소로 감싼다. |
| 23 | 설명 문단의 `<p>` 또는 `<strong>` 구조가 깨짐 | PASS — 한 progress 문단 안에서 요소가 정상적으로 닫힌다. |
| 24 | 새 기록이 이전 #893 항목 뒤에 놓여 chronology가 역전 | PASS — 최신 #903 기록을 #893 기록 앞에 둔다. |
| 25 | 기존 #893 progress 문단을 덮어씀 | PASS — 기존 항목은 그대로 보존하고 새 문단을 추가한다. |
| 26 | 공식 상태와 공개 로드맵이 계획 결과 또는 미완료 상태에서 어긋남 | PASS — 병합 SHA, 계획 단계, `72/213`, 미완료 범위가 일치한다. |
| 27 | 내부 링크의 markdown 원본이 빠짐 | PASS — 계획, 사전 비교, 리뷰 문서가 저장소에 존재한다. |
| 28 | 변경이 일반 CSS 지원 범위나 WPT 통과를 주장 | PASS — 두 주장을 추가하지 않는다. |
| 29 | PR 본문의 screenshot 링크가 머지된 고정 산출물을 사용하지 않음 | PASS — PR 본문은 #903에 포함된 immutable raw GitHub screenshot URL을 사용한다. |
| 30 | 변경 문서 검사만으로 앱·simulator·CI 성공을 주장 | PASS — 이 동기화는 상태 문서 변경이며 새 앱 실행이나 CI 성공을 주장하지 않는다. |

이 문서 변경은 C11.4/C11 구현, 일반 CSS 지원, WPT 적합성을 완료로 승격하지 않는다.
