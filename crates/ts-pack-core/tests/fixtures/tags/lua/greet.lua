local json = require("json")
local M = {}

local DEFAULT_NAME = "world"
local separator <const> = ", "
local greeted

M.VERSION = "1.0"
greet_count = 0

function M.greet(name)
  local message = "Hello" .. separator .. (name or DEFAULT_NAME)
  greet_count = greet_count + 1
  return message
end

local function add(a, b)
  return a + b
end

function M:describe()
  return add(1, 2)
end

M.encode = function(value)
  return json.encode(value)
end

local shout = function(text)
  return string.upper(text)
end

M.helpers = {
  trim = function(text)
    return (text:gsub("^%s+", ""))
  end,
}

function reset()
  greeted = nil
end

return M
