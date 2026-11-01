# C11.3.3 음수 named-line pair PR diff 실패 관점 검토 30개

검토 대상은 `origin/main`의 `915dbf39`부터 구현 commit `0096e39d`까지의 코드·fixture 변경과 이 evidence 변경이다. 앞선 구현/runtime 검토 20개와 구별해, 최종 diff의 경계·platform route·근거 연결·상태 문구를 다시 대조했다. 아래 30개 관점에서 새 결함은 발견하지 않았다. GitHub required check과 merge는 아직 실행하지 않았다.

1. **작업 브랜치가 의도한 공식 기준점에서 갈라졌는가?** — `git fetch origin` 뒤 `origin/main`은 `915dbf39`이고 구현 commit은 `0096e39d`; PR 후보는 공식 main에 없는 한 개의 구현 commit과 이번 문서/evidence 변경이다.
2. **코드 diff 범위가 이 slice 밖으로 번졌는가?** — 구현 diff는 21개 코드·fixture 파일, 1,097 additions/50 deletions다. 관련 문서와 C11.3.3 evidence 외 staged path가 섞이지 않았음을 path allowlist로 확인했다.
3. **런타임 결과가 검토한 구현 revision에 묶였는가?** — 전용 Android/iOS와 양 플랫폼 기존 matrix의 `source.commit`이 모두 `0096e39d59988abb60768129a5cdada0ec077497`를 가리킨다.
4. **새 placement mode가 기존 public/제품 경로에 노출됐는가?** — 변경은 internal `GridPlacementMode`와 `RuntimeBlockFlexGridTemplateV1` projection에만 연결된다. public CSS/API admission 변경은 없다.
5. **auto-repeat resolver와 negative implicit resolver가 충돌하는가?** — auto-repeat 축은 기존 `project_axis_for_auto_repeat` 경로를 사용하고 새 implicit usage flag를 false로 만든다. 대상 projection tests와 CSS reference 18개가 통과했다.
6. **area-backed placement가 no-area profile로 오인되는가?** — projection gate는 `template_areas.is_none()`이며 layout validation도 pair mode에서 areas를 오류로 거부한다. 기존 39-scene area matrix는 양쪽 플랫폼에서 재실행됐다.
7. **이름이 다른 endpoint가 line 번호가 같다는 이유만으로 통과하는가?** — 원래 custom-ident 일치 여부를 resolver 실행 전에 보존한다. 서로 다른 이름 회귀 입력은 unsupported auto columns로 거부하며 focused projection/integration tests가 통과했다.
8. **negative occurrence의 leading 방향 경계가 off-by-one인가?** — 3 explicit tracks의 leading pair는 `-4/-1`이 `-8/-5`로 projection된다. 10개 Chrome precomparison과 fixed-px/equal/reversed layout controls가 같은 결과를 고정한다.
9. **authored named line보다 implicit line이 먼저 선택되는가?** — authored occurrence control은 새 auto profile flag를 선택하지 않는다. 기존 named-line 우선순위 test 및 Chrome control을 유지했다.
10. **양쪽 축이 implicit이어도 새 profile로 통과하는가?** — pair flag는 정확히 한 축에서만 implicit usage가 있을 때 만들고 layout shape validator도 양축 pair를 거부한다. 양축 negative fixture는 unsupported rows error를 반환한다.
11. **반대 축의 auto/span placement가 definite placement로 둔갑하는가?** — 반대 축은 numeric `Line` 양끝 또는 단일 definite line의 auto 끝점만 인정한다. projection gate와 layout validator가 모두 확인한다.
12. **여러 participating item이 첫 item의 mode 선택으로 통과하는가?** — 최종 validation에서 보이는 direct participating child 수가 1인지 확인한다. 두 item control은 partial frame 없이 `InvalidGridPlacement`로 끝난다.
13. **fixed-size item 조건이 축 하나만 검사하는가?** — projection은 width와 height 모두 `LayoutDimension::Fixed`인지 확인한다. auto width control은 candidate mode에서 빠지고 unsupported auto-columns로 실패한다.
14. **positive pair 또는 signed fixed-px mode 상태가 음수 pair 선택 뒤 잘못 남는가?** — style projection은 negative pair 전용 enum을 선택하고 후속 shape validation이 pair 자체를 확인한다. 기존 positive/signed regression과 대상 Clippy가 통과했다.
15. **음수 pair mode에서 허용 track grammar가 넓어졌는가?** — `auto`와 비어 있지 않은 fixed-px list만 match한다. Auto/음수/비유한 입력 및 unsupported track kind 테스트가 거부를 확인한다.
16. **leading fixed pattern의 반복 phase가 end 방향으로 잘못 적용되는가?** — track expansion은 leading list를 반전 적용한 뒤 CSS start-to-end 순서를 저장한다. span/reverse/offset 사례의 expected track metadata와 Chrome 관측이 맞는다.
17. **equal endpoints가 빈 영역으로 처리되는가?** — equal-line end를 제거해 CSS 기본 1-track span을 쓴다. layout unit test와 전용 matrix equal scene이 고정 frame을 반환한다.
18. **reversed endpoints가 음수 span이나 endpoint swap 오류를 만드는가?** — reversed input은 정규화 후 정상 순서로 확장된다. forward/reverse pair의 CSS expected values와 Chrome screenshot sample이 일치한다.
19. **auto track item 기여분이 전체 implicit extent에 중복 반영되는가?** — 18px item의 3-track contribution은 `[6,6,6]px`, span 밖 track은 0px다. Rows/Columns layout controls가 이를 확인한다.
20. **large occurrence가 정수 overflow 또는 자원 과다 할당을 유발하는가?** — signed extent arithmetic은 checked conversion/add/negation을 쓰고 common 10,000-track limit 초과를 반환한다. 10,001-track negative candidate test가 통과했다.
21. **실패 뒤 부분 frame이 호출자에게 노출되는가?** — shape/track validation 오류는 layout 결과 생성 전에 반환된다. unsupported-shape integration tests는 `Err`만 받고 output frame을 얻지 않는다.
22. **track expansion 결과의 row/column 축이 뒤바뀌는가?** — 같은 profile을 Columns와 Rows 양쪽에 연결했다. layout tests, projection tests와 app scene 모두 두 축의 item frame을 각각 고정한다.
23. **신규 FFI scene id가 임의 문자열을 평가하는 통로가 되는가?** — scene source 함수는 정확한 id만 route하고 알 수 없는 id는 오류 처리한다. Rust fixture test에서 suffix injection id를 거부한다.
24. **Android 실행 allowlist만 갱신되고 iOS route가 빠졌는가?** — Rust source, Android MainActivity/demo, iOS demo의 scene allowlist와 별도 pixel sample 표를 검사했고 두 앱 build 및 runtime 실행이 성공했다.
25. **런타임 frame 기대 배열이 wrapper나 node 순서와 어긋나는가?** — mobile fixture의 17-frame 배열은 observation wrapper와 각 8개 root/item 쌍으로 생성되며 JSON test가 node count와 배열 전체를 비교한다.
26. **PixelCopy/Metal sample이 대상 item 밖을 읽는가?** — 8개 표본 좌표는 각 item frame 내부에 있으며 Android PixelCopy와 iOS Metal drawable이 각각 8/8 exact, 최대 channel error 0을 기록했다.
27. **oracle screenshot/observation이 바뀌어도 fixture가 조용히 통과하는가?** — reference JSON은 observation와 screenshot SHA-256을 고정하고 fixture test가 실제 byte hash를 비교한다. 저장된 Chrome 156 screenshot 및 observation hash가 선언값과 일치한다.
28. **새 fixture payload가 기본 Android 앱 artifact로 새어 나가는가?** — 기본 linked ELF에는 새 export/payload가 없고 opt-in fixture-test positive control에는 둘 다 있다. 이 근거는 README에서 Android linked ELF에만 한정한다.
29. **플랫폼 화면 근거가 표면/실기기 검증으로 과장됐는가?** — Android는 API 37 ARM64 AVD/SwiftShader, iOS는 iPhone 17 Pro Simulator의 compositor 전 Metal drawable이라고 기록했다. hardware GPU, 실기기, compositor 이후 표면은 주장하지 않는다.
30. **부분 구현이 공식 완료 상태를 잘못 승격하는가?** — `spec/STATUS.md`의 C11 상위/C11.3.3 checkbox를 유지하고 `0059` 숫자 버전 `0.1.0`, 전체 집계 `72/213`을 유지했다. 계획/evidence는 일반 CSS admission, WPT 및 전체 C11 범위를 명시적으로 남겨 둔다.

## 추가 diff hygiene

`git diff --cached --check`에서 원본 compiler/platform raw `.log` 안의 trailing whitespace만 보고됐다. 원문 artifact는 정규화하지 않았고, 해당 `.log`를 제외한 staged text diff는 whitespace check를 통과했다. 원본 실패 AVD 기록도 그대로 포함한다. PR 생성 뒤에는 GitHub diff와 required check 상태를 다시 확인한다.
