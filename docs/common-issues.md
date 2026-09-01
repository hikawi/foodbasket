---
title: Common Issues
---

## Summary

Document to outline most common issues that you might face setting up this project,
as well as caveats or gotchas hit on the road while I was working on this.

## Issues

Issues list for navigating:

### Issue 1: Caddy failed to bind (Linux/MacOS)

Check if you have a LISTEN port already open on port 80:

```bash
sudo netstat -ltnp | grep ":80"
```

If you do have it, please close the program responsible.

Then, because ports under port 1024 are considered privileged ports, I think, so
we need to make an exception for `caddy`. You can also run `sudo caddy run` instead,
but I didn't want to slap `sudo` everywhere.

```bash
sudo setcap 'cap_net_bind_service=+ep' $(which caddy)
```

Now `task dev` should run fine.
