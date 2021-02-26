# dns_exfil

## Prerequisites

* Have a domain name
* Have a server with a public IP (GCP, AWS, Azure works fine)

## How to use

1) We need to delegate the zone so that all DNS requests are routed to our server (I suggest using cloudflare)

2) Make sure port 53 is not used on your server (`sudo lsof -i :53`)

3) If something is running on port 53 (usually `systemd-resolve`) then do the following:

    * Stop `systemd-resolved` by running `sudo systemctl stop systemd-resolved`
    * Edit `/etc/systemd/resolved.conf` and uncomment and set `DNSStubListener=no` and `DNS=8.8.8.8`
    * Start `systemd-resolved` by running `sudo systemctl start systemd-resolved`

4) Run dns_exfil: `sudo dns_exfil`

5) Results will be written in the console

## More info

* https://hinty.io/devforth/dns-exfiltration-of-data-step-by-step-simple-guide/
