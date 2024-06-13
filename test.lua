local ACTIONS = {}
local frost = {}


---@param name string
function frost.action(name)
  ---@param action function
  return function(action)
    ACTIONS[name] = action
  end
end

---@param name string
function frost.invoke(name)
  local action = ACTIONS[name]
  if not action then
    return error("Unknown action: " .. name)
  end
  ---@param args table
  return function(args)
    action(args)
  end
end

frost.action ":greet" (function(args)
  local name = args[1]
  print("Hello " .. name)
end)

frost.invoke ":greet" { "World" }
