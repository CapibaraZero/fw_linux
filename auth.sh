#!/bin/sh

METHOD="$1"
MAC="$2"

case "$METHOD" in
  auth_client)
    USERNAME="$3"
    PASSWORD="$4"
    echo "Client logged(${MAC}): ${USERNAME} :: ${PASSWORD}" >> /home/capibarazero/log.txt
    echo 3600 0 0
    exit 0
    ;;
  client_auth|client_deauth|idle_deauth|timeout_deauth|ndsctl_auth|ndsctl_deauth|shutdown_deauth)
    INGOING_BYTES="$3"
    OUTGOING_BYTES="$4"
    SESSION_START="$5"
    SESSION_END="$6"

    echo "Client ${MAC} deauthenticated" >> /home/capibarazero/log.txt
    # client_auth: Client authenticated via this script.
    # client_deauth: Client deauthenticated by the client via splash page.
    # idle_deauth: Client was deauthenticated because of inactivity.
    # timeout_deauth: Client was deauthenticated because the session timed out.
    # ndsctl_auth: Client was authenticated by the ndsctl tool.
    # ndsctl_deauth: Client was deauthenticated by the ndsctl tool.
    # shutdown_deauth: Client was deauthenticated by Nodogsplash terminating.
    ;;
esac
