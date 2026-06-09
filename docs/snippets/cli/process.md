```bash title="CLI"
# Parse and show S-expression
ts-pack parse main.py --language python

# Parse as JSON
echo "fn main() {}" | ts-pack parse - --language rust --format json

# Full code intelligence
ts-pack process src/app.py --language python --all

# Structure + imports only
ts-pack process src/app.py --structure --imports
```
