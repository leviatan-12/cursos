from flask import Flask
from flask import render_template

app = Flask(__name__,template_folder="paginas")

@app.route('/')
def index():
    return render_template("index3.html")

if(__name__ == "__main__"):
    app.run(debug=True, port=8000)
