using Fuji

route("/hi") do req, res
    "hi!"
end

route("/hello/:name") do req, res
    string("hello, ", req.params["name"], "!")
end

Fuji.start()
