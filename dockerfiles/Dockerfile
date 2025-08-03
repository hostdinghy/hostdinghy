## Build
FROM soerenmeier/chuchi-build

COPY --chown=build . .

# first build the ui
WORKDIR /home/build/studio-webui

RUN npm ci
RUN npm run build

# now build the server
WORKDIR /home/build/studio-server

RUN cargo b --release

## release
FROM soerenmeier/chuchi-release

COPY --chown=release --from=0 /home/build/target/release/studio-server .
COPY --chown=release --from=0 /home/build/studio-webui/dist dist

CMD ["./studio-server"]
