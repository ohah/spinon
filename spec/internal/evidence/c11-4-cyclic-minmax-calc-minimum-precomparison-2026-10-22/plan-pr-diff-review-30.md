# C11.4 cyclic `minmax(calc())` minimum 계획 PR diff 검토 30개

검토 대상은 계획 PR의 초기 staged diff 14개 파일·535 insertions다. 이 파일은 그 diff를 검토한 뒤 추가했다. 각 관점은 계획 검토 20개와 다른 PR 문서·근거·fixture 통합 실패 경로를 확인한다.

| # | 적대적 관점 | 검토와 결과 |
| --- | --- | --- |
| 1 | STATUS checkbox가 계획만으로 완료될 수 있음 | C11.4 parent checkbox는 `[ ]` 그대로다. 통과. |
| 2 | 공식 완료 수가 새 계획을 구현 완료로 셈 | 신규 STATUS·공개 roadmap 문구 모두 `72/213` 유지와 미구현 상태를 명시한다. 통과. |
| 3 | 공개 roadmap 문구가 runtime이 이미 동작한다고 읽힘 | 계획·사전 비교로 한정하고 simulator 전임을 적었다. 통과. |
| 4 | 공개 계획 URL이 잘못된 경로를 가리킴 | `/spinon/plan/c11-4-cyclic-minmax-calc-minimum.html` 경로를 사용하며 기존 Rspress route 규칙과 일치한다. 통과. |
| 5 | 부모 계획의 새 heading이 기존 `fit-content(calc())` 절에 끼어듦 | 새 heading을 parent plan 말미의 독립 항목으로 배치하고 기존 절 문장을 보존했다. 통과. |
| 6 | parent plan 또는 STATUS 링크 상대 경로가 저장소 위치와 불일치 | `plan/` 및 `spec/`에서 계산되는 상대 링크를 각각 확인했다. 통과. |
| 7 | fixed slice 이름과 이번 구현 의미가 다름 | min sizing function에 calc를 두고 max-content를 maximum으로 사용한다. 통과. |
| 8 | 직전 calc maximum fixture를 재사용해 신규 minimum 동작을 증명하지 못함 | 입력의 calc는 전부 `minmax()` 첫 번째 인자다. 통과. |
| 9 | calc minimum 영향이 보이지 않는 값 조합 | 24/22px minimum이 10px max-content를 넘도록 200% basis를 선택했다. 통과. |
| 10 | percentage basis를 border-box로 오인 | root는 content-box 10px이고 border-box 14px; JSON과 문서에서 분리한다. 통과. |
| 11 | CSS border가 percentage denominator를 바꿈 | computed width/height와 root rect를 함께 기록해 basis 10px을 보인다. 통과. |
| 12 | cyclic과 definite sizing이 같은 단순 definite 결과만 확인 | cyclic column/row와 definite column/row control을 모두 별도 scene으로 둔다. 통과. |
| 13 | column/row가 같은 축 조건을 실수로 공유 | column은 max-content inline sizing, row는 max-content block sizing이며 반대 축 input을 고정한다. 통과. |
| 14 | Ahem 상대 URL이 fixture 폴더 기준으로 잘못됨 | `../../resources/c15/Ahem.ttf`가 `tests/fixtures/resources/c15/Ahem.ttf`에 resolve되고 browser가 loaded를 기록했다. 통과. |
| 15 | system fallback font로 Chrome geometry가 달라짐 | font status와 `document.fonts.check()` 결과를 observation에 남겼다. 통과. |
| 16 | Chrome 버전이 누락되어 oracle 재현 불가 | user agent에 HeadlessChrome 156.0.8078.48을 보존했다. 통과. |
| 17 | viewport/DPR 차이가 pixel 좌표에 섞임 | DOM observation과 기대 JSON에 viewport 1280×633 및 DPR 1을 기록했다. 통과. |
| 18 | computed track list만 있고 actual box geometry가 빠짐 | root, item DOMRect와 scroll extent를 네 장면 모두 기록했다. 통과. |
| 19 | outer border 14px을 root content size로 오인 | observation은 computed content size와 border-box DOMRect를 따로 담는다. 통과. |
| 20 | overflow sample이 item 내부만 읽음 | column/right, row/bottom root border 외부와 item 바깥 control을 나눈다. 통과. |
| 21 | sample 좌표의 row y 오프셋을 무시 | JSON은 absolute screenshot coordinates를 저장하고 README는 root-relative scene 해석을 적는다. 통과. |
| 22 | root border 존재가 overflow child에 가려져도 전체 border 통과로 잘못 판정 | 가려지지 않는 left border control과 overflow에 덮이는 right/bottom 지점을 분리했다. 통과. |
| 23 | 테스트 전용 테두리 overlay를 실제 CSS border라고 오인 | 계획은 runtime CSS/layout/paint source를 요구하고 overlay를 금지한다. 통과. |
| 24 | Chrome PNG RGB가 CSS computed sRGB와 다른 사실을 숨김 | computed border color와 screenshot sample의 차이를 README와 pixel JSON에 기록한다. 통과. |
| 25 | Chrome screenshot pixel 값을 Android/iOS exact RGB로 전용 | PNG는 공간/paint order oracle로만 사용하고 simulator 기대 RGB는 별도 고정하도록 범위를 정했다. 통과. |
| 26 | PixelCopy 또는 Metal coordinate 변환이 검증 계획에서 누락 | 구현 gate가 density/DPR, surface origin/scale, native samples의 원본 기록을 요구한다. 통과. |
| 27 | browser screenshot과 DOM observation이 서로 다른 fixture에서 나옴 | 같은 local HTML 경로에서 observation과 screenshot을 캡처하고 manifest로 digest를 묶었다. 통과. |
| 28 | 규범 기대값이 Chrome 관측에서 역산됨 | 독립 계산 JSON은 Ahem contribution, percentage basis, `minmax()` floor 규칙을 별도로 산출한다. 통과. |
| 29 | 검증 자료가 수정되어도 발견되지 않음 | fixture/font/observation/screenshot/pixel samples/W3C note/expectations/README/browser close의 SHA-256 manifest를 검증했다. 통과. |
| 30 | 계획 PR이 simulator 또는 runtime 구현까지 끝냈다고 보고 | README·plan·STATUS·public roadmap 모두 runtime 미실행 및 전체 C11.4 미완료를 구분한다. 통과. |

## 검토 이후 추가 수정

문서 빌드에서 이 evidence README가 `spec/` 밖 fixture로 향하는 상대 Markdown 링크와 JSON·PNG·TXT를 Rspress route로 해석한 링크 때문에 dead-link 보고에 포함됐다. 대상 GitHub 파일 링크로 바꾸고 SHA-256 manifest를 다시 계산했다. 재실행에서는 이 README가 오류 목록에서 빠졌고, 전체 빌드는 PR diff 범위 밖 문서 742개의 dead-link 보고로 계속 실패했다. 계획·runtime 범위는 바뀌지 않았다.

판정: 30개 서로 다른 PR 통합 실패 경로를 검토했다. CSS Grid 전체 지원, runtime 동작 또는 simulator 결과를 이 계획 PR이 완료했다고 주장하는 항목은 없다. 계획을 구현하고 난 뒤에는 별도 20개 implementation/runtime review를 새로 수행한다.
