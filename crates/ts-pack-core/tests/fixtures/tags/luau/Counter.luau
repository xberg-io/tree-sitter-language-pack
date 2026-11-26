--!strict
local Counter = {}
Counter.__index = Counter

type CounterState = {
	count: number,
}

export type Step = number

local MAX_COUNT = 10

function Counter.new(): CounterState
	return setmetatable({ count = 0 }, Counter)
end

function Counter:increment(step: Step)
	self.count = math.min(self.count + step, MAX_COUNT)
end

local function report(counter)
	print(counter.count)
end

local counter = Counter.new()
counter:increment(1)
report(counter)

return Counter
