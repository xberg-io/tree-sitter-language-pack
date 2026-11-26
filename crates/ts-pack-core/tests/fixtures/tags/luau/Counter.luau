--!strict
local Signal = require("./Signal")

local Counter = {}
Counter.__index = Counter
Counter.VERSION = "1.0"

type CounterState = {
	count: number,
}

export type Step = number

local MAX_COUNT = 10
local instances: number = 0
local lastStep: Step?

function Counter.new(): CounterState
	instances += 1
	return setmetatable({ count = 0 }, Counter)
end

function Counter:increment(step: Step)
	self.count = math.min(self.count + step, MAX_COUNT)
	lastStep = step
end

Counter.reset = function(counter)
	counter.count = 0
end

local function report(counter)
	print(counter.count)
end

local format = function(counter): string
	return tostring(counter.count)
end

local handlers = {
	onChange = function(counter)
		report(counter)
	end,
}

debugMode = false

local counter = Counter.new()
counter:increment(1)
report(counter)
Signal.fire(handlers, format(counter))

return Counter
