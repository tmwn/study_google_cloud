#!/bin/bash
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
