FROM python:3.10.1-alpine
WORKDIR /usr/src/app

ENV FLASK_APP=app.py
ENV FLASK_RUN_HOST=0.0.0.0

ENV FLASK_ENV=production
COPY requirements.txt requirements.txt

RUN pip install -r requirements.txt
COPY app.py app.py
COPY config.py config.py

CMD ["flask", "run"]
