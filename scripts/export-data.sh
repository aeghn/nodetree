#!/usr/bin/env bash

mkdir -p $HOME/nodetree-back && cd $HOME/nodetree-back || exit 1

PGPASSWORD="nodetree" pg_dump -h 127.0.0.1 -p 5432 -U nodetree -d nodetree -f $HOME/nodetree-back/dump-$(date +"%y%m%d_%H%M%S").sql
