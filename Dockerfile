FROM redhat/ubi9-minimal:latest

WORKDIR /usr/src/nakitin
COPY ./target/release/nakitin .

RUN chmod +x nakitin

CMD ["./nakitin"]
