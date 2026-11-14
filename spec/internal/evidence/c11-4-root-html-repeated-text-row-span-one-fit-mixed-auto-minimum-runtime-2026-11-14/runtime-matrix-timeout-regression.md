# Runtime matrix timeout regression

The initial 120,960-scene RuntimeSession run stopped after 1,006.03 seconds at scene 99449. Its six-row reverse-order topology used `fit-content(0%)`, `align-content:stretch`, and mixed automatic minima on items A and D. The assertion expected `Ready` but received `Pending`; the snapshot had no layout error.

The scene was added as a permanent regression test and run alone. It passed in 1.78 seconds, with all scene geometry and trace assertions intact. This isolated the failure to the matrix's 5-second asynchronous wait limit rather than a layout mismatch.

The matrix wait was raised to 30 seconds without changing scene inputs or result tolerances. The full run then completed all 120,960 scenes, all 900 track-list batches, and the final 169,344 eligible-item assertion in 2,633.00 seconds.
