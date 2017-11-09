var http = require('http'),
  fs = require('fs');


/**
* Con esto se crea el servidor y se manda la respuesta que quiere enviar el servidor
*/
http.createServer(function(req,res){
  fs.readFile('./index.html',function(err,html){
    var htmlString = html.toString();
    var infoCambiada = {nombre:'Ricardo'};

    var resultado = htmlString.replace(/\{(.*?)\}/g,function(match,key){
      return infoCambiada[key];
    });

    res.write(resultado);
    res.end();
  });
}).listen(8080);
