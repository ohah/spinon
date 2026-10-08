import MetalKit
import QuartzCore

func checkDrawablePresentationAPI(_ drawable: any CAMetalDrawable) {
    _ = drawable.drawableID
    drawable.addPresentedHandler { presentedDrawable in
        _ = presentedDrawable.presentedTime
    }
}

func checkPresentedTime(_ drawable: any CAMetalDrawable) {
    _ = drawable.presentedTime
}
