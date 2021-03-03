from socket import gethostname

from flask import Flask
app = Flask(__name__)


@app.route('/')
def hello_world():
    return f'Hello, world, from {gethostname()}!\n'
