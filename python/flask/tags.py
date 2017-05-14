from flask import Flask
from flask import render_template

#app = Flask(__name__)
app = Flask(__name__,template_folder="paginas")

@app.route('/')
def index():
    return render_template('index.html')

@app.route('/user')
@app.route('/user/<name>')
def usuario(name='No name'):
    lista = range(1,10)
    lista_dos = {"esta","es","una","prueba"}
    return render_template('index2.html',nombre=name,list=lista,list_dos=lista_dos)

if(__name__ == "__main__"):
    app.run(debug=True, port=8000)
