FROM python:3
# FROM rust:1.23.0
COPY . /rusty
WORKDIR /rusty
RUN pip install setuptools_rust 
RUN pip install .
ENTRYPOINT python -m hello_rusty.run
