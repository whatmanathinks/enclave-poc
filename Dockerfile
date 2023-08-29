FROM amazonlinux:2

RUN yum update -y && \
    yum install -y openssl

COPY target/x86_64-unknown-linux-musl/release/nitro_enclave_app /bin/

CMD ["/bin/nitro_enclave_app"]

