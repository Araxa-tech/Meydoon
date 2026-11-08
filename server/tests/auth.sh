#!/usr/bin/env bash

set -xe

# curl -s -X POST localhost:8000/api/auth/register -H 'Content-Type: application/json' \
#   -d '{"email":"a@b.com","user_name":"alice","phone_number":"09120000000","password":"secret123"}'
# echo

TOKEN=$(curl -s -X POST localhost:8000/api/auth/login -H 'Content-Type: application/json' \
  -d '{"identifier":"alice","password":"secret123"}' | jq -r .token)

curl -s localhost:8000/api/auth/me -H "Authorization: Bearer $TOKEN"
echo

# curl -s -X POST localhost:8000/api/auth/forgot-password -H 'Content-Type: application/json' \
#   -d '{"email":"a@b.com"}'
# echo
#
# curl -s -X POST localhost:8000/api/auth/reset-password -H 'Content-Type: application/json' \
#   -d '{"token":"<reset_token>","new_password":"newsecret123"}'
# echo

# logout
curl -s -X POST localhost:8000/api/auth/logout -H "Authorization: Bearer $TOKEN"
echo

# /me should now return 401
curl -s -o /dev/null -w "%{http_code}\n" localhost:8000/api/auth/me \
  -H "Authorization: Bearer $TOKEN"
