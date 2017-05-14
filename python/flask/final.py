from flask import (Flask,render_template,request,make_response)
import form

app = Flask(__name__,template_folder="paginas")

@app.route("/",methods = ['GET','POST'])
def index():
    comment_form = form.CommentForm(request.form)
    cookie_custom = request.cookies.get('custom')
    print(cookie_custom)
    if(request.method=='POST' and comment_form.validate()):
        sesion['username'] = login_form.username.data
    return render_template("index3.html", form=comment_form)

@app.route("/usuario")
def usuario():
    return render_template("usuario.html")

@app.route("/cookie")
def cookie():
    respuesta = make_response(render_template("cookie.html"))
    respuesta.set_cookie('custom','ricardo')
    return respuesta

if(__name__=="__main__"):
    app.run(debug=True,port=8000)
