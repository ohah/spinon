# Android corrected CSS input, before matcher correction

This is not the final passing run. The corrected scene completed layout with the expected root/item frames and reached draw/present (`boxes=3`), but the then-current app matcher rejected the real trace. The runtime output showed `layoutInputRevision.intrinsicSize=0`, collapsed outer gutters, repeated intrinsic contributions, and row contributions measured at each assigned column width. Those observations were used to update the plan and both native matchers. The final rebuilt matrix must pass under `android-avd/` and `ios-simulator/`.
