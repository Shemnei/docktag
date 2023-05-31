# docktag

## TODO

- [ ] [Command] Check for new version for all running containers
- [ ] [Other] Add support for more registries
	- Github (ghcr.io)
	- Google (gcr.io)
- [ ] [Other] Pretty output (colors, ...)
- [ ] [Other] Options for:
	- Amount of versions to return (e.g. latest 3)
	- Group by `pre` and/or `build metadata`
- [ ] [Other] Make container for checking with notification support
	- Examples: [`diun`](https://crazymax.dev/diun) / [`watchtower`](https://github.com/containrrr/watchtower)

## Non goals

- Automated container/image updates

## Check

- [ ] Check if `tokio` is really needed (lot of overhead + size)
