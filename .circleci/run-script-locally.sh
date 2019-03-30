#!/usr/bin/env bash
curl --user ${CIRCLE_TOKEN}: \
    --request POST \
    --form tag=0.3.3\
    --form config=@config.yml \
    --form notify=false \
        https://circleci.com/api/v1.1/project/github/arranf/HugoToJSON/