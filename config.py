import os

DEBUG = False
TESTING = False
CSRF_ENABLED = True
SECRET_KEY = '2314f2w35r2wfe32'
SQLALCHEMY_TRACK_MODIFICATIONS = False
SQLALCHEMY_DATABASE_URI = os.environ["DATABASE_URL"]
