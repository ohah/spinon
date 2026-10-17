# C13.1 fixture artifact 경계

대상은 spinon-ffi static archive의 C13.1 full viewport hit oracle marker와 payload다. 이 검사는 fixture test feature가 기본 artifact에 유입되지 않는지 negative control과 opt-in positive control을 분리해 확인한다. 결과는 static archive에 한정되며 Android/iOS 최종 bundle 크기나 release 산출물 전체를 말하지 않는다.

## 실행

~~~sh
cargo build --locked -p spinon-ffi --lib
strings target/debug/libspinon_ffi.a | rg -F -e 'hit_identity_grid_summary_json=' -e '279364905336ef53d225c781bae68ec26d3fe4d50cfc7d0cfce689ab5354eee2' -e '866073e0c6f092d4b2459fa3b5cfd2ca38dfb8935f582ee3c9614dba3bfc7483' -e 'SPINON_C131_STACKING_FULL_VIEWPORT_HIT_IDENTITY_GRID_SUMMARY'
cargo build --locked -p spinon-ffi --lib --features c13-1-overflow-clip-fixture-test
strings target/debug/libspinon_ffi.a | rg -F -e 'hit_identity_grid_summary_json=' -e '279364905336ef53d225c781bae68ec26d3fe4d50cfc7d0cfce689ab5354eee2' -e '866073e0c6f092d4b2459fa3b5cfd2ca38dfb8935f582ee3c9614dba3bfc7483' -e 'SPINON_C131_STACKING_FULL_VIEWPORT_HIT_IDENTITY_GRID_SUMMARY'
~~~

strings 실행을 archive마다 따로 하고 rg 출력만 보존했다. 이 파일의 match_count=0은 rg가 아무 signature도 찾지 못했다는 뜻이다.

## 결과

| Build | 결과 | 보존 로그 |
|---|---|---|
| 기본 spinon-ffi | 성공, 새 marker/oracle/digest signature 0개 | [build](./default-build.log) · [strings negative control](./default-strings-check.txt) |
| c13-1-overflow-clip-fixture-test opt-in | 성공, 4 signature match | [build](./fixture-opt-in-build.log) · [strings positive control](./fixture-opt-in-strings-check.txt) |

공통 target/debug/libspinon_ffi.a 경로를 사용하지만 각 build 직후 archive를 즉시 검사하고, opt-in build는 두 번째 실행으로 기본 archive 결과를 덮어쓴 뒤 positive control을 확인했다. 이 static archive 결과를 최종 앱 bundle이나 release artifact에 확대하지 않는다.
