# C11.4 cyclic max-content mixed text/image 계획 PR diff 검토 30개

계획과 사전 비교 PR diff를 서로 다른 실패 관점 30개로 검토했다. 이 검토는 후보 계획과 입력 근거만 다루며 runtime 구현 근거가 아니다.

| # | Diff 실패 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | 상태 업데이트가 C11.4 완료 checkbox를 바꾸는가 | 바꾸지 않았다. 상위 항목은 미완료로 남는다. |
| 2 | 상태 업데이트가 `77/213`을 올리는가 | 올리지 않는다. 공식 완료 수를 그대로 적었다. |
| 3 | 상태표가 이번 후보를 구현 완료로 기록하는가 | 아니다. runtime 미시작이라고 썼다. |
| 4 | 상위 계획이 replaced-image slice PR #520을 병합 완료로 기록하는가 | 그렇다. 병합 시각·SHA·빈 check rollup·required check 부재를 기록했다. |
| 5 | 상위 계획이 mixed text/image min-content slice PR #522를 병합 완료로 기록하는가 | 그렇다. 이전 runtime 미시작 문구를 실제 병합 정보로 고쳤다. |
| 6 | 새 계획이 C11.4 전체, WPT 또는 일반 CSS 지원을 주장하는가 | 주장하지 않고 모두 제외 범위에 적었다. |
| 7 | 네 scene ID가 HTML·계획·JSON에서 일치하는가 | max-content 정/역순 및 definite 정/역순 네 ID가 같다. |
| 8 | 역순 scene에서 DOM 순서 외 변수가 달라지는가 | item ID, text, image, style과 placement를 고정했다. |
| 9 | 두 축의 track expression이 서로 다른가 | 두 축 모두 `minmax(calc(10% + 2px), max-content)`다. |
| 10 | HTML과 계획에서 root sizing이 일치하는가 | intrinsic width는 `max-content`, control은 40px, height는 `max-content`다. |
| 11 | gap·border·box-sizing이 일관되는가 | 2px gap, 각 2px border, padding 0, content-box로 고정했다. |
| 12 | 이미지가 fixture 경로에서 로드되는가 | 저장소의 기존 24×16 PNG 상대 경로와 hash를 기록했다. |
| 13 | CSS min-width와 이미지 natural width를 혼동하는가 | CSS 40px과 natural 24px을 각각 기록했다. |
| 14 | 이미지 aspect ratio가 독립적으로 고정되어 있는가 | 24×16, 3:2, `80/3px` block contribution을 분리했다. |
| 15 | Ahem 변경이 text contribution에 영향을 주는가 | font hash, weight, size, line-height, text를 고정했다. |
| 16 | font/image 로드 전에 관측이 실행될 수 있는가 | HTML이 `document.fonts.load`와 각 image `decode()` 완료를 기다린다. |
| 17 | Chrome 결과를 독립 기대값의 출처로 썼는가 | 아니다. browser observation JSON과 CRD 산출 JSON을 분리했다. |
| 18 | max-content와 definite column sizing 결과를 합쳤는가 | `[107/3,125/3,86/3]`과 `[12,12,12]`를 구분했다. |
| 19 | row 계산에서 gutter/base를 빼기 전에 증가량을 나누는가 | 아니다. `(80/3 - 2 - 4)/2 = 31/3`으로 분배한 뒤 base를 더한다. |
| 20 | cyclic row minimum이 올바른 inner axis에서 resolve되는가 | `80/3px` inner height를 사용해 `14/3px`로 계산했다. |
| 21 | root border-box 크기에 양쪽 border가 포함되는가 | content 각 축에 총 4px를 더했다. |
| 22 | 40px control에서 실제 image overflow가 생기는가 | x=16px에서 폭 40px 이미지는 44px border box보다 12px 더 나간다. |
| 23 | text line width가 사용 가능한 grid area와 맞는가 | max-content에서 한 줄, definite 40px에서는 A 3줄/C 4줄을 고정했다. |
| 24 | frame을 DOM index로 비교해 reverse order가 깨지는가 | A/B/C identity 기준으로 frame을 비교한다. |
| 25 | fractional CSSOM 값이 유리수 기대값과의 exact 불일치로 실패하는가 | 각 사용값에 1/64 CSS px tolerance를 적용한다. |
| 26 | screenshot이 앱 실행 결과처럼 표기되는가 | Chrome Agent Browser 사전 비교라고 명시했다. |
| 27 | screenshot이 Android/iOS runtime 성공을 주장하는가 | 주장하지 않는다. 두 simulator는 후속 관문이다. |
| 28 | border 한 점을 전체 border-stroke 적합성으로 부풀리는가 | 선택 지점 검증과 전체 stroke 한계를 구분했다. |
| 29 | 브라우저 정리가 unrelated Chrome/tmux를 종료하는가 | 아니다. 지정한 격리 session만 닫았고 상태 파일은 `inactive`, `pid:null`이다. |
| 30 | 최종 diff에 공유 checkout의 미관련 미추적 근거가 들어가는가 | 아니다. 이 계획·로드맵, 관련 stale-state 수정과 후보 사전 비교 자료만 포함했다. |

## 결론

계획 PR diff의 파일 범위, 상태 기록, 고정 입력, 독립 산술, browser 관측과 후속 simulator 한계가 서로 일치한다. 차단할 계획/근거 결함은 발견하지 못했다.
