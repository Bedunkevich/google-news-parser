# Use an existing Rust image as the base
FROM  --platform=linux/amd64 rust:latest

# Set the working directory
WORKDIR /app

# Copy the application files into the image
COPY . .

RUN wget -q -O - https://dl-ssl.google.com/linux/linux_signing_key.pub | apt-key add -
RUN sh -c 'echo "deb http://dl.google.com/linux/chrome/deb/ stable main" >> /etc/apt/sources.list.d/google.list'
RUN apt-get update -qqy --no-install-recommends && apt-get install -qqy --no-install-recommends google-chrome-stable
RUN apt-get install -y dbus
RUN service dbus start

ENV DBUS_SESSION_BUS_ADDRESS autolaunch:

RUN cargo build --release

# Set the command to run the binary
CMD ["./target/release/fetcher"]