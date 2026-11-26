#import <Foundation/Foundation.h>

#define MAX_SIDES 8
#define CLAMP(x, lo, hi) ((x) < (lo) ? (lo) : ((x) > (hi) ? (hi) : (x)))

@class Canvas;

typedef struct {
    float x;
    float y;
} Point2D;

struct Size {
    float width;
    float height;
};

typedef enum {
    ShapeKindPolygon,
    ShapeKindCircle
} ShapeKind;

enum Fill { FillNone, FillSolid = 2 };

static const NSInteger kDefaultSides = 4;
NSString *shapeRegistryName = @"shapes";

@protocol Drawable <NSObject>
- (void)drawInContext:(Canvas *)canvas;
@optional
@property (nonatomic, readonly) NSString *label;
@end

@interface Shape : NSObject <Drawable>
@property (nonatomic) NSInteger sides;
- (float)area;
+ (instancetype)shapeWithSides:(NSInteger)sides;
@end

@interface Shape (Scaling)
- (void)scaleBy:(float)factor;
@end

@implementation Shape

- (float)area {
    float result = 0.0;
    return result;
}

- (void)drawInContext:(Canvas *)canvas {
    NSLog(@"%f", [self area]);
}

+ (instancetype)shapeWithSides:(NSInteger)sides {
    Shape *shape = [[Shape alloc] init];
    shape.sides = CLAMP(sides, 3, MAX_SIDES);
    return shape;
}

@end

@implementation Shape (Scaling)

- (void)scaleBy:(float)factor {
    [self setSides:(NSInteger)(self.sides * factor)];
}

@end

static int clamp_sides(int value) {
    int limit = MAX_SIDES;
    return value > limit ? limit : value;
}
