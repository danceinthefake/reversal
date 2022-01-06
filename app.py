import os
from flask import Flask
from flask_sqlalchemy import SQLAlchemy
from flask_migrate import Migrate

basedir = os.path.abspath(os.path.dirname(__file__))

app = Flask(__name__)
app.config.from_pyfile('config.py')

db = SQLAlchemy(app)
migrate = Migrate(app, db)

class Reversal(db.Model):
    __table__name = 'reversal'

    id = db.Column(db.Integer, primary_key=True)
    path = db.Column(db.String())
    result = db.Column(db.String())

    def __init__(self, path, result):
        self.path = path
        self.result = result

    def __repr__(self):
        return "<id {}>".format(self.id)



def reverse_text(text):
    text_l = list(text)
    text_l.reverse()
    rev = "".join(text_l)

    return rev

@app.route("/")
def welcome_reverse():
    return """
<h1>Welcome to emocleW.</h1>
<h2>When your request path become reversed.</h2>
<p>Try any path now</p>
"""

@app.route("/<text>")
def reverse_req(text):
    reversed = reverse_text(text)
    reversal = Reversal(path= text, result=reversed)
    db.session.add(reversal)
    db.session.commit()
    return """
<h1>htaP ruoY</h1>

<p>{}</p>
""".format(reversed)

if __name__ == '__main__':
    app.run()
