FROM registry.fedoraproject.org/fedora:33

COPY helloworld-py /app

RUN dnf -y install python3-pip \
 && pip3 install /app

ENV FLASK_APP=helloworld.app

CMD ["flask", "run", "--host=0.0.0.0"]
