#!/bin/bash

cd "$(dirname "$0")" || exit 1

touch .env && chmod 600 .env

while read -r line; do
    [[ $line =~ ^(#.*|)$ ]] && continue
    key="${line%%=*}"
    value="${line#*=}"
    if [[ -z "${value}" ]]; then
        value="$(gcloud secrets versions access latest --secret="${key}")"
    fi
    eval "${key}=${value}"
    echo "${key}=${!key}"
done < .env.tmpl > .env
chmod 400 .env
