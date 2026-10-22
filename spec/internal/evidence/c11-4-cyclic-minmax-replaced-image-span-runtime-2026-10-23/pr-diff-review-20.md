# C11.4 cyclic `minmax(calc())` replaced image PR diff 검토 20개

계획 검토와 별도로 구현 diff를 계층 간 불일치와 범위 누출 기준으로 검토했다.

1. **Fixture 코드가 기본 제품 빌드에 들어가는가?** 새 style/layout helper는 `c11-4-grid-track-sizing-fixture-test`로 제한되어 있다.
2. **Runtime 새 image adapter가 fixture feature 밖에 노출되는가?** 기존 C11.4 fixture module의 opt-in feature 경계를 그대로 사용한다.
3. **FFI scene builder가 임의 scene ID나 자유형 CSS를 받는가?** 네 개 static scene만 생성하고 미등록 ID는 오류다.
4. **Rust, FFI, capture script, Swift, Android Java harness의 scene ID가 다른가?** 다섯 계층에서 같은 네 exact ID를 사용하고 미등록 ID는 거부한다.
5. **Scene count를 추가한 뒤 전체 C11.4 matrix 검사가 갱신되지 않았는가?** capture matrix는 기존 267개에 네 scene을 더해 271개로 검사한다.
6. **forward/reverse의 source order가 key 또는 placement를 뒤섞는가?** scene builder는 A/B/C와 C/B/A 배열만 교체하고 ID·CSS geometry를 item key로 고정한다.
7. **HTML source와 runtime adapter가 같은 root/image topology를 기술하는가?** root ID, 직접 img 3개, fixed source URL, row/column placement가 일치한다.
8. **fixed PNG 교체를 자연 크기만으로 놓치는가?** precomparison은 저장소 PNG SHA-256을 기록하고 런타임은 고정 asset을 decode해 매 image의 크기를 검사한다. 시뮬레이터 표본도 지정 색과 경계를 확인한다.
9. **기존 image fixture root에 cyclic image IDs가 잘못 attach되는가?** FFI traversal은 네 새 root ID만 추가하고 item 수와 identity 배열도 scene별로 확인한다.
10. **scene 내 root 이외 node 수를 헷갈리는가?** 새 장면 root에는 image 3개만 허용하고 기존 anonymous-grid 장면은 이전 child 수를 유지한다.
11. **Test-only projection helper가 public 기능으로 보이는가?** public export 자체가 fixture-test feature에 묶여 있다.
12. **two-axis intrinsic 허용 플래그가 기존 image path에 기본 적용되는가?** 새 intrinsic root scene 외 경로는 모두 `false`로 초기화한다.
13. **계산 trace가 순서만 확인하고 intermediate values를 놓치는가?** style/layout와 capture validator가 Initialize, span batch, resolved basis와 final tracks를 별도로 검사한다.
14. **root-relative frame에서 reverse item node index를 잘못 연결하는가?** 기대 frame은 DOM index와 image identity를 묶어서 비교한다.
15. **definite scene에서 B의 x좌표가 intrinsic 값으로 복사되는가?** definite B는 14px, intrinsic B는 9px로 각각 고정한다.
16. **pixel sample이 border 바깥인지 안인지 혼동되는가?** root width의 마지막 border pixel과 root 바깥 첫 pixel을 별도 coordinate/color로 확인한다.
17. **모바일 adapter가 새 장면에 대해 track trace 없이 성공 처리하는가?** Swift와 Java가 exact pass count/axis/span/track base, intrinsic inputs, node frames, box count를 검증한다.
18. **Metal 표본이 compositor 이후 화면 동일성을 주장하는가?** evidence는 Metal drawable sample 결과만 기록하고 compositor/실기기 범위를 제외한다.
19. **Simulator binary가 검증한 source diff와 다르게 빌드되는가?** Android AVD와 iOS Simulator metadata가 동일 checkout, tree, tracked-diff SHA `3b761f6d…b69522e7` 및 플랫폼별 app hash를 기록한다.
20. **문서가 구현을 공식 완료 또는 전체 C11.4 지원으로 과장하는가?** `spec/STATUS.md`와 공식 `77/213`을 바꾸지 않고 두 simulator의 좁은 네 장면 범위, 실기기·compositor 제한, PR 검토/CI/merge 미완료를 유지한다.

**검토 결과:** 코드·adapter·capture의 scene 계약은 일치한다. 초기 root constraint와 Android harness의 세 누락은 runtime 검증 중 찾아 수정했다. Default/opt-in artifact 결과 및 Android PixelCopy/iOS Metal evidence를 PR 설명에 플랫폼별 한계와 함께 연결한다.
