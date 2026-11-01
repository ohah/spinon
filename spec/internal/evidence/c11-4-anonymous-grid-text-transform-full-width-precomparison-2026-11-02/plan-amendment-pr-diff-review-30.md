# `full-width` 계획 보완 PR diff 실패 관점 검토 30개

대상 diff: 계획의 Parley line-breaking 보완, 구현 발견 evidence, Chrome observations·expected/README 갱신, parent plan 링크, 고정 HTML의 computed `word-break` observation field. 보완 PR 병합 전에 서로 다른 실패 경로와 주장을 다시 확인했다.

| # | diff 관점 | 검토 결과 |
|---:|---|---|
| 1 | 초기 계획 검토가 새 engine finding까지 포함한 것으로 오인되는가 | 별도 보완 검토 20개로 기존 계획 검토와 분리했다. |
| 2 | Parley result를 Chrome semantic oracle로 표현하는가 | internal engine output과 Chrome candidate output을 분리했다. |
| 3 | Chrome `text-transform` 값이 지원되는 듯 보이는가 | computed `none`과 unsupported classification을 유지했다. |
| 4 | CSS `word-break` candidate 값을 모르는가 | target·candidate computed value `normal`을 저장했다. |
| 5 | target `word-break`가 normal이 아닌가 | HTML root fixture의 computed style 관측에서 normal이다. |
| 6 | candidate `word-break`가 normal이 아닌가 | candidate spans/Grids computed value도 normal로 관측됐다. |
| 7 | min-content candidate가 실제 CSS Grid track이 아닌가 | `grid-template-columns:min-content` fixed candidate를 유지했다. |
| 8 | candidate track width와 line count가 충돌하는가 | width 20px·height 20px는 2×10px line과 일치한다. |
| 9 | max-content candidate가 min track width로 잘못 측정되는가 | literal full-width max candidate는 50px이며 inline max candidate다. |
| 10 | repeated ideographic spaces collapse되었다고 오해되는가 | 3개 U+3000 candidate는 70px로 하나의 50px candidate와 구분된다. |
| 11 | source repeated U+0020이 보존 처리와 섞이는가 | product expected는 Phase I 뒤 변환된 단일 U+3000다. |
| 12 | expected string의 mapping이 바뀌었는가 | source mapping은 `ＡＢ　１２`로 유지했다. |
| 13 | source UTF-16 range 계약이 새 override로 사라지는가 | layout range preservation gate를 amendment plan에 남겼다. |
| 14 | KeepAll을 CSS computed value로 투영하는가 | 내부 Parley setting과 HostDocument computed `normal`을 명시적으로 분리했다. |
| 15 | KeepAll이 모든 text transform에 적용되는가 | FullWidth mode와 two source allowlist로 제한했다. |
| 16 | KeepAll이 uppercase/lowercase/capitalize regressions에 노출되는가 | 기존 transform regression tests 유지 조건을 기록했다. |
| 17 | source split text가 우회 통로가 되는가 | `sources.len()==1`을 profile contract에 둔다. |
| 18 | `AB 12` 밖의 punctuation/non-ASCII가 부분 적용되는가 | exact input allowlist에 없는 content를 거부한다. |
| 19 | wrong `lang`에서 workaround 적용이 가능한가 | exact direct `lang=ja` guard를 유지한다. |
| 20 | stale computed snapshot에서 workaround 적용이 가능한가 | style/document/render revision gate를 유지한다. |
| 21 | `word-break:break-all` 등 다른 computed value가 정답으로 간주되는가 | computed normal validation은 scene 계약으로 유지한다. |
| 22 | U+3000 뒤 soft wrap이 막히는가 | actual two-line runtime result를 구현 gate로 요구한다. |
| 23 | min-content total height가 누락되는가 | min-content 20px/2줄과 max-content 50px/1줄을 별도 기준으로 둔다. |
| 24 | max-content total height가 누락되는가 | 10px 한 줄을 별도 assertion으로 둔다. |
| 25 | title/date와 evidence path가 충돌하는가 | 2026-11-02 evidence path와 plan amendment title이 일치한다. |
| 26 | screenshot이 이전 HTML hash를 대표하는가 | 갱신된 screenshot과 fixture hash를 함께 기록했다. |
| 27 | observation JSON에 선언 style과 계산 style이 섞이는가 | declared transform과 computed transform을 별도 field로 둔다. |
| 28 | observation JSON에 설명 없는 engine mismatch가 숨겨지는가 | implementation-discovery.md에 Normal 10px/4줄 결과를 보존했다. |
| 29 | KeepAll 실험을 completed fix라 부르는가 | evidence/plan은 이를 미완성 experiment로 명시한다. |
| 30 | parent C11.4 plan이 공식 status를 올리는가 | parent entry는 미완료·`72/213` 유지 조건을 남긴다. |

이 review는 구현/runtime 실패 관점 20개 또는 implementation PR diff review를 대신하지 않는다.
