# `minmax(calc())` 겹치는 row span 계획 실패 관점 검토 20개

2026-10-22에 row-axis 고정 입력, Agent Browser 관측, CSS Grid 독립 계산, 범위와 구현 판정 관문을 검토했다. 이 문서는 계획 전용이며 runtime 구현 검토나 PR diff 검토가 아니다.

| # | 별도 실패 경로 | 계획에서 확인한 방어 |
|---:|---|---|
| 1 | 열 축 계획의 수치를 근거 없이 행 축에 복사 | 고정 column 10px 안의 실제 line count와 block contribution을 다시 계산하고 row-axis Chrome 관측을 따로 기록한다. |
| 2 | body 글꼴 fallback으로 Ahem 행 높이가 달라짐 | fixture의 Ahem 경로와 `document.fonts.check("10px Ahem")`을 확인하고 font SHA를 manifest에 고정한다. |
| 3 | 10px 폭에서 글자들이 의도대로 한 줄씩 감기지 않음 | 고정 column과 `word-break:break-all`을 입력에 넣고 observation의 7/8/14 line rect를 보존한다. |
| 4 | row contribution을 text glyph advance만으로 계산하고 line-height를 누락 | 10px line height에 줄 수를 곱해 70/80/140px을 산출한다. |
| 5 | row gap을 span-2 contribution에서 빼지 않음 | 두 행 span에는 내부 gap 2px 하나를 빼고 track별 34/39px increase를 계산한다. |
| 6 | span-3에서 gap 두 개를 하나 또는 세 개로 계산 | 140px에서 현재 track 합과 내부 gap 4px을 각각 빼 24px을 남긴다. |
| 7 | 같은 span-2 그룹의 중첩 가운데 track increase를 합산 | 같은 span 길이의 계획값을 track별 최대 `[34,39,39]`로 합친다. |
| 8 | span-3 item을 짧은 span group보다 먼저 처리 | §11.5.1 그룹 순서대로 span-2 결과 뒤에 각 행 +8px을 적용한다. |
| 9 | reverse 장면에서 DOM만 뒤집는 대신 row placement도 바뀜 | kind별 `grid-row` 값을 고정하고 markup 순서만 앞뒤로 바꾼다. |
| 10 | CSS line 번호 끝값을 track count로 오해해 implicit row가 생김 | 세 explicit row와 `1/3`, `2/4`, `1/4`의 여섯 scene을 명시한다. |
| 11 | `height:max-content` cyclic basis를 definite height로 취급 | cyclic height와 140/100px definite controls를 별도 입력으로 비교하고 §7.2.1 단계를 기록한다. |
| 12 | calc minimum에 percentage를 intrinsic sizing 중 확정 적용 | intrinsic 계산과 결과 height에 대한 final resolve를 분리해 16px cyclic minimum을 설명한다. |
| 13 | 100px control에서 gap 4px을 남은 크기 분배에 누락 | 세 12px minima와 두 2px gap 뒤 60px을 균등 분배해 32px 세 행을 계산한다. |
| 14 | grid `content-box`와 border-box 높이를 혼동 | content height 140/100px과 외곽 border box 높이 144/104px을 따로 표기한다. |
| 15 | spanning item frame height를 text block size로 오해 | 100px control에서 frame 66/66/100px과 text 70/80/140px을 각각 기록한다. |
| 16 | `scrollHeight`만으로 actual line breaks를 단정 | line rect 개수와 크기를 primary wrapping evidence로 두고 frame·scrollHeight를 별도 보조 관측한다. |
| 17 | root-relative overflow 좌표가 x/y, CSS px, backing pixel 또는 기준 원점을 혼동 | 다음 runtime 표본은 `(7,126)`, `(7,150)` CSS px로 고정하고 기록된 DPR·native viewport origin으로 변환한다. |
| 18 | browser screenshot의 page white를 native host clear로 판정 | browser 관측과 Android PixelCopy/iOS drawable 표본을 다른 표면으로 취급한다. |
| 19 | Agent Browser 프로세스가 테스트 후 남거나 사용자 Chrome 종료 | 실행 전 활성 session이 없음을 확인한 뒤 `close --all`을 사용하고, 실행 후 빈 목록과 전용 PID 종료를 확인하며 사용자 Chrome/Tailnet PID는 보존한다. |
| 20 | 여섯 장면 성공을 일반 row sizing 또는 C11.4 완료로 확대 | 명시 allowlist와 제외 범위를 쓰고 공식 완료 수 `77/213` 및 C11.4 미완료 상태를 유지한다. |

## 판정

20개 항목은 독립된 글꼴·줄바꿈·규범 분배·cyclic basis·box geometry·표면 경계·fixture 범위 실패를 다룬다. 구현 전에 입력이나 규범 계산이 바뀌면 사전 비교와 이 계획을 다시 검토한다.
