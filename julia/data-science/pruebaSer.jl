using HttpServer

sitio_basico = "<h2>Hola mundo</h2><p>Creo que ya se como crear un sitio básico en Julia</p>"
pagina_no_encontrada = "<h2>Lo sentimos</h2><p>La página que buscas no fue encontrada</p>"

function responder(req::String)

    if ismatch(r"^/hola/",req)
        return string("Hello ", split(req,'/')[3], "!")
    elseif ismatch(r"^/",req) && size(split(req,""))[1]<=1
        return sitio_basico
    end
    return pagina_no_encontrada
end

http = HttpHandler() do req::Request, res::Response

    # Response( ismatch(r"^/hello/",req.resource) ? string("Hello ", split(req.resource,'/')[3], "!") : pagina_no_encontrada )
    Response(responder(req.resource))
    # Response( ismatch(r"^/dinero",req.resource) ? sitio_basico : pagina_no_encontrada)
end

server = Server( http )
run( server, 8080 )
# or
# run(server, host=IPv4(127,0,0,1), port=8080)
