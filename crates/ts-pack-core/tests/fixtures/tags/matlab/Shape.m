classdef Shape < handle & matlab.mixin.Copyable
    % A shape with a name and a scale factor.
    properties
        Name = "shape"
        Scale (1,1) double = 1
    end

    properties (Constant)
        Sides = 0
    end

    events
        Resized
    end

    enumeration
        Small (1)
        Large (10)
    end

    methods
        function obj = Shape(name)
            obj.Name = name;
        end

        function r = area(obj, x)
            r = square(x) * pi * obj.Scale;
        end

        function resize(obj, factor)
            obj.Scale = obj.Scale * factor;
            notify(obj, 'Resized');
            disp done
        end
    end

    methods (Static)
        function s = unit()
            s = Shape("unit");
        end
    end
end

function y = square(x)
    y = x .^ 2;
end
