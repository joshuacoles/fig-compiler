FROM shepmaster/rust-nightly:latest AS build

WORKDIR /compile
COPY . .

RUN cargo build

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

FROM aergus/latex:latest

EXPOSE 8000

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

COPY --from=build /compile/target/debug/fig-compiler /bin

CMD "/bin/fig-compiler"
