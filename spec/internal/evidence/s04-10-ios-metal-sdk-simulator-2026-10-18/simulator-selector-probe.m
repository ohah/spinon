#import <UIKit/UIKit.h>
#import <Metal/Metal.h>
#import <QuartzCore/QuartzCore.h>

@interface ProbeViewController : UIViewController
@end

@implementation ProbeViewController
- (void)viewDidAppear:(BOOL)animated {
    [super viewDidAppear:animated];
    dispatch_after(dispatch_time(DISPATCH_TIME_NOW, (int64_t)(0.5 * NSEC_PER_SEC)), dispatch_get_main_queue(), ^{
        CAMetalLayer *layer = [CAMetalLayer layer];
        layer.frame = self.view.bounds;
        layer.contentsScale = self.view.traitCollection.displayScale;
        layer.device = MTLCreateSystemDefaultDevice();
        [self.view.layer addSublayer:layer];
        id<CAMetalDrawable> drawable = [layer nextDrawable];
        SEL handler = NSSelectorFromString(@"addPresentedHandler:");
        SEL time = NSSelectorFromString(@"presentedTime");
        SEL identifier = NSSelectorFromString(@"drawableID");
        NSLog(@"SPINON_METAL_RUNTIME drawable=%@ addPresentedHandler=%d presentedTime=%d drawableID=%d",
              drawable ? NSStringFromClass([(id)drawable class]) : @"nil",
              drawable ? [(id)drawable respondsToSelector:handler] : 0,
              drawable ? [(id)drawable respondsToSelector:time] : 0,
              drawable ? [(id)drawable respondsToSelector:identifier] : 0);
    });
}
@end

@interface ProbeAppDelegate : UIResponder <UIApplicationDelegate>
@property(nonatomic, strong) UIWindow *window;
@end

@implementation ProbeAppDelegate
- (BOOL)application:(UIApplication *)application didFinishLaunchingWithOptions:(NSDictionary *)launchOptions {
    self.window = [[UIWindow alloc] initWithFrame:UIScreen.mainScreen.bounds];
    self.window.rootViewController = [ProbeViewController new];
    [self.window makeKeyAndVisible];
    return YES;
}
@end

int main(int argc, char *argv[]) {
    @autoreleasepool {
        return UIApplicationMain(argc, argv, nil, NSStringFromClass(ProbeAppDelegate.class));
    }
}
