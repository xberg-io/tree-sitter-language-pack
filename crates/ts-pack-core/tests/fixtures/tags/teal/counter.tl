local record Counter
   count: integer
   increment: function(self: Counter, step: integer): integer
end

local enum Color
   "red"
   "green"
end

local type Step = integer

local MAX_COUNT <const>: integer = 10

local function clamp(value: integer): integer
   return math.min(value, MAX_COUNT)
end

function Counter:increment(step: Step): integer
   self.count = clamp(self.count + step)
   return self.count
end

function Counter.new(): Counter
   return setmetatable({ count = 0 }, { __index = Counter })
end

local counter = Counter.new()
print(counter:increment(2))

return Counter
