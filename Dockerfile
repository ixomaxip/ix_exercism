FROM python:3.8.12-buster

RUN pip install gdbgui

ENV PORT=5555
EXPOSE ${PORT}

CMD gdbgui -r -n --port ${PORT}

# FROM alpine:3.15.0

# ENV PORT=5555

# RUN apk --no-cache add --virtual runtime-dependencies \
#       python3 \
#       gdb &&\
#     apk --no-cache add --virtual build-dependencies \
#       build-base \
#       python3-dev \
#       py-pip &&\
#     python -m pip install --upgrade pip &&\
#     pip3 install gdbgui &&\
#     apk del --purge build-dependencies &&\
#     rm -rf /var/cache/apk/* 

# EXPOSE ${PORT}

# CMD gdbgui -r -n --port ${PORT}