-- Method and Headers
wrk.method = "POST"
wrk.headers["Content-Type"] = "application/json"

-- Body generation
function generate_random_slug(n)
  local chars = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789'
  local slug = ""
  for i = 1, n do
    local idx = math.random(1, #chars)
    slug = slug .. chars:sub(idx, idx)
  end
  return slug
end

-- Request generator
function request()
  local rand_url = "https://example.com/" .. generate_random_slug(6)
  local body = string.format('{"url":"%s"}', rand_url)
  return wrk.format("POST", "/api", nil, body)
end
