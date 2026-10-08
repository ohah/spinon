import QuartzCore

final class SpinonProbeMetalLayer: CAMetalLayer {
    override func nextDrawable() -> (any CAMetalDrawable)? {
        let drawable = super.nextDrawable()
        #if !targetEnvironment(simulator)
        drawable?.addPresentedHandler { presentedDrawable in
            _ = presentedDrawable.drawableID
            _ = presentedDrawable.presentedTime
        }
        #endif
        return drawable
    }
}
