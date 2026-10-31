# C11.4 anonymous Grid text `text-indent` 계획 PR diff 실패 관점 검토 30개

검토 대상은 계획 PR의 실제 staged diff다: 계획 1개, 계획 검토 1개, Chrome 사전 비교 README/JSON 2개. 구현 코드 변경은 없다. 계획 검토 20개와 분리한 PR 변경 검토다.

| # | diff 실패 관점 | 판정 |
|---:|---|---|
| 1 | 계획을 구현 완료로 오인할 수 있는 제목/상태인가 | 통과. 제목과 status가 구현 전 계획이라고 명시한다. |
| 2 | 상위 C11 계획 링크가 유효한가 | 통과. `plan/c11-css-grid.md`가 존재한다. |
| 3 | C11.4 track sizing 계획 링크가 유효한가 | 통과. `plan/c11-4-grid-track-sizing.md`가 존재한다. |
| 4 | 내부 계약 0059 링크가 엉뚱한 계약을 가리키는가 | 통과. CSS Grid 내부 계약 문서로 연결된다. |
| 5 | 공식 상태 링크가 list item에 보장되지 않는 synthetic anchor에 기대는가 | 통과. 애매한 fragment를 빼고 권위 있는 상태 대장 파일로 직접 연결했다. |
| 6 | 사전 비교 README 링크가 staged file에 없는가 | 통과. 계획과 evidence 폴더가 함께 추가된다. |
| 7 | JSON observation relative link가 잘못됐는가 | 통과. README의 sibling JSON 경로와 일치한다. |
| 8 | 계획 review 20 링크가 빠져 있거나 이름이 다른가 | 통과. 경로와 생성 파일이 일치한다. |
| 9 | PR diff review 30 링크가 빠져 있거나 이름이 다른가 | 통과. 경로와 생성 파일이 일치한다. |
| 10 | Ahem font SHA가 현재 fixture font와 다른가 | 통과. 계산 SHA가 `b719ec…b8448`과 일치한다. |
| 11 | browser version이나 Agent Browser 사용을 기록하지 않는가 | 통과. UA와 도구를 observation JSON/README에 둔다. |
| 12 | Ahem이 로드되지 않은 fallback font 결과를 기록하는가 | 통과. 최종 Grid probe와 block probe 모두 `fontLoaded=true`였다. |
| 13 | Grid probe가 직접 text item이 아닌 별도 element를 측정하는가 | 통과. anonymous Grid item으로 생성된 direct text node를 측정했다. |
| 14 | min-content Grid track과 block intrinsic measurement를 혼동하는가 | 통과. 둘의 관측값을 JSON의 별도 probe로 보존한다. |
| 15 | expected 결과를 Chrome 관측과 같은 데이터로 취급하는가 | 통과. expected는 font table과 규범식, observation은 독립 JSON이다. |
| 16 | 10px Ahem glyph advance를 잘못 환산하는가 | 통과. 1000/1000 UPEM을 10px로 환산한다. |
| 17 | scene 수 18이 matrix 곱과 맞지 않는가 | 통과. 세 text × 세 indent × 두 track kind다. |
| 18 | `XX` 한 단어의 min/max 계산이 정의되지 않는가 | 통과. 별도 기대값은 둘 다 `20px + d`다. |
| 19 | 후속 word 집합이 빈 경우 min-content 식이 모호한가 | 통과. 빈 최대값의 기본값을 0으로 적었다. |
| 20 | min-content에서 first-line indent를 다음 모든 line에 더하는가 | 통과. 첫 fragment와 후속 unbreakable word 최대를 구별한다. |
| 21 | max-content에서 collapsed separator advance가 누락되는가 | 통과. 두 separator의 10px advance를 포함한다. |
| 22 | trailing/repeated whitespace를 보존한 것으로 가정하는가 | 통과. normal collapse와 trailing trim 후 남는 source를 적었다. |
| 23 | percentage를 definite px 값으로 처리하는가 | 통과. computed percentage는 intrinsic basis 의존으로 제외한다. |
| 24 | negative indent를 단순 가산식으로 확장하는가 | 통과. Chrome negative control을 기록하고 fixture scope에서 제외한다. |
| 25 | valid `hanging`/`each-line`을 잘못된 CSS라 부르는가 | 통과. valid oracle input이나 미지원 fixture profile로 표현한다. |
| 26 | authored `calc(5px)` 같은 식의 원본을 computed value만으로 거부한다고 주장하는가 | 통과. computed serialization 경계와 author-source 비판정을 명시한다. |
| 27 | Parley API가 저장소 pinned version과 다른가 | 통과. 현재 dependency Parley 0.12.0의 `set_text_indent`를 쓴다. |
| 28 | simulator/AVD 결과를 실기기·hardware GPU·paint로 확대하는가 | 통과. 경계와 compositor 이전 상태를 명시한다. |
| 29 | 기본/fixture archive 분리 검사로 APK·IPA 제외를 주장하는가 | 통과. static archive에 한정한다고 명시한다. |
| 30 | staged diff에 계획 외 로컬 evidence/artifact가 섞였는가 | 통과. review 보고서를 제외한 정확한 네 계획/사전비교 파일만 변경됐다. 최종 PR에는 이 별도 review 보고서도 포함한다. |

## 결과

30개의 diff 경계를 확인했다. 검토 중 상태 list-item fragment가 생성된다고 보장할 수 없어 링크에서 fragment를 제거했고, 해당 link 관점을 다시 확인했다. 이 PR은 계획·사전 비교만 추가하며 공식 상태 체크·내부 계약 버전·C11 완료 수를 바꾸지 않는다. 실제 scene 실행 화면은 아직 없으므로 plan PR에는 app screenshot을 첨부할 수 없다.
