FROM rust:latest

# Install Heroku CLI
RUN apt-get update && \
    apt-get install -y curl gnupg software-properties-common && \
    curl -sL https://deb.nodesource.com/setup_18.x | bash - && \
    apt-get install -y nodejs && \
    curl https://cli-assets.heroku.com/install.sh | sh

WORKDIR /usr/src/app

COPY . .

CMD ["cargo", "run"]
