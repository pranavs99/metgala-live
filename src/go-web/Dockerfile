# indicate base image for Golang runtime
FROM golang:1.21

# cd into /app directory in Docker container (default for running)
WORKDIR /app

# copy all files at go-web/ level into container's /app
COPY . ./

# build Go code into binary named "mg-web-test"
RUN go build -o /mg-web-test

# expose port 8081, which we've configured to receive HTTP(S) requests
EXPOSE 8081:8081

# run Go binary
CMD ["/mg-web-test"]