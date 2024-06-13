#!/usr/bin/env bash

cd /home/chin/Projects/react-arborist

pnpm run build

cd /home/chin/Projects/chnots/chnots-web

find -name 'file*react-arborist*' -exec rm -rf {} \;
pnpm install

pnpm run dev
