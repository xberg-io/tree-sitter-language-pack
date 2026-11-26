local M = {}

function M.greet(name)
  return "Hello " .. name
end

local function add(a, b)
  return a + b
end

function M:describe()
  return add(1, 2)
end

return M
