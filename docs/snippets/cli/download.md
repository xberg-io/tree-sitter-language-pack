```bash title="CLI"
# Download specific languages
ts-pack download python javascript rust go

# Download all available languages
ts-pack download --all

# Download a language group
ts-pack download --groups web,systems

# Fresh download (clear cache first)
ts-pack download --fresh python

# Check what's cached
ts-pack list --downloaded
```
