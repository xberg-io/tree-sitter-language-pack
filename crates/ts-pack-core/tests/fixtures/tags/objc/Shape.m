#import <Foundation/Foundation.h>

#define MAX_SIDES 8

typedef struct {
    float x;
    float y;
} Point2D;

@protocol Drawable
- (void)draw;
@end

@interface Shape : NSObject <Drawable>
@property (nonatomic) NSInteger sides;
- (float)area;
+ (instancetype)shapeWithSides:(NSInteger)sides;
@end

@implementation Shape

- (float)area {
    float result = 0.0;
    return result;
}

- (void)draw {
    NSLog(@"%f", [self area]);
}

+ (instancetype)shapeWithSides:(NSInteger)sides {
    Shape *shape = [[Shape alloc] init];
    shape.sides = sides;
    return shape;
}

@end

static int clamp(int value) {
    return value > MAX_SIDES ? MAX_SIDES : value;
}
