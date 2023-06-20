FROM python:3.11

# maybe only libgl1?
RUN apt-get update && apt-get install ffmpeg libsm6 libxext6  -y

RUN curl -sSL https://install.python-poetry.org | python -

COPY poetry.lock pyproject.toml /app/
WORKDIR /app
ENV PATH="$PATH:/root/.local/bin/"
RUN poetry install -n

COPY . /app