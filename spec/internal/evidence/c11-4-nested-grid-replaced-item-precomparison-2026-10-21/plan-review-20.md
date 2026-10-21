# 계획 실패 관점 검토 20개

계획 고정 후 구현 전에 20개의 서로 다른 실패 경로를 검토했다. 기존 nested text 계획 검토는 재사용하지 않았다.

| # | 실패 관점 | 계획에서 확인한 경계 | 결과 |
| --- | --- | --- | --- |
| 1 | Chrome 입력이 Ahem을 못 불러와 fallback font로 계산 | `document.fonts.check('10px Ahem')`를 저장하고 true 요구 | 통과 |
| 2 | PNG가 로드되지 않아 image size가 0 | `complete`, natural dimensions 24×16 기록 | 통과 |
| 3 | 코드가 이미지 natural width를 읽지 않고 fixture 숫자만 신뢰 | 고정 URL의 PNG bytes를 `ImageRequest::decode` 후 metadata로 확인 | 통과 |
| 4 | Browser oracle을 규범 기대값으로 오인 | 70/100px text, 24px image, 4px gap 독립 산술을 별도 기술 | 통과 |
| 5 | max-content 값 128px을 실제 acceptance에서 측정했다고 과장 | max-content는 계산 참고값, 측정 장면은 아님을 명시 | 통과 |
| 6 | 바깥 track 계산에서 gutter를 빠뜨림 | span-two `98 - 4 = 94`, 두 track 각 47 | 통과 |
| 7 | one-track의 빈 두 번째 track 또는 outer gap이 root 폭에서 빠짐 | 98/0 tracks와 4px gutter로 root 102px 고정 | 통과 |
| 8 | definite flex scene에서 track을 60/60으로 잘못 계산 | 120px - 4px gap = 116px, 각 flex factor 1로 58/58 | 통과 |
| 9 | span 두 개의 flex item auto minimum이 content-size 98로 잘못 처리 | §6.6 control expectation `Minimum=0px`로 별도 검사 | 통과 |
| 10 | nested Grid를 outer의 leaf text/image로 위장 | outer contribution item identity는 nested Grid node로 요구 | 통과 |
| 11 | 이미지 node가 nested Grid 자식이 아닌 곳에 잘못 위치 | exact parent-child topology와 source order 검증 | 통과 |
| 12 | 다른 namespace의 `img`를 HTML replaced element로 받음 | HTML namespace + local name `img` exact 검사 | 통과 |
| 13 | 비허용 image URL이나 다중 이미지가 허용됨 | 단일 고정 source/단일 image ID 및 exact subtree 검사 | 통과 |
| 14 | 잘못된 scene ID가 유효 경로를 선택하거나 fallback됨 | 세 scene allowlist, 같은 family 미등록 root fail closed | 통과 |
| 15 | 텍스트나 item 스타일이 바뀌어 oracle과 입력이 달라짐 | text/font/placement/track/box style를 computed style과 비교 | 통과 |
| 16 | intrinsic size snapshot revision이 render freshness에 반영되지 않음 | mixed input의 image revision을 current inputs에 전달하도록 구현 계획 | 통과 |
| 17 | image 데이터가 기본 product binary에 포함됨 | default와 opt-in artifact positive/negative 비교를 acceptance에 포함 | 통과 |
| 18 | Android/iOS 결과를 실기기 또는 compositor 증거로 확대 해석 | API 37 AVD와 iOS 26.2 Simulator 경계를 명시 | 통과 |
| 19 | text 및 image 어댑터의 우선순위 때문에 한 쪽 입력이 누락 | 한 `C11_4MixedInput`으로 합친 전용 adapter와 runtime 선택 검증 | 통과 |
| 20 | PR 이미지 URL이 branch/임시 파일에 묶여 merge 후 깨짐 | 병합 commit SHA 고정 링크를 Agent Browser로 검증한 후 process 종료 | 통과 |

검토로 계획 범위 변경은 없었다. 브라우저 fixture를 처음 구성하면서 발견한 alignment 변경과 누락된 placement selector는 최종 고정 입력에서 제거/수정했으며, 저장된 HTML·JSON·PNG가 최종 oracle다.
