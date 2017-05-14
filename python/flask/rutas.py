from flask import Flask
from flask import request

app = Flask(__name__)

@app.route('/')
def index():
    return "Hola mundo"

#Para definir paramentos que sean con signo de interrogación saluda?name=XXX
"""
@app.route('/saluda')
def saludo_one():
    name = request.args.get('name','No se ha encontrado nombre')
    return 'Otro mensaje para {}'.format(name)
"""
#Para definir paramentos que sean con diagonal saluda/XXX
#La primera línea es para que acepte la url sin la diagonal y no salga error, se debe poner a la variable un valor por default
#Para obtener parámetros sin que sean necesariamente String, se agrega: <tipo:nombre>
@app.route('/saluda/')
@app.route('/saluda/<name>/')
@app.route('/saluda/<name>/<int:num>/')
def saludo(name='default',num=0):
    num *=2
    return 'Otro mensaje para {} {}'.format(name,num)

if(__name__ == "__main__"):
    app.run(debug=True, port=8000)
