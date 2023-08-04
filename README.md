# Stacked Fourside

This is a connect-4 web game where pieces can be stacked from either side. The backend is a Rust webserver backed by a Postgres database and the frontend is written in React.

# Getting started

## Local deploy

Using [docker-compose](https://docs.docker.com/compose/), you can build and start a local development instance by running

```bash
$ docker-compose up --build
```

This will start the service listening on `localhost:4321`.

This is a release build, which can take a while to create, but it will run locally.

## Deploying to prod

In order to start and run a build on production, you must supply the `BASE_URL` of the location where the game will ultimately run, for example,

```bash
BASE_URL=fourside.jordigh.com docker-compose up --build
```

and you must have websockets and TLS properly set up at the domain you specified. As an example, here is a certbot-managed nginx configuration:

```nginx
server {
    server_name fourside.jordigh.com;

	  location / {
        proxy_pass         http://127.0.0.1:4321/;
        proxy_redirect     off;

        proxy_set_header   Host             $host;
        proxy_set_header   X-Real-IP        $remote_addr;
        proxy_set_header   X-Forwarded-For  $proxy_add_x_forwarded_for;

        # WebSocket support
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
	  }

    listen 445 ssl; # managed by Certbot
    ssl_certificate /etc/letsencrypt/live/fourside.jordigh.com/fullchain.pem; # managed by Certbot
    ssl_certificate_key /etc/letsencrypt/live/fourside.jordigh.com/privkey.pem; # managed by Certbot
    include /etc/letsencrypt/options-ssl-nginx.conf; # managed by Certbot
    ssl_dhparam /etc/letsencrypt/ssl-dhparams.pem; # managed by Certbot

}
server {
    if ($host = fourside.jordigh.com) {
        return 301 https://$host$request_uri;
        } # managed by Certbot

        listen 80;
        server_name fourside.jordigh.com;
        return 404; # managed by Certbot
}
```

## Faster prototyping

Since the full release build takes a long time, it is faster to prototype as follows:

1. Install Rust 1.71 and node 20.
2. Install a Postgres server on localhost, and create a user that has database creation permissions.
3. Run `cd frontend && npm ci && npm start` to start the frontend
4. In a separate termainal, run `RUST_BACKTRACE=1 RUST_LOG=warp DATABASE_URL=postgresql://user:password@localhost cargo run` to start the backend.
5. The web frontend will automatically reload if you make changes, but if you modify the backend, you'll have to stop it by hitting `Ctrl-C` and re-running the command above.

# How to play

Point a web browser to the URL where you deployed the game. Ask a friend or open another web browser instance to the same URL. Stacked Foursdie will automatically create new games and pair players with waiting players. If a player leaves or disconnects, however, games will linger forever until they are over by a win, lose, or draw.

There is absolutely no authentication, so you can also play against yourself or even log in as your opponent and make moves for them.

# Details of design

The Rust backend uses [warp](https://docs.rs/warp/latest/warp/) for websockets as well as for serving static file in the production build. For handling the database, it uses [Sea-ORM](https://github.com/SeaQL/sea-orm). The frontend is fairly vanilla React.
