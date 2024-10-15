FROM debian:latest

COPY --chmod=777 bulletin-board-server /bin/

ENV BB_TMP_DIR="/data/tmp"

ENV BB_ACV_DIR="/data/acv"

ENV BB_LOG_FILE="/bulletin-board.log"

ENV BB_LISTEN_ADDR="0.0.0.0:7578"

EXPOSE 7578/tcp

ENTRYPOINT ["/bin/bulletin-board-server", "-d"]